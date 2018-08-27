use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::ChildStdout;

fn trim_eol(mut s: String) -> String {
    let len = s.trim().len();
    s.truncate(len);
    s
}

pub fn read_stdin() -> String {
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    trim_eol(line)
}

pub fn read_stdout(reader: &mut BufReader<ChildStdout>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).ok();
    trim_eol(line)
}

pub fn wipe_line() {
    print!("\r");
    for _ in 0..5 {
        print!("                    ");
    }
    print!("\r");
}

pub fn read_lines(
    reader: &mut BufReader<ChildStdout>,
    exit: &str,
    flush: &Fn() -> (),
    print: &Fn(&str) -> (),
    wipe: &Fn() -> (),
) -> Vec<String> {
    let mut result = vec![];
    loop {
        let line = read_stdout(reader);
        result.push(line.clone());
        if result.last().unwrap().starts_with(exit) {
            flush();
            print(&line);
            flush();
            break;
        } else {
            wipe();
            print(&line);
        }
    }
    result
}

pub struct LineReader<'a, 'b> {
    reader: &'a mut BufReader<ChildStdout>,
    exit: &'b str,
    done: bool,
}

impl<'a, 'b> LineReader<'a, 'b> {
    pub fn create(reader: &'a mut BufReader<ChildStdout>, exit: &'b str) -> LineReader<'a, 'b> {
        LineReader {
            reader,
            exit,
            done: false,
        }
    }
}

impl<'a, 'b> Iterator for LineReader<'a, 'b> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.done {
            return None;
        }
        let line = read_stdout(self.reader);
        self.done = line.starts_with(self.exit);
        Some(line)
    }
}
