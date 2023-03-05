pub fn sync() {
    debug!("Synchronizing I/O operations...");
    unsafe { libc::sync() };
}
