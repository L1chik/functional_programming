use std::fs::File;
use std::io;
use std::io::{Read, BufRead, BufReader, BufWriter, Write};


fn main() {
    // open_file();
    // read_bytes();
    // write_bytes();
    // stdin1();
    // stdin2();
    // iterators1();
    // iterators2();
    let a = errors1();
}

fn open_file() {
    let mut f = File::open("records.txt").unwrap();
    let mut buffer = [0; 1024];
    let _ = f.read(&mut buffer[..]).unwrap();

    println!("{:?}", buffer)
}

fn read_bytes() {
    let f = File::open("records.txt").unwrap();
    let mut buf_reader = BufReader::new(f);

    let mut buffer = String::new();
    buf_reader.read_line(&mut buffer).unwrap();

    println!("Read the following: {}", buffer);
}

fn write_bytes() {
    let f = File::create("result.txt").unwrap();
    let mut buf_writer = BufWriter::new(f);
    let buffer = String::from("Hello, world");

    buf_writer.write(buffer.as_bytes()).unwrap();
}

fn stdin1() {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer).unwrap();
    io::stdout().write(&mut buffer.as_bytes()).unwrap();
}

fn stdin2() {
    let mut buffer = [8; 1024];
    let stdin_handle = std::io::stdin();
    let mut locked_stdin_handle = stdin_handle.lock();
    locked_stdin_handle.read(&mut buffer).unwrap();

    let stdout_handle = std::io::stdout();
    let mut locked_stdout_handle = stdout_handle.lock();
    locked_stdout_handle.write(&mut buffer).unwrap();
}

fn iterators1() {
    let s = std::io::stdin();
    let file_reader = BufReader::new(s);

    for line in file_reader.lines() {
        println!("You typed: {}", line.unwrap());
    }
}

fn iterators2() {
    let f1 = File::open("file1.txt").unwrap();
    let f2 = File::open("file2.txt").unwrap();

    let mut chained_handle = f1.chain(f2);
    let mut buffer = String::new();

    chained_handle.read_to_string(&mut buffer).unwrap();
    println!("Chainded handle: \n {}", buffer);
}

fn errors1() -> std::io::Result<()> {
    let f1 = File::open("file1.txt")?;
    let f2 = File::open("file3.txt")?;
    //Chain the two file handles
    let mut chained_handle = f1.chain(f2);
    // Create a buffer to read into
    let mut buffer = String::new();
    // Read from chained handle into buffer
    chained_handle.read_to_string(&mut buffer)?;
    println!("Read from chained handle: {}", buffer);
    Ok(())
}

