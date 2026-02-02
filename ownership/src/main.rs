fn main() {
    let s = String::from("tanmay"); // this string is stored in heap
    let s2 = s; // ownership moved from s to s2, s is no longer valid

    // println!("s is {s}"); // this will give error because s is moved to s2, s has moved to s2 to make sure that the double free error does not occur, double free error is when we try to free the same memory twice. because s is created in heap and when we did s2 = s then s2 also points to the same memory location as s, so when s goes out of scope it will try to free the memory twice.

    // clone creates the copy of s2 in heap and s2 is still valid as s3 points to different memory location for the same data as of s2.
    let s3 = s2.clone(); // though rust never create the deep copy, as it is an expensive operation on the heap, but we can do it using clone();

    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let num = 10;
    let result = add(num);

    let name = String::from("tanmay"); // name has the ownership of the string "tanmay"

    takes_ownership(name);

    // println!("name is {name}"); // this will give error because name is moved to takes_ownership

    let name2 = gives_ownership(); // name2 has the ownership of the string "tanmay" returned by gives_ownership function
    println!("name2 is {name2}");

    println!("num is {num} and result is {result}");
}

fn takes_ownership(s: String) {
    println!("inside ownership {s}")
}

fn gives_ownership() -> String {
    let name = String::from("tanmay");
    name // return values transfer the ownerships
}

fn add(x: i32) -> i32 {
    x + 10
}
