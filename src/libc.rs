pub fn sync() {
    unsafe { libc::sync() };
}
