#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/ptrace.h>
#include <sys/wait.h>
#include <sys/user.h>

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <command> [args...]\n", argv);
        return 1;
    }

    pid_t child = fork();
    if (child == 0) {
        // Child process: allow parent to trace us, then execute target command
        ptrace(PTRACE_TRACEME, 0, NULL, NULL);
        execvp(argv, &argv);
        perror("execvp");
        exit(1);
    } else if (child > 0) {
        int status;
        long syscall_count = 0;
        int in_syscall = 0;

        // Wait for child's initial stop on execvp
        waitpid(child, &status, 0);

        while (!WIFEXITED(status)) {
            // Resume child and tell kernel to stop it at the next syscall entry/exit
            ptrace(PTRACE_SYSCALL, child, NULL, NULL);
            waitpid(child, &status, 0);

            if (WIFSTOPPED(status)) {
                // Toggle between syscall entry and syscall exit
                if (!in_syscall) {
                    struct user_regs_struct regs;
                    ptrace(PTRACE_GETREGS, child, NULL, &regs);
                    syscall_count++;

                    if (syscall_count <= 10) {
                        // On x86_64 Linux, orig_rax holds the system call ID number
                        printf("Syscall #%ld -> Kernel ID: %llu\n",
                               syscall_count, (unsigned long long)regs.orig_rax);
                    }
                }
                in_syscall = !in_syscall;
            }
        }

        printf("Total system calls made by '%s': %ld\n", argv, syscall_count);
    } else {
        perror("fork");
        return 1;
    }

    return 0;
}

