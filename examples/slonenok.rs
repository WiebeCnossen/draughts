extern crate draughts;

use draughts::algorithm::bns::best_node_search;
use draughts::algorithm::metric::Metric;
use draughts::algorithm::mtdf::mtd_f;
use draughts::algorithm::scope::DepthScope;
use draughts::board::bitboard::BitboardPosition;
use draughts::board::generator::Generator;
use draughts::board::position::Game;
use draughts::engine::judge::Judge;
use draughts::engine::slonenok::Slonenok;
use draughts::uci::io::{read_stdin};

fn bns(judge: &mut Slonenok, position: &BitboardPosition) {
  let mut depth = 0u8;
  let mut cut = 0;
  judge.reset();
  loop {
    let bns = best_node_search(judge, position, &DepthScope::from_depth(depth), cut);
    cut = bns.cut;
    println!("BNS {} @ {} | {} @ {} ({} nodes)", judge.display_name(), depth, bns.mv, cut, bns.meta.get_nodes());
    if depth >= 63 || bns.meta.get_nodes() > 10_000_000 { break }
    depth = depth + 1;
  }
}

fn mtd(judge: &mut Slonenok, position: &BitboardPosition) {
  let mut depth = 0u8;
  let mut cut = 0;
  judge.reset();
  loop {
    let mtd = mtd_f(judge, position, &DepthScope::from_depth(depth), cut);
    cut = mtd.evaluation;
    println!("MTD {} @ {} | {} @ {} ({} nodes)", judge.display_name(), depth, mtd.mv, mtd.evaluation, mtd.meta.get_nodes());
    if depth >= 63 || mtd.meta.get_nodes() > 10_000_000 { break }
    depth = depth + 1;
  }
}

pub fn main() {
  let judge = &mut Slonenok::create(Generator::create());
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
    bns(judge, &position);
    mtd(judge, &position);
  }
}
