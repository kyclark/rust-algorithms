use std::{cmp::Ordering::*, fmt::Debug};

fn main() {
    let vals: &[usize] = &[];
    println!("largest two {:?} = {:?}", vals, largest_two(vals));

    let vals = &[1];
    println!("largest two {:?} = {:?}", vals, largest_two(vals));

    let vals = &[1, 2];
    println!(
        "largest two {:?} = {:?}",
        vals,
        largest_two(vals).map(|(i, j)| (vals[i], vals[j]))
    );

    let vals = &[2, 1];
    println!(
        "largest two {:?} = {:?}",
        vals,
        largest_two(vals).map(|(i, j)| (vals[i], vals[j]))
    );

    let vals = &[4, 1, 3, 2];
    println!(
        "largest two {:?} = {:?}",
        vals,
        largest_two(vals).map(|(i, j)| (vals[i], vals[j]))
    );
}

fn largest_two<T: Ord + Debug>(vals: &[T]) -> Option<(usize, usize)> {
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
            let (mut max1, mut max2) =
                if vals[0] > vals[1] { (1, 0) } else { (0, 1) };

            for i in 0..vals.len() {
                if vals[i] > vals[max1] {
                    max2 = max1;
                    max1 = i;
                } else if vals[i] > vals[max2] {
                    max2 = i;
                }
            }

            Some((max1, max2))
        }
    }
}

#[test]
fn test_largest_two() {
    let empty: &[usize] = &[];
    assert_eq!(largest_two(empty), None);
    assert_eq!(largest_two(&[1]), None);
    assert_eq!(largest_two(&[1, 2]), Some((1, 0)));
    assert_eq!(largest_two(&[2, 1]), Some((0, 1)));
    assert_eq!(largest_two(&[1, 2, 3, 4]), Some((3, 2)));
    assert_eq!(largest_two(&[4, 1, 3, 2]), Some((0, 2)));
}
