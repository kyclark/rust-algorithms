use regex::Regex;

fn main() {
    for text in &["Madam", "Sir", "Mom", "A man, a plan, a canal. Panama!"] {
        println!("{:5} {}", is_palindrome(&text), &text);
    }
}

fn is_palindrome(text: &str) -> bool {
    let re = Regex::new(r"[a-zA-Z0-9]").unwrap();
    let chars: Vec<_> = text
        .to_lowercase()
        .chars()
        .filter(|c| re.is_match(&c.to_string()))
        .collect();

    // Requires more space, copies the original chars
    //let mut rev = chars.clone();
    //rev.reverse();
    //chars == rev

    let end = chars.len() - 1;

    // This compares all positions for matches
    // (0..=end).map(|i| chars[i] == chars[end - i]).all(|v| v)

    // This stops at the first mismatch
    //match (0..=end).find(|&i| chars[i] != chars[end - i]) {
    //    Some(_) => false,
    //    _ => true,
    //}

    // This stops at the first mismatch
    //match (0..=end).position(|i| chars[i] != chars[end - i]) {
    //    Some(_) => false,
    //    _ => true,
    //}

    // This tests all pairs for matches
    //(0..=end)
    //    .zip((0..=end).rev())
    //    .all(|(x, y)| chars[x] == chars[y])

    // This short-circuits and stops at first mismatch
    !(0..=end)
        .zip((0..=end).rev())
        .any(|(x, y)| chars[x] != chars[y])
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("Madam"));
    assert!(!is_palindrome("Sir"));
    assert!(is_palindrome("Mom"));
    assert!(is_palindrome("A man, a plan, a canal. Panama!"));
}
