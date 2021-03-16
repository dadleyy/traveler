use std::io;

fn abs(first: usize, second: usize) -> usize {
  if first > second {
    first - second
  } else {
    second - first
  }
}

fn surface_area(width: usize, height: usize, elevations: &Vec<usize>) -> usize {
  elevations
    .iter()
    .enumerate()
    .fold(0, |total_area, (index, elevation)| {
      let row = index.checked_div_euclid(width).unwrap_or(0);
      let col = index.checked_rem_euclid(width).unwrap_or(0);

      // If any elevation, it is safe to say we will have 2 square units of space on top + bottom.
      let mut inc = if *elevation > 0 { 2 } else { 0 };

      // If we're the top row, our elevation is exposed on its north face.
      if row == 0 {
        inc += elevation
      } else {
        // If we're not the top row, we should add the difference between us and our northern
        // neightbor.
        let ni = index - width;
        let en = elevations.get(ni).cloned().unwrap_or_default();
        inc += abs(en, *elevation);
      }

      // If we're the first column, our elevation is exposed entirely on the west.
      if col == 0 {
        inc += elevation
      } else {
        // If we're not the first column, we should add the difference between us and our eastern
        // neightbor.
        let ei = index - 1;
        let en = elevations.get(ei).cloned().unwrap_or_default();
        inc += abs(en, *elevation);
      }

      // If we're the last row, our elevation is exposed entirely on the south.
      if row == height - 1 {
        inc += elevation
      }

      // If we're the last column, our elevation is exposed entirely on the east.
      if col == width - 1 {
        inc += elevation
      }

      total_area + inc
    })
}

fn log_err<E>(e: E) -> io::Error
where
  E: std::error::Error,
{
  println!("error: {:?}", e);
  io::Error::from_raw_os_error(22)
}

fn main() -> io::Result<()> {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  let dimensions = buffer
    .trim()
    .split(" ")
    .map(|s| s.parse::<usize>().map_err(log_err))
    .collect::<io::Result<Vec<usize>>>()?;
  let (height, width) = (
    dimensions.get(0).cloned().unwrap_or_default(),
    dimensions.get(1).cloned().unwrap_or_default(),
  );
  let mut elevations = Vec::with_capacity(width * height);
  let mut lines_read = 0;

  while lines_read < height {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut row = buffer
      .trim()
      .split(" ")
      .take(width)
      .map(|s| s.parse::<usize>().map_err(log_err))
      .collect::<io::Result<Vec<usize>>>()?;
    elevations.append(&mut row);
    lines_read += 1;
  }

  println!("{}", surface_area(width, height, &elevations));
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::surface_area;

  #[test]
  fn first() {
    let elevations = vec![1, 3, 4, 2, 2, 3, 1, 2, 4];
    assert_eq!(surface_area(3, 3, &elevations), 60)
  }

  #[test]
  fn second() {
    let elevations = vec![1];
    assert_eq!(surface_area(1, 1, &elevations), 6)
  }

  #[test]
  fn third() {
    let elevations = vec![51, 32, 28, 49, 28, 21, 98, 56, 99, 77];
    assert_eq!(surface_area(1, 10, &elevations), 1482)
  }
}
