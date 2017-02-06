extern crate draughts;

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdout, Command, Stdio};

use draughts::board::bitboard::BitboardPosition;
use draughts::board::generator::Generator;
use draughts::board::mv::Move;
use draughts::board::position::{Game, Position};

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

pub fn read_stdout(reader: &mut BufReader<ChildStdout>) -> String {
  let mut line = String::new();
  reader.read_line(&mut line).ok();
  trim_eol(line)
}

pub fn read_lines(reader: &mut BufReader<ChildStdout>, exit: &str) -> Vec<String> {
  let mut result = vec![];
  loop {
    let line = read_stdout(reader);
    println!("[scan] {}", line);
    result.push(line);
    if result.last().unwrap().starts_with(exit) { break }
  }
  result
}

fn main() {
  let mut child = Command::new("/mnt/c/Users/wiebe/scan_20/scan")
                          .arg("hub")
                          .current_dir("/mnt/c/Users/wiebe/scan_20")
                          .stdin(Stdio::piped())
                          .stdout(Stdio::piped())
                          .stderr(Stdio::piped())
                          .spawn()
                          .expect("Failed to execute child");

  let mut stdin = child.stdin.take().expect("Bimmer");
  let mut stdout = BufReader::new(child.stdout.take().expect("Bommer"));
  let generator = Generator::create();
  let mut current = BitboardPosition::initial();
  let mut moves = generator.legal_moves(&current);
  let mut suggestion : Option<Move> = None;

  stdin.write("init\n".as_bytes()).ok();
  read_lines(&mut stdout, "ready");

  loop {
    let line = read_stdin();
    let mut command = line.split(" ");
    let head = match command.nth(0) {
      Some(word) => word,
      None => {
        println!("[error] Empty command");
        continue
      }
    };
    if head == "quit" {
      stdin.write("quit\n".as_bytes()).ok();
      break
    }
    if head == "peek" {
      stdin.write("level 1 1000 0\n".as_bytes()).ok();
      stdin.write("analyse\n".as_bytes()).ok();
      let line = read_lines(&mut stdout, "move").pop().unwrap();
      let move_string = line.split(" ").nth(1).unwrap();
      suggestion = moves.clone().into_iter().find(|m| m.as_full_string() == move_string);
    }
    else if head == "go" {
      stdin.write("level 1 10000 0\n".as_bytes()).ok();
      stdin.write("analyse\n".as_bytes()).ok();
      let line = read_lines(&mut stdout, "move").pop().unwrap();
      let move_string = line.split(" ").nth(1).unwrap();
      suggestion = moves.clone().into_iter().find(|m| m.as_full_string() == move_string);
    }
    else if head == "ponder" {
      stdin.write("level 1 100000 0\n".as_bytes()).ok();
      stdin.write("analyse\n".as_bytes()).ok();
      let line = read_lines(&mut stdout, "move").pop().unwrap();
      let move_string = line.split(" ").nth(1).unwrap();
      suggestion = moves.clone().into_iter().find(|m| m.as_full_string() == move_string);
    }
    else if head == "pos" {
      let fen = match command.nth(0) {
        Some(word) => word,
        None => {
          println!("[error] Empty fen");
          continue
        }
      };
      current = match BitboardPosition::parse(fen) {
        Ok(position) => {
          position
        },
        Err(msg) => {
          println!("[error] {}", msg);
          continue
        }
      };
      moves = generator.legal_moves(&current);
      while moves.len() == 1 {
        let mv = moves.pop().unwrap();
        println!("[rescan] auto-move {}", mv.as_full_string());
        current = current.go(mv);
        moves = generator.legal_moves(&current);
      }
      println!("\r\n{}\r\n", current.ascii());
      stdin.write(format!("pos {}\n", current.fen()).as_bytes()).ok();
      stdin.write("level 1 10000 0\n".as_bytes()).ok();
      stdin.write("analyse\n".as_bytes()).ok();
      let line = read_lines(&mut stdout, "move").pop().unwrap();
      let move_string = line.split(" ").nth(1).unwrap();
      suggestion = moves.clone().into_iter().find(|m| m.as_full_string() == move_string);
    }
    else {
      let mv = if head == "ok" {
        match suggestion {
          Some(mv) => mv,
          None => {
            println!("[error] No suggestion");
            continue
          }
        }
      }
      else {
        match moves.clone().into_iter().find(|m| m.as_string() == head) {
          Some(mv) => mv,
          None => {
            println!("[error] Unknown move");
            continue
          }
        }
      };
      current = current.go(mv);
      moves = generator.legal_moves(&current);
      while moves.len() == 1 {
        let mv = moves.pop().unwrap();
        println!("[rescan] auto-move {}", mv.as_full_string());
        current = current.go(mv);
        moves = generator.legal_moves(&current);
      }
      println!("\r\n{}\r\n", current.ascii());
      stdin.write(format!("pos {}\n", current.fen()).as_bytes()).ok();
      stdin.write("level 1 10000 0\n".as_bytes()).ok();
      stdin.write("analyse\n".as_bytes()).ok();
      let line = read_lines(&mut stdout, "move").pop().unwrap();
      let move_string = line.split(" ").nth(1).unwrap();
      suggestion = moves.clone().into_iter().find(|m| m.as_full_string() == move_string);
    }
  }

  child.wait().expect("Failed to wait on child");
}
