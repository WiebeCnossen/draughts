extern crate draughts;

use draughts::board::position::Position;
use draughts::board::generator::Generator;

fn moves() -> Vec<String> {
    vec![
        "34-29".into(),
        "19-24".into(),
        "40-34".into(),
        "14-19".into(),
        "45-40".into(),
        "16-21".into(),
        "50-45".into(),
        "21-26".into(),
        "34-30".into(),
        "20-25".into(),
        "29x20".into(),
        "25x14".into(),
        "30-25".into(),
        "15-20".into(),
        "40-34".into(),
        "10-15".into(),
        "44-40".into(),
        "11-16".into(),
        "34-30".into(),
        "7-11".into(),
        "30-24".into(),
        "20x29".into(),
        "33x24".into(),
        "19x30".into(),
        "25x34".into(),
        "14-19".into(),
        "38-33".into(),
        "1-7".into(),
        "42-38".into(),
        "17-21".into(),
        "47-42".into(),
        "21-27".into(),
        "32x21".into(),
        "26x17".into(),
        "31-26".into(),
        "5-10".into(),
        "37-32".into(),
        "10-14".into(),
        "41-37".into(),
        "19-23".into(),
        "46-41".into(),
        "14-19".into(),
        "35-30".into(),
        "9-14".into(),
        "30-24".into(),
        "19x30".into(),
        "34x25".into(),
        "13-19".into(),
        "33-29".into(),
        "23x34".into(),
        "39x30".into(),
        "8-13".into(),
        "40-35".into(),
        "2-8".into(),
        "49-44".into(),
        "15-20".into(),
        "44-39".into(),
        "4-10".into(),
        "39-33".into(),
        "10-15".into(),
        "30-24".into(),
        "19x30".into(),
        "25x34".into(),
        "13-19".into(),
        "43-39".into(),
        "8-13".into(),
        "36-31".into(),
        "3-8".into(),
        "41-36".into(),
        "17-21".into(),
        "26x17".into(),
        "12x21".into(),
        "45-40".into(),
        "7-12".into(),
        "31-26".into(),
        "20-25".into(),
        "26x17".into(),
        "12x21".into(),
        "37-31".into(),
        "21-26".into(),
        "32-27".into(),
        "26x37".into(),
        "42x31".into(),
        "8-12".into(),
        "31-26".into(),
        "12-17".into(),
        "27-21".into(),
        "16x27".into(),
        "38-32".into(),
        "27x29".into(),
        "34x21".into(),
        "11-16".into(),
        "21-17".into(),
        "13-18".into(),
        "36-31".into(),
        "19-23".into(),
        "31-27".into(),
        "23-28".into(),
        "40-34".into(),
        "15-20".into(),
    ]
}

pub fn main() {
    println!("yo: {}", moves().len());
    let generator = Generator::create();
    let mut position = Position::initial();
    println!("{}", position.ascii());
    for move_string in moves() {
        let legal = generator.legal_moves(&position);
        let mv = legal
            .into_iter()
            .find(|m| m.as_string() == move_string)
            .unwrap();
        println!("{}", mv.as_full_string());
        position = position.go(&mv);
        println!("{}", position.ascii());
    }
}
