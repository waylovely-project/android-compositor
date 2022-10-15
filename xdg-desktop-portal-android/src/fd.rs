use zbus::zvariant::Fd;
/**
 * 
 */
pub fn zbusfd_to_pathbuf(file: &Fd) {
    let uri = format!("/proc/self/fd/{:?}", file.as_raw_fd());

    std::fs::canonicalize(uri)
}
