#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/inotify.h>
#include <limits.h>

#define BUF_LEN (10 * (sizeof(struct inotify_event) + NAME_MAX + 1))

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <directory_to_watch>\n", argv);
        return 1;
    }

    // 1. Initialize an inotify instance inside the Linux kernel
    int fd = inotify_init();
    if (fd < 0) {
        perror("inotify_init");
        return 1;
    }

    // 2. Watch for file creations, modifications, and deletions
    int wd = inotify_add_watch(fd, argv, IN_CREATE | IN_MODIFY | IN_DELETE);
    if (wd < 0) {
        perror("inotify_add_watch");
        close(fd);
        return 1;
    }

    printf("Watching '%s' for kernel file events (Press Ctrl+C to stop)...\n", argv);

    char buf[BUF_LEN];
    while (1) {
        // Program sleeps here using 0% CPU until the kernel delivers an event
        ssize_t len = read(fd, buf, BUF_LEN);
        if (len < 0) {
            perror("read");
            break;
        }

        for (char *ptr = buf; ptr < buf + len; ) {
            struct inotify_event *event = (struct inotify_event *)ptr;
            if (event->len > 0) {
                if (event->mask & IN_CREATE) printf("[CREATED]  %s\n", event->name);
                if (event->mask & IN_MODIFY) printf("[MODIFIED] %s\n", event->name);
                if (event->mask & IN_DELETE) printf("[DELETED]  %s\n", event->name);
            }
            ptr += sizeof(struct inotify_event) + event->len;
        }
    }

    inotify_rm_watch(fd, wd);
    close(fd);
    return 0;
}

