use algorithm::judge::Eval;

pub trait Scope: Sized {
    fn from_depth(depth: u8) -> Self;
    fn next(&self, quiet: bool, gap: Eval) -> Option<Self>;
    fn depth(&self) -> u8;
}
