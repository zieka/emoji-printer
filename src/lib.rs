extern crate regex;
use regex::Regex;

pub mod emojis;

/// Converts emoji shortcodes to unicode characters.
/// 
/// Example:
/// ```
/// use emoji_printer::print_emojis;
/// 
/// let greeting = print_emojis(":waving_hand: Hello, :globe_showing_Americas: World!");
/// //  👋 Hello, 🌎 World!
/// ```
pub fn print_emojis(text: &str) -> std::string::String {
    let emojis_map = emojis::hash_map();

    let re = Regex::new(r"(?P<emoji>:[a-zA-Z0-9_]+:)").unwrap();

    re.replace_all(text, |caps: &regex::Captures| {
        let emoji = caps.name("emoji").unwrap().to_owned().as_str();
        if emojis_map.contains_key(emoji){
            return emojis_map.get(emoji).unwrap().to_owned().to_string();
        } 
        emoji.to_string()
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_emojis() {
        let result = print_emojis("Do you like :sushi: sushi?");
        assert_eq!(result, "Do you like 🍣 sushi?");
    }

    #[test]
    fn it_leaves_unsupported_emojis() {
        let result = print_emojis("Do you like :not_a_supported_emoji: sushi?");
        assert_eq!(result, "Do you like :not_a_supported_emoji: sushi?");
    }

    #[test]
    fn it_handles_adjacent_punctuation() {
        let result = print_emojis("Hello :globe_showing_Americas:!");
        assert_eq!(result, "Hello 🌎!");
    }
}
