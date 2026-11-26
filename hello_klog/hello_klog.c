#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Debian Learner");
MODULE_DESCRIPTION("A minimal kernel module that writes to the kernel log");

static int __init hello_klog_init(void) {
    pr_info("hello_klog: Module loaded into Debian 13 kernel!\n");
    return 0;
}

static void __exit hello_klog_exit(void) {
    pr_info("hello_klog: Module unloaded cleanly.\n");
}

module_init(hello_klog_init);
module_exit(hello_klog_exit);
