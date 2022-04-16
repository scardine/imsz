#!/usr/bin/env python3

from typing import List

import os

from os.path import dirname, abspath, join as joinpath, splitext

base_dir = dirname(abspath(__file__))
files = os.listdir(joinpath(base_dir, "testdata"))

buf: List[str] = [
"""
fn get_testdata(fname: &str) -> std::path::PathBuf {
    let mut path = std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().parent().unwrap().to_owned();
    path.push("testdata");
    path.push(fname);
    return path;
}
"""
]

for fname in sorted(files):
    ident = fname.replace('.', '_')
    path = f"testdata/{fname}"

    _, ext = splitext(fname)
    format = ext[1:].upper() if ext != ".exr" else "OpenEXR"

    buf.append(f"""
#[test]
fn {ident}() {{
    let info = imsz::imsz(get_testdata("{fname}"));
    match info {{
        Ok(info) => {{
            assert_eq!(info.format, imsz::ImFormat::{format});
            assert_eq!(info.width,  32);
            assert_eq!(info.height, 16);
        }}
        Err(error) => {{
            assert!(false, "{{}}", error);
        }}
    }}
}}
""")

with open(joinpath(base_dir, "tests", "ok_files.rs"), "w") as fp:
    fp.write('\n'.join(buf))
