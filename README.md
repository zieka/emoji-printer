# Emoji Printer

## Intro

Utility to convert strings with emoji shortcodes (`:sushi:`) to strings with the
emoji unicode (🍣)

## Install

```
cargo add emoji-printer
```

## How to use

```rs
use emoji_printer::print_emojis;

fn main() {
    let greeting = print_emojis(":waving_hand: Hello, :globe_showing_Americas: World");
    println!("{}", greeting); // 👋 Hello, 🌎 World
}
```
