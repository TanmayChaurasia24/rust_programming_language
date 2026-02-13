fn main() {
    let str1 = String::from("tanmay");
    let ans;

    {
        let str2 = String::from("chaurasia");
        ans = longest_string(&str1, &str2); // ans may point to str1 or str2 if it points to str1 then there is no mem issues but if its points to str2 then there will be a mem issue
        println!("{}", ans);
    }

    // ans will be a dangling pointer if str2 > str1
}

fn longest_string<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() { s1 } else { s2 }
}
