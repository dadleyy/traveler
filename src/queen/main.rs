use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Default)]
pub struct Vector {
  x: i64,
  y: i64,
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

fn travel_acc(
  start: &Vector,
  velocity: &Vector,
  size: i64,
  obstacles: &HashMap<(i64, i64), bool>,
  acc: i64,
) -> i64 {
  let next = start.add(velocity);
  match obstacles.get(&(next.x, next.y)) {
    Some(_) => acc,
    None => match next.x < 1 || next.x > size || next.y < 1 || next.y > size {
      true => acc,
      false => travel_acc(&next, velocity, size, obstacles, acc + 1),
    },
  }
}

fn travel(
  start: &Vector,
  velocity: &Vector,
  size: i64,
  obstacles: &HashMap<(i64, i64), bool>,
) -> i64 {
  travel_acc(start, velocity, size, obstacles, 0)
}

pub fn count_moves(
  size: i64,
  _obstacle_count: i64,
  queen_x: i64,
  queen_y: i64,
  obstacles: Vec<(i64, i64)>,
) -> i64 {
  let map = obstacles.iter().fold(HashMap::new(), |mut acc, obstacle| {
    let x = obstacle.0;
    let y = obstacle.1;
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

#[derive(Default, Debug)]
struct Builder {
  board_features: Option<(i64, i64)>,
  queen_features: Option<(i64, i64)>,
  obstacles: Option<Vec<(i64, i64)>>,
}

fn get_bit(vec: &Vec<String>, index: usize) -> io::Result<i64> {
  vec
    .get(index)
    .cloned()
    .unwrap_or_default()
    .parse::<i64>()
    .map_err(|_e| io::Error::from_raw_os_error(22))
}

impl Builder {
  pub fn run(self) -> io::Result<()> {
    if let (Some((size, c)), Some((qx, qy)), obs) =
      (self.board_features, self.queen_features, self.obstacles)
    {
      println!(
        "{}",
        count_moves(size, c, qx, qy, obs.unwrap_or(Vec::new()))
      );
    }
    Ok(())
  }

  pub fn done(&self) -> bool {
    match (
      self.board_features,
      self.queen_features,
      self.obstacles.as_ref(),
    ) {
      (Some((_, count)), Some(_), Some(obstacles)) => (obstacles.len() as i64) == count,
      (Some((_, count)), Some(_), None) => count == 0,
      _ => false,
    }
  }

  pub fn consume(self, line: String) -> io::Result<Self> {
    let parts = line
      .trim()
      .split(" ")
      .map(|s| String::from(s))
      .collect::<Vec<String>>();
    let first = get_bit(&parts, 0)?;
    let second = get_bit(&parts, 1)?;
    match self.board_features {
      None => {
        return Ok(Builder {
          board_features: Some((first, second)),
          ..Builder::default()
        });
      }
      Some(board_features) => match self.queen_features {
        None => Ok(Builder {
          board_features: Some(board_features),
          queen_features: Some((first, second)),
          ..Builder::default()
        }),
        Some(queen_features) => {
          let obstacles = match self.obstacles {
            Some(mut current) => {
              current.push((first, second));
              current
            }
            None => {
              let mut alloc = Vec::new();
              alloc.push((first, second));
              alloc
            }
          };
          Ok(Builder {
            board_features: Some(board_features),
            queen_features: Some(queen_features),
            obstacles: Some(obstacles),
          })
        }
      },
    }
  }
}

pub fn main() -> io::Result<()> {
  let mut builder = Builder::default();

  while builder.done() == false {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    builder = builder.consume(buffer)?;
  }

  builder.run()
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
    assert_eq!(count_moves(5, 3, 4, 3, vec![(5, 5), (4, 2), (2, 3)]), 10);
  }

  #[test]
  fn third() {
    assert_eq!(count_moves(1, 0, 1, 1, vec![]), 0);
  }
}
