#include <stdio.h>
#include <string.h>
#include <inttypes.h>

#include "imsz.h"

int main(int argc, char *argv[]) {
    int status = 0;

    for (int index = 1; index < argc; ++ index) {
        const char *fname = argv[index];
        ImInfo info = IMSZ_INIT;

        int error = imsz(fname, &info);
        switch (error) {
            case IMSZ_OK:
                printf("%s %s, %" PRIu64 " x %" PRIu64 "\n", fname, imsz_format_name(info.format), info.width, info.height);
                break;

           case IMSZ_ERR_IO:
                fprintf(stderr, "%s IO Error\n", fname);
                status = 1;
                break;

           case IMSZ_ERR_PARSER:
                fprintf(stderr, "%s Parser Error %s\n", fname, imsz_format_name(info.format));
                status = 1;
                break;

           case IMSZ_ERR_UNSUPPORTED:
                fprintf(stderr, "%s IO Error\n", fname);
                status = 1;
                break;

           default:
                fprintf(stderr, "%s %s\n", fname, strerror(error));
                status = 1;
                break;
        }
    }

    return status;
}
