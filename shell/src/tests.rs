extern crate core;

use std::io::Error;
use std::io::{Read, Write};
use std::{panic, process};
use std::process::{Command, Stdio};
use signal_hook::iterator::Signals;

pub const SIGINT: i32 = 2;
pub const SIGTERM: i32 = 15;

fn main() -> Result<(), Error>{
    // ls_test();
    // out_test();
    // terminate_abort();
    // terminate_exit();
    // child_status();
    // child_io();
    // child_pipe();
    // abort_cur_proc();

    let mut signals = Signals::new(
        &[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT])?;

    'singal_loop: loop {
        for signal in signals.pending() {
            match signal {
                signal_hook::consts::SIGINT => {
                    println!("Received signal SIGINT");
                }
                signal_hook::consts::SIGTERM => {
                    println!("Received signal SIGTERM");
                    break 'singal_loop;
                }
                _ => unreachable!(),
            }
        }
    }


    println!("Terminating program");
    Ok(())
}

fn ls_test() {
    Command::new("ls")
        .current_dir("..")
        // .arg("-l")
        // .arg("-h")
        .args(&["-l", "-h"])
        .spawn()
        .expect("ls command failed to start");
}

fn out_test() {
    let output = Command::new("cat").arg("a.txt").output().unwrap();

    if !output.status.success() {
        println!("Command executed with fail")
    }
    println!("printing: {}", String::from_utf8(output.stdout).unwrap());
}

fn terminate_abort() {
    println!("Going to abort process");

    process::abort();

    println!("Imposter?")
}

fn terminate_exit() {
    println!("Going to exit process");
    process::exit(64);
    println!("process exited")
}

fn child_status() {
    let status = Command::new("cat")
        .arg("a.txt")
        .status()
        .expect("failed to execute cat");

    if status.success() {
        println!("Successful operation");
    } else {
        println!("Unsuccessful operation");
    }
}

fn child_io() {
    let process = match Command::new("ps").stdout(Stdio::piped()).spawn() {
        Ok(process) => process,
        Err(err) => panic!("couldn't spawn ps: {}", err),
    };

    let mut ps_output = String::new();

    match process.stdout.unwrap().read_to_string(&mut ps_output) {
        Ok(_) => println!("ps output from child process is: \n{}", ps_output),
        Err(err) => panic!("couldn't read ps stdout: {}", err),
    }
}

fn child_pipe() {
    let process = match Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()

    {
        Ok(process) => process,
        Err(err) => panic!("couldn't spawn rev: {}", err),
    };

    match process.stdin.unwrap().write_all("palindrome".as_bytes()) {
        Ok(_) => println!("sent text to rev command"),
        Err(err) => panic!("couldn't write to stdin: {}", err),
    }

    let mut child_output = String::new();

    match process.stdout.unwrap().read_to_string(&mut child_output) {
        Ok(_) => println!("Output from child process is: \n{} \n", child_output),
        Err(err) => panic!("couldn't read stdout: {}", err),
    }
}

fn abort_cur_proc() {
    panic::set_hook(Box::new(|_| {
        println!("Custom panic hook")
    }));

    let _child_proc = match Command::new("wrong-command")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()

    {
        Ok(new_proc) => {
            println!("Successfully spawned child process");
            new_proc
        }
        Err(err) => panic!("Unable to spawn child process {}", err),
    };
}
