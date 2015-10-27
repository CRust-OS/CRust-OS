#include <stdint.h>
#define __STR(x) #x
#define STR(x) __STR(x)
#define _hypercall2(type, name, a1, a2)				\
({								\
	long __res, __ign1, __ign2;				\
	asm volatile (						\
		"call hypercall_page + ("STR(__HYPERVISOR_##name)" * 32)"\
		: "=a" (__res), "=D" (__ign1), "=S" (__ign2)	\
		: "1" ((long)(a1)), "2" ((long)(a2))		\
		: "memory" );					\
	(type)__res;						\
})


#define EVTCHNOP_send             4
typedef uint32_t evtchn_port_t;
/*
 * EVTCHNOP_send: Send an event to the remote end of the channel whose local
 * endpoint is <port>.
 */
struct evtchn_send {
    /* IN parameters. */
    evtchn_port_t port;
};
typedef struct evtchn_send evtchn_send_t;

static inline int HYPERVISOR_event_channel_op(int cmd, void *op) {
    return _hypercall2(int, event_channel_op, cmd, op);
}

static inline int notify_remote_via_evtchn(evtchn_port_t port)
{
    evtchn_send_t op;
    op.port = port;
    return HYPERVISOR_event_channel_op(EVTCHNOP_send, &op);
}


static inline void notify_daemon(struct consfront_dev *dev)
{
    /* Use evtchn: this is called early, before irq is set up. */
    /*if (!dev)*/
        /*notify_remote_via_evtchn(start_info.console.domU.evtchn);*/
    /*else*/
    notify_remote_via_evtchn(dev->evtchn);
}

struct consfront_dev dev_s;
struct consfront_dev *xencons_ring_init(void)
{
	int err;
	struct consfront_dev *dev = &dev_s;

	/*if (!start_info.console.domU.evtchn)*/
		/*return 0;*/

	/*dev = malloc(sizeof(struct consfront_dev));*/
	memset(dev, 0, sizeof(struct consfront_dev));
	dev->nodename = "device/console";
	dev->dom = 0;
	dev->backend = 0;
	dev->ring_ref = 0;

/*#ifdef HAVE_LIBC*/
	/*dev->fd = -1;*/
/*#endif*/
	dev->evtchn = start_info.console.domU.evtchn;
	dev->ring = (struct xencons_interface *) mfn_to_virt(start_info.console.domU.mfn);

	err = bind_evtchn(dev->evtchn, console_handle_input, dev);
	if (err <= 0) {
		/*printk("XEN console request chn bind failed %i\n", err);*/
                /*free(dev);*/
		return NULL;
	}
        unmask_evtchn(dev->evtchn);

	/* In case we have in-flight data after save/restore... */
	notify_daemon(dev);

	return dev;
}

