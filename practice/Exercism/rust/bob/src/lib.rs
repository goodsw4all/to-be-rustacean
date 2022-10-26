pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message.ends_with('?');
    let is_yelling = message
        .chars()
        .any(|c| c.is_alphabetic())
        && message.to_uppercase() == message;

    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Sure.",
        (true, false) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
