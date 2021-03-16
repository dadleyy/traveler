pub use std::collections::HashMap;

pub mod leaderboard;
pub mod queens_attack;

const MOD: i32 = 10i32.pow(9) + 7;

fn main() {
  assert_eq!(count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
}

#[derive(Debug)]
struct Movement {
  current: usize,
  end: usize,

  fuel_remaining: i32,
  path_count: i32,
}

fn inner_count(
  locations: &Vec<i32>,
  mut state: Movement,
  cache: &mut HashMap<usize, HashMap<i32, i32>>,
) -> Movement {
  if state.fuel_remaining < 0 {
    return state;
  }

  let current_position = locations
    .get(state.current)
    .map(|v| v.clone())
    .unwrap_or_default();

  if state.current == state.end {
    state = Movement {
      path_count: state.path_count + 1,
      ..state
    }
  }

  let path_count = state.path_count
    + locations
      .iter()
      .enumerate()
      .fold(0, |acc, (index, other_position)| {
        if index == state.current {
          acc
        } else {
          let cost = (other_position - current_position).abs();
          let fuel_remaining = state.fuel_remaining - cost;

          let next = Movement {
            fuel_remaining,
            current: index,
            path_count: 0,
            end: state.end,
          };

          acc
            + cache
              .get(&index)
              .and_then(|entry| entry.get(&fuel_remaining))
              .map(|ref_count| ref_count.clone())
              .unwrap_or_else(|| inner_count(locations, next, cache).path_count)
        }
      });

  let path_count = path_count % MOD;
  match cache.get_mut(&state.current) {
    Some(other) => other.insert(state.fuel_remaining, path_count),
    None => {
      let mut root = HashMap::new();
      root.insert(state.fuel_remaining, path_count);
      cache.insert(state.current, root);
      Some(path_count)
    }
  };

  return Movement {
    path_count,
    ..state
  };
}

fn count_routes(locations: Vec<i32>, start: usize, end: usize, fuel_remaining: i32) -> i32 {
  let mut cache = HashMap::new();
  inner_count(
    &locations,
    Movement {
      current: start,
      path_count: 0,
      end,
      fuel_remaining,
    },
    &mut cache,
  )
  .path_count
}

#[cfg(test)]
mod tests {
  use super::count_routes;

  #[test]
  fn first() {
    assert_eq!(count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
  }

  #[test]
  fn second() {
    assert_eq!(count_routes(vec![4, 3, 1], 1, 0, 6), 5);
  }

  #[test]
  fn third() {
    assert_eq!(count_routes(vec![5, 2, 1], 0, 2, 3), 0);
  }

  #[test]
  fn fourth() {
    assert_eq!(count_routes(vec![2, 1, 5], 0, 0, 3), 2);
  }

  #[test]
  fn fifth() {
    assert_eq!(count_routes(vec![1, 2, 3], 0, 2, 40), 615088286);
  }
}
