extern crate draughts;

use draughts::algorithm::metric::Metric;
use draughts::board::bitboard::BitboardPosition;
use draughts::board::generator::Generator;
use draughts::board::piece::Color;
use draughts::board::position::{Position, Game};
use draughts::engine::{Engine, EngineResult};
use draughts::engine::slonenok::Slonenok;
use draughts::uci::slagzet::Slagzet;

type Score = u8;
fn game(white: &mut Engine<Item = EngineResult>,
        black: &mut Engine<Item = EngineResult>,
        initial: &Position,
        nodes: usize)
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
                    println!("{}", black.display_name());
                    println!("{}{}", position.ascii(), white.display_name());
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
                    println!("{}: {} {} | {} @ {}",
                             white.display_name(),
                             result.mv,
                             result.evaluation,
                             result.meta.get_nodes(),
                             result.meta.get_depth());
                }
            }
        } else {
            black.set_position(&position);
            while let Some(next) = black.next() {
                result = next;
                if show {
                    println!("{}: {} {} | {} @ {}",
                             black.display_name(),
                             result.mv,
                             -result.evaluation,
                             result.meta.get_nodes(),
                             result.meta.get_depth());
                }
            }
        };
        let next = position.go(&result.mv);
        prev.push(position);
        position = next;

        if show {
            println!("{}", black.display_name());
            println!("{}{}", position.ascii(), white.display_name());
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
    for level in 0..15 {
        println!("Level {}\r\n----", level);
        let nodes = 100 << level;
        let one = &mut Slonenok::create(nodes);
        let two = &mut Slagzet::create(nodes * 5);
        let mut ss = 0;
        let mut sr = 0;
        for fen in &positions[..] {
            let position = &BitboardPosition::parse(fen).unwrap();
            {
                let (ds, dr) = game(one, two, position, nodes);
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
                let (dr, ds) = game(two, one, position, nodes);
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
