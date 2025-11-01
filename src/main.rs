fn sum(x: i32, y: i32) -> i32 { 
    x + y // sum(x, y)
}

fn main() {
    let a = 10;
    let mut b = 5;
    b = 15;
    let result = sum(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
}