fn main(){
    let s = String::from("hello");
    let s1 = String::from("world");
    println!("{}", reverse(&s));
    println!("{}", reverse(&s1));
}

fn reverse(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars().rev() {
        result.push(c);
    }
    result
}