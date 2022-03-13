use std::arch::x86_64::CpuidResult;
use std::io::{Read, Write};
use std::io::{stdin, stdout};
use std::process::Command;


fn main () {
    loop {
        print!("🥺 ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unable to read input");

        let command = input.trim();
        let args: Vec<&str> = command.split_whitespace().collect();

        let mut child = Command::new(args[0])
            .args(&args[1..])
            .spawn()
            .expect("Unable to run command");

        child.wait().unwrap();
    }
}