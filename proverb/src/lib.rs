pub fn build_proverb(list: Vec<&str>) -> String {
    let wants = list.iter();
    let losts = list.iter().skip(1);
    let lts: Vec<&str> = losts.cloned().filter(|_| true).collect();
    println!("WANTS: {:?}", list);
    println!("LOSTS: {:?}", lts);
    let proverb: String = wants
        .zip(lts.iter())
        .map(|(want, lost)| format!("For want of a {} the {} was lost.\n", want, lost))
        .collect();
    
    match list.first() {
        Some(want) => proverb + &format!("And all for the want of a {}.", want),
        None => proverb,
    }
}

