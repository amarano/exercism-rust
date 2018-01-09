pub fn reply(message: &str) -> &str {

    let m = message.trim();

    if m.is_empty() {
        "Fine. Be that way!"
    } else if m.chars().last() == Some('?') {
        "Sure."
    } else if m.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}

