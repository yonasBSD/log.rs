use crate::logging::log;

pub fn ok(msg: &str) {
    log().ok(msg);
}

pub fn warn(msg: &str) {
    log().warn(msg);
}

pub fn err(msg: &str) {
    log().err(msg);
}

pub fn info(msg: &str) {
    log().info(msg);
}

pub fn dim(msg: &str) {
    log().dim(msg);
}

pub fn intro(msg: &str) {
    log().intro(msg);
}

pub fn outro(msg: &str) {
    log().outro(msg);
}

pub fn done() {
    log().done();
}

pub fn step(msg: &str) {
    log().step(msg);
}

pub fn debug(msg: &str) {
    log().debug(msg);
}

pub fn trace(msg: &str) {
    log().trace(msg);
}
