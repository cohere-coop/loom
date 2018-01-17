use std::process::{Command, Stdio};

fn main() {
    let child = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    match output.status.code() {
        Some(code) => println!("process exited with code: {}", code),
        None       => println!("process terminated by signal")
    }

    match String::from_utf8(output.stdout) {
        Ok(output_string) => println!("process output: {}", output_string),
        Err(e)            => println!("could not parse process output: {:?}", e)
    }
}
