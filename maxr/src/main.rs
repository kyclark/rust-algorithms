use std::fmt::Debug;

fn main() {
    let empty: Vec<usize> = vec![];
    println!("{:?} = {:?}", &empty, my_max(&empty));

    let ints = vec![9, 5, 2, 1, 3, 4];
    println!("{:?} = {:?}", &ints, my_max(&ints).map(|i| ints[i]));

    let ints = vec![1, 2, 3, 4, 5, 9];
    println!("{:?} = {:?}", &ints, my_max(&ints).map(|i| ints[i]));

    let strs = vec!["banana", "apple", "cherry"];
    println!("{:?} = {:?}", &strs, my_max(&strs).map(|i| strs[i]));
}

#[test]
fn test_my_max() {
    let empty: Vec<usize> = vec![];
    assert_eq!(my_max(&empty), None);

    let ints = vec![4, 1, 10, 3];
    assert_eq!(my_max(&ints), Some(2));

    let strs = vec!["banana", "apple", "cherry"];
    assert_eq!(my_max(&strs), Some(2));
}

fn my_max<T: Ord + Debug>(vals: &Vec<T>) -> Option<usize> {
    match vals.len() {
        0 => None,
        n => {
            let mut largest = 0;
            for i in 0..n {
                if vals[i] > vals[largest] {
                    largest = i;
                }
            }
            Some(largest)
        }
    }
}

//fn my_max<T: Ord + Debug>(vals: &Vec<T>) -> Option<&T> {
//    match vals.is_empty() {
//        true => None,
//        false => {
//            for val1 in vals.iter() {
//                let mut val1_is_largest = true;
//                for val2 in vals.iter() {
//                    println!("{:?} < {:?} = {:?}", &val1, &val2, val1 < val2);
//                    if val1 < val2 {
//                        val1_is_largest = false;
//                        break;
//                    }
//                }
//                if val1_is_largest {
//                    return Some(val1);
//                }
//            }
//            None
//        }
//    }
//}

//fn my_max<T: Ord + Debug>(vals: &Vec<T>) -> Option<&T> {
//    match vals.len() {
//        0 => None,
//        len => {
//            for i in 0..len {
//                let mut val1_is_largest = true;
//                let val1 = &vals[i];
//                for j in i + 1..len {
//                    let val2 = &vals[j];
//                    println!("{:?} < {:?} = {:?}", &val1, &val2, val1 < val2);
//                    if val1 < val2 {
//                        val1_is_largest = false;
//                        break;
//                    }
//                }
//                if val1_is_largest {
//                    return Some(val1);
//                }
//            }
//            None
//        }
//    }
//}
