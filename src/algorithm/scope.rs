use algorithm::judge::Eval;

pub type Depth = u8;
pub trait Scope: Sized {
    fn from_depth(depth: Depth) -> Self;
    fn next(&self, len: usize, quiet: bool, gap: Eval) -> Option<Self>;
    fn depth(&self) -> Depth;
}
