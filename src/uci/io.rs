use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::ChildStdout;

fn trim_eol(mut s: String) -> String {
  match s.pop() {
    None => s,
    Some('\n') | Some('\r') => trim_eol(s),
    Some(c) => { s.push(c); s }
  }
}

pub fn read_stdin() -> String {
  print!("[user] ");
  io::stdout().flush().ok();
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok();
  trim_eol(line)
}

fn read_stdout(reader: &mut BufReader<ChildStdout>) -> String {
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

pub fn read_lines(reader: &mut BufReader<ChildStdout>, exit: &str) -> Vec<String> {
  let mut result = vec![];
  loop {
    let line = read_stdout(reader);
    result.push(line.clone());
    if result.last().unwrap().starts_with(exit) {
      println!("");
      println!("[scan] {}", line);
      break
    }
    else {
      wipe_line();
      print!("[scan] {}", line);
      io::stdout().flush().expect("no flush");
    }
  }
  result
}

pub struct LineReader<'a, 'b> {
  reader: &'a mut BufReader<ChildStdout>,
  exit: &'b str,
  done: bool
}

impl<'a, 'b> LineReader<'a, 'b> {
  pub fn create(reader: &'a mut BufReader<ChildStdout>, exit: &'b str) -> LineReader<'a, 'b> {
    LineReader { reader: reader, exit: exit, done: false }
  }
}

impl<'a, 'b> Iterator for LineReader<'a, 'b> {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    if self.done { return None }
    let line = read_stdout(self.reader);
    self.done = line.starts_with(self.exit);
    Some(line)
  }
}
