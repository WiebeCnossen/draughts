use algorithm::metric::Meta;
use board::bitboard::BitboardPosition;
use board::generator::Generator;
use board::position::{Game, Position};
use engine::{Engine, EngineResult};
use uci::io::read_stdin;

pub struct User {
    position: Option<BitboardPosition>,
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
                match self.generator
                          .legal_moves(position)
                          .into_iter()
                          .find(|m| m.as_full_string() == move_string) {
                    Some(mv) => {
                        temp = Some(EngineResult::create(mv, 0, Meta::create()));
                        break;
                    }
                    _ => {
                        for mv in self.generator.legal_moves(position).into_iter() {
                            println!("Maybe {} ?", mv);
                        }
                    }
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
        self.position = Some(BitboardPosition::clone(position));
    }
}
