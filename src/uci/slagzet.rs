use std::io::{BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

use algorithm::metric::{Meta, Nodes};
use board::generator::Generator;
use board::position::Position;
use engine::{Engine, EngineResult};
use uci::io::read_stdout;

pub struct Slagzet {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    position: Option<Position>,
    generator: Generator,
}

impl Slagzet {
    pub fn create(max_nodes: Nodes) -> Slagzet {
        let mut child = Command::new("node")
            .arg("/mnt/c/develop/extern/slagzet/slagzet.js")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute child");

        let mut stdin = child.stdin.take().expect("No stdin on Slagzet");
        let stdout = BufReader::new(child.stdout.take().expect("No stdout on Slagzet"));
        stdin.write(format!("{}\n", max_nodes).as_bytes()).ok();
        Slagzet {
            stdin,
            stdout,
            position: None,
            generator: Generator::create(),
        }
    }
}

impl Iterator for Slagzet {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        let result = if let Some(ref position) = self.position {
            self.stdin.write(position.fen().as_bytes()).ok();
            self.stdin.write(b"\n").ok();
            let move_string = read_stdout(&mut self.stdout);
            let mv = self.generator
                .legal_moves(position)
                .into_iter()
                .find(|m| m.as_string() == move_string)
                .expect("No move found");
            Some(EngineResult::create(mv, 0, Meta::create()))
        } else {
            None
        };
        self.position = None;
        result
    }
}

const NAME: &str = "Slagzet";
impl Engine for Slagzet {
    fn display_name(&self) -> &str {
        NAME
    }
    fn set_position(&mut self, position: &Position) {
        self.position = Some(Position::clone(position));
    }
}
