#[derive(Debug)]
struct Position {
  target_score: u32,
  last_score: u32,
  current_peak: u32,
  rank: u32,
}

impl Position {
  pub fn start(target: u32) -> Self {
    Position {
      target_score: target,
      last_score: 0,
      current_peak: 0,
      rank: 1,
    }
  }

  pub fn apply_move(&self, other_score: u32) -> Self {
    let mut rank_inc = if other_score == self.last_score { 0 } else { 1 };
    let mut current_peak = self.current_peak;

    if self.target_score >= other_score {
      current_peak = self.target_score;
      rank_inc = 0;
    }

    Position {
      target_score: self.target_score,
      rank: self.rank + rank_inc,
      last_score: other_score,
      current_peak: current_peak,
    }
  }
}

pub fn place_ranks(current_leaderboard: Vec<u32>, scores: Vec<u32>) -> Vec<u32> {
  scores
    .iter()
    .map(|current_player_score| {
      current_leaderboard
        .iter()
        .fold(
          Position::start(*current_player_score),
          |acc, other_score| acc.apply_move(*other_score),
        )
        .rank
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::place_ranks;

  #[test]
  pub fn first() {
    assert_eq!(
      place_ranks(vec![100, 100, 50, 40, 40, 20, 10], vec![5, 25, 50, 120]),
      vec![6, 4, 2, 1]
    )
  }

  #[test]
  pub fn second() {
    assert_eq!(
      place_ranks(vec![100, 90, 90, 80, 75, 60], vec![50, 65, 77, 90, 102]),
      vec![6, 5, 4, 2, 1]
    )
  }
}
