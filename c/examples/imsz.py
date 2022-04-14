#!/usr/bin/env python3

import cffi

from typing import NamedTuple, Optional, Union
from enum import Enum
from os.path import join as join_path, abspath, dirname
from os import fsencode, PathLike

ffi = cffi.FFI()
ffi.cdef("""
typedef struct ImInfo {
    unsigned int format;
    uint64_t width;
    uint64_t height;
} ImInfo;
int imsz(const char *fname, ImInfo *info_ptr);
""")

libpath = join_path(dirname(abspath(__file__)), "..", "target", "debug" if __debug__ else "release", "libimsz.so")

_imsz = ffi.dlopen(libpath)

class ImFormat(Enum):
    GIF     =  1
    PNG     =  2
    BMP     =  3
    JPEG    =  4
    WEBP    =  5
    QOI     =  6
    PSD     =  7
    XCF     =  8
    ICO     =  9
    AVIF    = 10
    TIFF    = 11
    OpenEXR = 12

class ImInfo(NamedTuple):
    format: ImFormat
    width:  int
    height: int

class ImError(Exception):
    __slots__ = ()

class IOError(ImError):
    __slots__ = 'errno',
    errno: Optional[int]

    def __init__(self, errno: Optional[int]=None) -> None:
        super().__init__()
        self.errno = errno

class ParserError(ImError):
    __slots__ = 'format',
    format: ImFormat

    def __init__(self, format: ImFormat) -> None:
        super().__init__()
        self.format = format

class UnknownFormat(ImError):
    __slots__ = ()

def imsz(path: Union[str, bytes, PathLike]) -> ImInfo:
    info_ptr = ffi.new("ImInfo[]", 1)
    result = _imsz.imsz(fsencode(path), info_ptr)

    info = info_ptr[0]
    if result == 0:
        return ImInfo(ImFormat(info.format), info.width, info.height)

    if result == -1:
        raise IOError()
    elif result == -2:
        raise ParserError(info.format)
    elif result == -3:
        raise UnknownFormat()
    elif result > 0:
        raise IOError(result)
    else:
        raise ImError()

if __name__ == '__main__':
    import sys
    for fname in sys.argv[1:]:
        try:
            info = imsz(fname)
        except Exception as error:
            print(f"{fname}: {error}", file=sys.stderr)
        else:
            print(f"{fname}: {info.format.name.lower()}, {info.width} x {info.height}")
