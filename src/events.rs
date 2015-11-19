use sharedinfo;

// type evtchn_port_t = u32;
#[repr(C)]
struct evtchn_send {
    port : u32
}

#[repr(C)]
#[derive(Clone,Copy)]
struct ev_action {
    handler:    u64,
    data:       u64,
    count:      u32
}


pub fn remote_via_event_channel(port : u32) {
    let op = evtchn_send{port : port};
    hypercall!(CMD!(EVENT_CHANNEL_OP), EVENTCHANOP!(SEND), &op as *const evtchn_send);
}
