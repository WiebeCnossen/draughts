use std::cmp::max;
use std::io::{BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

use super::io::read_stdout;
use crate::algorithm::meta::{Meta, Nodes};
use crate::board::generator::Generator;
use crate::board::position::Position;
use crate::engine::{Engine, EngineResult};

pub struct Scan {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    position: Option<Position>,
    generator: Generator,
    max_nodes: Nodes,
}

impl Scan {
    pub fn create(max_nodes: Nodes) -> Scan {
        let mut child = Command::new("/home/wiebe/draughts/scan/scan")
            .arg("hub")
            .current_dir("/home/wiebe/draughts/scan")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute child");

        let mut stdin = child.stdin.take().expect("No stdin on Scan");
        let mut stdout = BufReader::new(child.stdout.take().expect("No stdout on Scan"));

        read_stdout(&mut stdout);
        stdin.write(b"init\n").ok();
        loop {
            let line = read_stdout(&mut stdout);
            if line == "ready" {
                break;
            }
        }

        Scan {
            stdin,
            stdout,
            position: None,
            generator: Generator::create(),
            max_nodes,
        }
    }
}

impl Iterator for Scan {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        let result = if let Some(ref position) = self.position {
            self.stdin
                .write(format!("pos {}\n", position.fen()).as_bytes())
                .ok();
            self.stdin
                .write(format!("level 1 {} 0\n", max(1, self.max_nodes / 30_000)).as_bytes())
                .ok();
            self.stdin.write(b"analyse\n").ok();
            let temp;
            loop {
                let mut move_string = read_stdout(&mut self.stdout);
                if move_string.starts_with("move ") {
                    for _ in 0..5 {
                        move_string.remove(0);
                    }
                    let mv = self
                        .generator
                        .legal_moves(position)
                        .into_iter()
                        .find(|m| m.as_full_string() == move_string)
                        .expect("No move found");
                    temp = Some(EngineResult::create(mv, 0, Meta::create()));
                    break;
                }
            }
            temp
        } else {
            None
        };
        self.position = None;
        result
    }
}

const NAME: &str = "Scan";
impl Engine for Scan {
    fn display_name(&self) -> &str {
        NAME
    }
    fn set_position(&mut self, position: &Position) {
        self.position = Some(*position);
    }
}
