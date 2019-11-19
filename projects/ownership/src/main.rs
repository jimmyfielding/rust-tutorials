fn main() {
    let s = String::from("hello");
    println!("In scope: {}", s);
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    println!("In scope: {}", x);
    // println!("Out of scope {}", s);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back_ownership(s2);
    let (s4, len) = calculate_length(s3);

    println!("The length of '{}' is {}.", s4, len);

    let s5 = String::from("hello");
    let len = calculate_length_ref(&s5);

    println!("The length of '{}' is {}.", s1, len);

    let mut s6 = String::from("hello");
    change(&mut s6);

    let mut s7 = String::from("hello");
    {
        let _r1 = &mut s7;
    }

    let r1 = &s7;
    let r2 = &s7;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s7; // no problem
    println!("{}", r3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
