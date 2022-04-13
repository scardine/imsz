#ifndef IMSZ_H
#define IMSZ_H
#pragma once

#include <stdint.h>

#ifdef __cpluspluc
extern "C" {
#endif

#if defined(_WIN32) || defined(_WIN64) || defined(__CYGWIN__)
    #ifdef IMSZ_STATIC
        #define IMSZ_EXPORT
    #else
        #define IMSZ_EXPORT __declspec(dllimport)
    #endif
#else
    #if (defined(__GNUC__) && __GNUC__ >= 4) || defined(__clang__)
        #define IMSZ_EXPORT __attribute__ ((visibility ("default")))
    #else
        #define IMSZ_EXPORT extern
    #endif
#endif

#define IMSZ_OK               0
#define IMSZ_ERR_IO          -1
#define IMSZ_ERR_PARSER      -2
#define IMSZ_ERR_UNSUPPORTED -3

#define IMSZ_GIF   1
#define IMSZ_PNG   2
#define IMSZ_BMP   3
#define IMSZ_JPEG  4
#define IMSZ_WEBP  5
#define IMSZ_QOI   6
#define IMSZ_PSD   7
#define IMSZ_XCF   8
#define IMSZ_ICO   9
#define IMSZ_AVIF 10
#define IMSZ_TIFF 11

#define IMSZ_INIT { .format = 0, .width = (uint64_t)0, .height = (uint64_t)0 }

typedef struct ImInfo {
    int format;
    uint64_t width;
    uint64_t height;
} ImInfo;

IMSZ_EXPORT int imsz(const char *fname, ImInfo *info_ptr);

#define IMSZ_FORMAT_NAMES (char*[]){ NULL, "gif", "png", "bmp", "jpeg", "webp", "qoi", "psd", "xcf", "ico", "avif", "tiff" }
#define imsz_format_name(format) ((format) <= 0 || (format) >= sizeof(IMSZ_FORMAT_NAMES) / sizeof(char*) ? "(unknown)" : (IMSZ_FORMAT_NAMES)[(format)])

#ifdef __cpluspluc
}
#endif

#endif
