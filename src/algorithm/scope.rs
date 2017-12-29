use algorithm::judge::Eval;

pub type Depth = u8;
pub trait Scope: Sized + Clone {
    fn from_depth(depth: Depth) -> Self;
    fn next(&self, len: usize, quiet: bool, gap: Eval) -> Option<Self>;
    fn depth(&self) -> Depth;
}
