fn main() {
    let mut s: String = String::from("tanmaykumar chaurasia");
    let res = find_index(&s);

    // as slice is returning a reference so i can use s.clear here
    // s.clear();

    println!("the index of space for {s}: {res}");

    s.clear();
}

fn find_index(input: &String) -> &str {
    let s: &[u8] = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}
