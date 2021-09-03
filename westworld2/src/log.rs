use std::io::Write;

pub trait Named<'a> {
    fn name(&'a self) -> &'a str;
}

pub trait Log {
    fn log<'a, N: Named<'a>>(&self, named: &'a N, msg: String);
}

pub struct ConsoleLog;

impl Log for ConsoleLog {
    fn log<'a, N: Named<'a>>(&self, named: &'a N, msg: String) {
        println!("{}: {}", named.name(), msg);
    }
}
