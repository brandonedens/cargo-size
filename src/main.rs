
use std::ffi::OsStr;
use std::process::{Command, Stdio};

fn run_size_cmd(args: &[&OsStr]) {
    let child = Command::new("size")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failure to run size command.");

    let output = child
        .wait_with_output()
        .expect("failure to wait on child");

    let out = std::str::from_utf8(&output.stdout)
        .expect("Failure to decode size command stdout");
    let err = std::str::from_utf8(&output.stderr)
        .expect("Failure to decode size command stderr");

    if err.len() > 0 {
        print!("{}", err);
    }
    if out.len() > 0 {
        print!("{}", out);
    }
}

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let args: Vec<&OsStr> = v.iter().map(|x| OsStr::new(x)).collect();

    run_size_cmd(&args[1..]);
}
