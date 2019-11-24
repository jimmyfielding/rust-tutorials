fn main() {
    vectors();
}

fn vectors() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];

    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
