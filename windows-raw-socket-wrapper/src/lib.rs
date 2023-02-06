// From https://github.com/bk-rs/ssh-rs/pull/38/files#diff-1c029370ea962fd4f49c2ccdb434750c2ea94fbd2973520b4e8e956c04ab650f

#[cfg(windows)]
pub struct RawSocketWrapper(pub std::os::windows::io::RawSocket);

#[cfg(windows)]
impl std::os::windows::io::AsRawSocket for RawSocketWrapper {
    #[inline]
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.0
    }
}
