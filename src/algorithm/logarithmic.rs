use algorithm::judge::Eval;
use algorithm::scope::{Depth, Scope};

pub struct LogarithmicScope {
    nodes: usize,
}

const UBASE: usize = 5;
const POWER: [usize; 28] = [
    1,
    5,
    25,
    125,
    625,

    3_125,
    15_625,
    78_125,
    390_625,
    1_953_125,

    9_765_625,
    48_828_125,
    244_140_625,
    1_220_703_125,
    6_103_515_625,

    30_517_578_125,
    152_587_890_625,
    762_939_453_125,
    3_814_697_265_625,
    19_073_486_328_125,

    95_367_431_640_625,
    476_837_158_203_125,
    2_384_185_791_015_625,
    11_920_928_955_078_125,
    59_604_644_775_390_625,

    298_023_223_876_953_125,
    1_490_116_119_384_765_625,
    7_450_580_596_923_828_125,
];

impl Scope for LogarithmicScope {
    fn from_depth(depth: Depth) -> LogarithmicScope {
        LogarithmicScope { nodes: UBASE.pow(u32::from(depth.min(27))) }
    }

    fn next(&self, moves: usize, _: bool, _: Eval) -> Option<LogarithmicScope> {
        match self.nodes / moves {
            0 => None,
            nodes => Some(LogarithmicScope { nodes }),
        }
    }

    fn depth(&self) -> Depth {
        match POWER.binary_search(&self.nodes) {
            Ok(pos) => pos as u8,
            Err(pos) => pos as u8 + 1,
        }
    }
}
