use std::collections::HashMap;


fn main () {
    let text = String::from("hello world wonderful world");
    let mut map: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}