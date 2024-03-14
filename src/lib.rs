pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.ends_with('?') {
        "Sure."
    } else if message.to_uppercase() == message {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

