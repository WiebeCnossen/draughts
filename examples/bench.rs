use draughts::algorithm::meta::Nodes;
use draughts::board::position::Position;
use draughts::engine::randaap::RandAap;
use draughts::engine::sherlock::Sherlock;
use draughts::engine::slonenok::Slonenok;
use draughts::engine::{Engine, EngineResult};
use draughts::uci::io::read_stdin;
// use draughts::uci::slagzet::Slagzet;

const MAX_NODES: Nodes = 500_000;

const DASH: &str = "--------------------------------";

fn run(engine: &mut dyn Engine<Item = EngineResult>) {
    let start = time::precise_time_ns();
    let mut total_nodes = 0;
    loop {
        let line = read_stdin();
        if line == "quit" {
            break;
        }
        let position = match Position::parse(line.as_str()) {
            Err(msg) => {
                println!("Invalid position: {}", msg);
                (continue)
            }
            Ok(pos) => pos,
        };
        engine.set_position(&position);
        let mut position_nodes = 0;
        while let Some(result) = engine.next() {
            position_nodes = result.meta.get_nodes();
        }

        total_nodes += position_nodes;
    }

    let ns = time::precise_time_ns() - start;
    println!(
        "\n{}\n{} : {} nodes/s ({} ms)\n{}",
        DASH,
        engine.display_name(),
        1_000_000_000 * total_nodes / ns as usize,
        ns / 1_000_000,
        DASH
    );
}

pub fn main() {
    for _ in 0..2 {
        run(&mut RandAap::create(MAX_NODES));
        run(&mut Sherlock::create(MAX_NODES));
        run(&mut Slonenok::create(MAX_NODES));
        // run(&mut Slagzet::create(MAX_NODES));
    }
}
