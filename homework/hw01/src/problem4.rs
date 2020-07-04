/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
  A,
  B,
  C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

fn rec_move(disk: u32, src: Peg, aux: Peg, dst: Peg, move_vec: &mut Vec<Move>) {
  if disk == 1 {
    move_vec.push((src, dst));
  } else {
    rec_move(disk - 1, src, dst, aux, move_vec);
    move_vec.push((src, dst));
    rec_move(disk - 1, aux, src, dst, move_vec);
  }
}

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
  let mut move_vec = Vec::new();
  rec_move(num_discs, src, aux, dst, &mut move_vec);
  move_vec
}
