
use std::ffi::OsStr;
use std::process::{Command, Stdio};

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let args: Vec<&OsStr> = v.iter().map(|x| OsStr::new(x)).collect();

    let child = Command::new("size")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failure to run size command.");

    let output = child
        .wait_with_output()
        .expect("failure to wait on child");

    println!("size exited {:?} with stdout: {:?} stderr: {:?}",
             output.status.success(), output.stdout, output.stderr);
}
