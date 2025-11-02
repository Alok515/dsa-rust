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

// fn fizz_buzz( start: i32, end: i32 ) {
//     for i in start..=end {
//         if i % 15 == 0 {
//             print!("FizzBuzz,");
//         }else if i % 3 == 0 {
//             print!("Fizz,");
//         }else if i % 5 == 0 {
//             print!("Buzz,");
//         }else {
//             print!("{},", i);
//         }
//     }
// }

// fn calculate_length( s: &String ) -> usize {
//     s.len()
// }

// fn change( s: &mut String ){
//     s.push_str(", world");
// }

fn first_word( s: &str ) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn main() {
    let my_string = String::from("Hello World!");
    let first_string = first_word(&my_string[..]);
    println!("The first word is: {}", first_string);
    let my_litral = "Good World!";
    let first_litral = first_word(my_litral);
    println!("The first word is: {}", first_litral);
}