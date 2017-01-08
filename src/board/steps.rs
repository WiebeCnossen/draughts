use board::coords::{Coords,MinXY};

fn all<F, G>(generator: F) -> Vec<G> where F: Fn(usize) -> G {
  let mut result = Vec::with_capacity(50);
  for field in 0..50 {
    result.push(generator(field));
  }
  result
}

fn white_steps(field: usize) -> Vec<usize> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.max_x() > coords.x {
    result.push(usize::from(Coords { x: coords.x + 1, y: coords.y }));
  }
  if coords.max_y() > coords.y {
    result.push(usize::from(Coords { x: coords.x, y: coords.y + 1 }));
  }
  result
}

#[test]
fn white_steps_side() {
  let steps = white_steps(35);
  assert_eq!(steps.len(), 1);
  for step in steps.into_iter() {
    assert!(
      match step {
        30 => true,
        _ => false
      });
  }
}

#[test]
fn white_steps_center() {
  let steps = white_steps(36);
  assert_eq!(steps.len(), 2);
  for step in steps.into_iter() {
    assert!(
      match step {
        30 | 31 => true,
        _ => false
      });
  }
}

fn black_steps(field: usize) -> Vec<usize> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.min_x() < coords.x {
    result.push(usize::from(Coords { x: coords.x - 1, y: coords.y }));
  }
  if coords.min_y() < coords.y {
    result.push(usize::from(Coords { x: coords.x, y: coords.y - 1 }));
  }
  result
}

#[test]
fn black_steps_side() {
  let steps = black_steps(35);
  assert_eq!(steps.len(), 1);
  for step in steps.into_iter() {
    assert!(
      match step {
        40 => true,
        _ => false
      });
  }
}

#[test]
fn black_steps_center() {
  let steps = black_steps(36);
  assert_eq!(steps.len(), 2);
  for step in steps.into_iter() {
    assert!(
      match step {
        40 | 41 => true,
        _ => false
      });
  }
}

fn short_jumps(field: usize) -> Vec<(usize, usize)> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.max_x() > coords.x + 1 {
    result.push((
      usize::from(Coords { x: coords.x + 1, y: coords.y }),
      usize::from(Coords { x: coords.x + 2, y: coords.y })
    ));
  }
  if coords.min_x() < coords.x - 1 {
    result.push((
      usize::from(Coords { x: coords.x - 1, y: coords.y }),
      usize::from(Coords { x: coords.x - 2, y: coords.y })
    ));
  }
  if coords.max_y() > coords.y + 1 {
    result.push((
      usize::from(Coords { x: coords.x, y: coords.y + 1 }),
      usize::from(Coords { x: coords.x, y: coords.y + 2 })
    ));
  }
  if coords.min_y() < coords.y - 1 {
    result.push((
      usize::from(Coords { x: coords.x, y: coords.y - 1 }),
      usize::from(Coords { x: coords.x, y: coords.y - 2 })
    ));
  }
  result
}

#[test]
fn short_jumps_side() {
  let steps = short_jumps(30);
  assert_eq!(steps.len(), 2);
  for step in steps.into_iter() {
    assert!(
      match step {
        (26, 21) | (36, 41) => true,
        _ => false
      });
  }
}

#[test]
fn short_jumps_center() {
  let steps = short_jumps(31);
  assert_eq!(steps.len(), 4);
  for step in steps.into_iter() {
    assert!(
      match step {
        (26, 20) | (27, 22) | (36, 40) | (37, 42) => true,
        _ => false
      });
  }
}

fn long_paths_vec(field: usize, min_size: i8) -> Vec<Vec<usize>> {
  let mut super_result = vec![];
  let coords = Coords::from(field);
  if coords.max_x() > coords.x + min_size - 1 {
    let mut result = vec![];
    for x in coords.x + min_size .. coords.max_x() + 1 {
      result.push(usize::from(Coords { x: x, y: coords.y }));
    }
    super_result.push(result);
  }
  else {
    super_result.push(vec![]);
  }
  if coords.min_x() < coords.x - min_size + 1 {
    let mut result = vec![];
    for x in coords.min_x() .. coords.x - min_size + 1 {
      result.push(usize::from(Coords { x: x, y: coords.y }));
    }
    super_result.push(result);
  }
  else {
    super_result.push(vec![]);
  }
  if coords.max_y() > coords.y + min_size - 1 {
    let mut result = vec![];
    for y in coords.y + min_size .. coords.max_y() + 1 {
      result.push(usize::from(Coords { x: coords.x, y: y }));
    }
    super_result.push(result);
  }
  else {
    super_result.push(vec![]);
  }
  if coords.min_y() < coords.y - min_size + 1 {
    let mut result = vec![];
    for y in coords.min_y() .. coords.y - min_size + 1 {
      result.push(usize::from(Coords { x: coords.x, y: y }));
    }
    super_result.push(result);
  }
  else {
    super_result.push(vec![]);
  }
  super_result
}

fn long_paths(field: usize, min_size: i8) -> [Vec<usize>; 4] {
  let mut v = long_paths_vec(field, min_size);
  let v3 = v.pop().unwrap();
  let v2 = v.pop().unwrap();
  let v1 = v.pop().unwrap();
  let v0 = v.pop().unwrap();
  [v0, v1, v2, v3]
}

#[cfg(test)]
fn king_roads(field: usize, min_size: i8) -> Vec<usize> {
  let mut v = long_paths_vec(field, min_size);
  let mut v3 = v.pop().unwrap();
  let mut v2 = v.pop().unwrap();
  let mut v1 = v.pop().unwrap();
  let mut v0 = v.pop().unwrap();
  v0.append(&mut v1);
  v0.append(&mut v2);
  v0.append(&mut v3);
  v0
}

#[cfg(test)]
fn long_steps(field: usize) -> Vec<usize> {
  return king_roads(field, 1)
}

#[test]
fn long_steps_side() {
  let steps = long_steps(30);
  assert_eq!(steps.len(), 11);
  for step in steps.into_iter() {
    assert!(
      match step {
        25 | 26 | 21 | 17 | 12 | 8 | 3 | 35 | 36 | 41 | 47 => true,
        _ => false
      });
  }
}

#[test]
fn long_steps_center() {
  let steps = long_steps(31);
  assert_eq!(steps.len(), 15);
  for step in steps.into_iter() {
    assert!(
      match step {
        26 | 20 | 15 | 27 | 22 | 18 | 13 | 9 | 4 | 36 | 40 | 45 | 37 | 42 | 48 => true,
        _ => false
      });
  }
}

#[cfg(test)]
fn long_jumps(field: usize) -> Vec<usize> {
  return king_roads(field, 2)
}

#[test]
fn long_jumps_side() {
  let steps = long_jumps(30);
  assert_eq!(steps.len(), 7);
  for step in steps.into_iter() {
    assert!(
      match step {
        21 | 17 | 12 | 8 | 3 | 41 | 47 => true,
        _ => false
      });
  }
}

#[test]
fn long_jumps_center() {
  let steps = long_jumps(31);
  assert_eq!(steps.len(), 11);
  for step in steps.into_iter() {
    assert!(
      match step {
        20 | 15 | 22 | 18 | 13 | 9 | 4 | 40 | 45 | 42 | 48 => true,
        _ => false
      });
  }
}

pub struct Steps {
  all_white_steps: Vec<Vec<usize>>,
  all_black_steps: Vec<Vec<usize>>,
  all_short_jumps: Vec<Vec<(usize, usize)>>,
  all_long_paths: Vec<[Vec<usize>; 4]>,
}

impl Steps {
  pub fn create() -> Steps {
    Steps {
      all_white_steps: all(|field| white_steps(field)),
      all_black_steps: all(|field| black_steps(field)),
      all_short_jumps: all(|field| short_jumps(field)),
      all_long_paths: all(|field| long_paths(field, 1)),
    }
  }

  pub fn white_steps(&self, field: usize) -> &[usize] {
    &self.all_white_steps[field][..]
  }
  pub fn black_steps(&self, field: usize) -> &[usize] {
    &self.all_black_steps[field][..]
  }
  pub fn short_jumps(&self, field: usize) -> &[(usize, usize)] {
    &self.all_short_jumps[field][..]
  }
  pub fn long_paths(&self, field: usize) -> [&[usize]; 4] {
    let ref vecs = self.all_long_paths[field];
    [&vecs[0][..], &vecs[1][..], &vecs[2][..], &vecs[3][..]]
  }
}
