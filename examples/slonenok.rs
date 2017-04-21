extern crate draughts;

use draughts::algorithm::alphabeta::DepthScope;
use draughts::algorithm::bns::best_node_search;
use draughts::algorithm::metric::Metric;
use draughts::board::bitboard::BitboardPosition;
use draughts::board::generator::Generator;
use draughts::board::position::Game;
use draughts::engine::judge::Judge;
use draughts::engine::slonenok::Slonenok;
use draughts::uci::io::{read_stdin};

pub fn main() {
  let judge = &Slonenok::create(Generator::create());
  loop {
    let line = read_stdin();
    if line == "quit" { break }
    let position = match BitboardPosition::parse(line.as_str()) {
      Err(msg) => {
        println!("Invalid position: {}", msg);
        continue;
      },
      Ok(pos) => pos
    };
    let mut depth = 1u8;
    let mut cut = 0;
    loop {
      let bns = best_node_search(judge, &position, &DepthScope::from_depth(depth), cut, 1);
      cut = bns.cut;
      println!("{} @ {} | {} @ {} ({} nodes)", judge.display_name(), depth, bns.mv, cut, bns.meta.get_nodes());
      if bns.meta.get_nodes() > 10_000_000 { break }
      depth = depth + 1;
    }
  }
}
