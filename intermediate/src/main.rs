extern crate core;


use std::process::Command;
use std::io::ErrorKind;


fn os_commands_example1() {
    let echo_cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello from windows"])
            .output()
            .expect("Failed to execute command.")
    } else {
        Command::new("sh")
            .args(["-c", "echo hello from linux"])
            .output()
            .expect("Failed to execute command.")
    };

    println!("\n");

    let cmd_output = String::from_utf8(echo_cmd.stdout).unwrap();
    println!("{}", cmd_output);
}

fn os_commands_example2(dir: &str) {
    let mut cmd_root = Command::new("ls");

    match cmd_root.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Directory not found");
                None
            },
            _ => panic! ("Unexpected error")
        }
    };

}

fn main() {
    // os_commands_example1();
    os_commands_example2("src");
    os_commands_example2("lib");
    os_commands_example2("--fd");
}
