use std::{fmt::Debug, mem};

fn main() {
    for vals in &[vec![], vec![1], vec![10, 1], vec![2, 1, 3, 4, 1]] {
        match max_tournament(&vals) {
            Some(i) => println!("{:?} = {}", &vals, &vals[i]),
            _ => println!("{:?} = NA", &vals),
        }
    }
}

fn max_tournament<T: Debug + PartialEq + std::cmp::PartialOrd>(
    vals: &[T],
) -> Option<usize> {
    let mut positions: Vec<_> = (0..vals.len()).collect();
    while positions.len() > 1 {
        let mut winners: Vec<usize> = vec![];
        for chunk in positions.chunks(2) {
            match chunk {
                [i, j] => {
                    winners.push(if vals[*i] > vals[*j] { *i } else { *j });
                }
                [i] => {
                    winners.push(*i);
                }
                _ => (),
            }
        }
        positions = mem::take(&mut winners);
    }
    positions.get(0).copied()
}
