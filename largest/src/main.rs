fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

fn main() {
    let nlist = vec![43, 50, 25, 100, 65];

    let res = largest(&nlist);

    println!("The largest number is {}", res);

    let clist = vec!['y', 'm', 'a', 'q'];

    let res = largest(&clist);

    println!("The largest char is {}", res);
}
