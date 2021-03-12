pub use std::collections::HashMap;

const MOD: i32 = (10 ^ 9) + 7;

fn main() {
  assert_eq!(count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
}

#[derive(Debug)]
struct Movement {
  current: usize,
  end: usize,

  fuel: i32,
  paths: i32,
}

fn inner_count(
  locations: &Vec<i32>,
  mut state: Movement,
  cache: &mut HashMap<usize, HashMap<i32, i32>>,
) -> Movement {
  if state.fuel < 0 {
    return state;
  }

  let current_position = locations
    .get(state.current)
    .map(|v| v.clone())
    .unwrap_or_default();

  if state.current == state.end {
    state = Movement {
      paths: state.paths + 1,
      ..state
    }
  }

  let paths = state.paths
    + locations
      .iter()
      .enumerate()
      .fold(0, |acc, (index, other_position)| {
        if index == state.current {
          acc
        } else {
          let cost = (other_position - current_position).abs();
          let fuel = state.fuel - cost;

          let next = Movement {
            fuel,
            current: index,
            paths: 0,
            end: state.end,
          };

          acc
            + cache
              .get(&index)
              .and_then(|entry| entry.get(&fuel))
              .map(|ref_count| ref_count.clone())
              .unwrap_or_else(|| inner_count(locations, next, cache).paths)
        }
      });

  let paths = paths % MOD;
  match cache.get_mut(&state.current) {
    Some(other) => other.insert(state.fuel, paths),
    None => {
      let mut root = HashMap::new();
      root.insert(state.fuel, paths);
      cache.insert(state.current, root);
      Some(paths)
    }
  };

  return Movement { paths, ..state };
}

fn count_routes(locations: Vec<i32>, start: usize, end: usize, fuel: i32) -> i32 {
  let mut cache = HashMap::new();
  inner_count(
    &locations,
    Movement {
      current: start,
      paths: 0,
      end,
      fuel,
    },
    &mut cache,
  )
  .paths
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
    assert_eq!(count_routes(vec![1, 2, 3], 0, 2, 40), 0);
  }
}
