use std::iter;

use board::position::Field;

pub type Captures = u8;

const NULL_FIELD: Field = 50;
const MAX_TAKEN: usize = 12;
const MAX_CAPTURES: u8 = 12;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    from: Field,
    to: Field,
    num_taken: Captures,
    taken: [Field; MAX_TAKEN],
}

impl Move {
    pub fn shift(from: Field, to: Field) -> Move {
        Move {
            from,
            to,
            num_taken: 0,
            taken: [NULL_FIELD; MAX_TAKEN],
        }
    }

    pub fn take_one(from: Field, to: Field, via: Field) -> Move {
        let mut taken = [NULL_FIELD; MAX_TAKEN];
        taken[0] = via;
        Move {
            from,
            to,
            num_taken: 1,
            taken,
        }
    }

    pub fn take(from: Field, to: Field, via: &[Field]) -> Move {
        let mut taken = [NULL_FIELD; MAX_TAKEN];
        let mut temp = via.to_vec();
        temp.sort();
        for (i, &v) in temp.iter().enumerate() {
            taken[i] = v;
        }
        Move {
            from,
            to,
            num_taken: via.len() as u8,
            taken,
        }
    }

    pub fn take_more(&self, via: Field, to: Field) -> Self {
        if self.num_taken == MAX_CAPTURES {
            panic!("Cannot take more than {}", MAX_TAKEN);
        }

        let mut taken = self.taken;
        match self.taken
            .iter()
            .take(self.num_taken as usize)
            .position(|&taken| taken > via)
        {
            Some(p) => {
                for p in (p..self.num_taken as usize).rev() {
                    taken[p + 1] = taken[p];
                }
                taken[p] = via;
            }
            None => taken[self.num_taken as usize] = via,
        }
        Move {
            from: self.from,
            to,
            num_taken: self.num_taken + 1,
            taken,
        }
    }

    pub fn null() -> Move {
        Move::shift(NULL_FIELD, NULL_FIELD)
    }

    pub fn from(&self) -> Field {
        self.from
    }

    pub fn to(&self) -> Field {
        self.to
    }

    pub fn num_taken(&self) -> Captures {
        self.num_taken
    }

    pub fn taken(&self) -> &[Field] {
        &self.taken[0..self.num_taken as usize]
    }

    pub fn goes_via(&self, via: Field) -> bool {
        self.taken
            .iter()
            .take(self.num_taken as usize)
            .any(|&taken| taken == via)
    }

    pub fn as_string(&self) -> String {
        let c = if self.num_taken == 0 { '-' } else { 'x' };
        format!("{}{}{}", self.from() + 1, c, self.to() + 1)
    }

    pub fn as_full_string(&self) -> String {
        iter::once(self.as_string())
            .chain(
                self.taken
                    .iter()
                    .take(self.num_taken as usize)
                    .map(|&via| format!("x{}", via + 1)),
            )
            .collect()
    }
}

use std::fmt;

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_full_string())
    }
}
