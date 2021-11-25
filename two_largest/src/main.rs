use std::fmt::Debug;

fn main() {
    let empty: Vec<usize> = vec![];
    println!("{:?} = {:?}", &empty, two_largest(&empty));

    let ints = vec![9, 5, 2, 1, 3, 4];
    println!("{:?} = {:?}", &ints, two_largest(&ints));

    let ints = vec![1, 2, 3, 4, 5, 9];
    println!("{:?} = {:?}", &ints, two_largest(&ints));

    let strs = vec!["banana", "apple", "cherry"];
    println!("{:?} = {:?}", &strs, two_largest(&strs));
}

fn two_largest<T: Iterator + Ord + Debug + Copy>(
    vals: &[T],
) -> Option<(T, T)> {
    if let Some([x, y]) = vals.get(0..2) {
        let (mut x, mut y) = (x.copied(), y.copied());
        for val in vals {
            if val > &x {
                x = val.copied();
            }
            if val > &y {
                y = val.copied();
            }
        }
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        Some((x, y))
    } else {
        None
    }
}

#[test]
fn test_two_largest() {
    let empty: Vec<usize> = vec![];
    assert_eq!(two_largest(&empty), None);

    assert_eq!(two_largest(&[1]), None);

    assert_eq!(two_largest(&[4, 1, 10, 3]), Some((10, 4)));

    assert_eq!(two_largest(&["apple".to_string()]), None);

    assert_eq!(
        two_largest(&[
            "banana".to_string(),
            "apple".to_string(),
            "cherry".to_string()
        ]),
        Some(("cherry".to_string(), "banana".to_string()))
    );
}
