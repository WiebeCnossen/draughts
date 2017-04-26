use engine::judge::Eval;

pub trait Scope : Sized {
  fn next(&self, quiet: bool, gap: Eval) -> Option<Self>;
  fn depth(&self) -> u8;
}
