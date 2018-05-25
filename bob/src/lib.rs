pub fn reply(message: &str) -> &str {
    let mut reply = "Whatever.";
    let yell = message == message.to_uppercase();
    let question = message.contains("?");
    let yell_question = yell && question;
    if yell_question {
        reply = "Calm down, I know what I'm doing!";
    } else if yell {
        reply = "Whoa, chill out!"
    } else if question {
        reply = "Sure."
    }
    return reply;
}
