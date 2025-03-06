use std::io::{Read, Write, stdin, stdout};

use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = stdout().lock().into_raw_mode().unwrap();
    let stdin = stdin().lock();

    let mut bytes = stdin.bytes();
    loop {
        let byte = bytes.next().unwrap().unwrap();
        if byte == b'q' {
            return;
        }
        write!(stdout, "{}", byte as char).unwrap();
        stdout.flush().unwrap();
    }
}
