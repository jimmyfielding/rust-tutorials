fn main() {
    println!("Hello, world!");

    another_function();

    yet_another_function(5);

    and_another_function(1, 2);

    expression_example_function();

    let x = five();

    println!("The value of x is: {}", x);

    let y = plus_one(x);

    println!("The value of y is: {}", y);
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn and_another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expression_example_function() {
    let y = {
        let x = 3;
        x + 1 //expression do not include ending semicolons
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 //most functions return the last expression implicitly
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
