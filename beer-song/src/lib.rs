fn next_round(n: i32) -> i32 {
    match n {
        0 => 99,
        _ => n,
    }
}

fn bottles<'a>(n: i32) -> &'a str {
    match n {
        0 => "No more bottles of beer",
        1 => "1 bottle of beer",
        _ => format!("{} bottles of beer", n).as_str(),
    }
}

fn action<'a>(n: i32) -> &'a str {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down pass it around",
        _ => "Take one down pass it around",
    }
}

pub fn verse(n: i32) -> String {
    if n == 0 {
        return final_verse();
    }
    get_verse(n)
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    let reversed = start > end;
    if reversed {
        for i in (end..=start).rev() {
            if i == 0 {
                song += &final_verse();
            } else {
                song += &get_verse(i);
            }
            if i != end {
                song += "\n";
            }
        }
    }
    song
}

fn final_verse() -> String {
    String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
}

fn get_verse(bottles_of_beer: i32) -> String {
    let mut left = (bottles_of_beer - 1).to_string();
    let plural = bottles_of_beer > 1;
    let mut s = "";
    let mut noun = "it";
    if plural {
       s = "s"; 
       noun = "one";
    }
    if left == "0" {
        left = String::from("no more bottles");
    } else if left == "1" {
        left = String::from("1 bottle");
    } else {
        left = format!("{} bottles", left);
    }
    String::from(format!("{0} bottle{2} of beer on the wall, {0} bottle{2} of beer.\nTake {3} down and pass it around, {1} of beer on the wall.\n", bottles_of_beer, left, s, noun))
}
