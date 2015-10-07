#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/device.h>
#include <linux/slab.h>
#include <linux/bug.h>
#include <linux/fs.h>

/* Rust functions*/
extern void rust_main(void);
extern ssize_t device_read(struct file *, char *, size_t, loff_t *);
extern ssize_t device_write(struct file *, const char *, size_t, loff_t *);

char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

/*  Prototypes - this would normally go in a .h file */
static int device_open(struct inode *, struct file *);
static int device_release(struct inode *, struct file *);

#define SUCCESS 0
#define DEVICE_NAME "mmcc" /* Dev name as it appears in /proc/devices	*/
#define CLASS_NAME "mmcc_char"
#define BUF_LEN 80				/* Max length of the message from the device */


/* Global variables are declared as static, so are global within the file. */

static int major;				/* major number assigned to our device driver */
static int Device_Open = 0;  /* Is device open?  Used to prevent multiple
													 access to the device */
static char msg[BUF_LEN];	 /* The msg the device will give when asked	 */
static char *msg_Ptr;

static struct class*  mmcc_class  = NULL; ///< The device-driver class struct pointer
static struct device* mmcc_dev = NULL; ///< The device-driver device struct pointer

static struct file_operations fops = {
  .read = device_read,
  .write = device_write,
  .open = device_open,
  .release = device_release
};


void
abort(void) {
	 BUG();
}

void
*k_malloc(size_t sz) {
	 return kmalloc(sz, GFP_KERNEL);
}

void
k_free(void *ptr) {
	 kfree(ptr);
}

void *
k_realloc(void *ptr, size_t sz) {
	 return krealloc(ptr, sz, GFP_KERNEL);
}

static int
ffs_init(void) {
	 printk("initializing device...\n");
	 major = register_chrdev(0, DEVICE_NAME, &fops);

	 if (major < 0) {
		  printk ("Registering the character device failed with %d\n", major);
		  return major;
	 }
	 printk("done!\n");
	 rust_main();
	// Register the device class
	mmcc_class = class_create(THIS_MODULE, CLASS_NAME);
	if (IS_ERR(mmcc_class)){					 // Check for error and clean up if there is
		unregister_chrdev(major, DEVICE_NAME);
		printk(KERN_ALERT "Failed to register device class\n");
		return PTR_ERR(mmcc_class);			 // Correct way to return an error on a pointer
	}
	printk(KERN_INFO "mmcc_char device class registered correctly\n");
 
	// Register the device driver
	mmcc_dev = device_create(mmcc_class, NULL, MKDEV(major, 0), NULL, DEVICE_NAME);
	if (IS_ERR(mmcc_dev)){					// Clean up if there is an error
		class_destroy(mmcc_class);			  // Repeated code but the alternative is goto statements
		unregister_chrdev(major, DEVICE_NAME);
		printk(KERN_ALERT "Failed to create the device\n");
		return PTR_ERR(mmcc_dev);
	}
	printk(KERN_INFO "mmcc device class created correctly\n"); // Made it! device was initialized
	return 0;
}

static int device_open(struct inode *i, struct file *fl) {
	static int counter = 0;
	if (Device_Open) return -EBUSY;

	Device_Open++;
	sprintf(msg,"I already told you %d times Hello world!\n", counter++);
	msg_Ptr = msg;
	//MOD_INC_USE_COUNT;

	return SUCCESS;
}

static int device_release(struct inode *i, struct file *fl) {
	Device_Open --;	  /* We're now ready for our next caller */

	/* Decrement the usage count, or else once you opened the file, you'll
						  never get get rid of the module. */
	//MOD_DEC_USE_COUNT;

	return 0;
}


static void
ffs_exit(void) {
	unregister_chrdev(major, DEVICE_NAME);
	printk(KERN_INFO "ffs: exit\n");
	device_destroy(mmcc_class, MKDEV(major, 0));	  // remove the device
	class_unregister(mmcc_class);								  // unregister the device class
	class_destroy(mmcc_class);									  // remove the device class
	unregister_chrdev(major, DEVICE_NAME);				 // unregister the major number
	printk(KERN_INFO "mmcc: Goodbye from the LKM!\n");
}

module_init(ffs_init);
module_exit(ffs_exit);

MODULE_LICENSE("GPL\0(but actually ISC)");
