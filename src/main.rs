// fn sum(x: i32, y: i32) -> i32 { 
//     x + y // sum(x, y)
// }

// fn main() {
//     let a = 10;
//     let mut b = 5;
//     b = 15;
//     let result = sum(a, b);
//     println!("The sum of {} and {} is {}", a, b, result);
// }

//day 3

fn fizz_buzz( start: i32, end: i32 ) {
    for i in start..=end {
        if i % 15 == 0 {
            print!("FizzBuzz,");
        }else if i % 3 == 0 {
            print!("Fizz,");
        }else if i % 5 == 0 {
            print!("Buzz,");
        }else {
            print!("{},", i);
        }
    }
}

fn main() {
    let start = 1;
    let end = 100;
    fizz_buzz(start, end);
}