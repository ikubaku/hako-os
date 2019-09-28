pub enum Syscall {
    Run {
        func: fn(isize, *mut [u8]) -> isize,
        name: *const u8,
        priority: isize,
        stacksize: isize,
        argc: isize,
        argv: *mut [u8],
        ret: u32,
    },
    Exit,
    Wait {
        ret: isize,
    },
}

pub fn run(
    func: fn(isize, *mut [u8]) -> isize,
    name: *const u8,
    stacksize: isize,
    argc: isize,
    argv: *mut [u8],
) {
    return;
}

pub fn exit() -> ! {}
