pub trait Scope : Sized {
  fn next(&self, quiet: bool) -> Option<Self>;
}
