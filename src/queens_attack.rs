use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Vector {
  x: i32,
  y: i32,
}

impl Vector {
  pub fn north() -> Self {
    Vector { x: 0, y: 1 }
  }

  pub fn north_east() -> Self {
    Vector { x: 1, y: 1 }
  }

  pub fn east() -> Self {
    Vector { x: 1, y: 0 }
  }

  pub fn south_east() -> Self {
    Vector { x: 1, y: -1 }
  }

  pub fn south() -> Self {
    Vector { x: 0, y: -1 }
  }

  pub fn south_west() -> Self {
    Vector { x: -1, y: -1 }
  }

  pub fn west() -> Self {
    Vector { x: -1, y: 0 }
  }

  pub fn north_west() -> Self {
    Vector { x: -1, y: 1 }
  }

  pub fn add(&self, velocity: &Vector) -> Self {
    let x = self.x + velocity.x;
    let y = self.y + velocity.y;
    Vector { x, y }
  }
}

fn travel(
  start: &Vector,
  velocity: &Vector,
  size: i32,
  obstacles: &HashMap<(i32, i32), bool>,
) -> i32 {
  let next = start.add(velocity);

  match obstacles.get(&(next.x, next.y)) {
    Some(_) => return 0,
    None => {
      if next.x < 1 || next.x > size || next.y < 1 || next.y > size {
        return 0;
      }

      1 + travel(&next, velocity, size, obstacles)
    }
  }
}

pub fn count_moves(
  size: i32,
  _obstacle_count: i32,
  queen_x: i32,
  queen_y: i32,
  obstacles: Vec<Vec<i32>>,
) -> i32 {
  let map = obstacles.iter().fold(HashMap::new(), |mut acc, obstacle| {
    let x = obstacle.get(0).cloned().unwrap_or_default();
    let y = obstacle.get(1).cloned().unwrap_or_default();
    acc.insert((x, y), true);
    acc
  });
  let queen = Vector {
    x: queen_x,
    y: queen_y,
  };

  travel(&queen, &Vector::north(), size, &map)
    + travel(&queen, &Vector::east(), size, &map)
    + travel(&queen, &Vector::south(), size, &map)
    + travel(&queen, &Vector::west(), size, &map)
    + travel(&queen, &Vector::north_east(), size, &map)
    + travel(&queen, &Vector::south_east(), size, &map)
    + travel(&queen, &Vector::south_west(), size, &map)
    + travel(&queen, &Vector::north_west(), size, &map)
}

#[cfg(test)]
mod tests {
  use super::count_moves;

  #[test]
  fn first() {
    assert_eq!(count_moves(4, 0, 4, 4, vec![]), 9);
  }

  #[test]
  fn second() {
    assert_eq!(
      count_moves(5, 3, 4, 3, vec![vec![5, 5], vec![4, 2], vec![2, 3]]),
      10
    );
  }

  #[test]
  fn third() {
    assert_eq!(count_moves(1, 0, 1, 1, vec![]), 0);
  }
}
