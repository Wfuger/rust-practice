pub fn raindrops(n: usize) -> String {
    let mut sound = String::from("");
    if n % 3 == 0 {
        sound += "Pling"
    }
    if n % 5 == 0 {
        sound += "Plang"
    }
    if n % 7 == 0 {
        sound += "Plong"
    }
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        sound = String::from(format!("{}", n));
    }
    println!("sound {}", sound);
    return sound
}
