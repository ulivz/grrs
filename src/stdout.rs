#![allow(unused)]

use std::io::{self, Write};

pub fn log() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for index in 0..10000 {
        writeln!(handle, "index: {}", index);
    }

    handle.flush();
}
