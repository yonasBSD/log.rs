pub trait ScreenLogger {
    fn ok(&self, m: &str);
    fn warn(&self, m: &str);
    fn err(&self, m: &str);
    fn info(&self, m: &str);
    fn dim(&self, m: &str);
    fn intro(&self, m: &str);
    fn outro(&self, m: &str);
    fn done(&self);
    fn step(&self, m: &str);
    fn debug(&self, m: &str);
    fn trace(&self, m: &str);
    fn dump_tree(&self);
    fn progress(&self, label: &str, current: u64, total: Option<u64>, finished: bool);
}
