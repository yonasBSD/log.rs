#[cfg(test)]
use gag::BufferRedirect;
use std::io::Read;

#[cfg(test)]
pub fn capture_stdout<F: FnOnce()>(f: F) -> String {
    let mut buf = Vec::new();
    let mut redirect = BufferRedirect::stdout().unwrap();
    f();
    redirect.read_to_end(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
pub fn capture_stderr<F: FnOnce()>(f: F) -> String {
    let mut buf = Vec::new();
    let mut redirect = BufferRedirect::stderr().unwrap();
    f();
    redirect.read_to_end(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
