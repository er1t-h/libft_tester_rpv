mod putchar;
mod putendl;
mod putnbr;
mod putstr;

unsafe fn pipe() -> [libc::c_int; 2] {
    let mut pipes = [0; 2];
    let ret = libc::pipe(pipes.as_mut_ptr());
    assert!(
        ret == 0,
        "DPS: couldn't create a pipe. Errno: {}",
        *libc::__errno_location()
    );
    pipes
}
