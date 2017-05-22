extern crate draughts;

use std::io;
use std::io::Write;

use draughts::algorithm::metric::{Metric, Nodes};
use draughts::board::bitboard::BitboardPosition;
use draughts::board::generator::Generator;
use draughts::board::piece::Color;
use draughts::board::position::{Position, Game};
use draughts::engine::{Engine, EngineResult};
//use draughts::engine::randaap::RandAap;
use draughts::engine::sherlock::Sherlock;
use draughts::engine::slonenok::Slonenok;
//use draughts::uci::scan::Scan;
//use draughts::uci::slagzet::Slagzet;

type Score = u8;
fn game(white: &mut Engine<Item = EngineResult>,
        black: &mut Engine<Item = EngineResult>,
        white_score: Score,
        black_score: Score,
        initial: &Position,
        nodes: Nodes)
        -> (Score, Score) {
    let generator = Generator::create();
    let mut position = BitboardPosition::clone(initial);
    let mut prev = vec![];
    let show = nodes > 10_000;
    loop {
        let before = prev.iter()
            .fold(0, |a, p| a + if *p == position { 1 } else { 0 });
        if before >= 1 {
            return (1, 1);
        }

        let white_to_move = position.side_to_move() == Color::White;
        let moves = generator.legal_moves(&position);
        match moves.len() {
            0 => return if white_to_move { (0, 2) } else { (2, 0) },
            1 => {
                let next = position.go(&moves[0]);
                prev.push(position);
                position = next;
                if show {
                    println!("{}", moves[0]);
                    println!("{} ({})", black.display_name(), black_score);
                    println!("{}{} ({})",
                             position.ascii(),
                             white.display_name(),
                             white_score);
                }
                continue;
            }
            _ => (),
        }

        let white_to_move = position.side_to_move() == Color::White;
        let mut result = EngineResult::empty();
        if white_to_move {
            white.set_position(&position);
            while let Some(next) = white.next() {
                result = next;
                if show {
                    print!("\r                                                  \r");
                    print!("{}: {} {} | {} @ {}",
                           white.display_name(),
                           result.mv,
                           result.evaluation,
                           result.meta.get_nodes(),
                           result.meta.get_depth());
                    io::stdout().flush().expect("no flush");
                }
            }
        } else {
            black.set_position(&position);
            while let Some(next) = black.next() {
                result = next;
                if show {
                    print!("\r                                                  \r");
                    print!("{}: {} {} | {} @ {}",
                           black.display_name(),
                           result.mv,
                           -result.evaluation,
                           result.meta.get_nodes(),
                           result.meta.get_depth());
                    io::stdout().flush().expect("no flush");
                }
            }
        };
        if show {
            println!("")
        }

        let next = position.go(&result.mv);
        prev.push(position);
        position = next;

        if show {
            println!("{} ({})", black.display_name(), black_score);
            println!("{}{} ({})",
                     position.ascii(),
                     white.display_name(),
                     white_score);
        }
    }
}

pub fn main() {
    let positions = vec!["w kcekaeb2b2/5rweirr", //20449
                         "w kbeakk2b2/eh2ethehrr", //2010
                         "w kkka22beb/3hhehterr", //890
                         "w kcekk2b2/3werrter", //4388
                         "w kbeakkeb3/2w2rrweir", //1034
                         "w kkkeaeb4/2wewwewewiewrr", //1599
                         "w kcekk2b2/w4rretr", //1265
                         "w kkcece3l4wrrter" //354
                         ];
    let mut ss = 0;
    let mut sr = 0;
    for level in 0..15 {
        println!("Level {}\r\n----", level);
        let nodes = 100 << level;
        //let one = &mut RandAap::create(5 * nodes);
        //let one = &mut Scan::create(0);
        //let one = &mut Slagzet::create(nodes / 2);
        let one = &mut Slonenok::create(nodes);
        let two = &mut Sherlock::create(nodes);
        for fen in &positions[..] {
            let position = &BitboardPosition::parse(fen).unwrap();
            {
                let (ds, dr) = game(one, two, ss, sr, position, nodes);
                ss = ss + ds;
                sr = sr + dr;
                println!("{}-{} / {} - {} : {} - {}",
                         ds,
                         dr,
                         one.display_name(),
                         two.display_name(),
                         ss,
                         sr);
            }
            {
                let (dr, ds) = game(two, one, sr, ss, position, nodes);
                ss = ss + ds;
                sr = sr + dr;
                println!("{}-{} / {} - {} : {} - {}",
                         dr,
                         ds,
                         one.display_name(),
                         two.display_name(),
                         ss,
                         sr);
            }
            println!("--");
        }
    }
}
