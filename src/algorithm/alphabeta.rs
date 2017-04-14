use board::position::Position;
use board::mv::Move;

type Eval = i16;
pub const MAX_EVAL : Eval = 30000i16;
pub const ZERO_EVAL : Eval = 0i16;
pub const DRAW_EVAL : Eval = 0i16;
pub const MIN_EVAL : Eval = -30000i16;

trait Judge {
  fn evaluate(&self, position: &Position) -> Eval;
  fn quiet(&self, position: &Position, moves: &[Move]) -> bool;
  fn moves(&self, position: &Position) -> Vec<Move>;
  fn apply(&self, position: &Position, mv: &Move) -> &Position;

  fn alpha_beta(&self, position: &Position, depth: u8, cut: Eval) -> bool {
    if cut == ZERO_EVAL { return true }

    let moves = self.moves(position);
    if moves.len() == 0 { return false }

    let quiet = self.quiet(position, &moves);
    if quiet && depth == 0 {
      self.evaluate(position) >= cut
    }
    else {
      moves.iter().any(|mv| {
        let next = self.apply(position, mv);
        self.alpha_beta(next, if quiet { depth - 1} else { depth }, -cut)
      })
    }
  }
}

/*
01 function alphabeta(node, depth, α, β, maximizingPlayer)
02      if depth = 0 or node is a terminal node
03          return the heuristic value of node
04      if maximizingPlayer
05          v := -∞
06          for each child of node
07              v := max(v, alphabeta(child, depth – 1, α, β, FALSE))
08              α := max(α, v)
09              if β ≤ α
10                  break (* β cut-off *)
11          return v
12      else
13          v := +∞
14          for each child of node
15              v := min(v, alphabeta(child, depth – 1, α, β, TRUE))
16              β := min(β, v)
17              if β ≤ α
18                  break (* α cut-off *)
19          return v
*/

