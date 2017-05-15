extern crate draughts;

use draughts::algorithm::metric::{Metric, Nodes};
use draughts::board::bitboard::BitboardPosition;
use draughts::board::position::Game;
use draughts::engine::Engine;
use draughts::engine::slonenok::Slonenok;
use draughts::uci::io::read_stdin;

const MAX_NODES: Nodes = 1_000_000;

pub fn main() {
    let mut slonenok = Slonenok::create(MAX_NODES);
    loop {
        let line = read_stdin();
        if line == "quit" {
            break;
        }
        let position = match BitboardPosition::parse(line.as_str()) {
            Err(msg) => {
                println!("Invalid position: {}", msg);
                continue;
            }
            Ok(pos) => pos,
        };
        slonenok.set_position(&position);
        while let Some(result) = slonenok.next() {
            println!("{}: {} {} | {} @ {}",
                     slonenok.display_name(),
                     result.mv,
                     result.evaluation,
                     result.meta.get_nodes(),
                     result.meta.get_depth());
        }
    }
}
