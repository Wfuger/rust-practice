pub fn twofer(name: &str)-> String {
    let mut name = String::from(name);
    if name == "" {
        name = String::from("you");
    }
    format!("One for {}, one for me.", name)
}
