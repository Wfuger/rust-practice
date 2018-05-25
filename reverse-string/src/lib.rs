
pub fn reverse(input: &str) -> String {
    let mut collector: Vec<char> =  Vec::new();
    for i in input.chars() {
        collector.insert(0, i);
    }
    println!("Collector: {:?}", collector);
    let mut reversed = String::from("");
    for char in collector {
        reversed.push(char)
    }
    println!("reversed: {}", reversed);
    return reversed
}
