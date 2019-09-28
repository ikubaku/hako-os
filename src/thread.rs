use crate::{interrupt, syscall};

const THREAD_NUM: usize = 6;
const THREAD_NAME_SIZE: usize = 15;

#[repr(C)]
struct Context {
    sp: usize,
}

struct Init {
    func: fn(isize, *mut [u8]) -> isize,
    argc: isize,
    argv: *mut [u8],
}

struct Thread {
    next: *mut Thread,
    name: [u8; THREAD_NAME_SIZE + 1],
    stack: *mut usize,
    init: Init,
    syscall: *mut syscall::Syscall,
    context: Context,
}

struct Readyque {
    head: *mut Thread,
    tail: *mut Thread,
}

static CURRENT: *mut Thread = core::ptr::null_mut();
static THREADS: [Thread; THREAD_NUM] = [Thread {
    next: core::ptr::null_mut(),
    name: [0; THREAD_NAME_SIZE + 1],
    stack: core::ptr::null_mut(),
    init: Init {
        func: core::ptr::null_mut(),
        argc: 0,
        argv: core::ptr::null_mut(),
    },
    syscall: syscall::exit(),
    context: Context { sp: 0 },
}; THREAD_NUM];

static HANDLERS: [fn(); interrupt::SOFTVEC_TYPE_NUM] =
    [core::ptr::null_mut(); interrupt::SOFTVEC_TYPE_NUM];

extern "C" {
    fn dispatch(context: *mut Context);
}
