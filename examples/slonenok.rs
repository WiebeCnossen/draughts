extern crate draughts;
extern crate time;

use draughts::algorithm::metric::{Metric, Nodes};
use draughts::board::bitboard::BitboardPosition;
use draughts::board::position::Game;
use draughts::engine::{Engine, EngineResult};
use draughts::engine::randaap::RandAap;
use draughts::engine::slonenok::Slonenok;
use draughts::engine::sherlock::Sherlock;
use draughts::uci::io::read_stdin;
use draughts::uci::slagzet::Slagzet;

const MAX_NODES: Nodes = 100_000;

fn run(engine: &mut Engine<Item = EngineResult>) {
    let start = time::precise_time_ns();
    let mut total_nodes = 0;
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
        engine.set_position(&position);
        let mut position_nodes = 0;
        while let Some(result) = engine.next() {
            /*
            println!("{}: {} {} | {} @ {}",
                     engine.display_name(),
                     result.mv,
                     result.evaluation,
                     result.meta.get_nodes(),
                     result.meta.get_depth());
            */
            position_nodes = result.meta.get_nodes();
        }

        total_nodes += position_nodes;
    }

    let ns = time::precise_time_ns() - start;
    println!(
        "\n{}\n{} : {} nodes/s ({} ms)\n{}",
        "--------------------------------",
        engine.display_name(),
        1_000_000_000 * total_nodes / ns as usize,
        ns / 1_000_000,
        "--------------------------------"
    );
}

pub fn main() {
    for _ in 0..2 {
        run(&mut RandAap::create(MAX_NODES));
        run(&mut Sherlock::create(MAX_NODES));
        run(&mut Slonenok::create(MAX_NODES));
        run(&mut Slagzet::create(MAX_NODES));
    }
}
