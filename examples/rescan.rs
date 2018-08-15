use std::io::{BufReader, Write};
use std::process::{Command, Stdio};

use draughts::board::generator::Generator;
use draughts::board::mv::Move;
use draughts::board::position::Position;
use draughts::uci::io::{read_lines, read_stdin};

fn main() {
    let mut child = Command::new("/home/wiebe/draughts/scan/scan")
        .arg("hub")
        .current_dir("/home/wiebe/draughts/scan")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute child");

    let mut stdin = child.stdin.take().expect("Bimmer");
    let mut stdout = BufReader::new(child.stdout.take().expect("Bommer"));
    let generator = Generator::create();
    let mut current = Position::initial();
    let mut moves = generator.legal_moves(&current);
    let mut suggestion: Option<Move> = None;

    stdin.write(b"init\n").ok();
    read_lines(&mut stdout, "ready");

    loop {
        let line = read_stdin();
        let mut command = line.split(' ');
        let head = match command.nth(0) {
            Some(word) => word,
            None => {
                println!("[error] Empty command");
                (continue)
            }
        };
        if head == "quit" {
            stdin.write(b"quit\n").ok();
            break;
        }
        if head == "peek" {
            stdin.write(b"level move-time=1\n").ok();
            stdin.write(b"go think\n").ok();
            let line = read_lines(&mut stdout, "done").pop().unwrap();
            let move_string = line.split(' ').nth(1).unwrap().split('=').nth(1).unwrap();
            suggestion = moves
                .clone()
                .into_iter()
                .find(|m| m.as_full_string() == move_string);
        } else if head == "go" {
            stdin.write(b"level move-time=10\n").ok();
            stdin.write(b"go think\n").ok();
            let line = read_lines(&mut stdout, "done").pop().unwrap();
            let move_string = line.split(' ').nth(1).unwrap().split('=').nth(1).unwrap();
            suggestion = moves
                .clone()
                .into_iter()
                .find(|m| m.as_full_string() == move_string);
        } else if head == "ponder" {
            stdin.write(b"level move-time=100\n").ok();
            stdin.write(b"go think\n").ok();
            let line = read_lines(&mut stdout, "done").pop().unwrap();
            let move_string = line.split(' ').nth(1).unwrap().split('=').nth(1).unwrap();
            suggestion = moves
                .clone()
                .into_iter()
                .find(|m| m.as_full_string() == move_string);
        } else if head == "pos" {
            let fen = match command.nth(0) {
                Some(word) => word,
                None => {
                    println!("[error] Empty fen");
                    (continue)
                }
            };
            current = match Position::parse(fen) {
                Ok(position) => position,
                Err(msg) => {
                    println!("[error] {}", msg);
                    (continue)
                }
            };
            moves = generator.legal_moves(&current);
            while moves.len() == 1 {
                let mv = moves.pop().unwrap();
                println!("[rescan] auto-move {}", mv.as_full_string());
                current = current.go(&mv);
                moves = generator.legal_moves(&current);
            }
            println!("\r\n{}\r\n", current.ascii());
            stdin
                .write(format!("pos pos={}\n", current.hfen()).as_bytes())
                .ok();
            stdin.write(b"level move-time=1\n").ok();
            stdin.write(b"go think\n").ok();
            let line = read_lines(&mut stdout, "done").pop().unwrap();
            let move_string = line.split(' ').nth(1).unwrap().split('=').nth(1).unwrap();
            suggestion = moves
                .clone()
                .into_iter()
                .find(|m| m.as_full_string() == move_string);
        } else {
            let mv = if head == "ok" {
                match suggestion {
                    Some(mv) => mv,
                    None => {
                        println!("[error] No suggestion");
                        (continue)
                    }
                }
            } else {
                match moves.clone().into_iter().find(|m| m.as_string() == head) {
                    Some(mv) => mv,
                    None => {
                        println!("[error] Unknown move");
                        (continue)
                    }
                }
            };
            current = current.go(&mv);
            moves = generator.legal_moves(&current);
            while moves.len() == 1 {
                let mv = moves.pop().unwrap();
                println!("[rescan] auto-move {}", mv.as_full_string());
                current = current.go(&mv);
                moves = generator.legal_moves(&current);
            }
            println!("\r\n{}\r\n{}", current.ascii(), current.sfen());
            stdin
                .write(format!("pos pos={}\n", current.hfen()).as_bytes())
                .ok();
            stdin.write(b"level move-time=1\n").ok();
            stdin.write(b"go think\n").ok();
            let line = read_lines(&mut stdout, "done").pop().unwrap();
            let move_string = line.split(' ').nth(1).unwrap().split('=').nth(1).unwrap();
            suggestion = moves
                .clone()
                .into_iter()
                .find(|m| m.as_full_string() == move_string);
        }
    }

    child.wait().expect("Failed to wait on child");
}
