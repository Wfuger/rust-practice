pub fn reply(message: &str) -> &str {
    let mut reply = "Whatever.";
    let trimmed = message.trim();
    let yell = (trimmed.to_lowercase() != trimmed) && (trimmed == trimmed.to_uppercase());
    let mut question = false;
    if let Some('?') = trimmed.chars().last() {
        question = true;
    }

    let yell_question = yell && question;
    if trimmed.len() == 0 {
        reply = "Fine. Be that way!";
    } else if yell_question {
        reply = "Calm down, I know what I'm doing!";
    } else if question {
        reply = "Sure.";
    } else if yell {
        reply = "Whoa, chill out!";
    }
    return reply;
}
