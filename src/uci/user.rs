use super::io::read_stdin;
use crate::algorithm::meta::Meta;
use crate::board::generator::Generator;
use crate::board::position::Position;
use crate::engine::{Engine, EngineResult};

pub struct User {
    position: Option<Position>,
    generator: Generator,
}

impl User {
    pub fn create() -> User {
        User {
            position: None,
            generator: Generator::create(),
        }
    }
}

impl Iterator for User {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        let result = if let Some(ref position) = self.position {
            let temp;
            loop {
                let move_string = read_stdin();
                match self
                    .generator
                    .legal_moves(position)
                    .into_iter()
                    .find(|m| m.as_full_string() == move_string)
                {
                    Some(mv) => {
                        temp = Some(EngineResult::create(mv, 0, Meta::create()));
                        break;
                    }
                    _ => for mv in self.generator.legal_moves(position) {
                        println!("Maybe {} ?", mv);
                    },
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

const NAME: &str = "User";
impl Engine for User {
    fn display_name(&self) -> &str {
        NAME
    }
    fn set_position(&mut self, position: &Position) {
        self.position = Some(*position);
    }
}
