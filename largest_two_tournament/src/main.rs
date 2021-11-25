use std::{cmp::Ordering::*, fmt::Debug};

fn main() {
    for vals in vec![vec![], vec![1], vec![10, 1], vec![2, 1, 3, 4, 1]] {
        match largest_two(&vals) {
            Some(v) => println!("{:?} = {:?}", &vals, v),
            _ => println!("{:?} = NA", &vals),
        }
    }

    for vals in vec![
        vec!["foo"],
        vec!["foo", "bar"],
        vec!["bar", "foo"],
        vec!["foo", "bar", "baz"],
    ] {
        match largest_two(&vals) {
            Some(v) => println!("{:?} = {:?}", &vals, &v),
            _ => println!("{:?} = NA", &vals),
        }
    }
}

fn tournament<T: Debug + PartialEq + Ord>(
    vals: &[T],
    positions: &[usize],
) -> Option<Vec<usize>> {
    match positions.len() > 1 {
        true => {
            let mut winners: Vec<usize> = vec![];
            for chunk in positions.chunks(2) {
                match chunk {
                    [i, j] => {
                        winners.push(if vals[*i] > vals[*j] {
                            *i
                        } else {
                            *j
                        });
                    }
                    [i] => {
                        winners.push(*i);
                    }
                    _ => (),
                }
            }
            Some(winners)
        }
        false => None,
    }
}

fn largest_two<T: Debug + PartialEq + Ord>(
    vals: &[T],
) -> Option<(usize, usize)> {
    match vals.len().cmp(&2) {
        Less => None,
        Equal => {
            if vals[0] < vals[1] {
                Some((1, 0))
            } else {
                Some((0, 1))
            }
        }
        Greater => {
            let mut rounds = vec![(0..vals.len()).collect::<Vec<_>>()];
            loop {
                if let Some(positions) = rounds.last() {
                    if let Some(winners) = tournament(&vals, &positions) {
                        println!("winners {:?}", winners);
                        rounds.push(winners.clone());
                        if &winners.len() <= &1 {
                            break;
                        }
                    }
                }
            }
            let ultima = rounds.pop();
            let penultima = rounds.pop();
            if let (Some(x), Some(y)) = (ultima, penultima) {
                let largest = x[0];
                let second: Vec<_> =
                    y.iter().filter(|v| v != &&largest).collect();
                if let Some(second_largest) = second.get(0) {
                    Some((largest, **second_largest))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

#[test]
fn test() {
    let empty: &[usize] = &[];
    assert_eq!(largest_two(empty), None);
    assert_eq!(largest_two(&[3]), None);
    assert_eq!(largest_two(&[1, 3]), Some((1, 0)));
    assert_eq!(largest_two(&[3, 1]), Some((0, 1)));
    assert_eq!(largest_two(&[1, 2, 3]), Some((2, 1)));
    assert_eq!(largest_two(&[3, 1, 2]), Some((0, 2)));
    assert_eq!(largest_two(&[1, 1]), Some((0, 1)));
    //assert_eq!(largest_two(&[1, 1, 1]), Some((0, 1))); // handle dups?

    assert_eq!(largest_two(&["foo"]), None);
    assert_eq!(largest_two(&["foo", "bar"]), Some((0, 1)));
    assert_eq!(largest_two(&["bar", "foo"]), Some((1, 0)));
    assert_eq!(largest_two(&["foo", "bar", "baz"]), Some((0, 2)));
}
