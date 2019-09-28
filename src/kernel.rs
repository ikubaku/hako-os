use crate::syscall;

pub fn start(func: fn(isize, **mut u8) -> isize, name: *const u8, stacksize: isize, argc: isize, argv: **mut u8) -> ! {

}

pub fn sysdown() -> ! {

}

pub fn syscall(syscall: syscall::Syscall) {
    return;
}
