mod emojis;

/// Converts emoji shortcodes to unicode characters.
/// 
/// Example:
/// ```
/// use emojis::emojify;
/// 
/// let greeting = print_emojis(":waving_hand: Hello, :globe_showing_Americas: World");
/// //  👋 Hello, 🌎 World
/// ```
/// 
pub fn print_emojis(text: &str) -> std::string::String {
    let emojis_map = emojis::get_emojis_hash_map();

    let words = text.split_whitespace();

    words.map(|word| {
        if emojis_map.contains_key(word){
            return emojis_map.get(word).unwrap().to_owned().to_string();
        } 
        word.to_string()
    }).collect::<Vec<String>>().join(" ")

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
}
