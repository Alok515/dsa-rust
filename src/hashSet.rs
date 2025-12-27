use std::collections::HashSet;


fn main(){
    let text = String::from("the quick brown fox jumps over the lazy dog dog");
    let mut words = HashSet::new();

    for word in text.split_whitespace() {
        words.insert(word);
    }

    println!("Total words Count: {}", text.split_whitespace().count());
    println!("Unique words Count: {}", words.len());
    println!("HashSet: {:?}", words);

    println!("Has fox: {}", words.contains("fox"));
    println!("Has cat: {}", words.contains("cat"));

}