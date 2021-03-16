use std::collections::HashMap;
use std::io;

fn compare(first: &String, second: &String) -> u64 {
  first
    .as_bytes()
    .iter()
    .enumerate()
    .fold(0, |total, (indx, v)| {
      if *v == 49 {
        total + 1
      } else {
        match second.as_bytes().get(indx) {
          Some(other) if *other == 49 => total + 1,
          _ => total,
        }
      }
    })
}

pub fn team_matches(members: &Vec<String>) -> io::Result<(u64, u64)> {
  let mut visited = HashMap::new();

  Ok(
    members
      .iter()
      .enumerate()
      .fold((0u64, 0u64), |acc, (ai, member)| {
        members
          .iter()
          .enumerate()
          .fold(acc, |acc, (bi, other)| match visited.get(&(ai, bi)) {
            Some(_) => acc,
            None => {
              if ai == bi {
                acc
              } else {
                let count = compare(member, other);
                visited.insert((ai, bi), true);
                visited.insert((bi, ai), true);

                if count > acc.0 {
                  (count, 1)
                } else if count == acc.0 {
                  let times = acc.1 + 1;
                  (count, times)
                } else {
                  acc
                }
              }
            }
          })
      }),
  )
}

pub fn main() -> io::Result<()> {
  let mut members = Vec::new();
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  let count = buffer
    .trim()
    .split(" ")
    .map(|s| String::from(s))
    .collect::<Vec<String>>()
    .get(0)
    .cloned()
    .unwrap_or_default()
    .parse::<usize>()
    .map_err(|_e| io::Error::from_raw_os_error(22))?;

  while members.len() < count {
    let mut mb = String::new();
    io::stdin().read_line(&mut mb)?;
    members.push(mb.trim().to_string());
  }

  let res = team_matches(&members)?;
  println!("{}", res.0);
  println!("{}", res.1);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::team_matches;

  #[test]
  fn first() {
    assert_eq!(
      team_matches(&vec![
        String::from("10101"),
        String::from("11110"),
        String::from("00010")
      ])
      .unwrap(),
      (5, 1)
    );
  }

  #[test]
  fn second() {
    assert_eq!(
      team_matches(&vec![
        String::from("10101"),
        String::from("11100"),
        String::from("11010"),
        String::from("00101")
      ])
      .unwrap(),
      (5, 2)
    );
  }
}
