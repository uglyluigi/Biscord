use clipboard::{
    windows_clipboard::WindowsClipboardContext as Clipboard,
    ClipboardProvider
};

use std::env;
use std::collections::HashMap;

#[macro_use] extern crate maplit;

fn main() {
    if let Ok(mut provider) = Clipboard::new() {
        let number_map: HashMap<char, &str> = hashmap!{
            '0' => "zero",
            '1' => "one",
            '2' => "two",
            '3' => "three",
            '4' => "four",
            '5' => "five",
            '6' => "six",
            '7' => "seven",
            '8' => "eight",
            '9' => "nine"
        };
        
        let mut contents: String = String::new();
        let cli_args: Vec<String> = env::args().collect();

        if cli_args.len() > 1 {
            /*
                Ignore the first element in the args Vec because it's just the executable name
                This slicing stuff is freaking awesome I love this language
                Also Joins are sweet too, Java always playing catch up. This would require
                Arrays.copyOfRange instead of a simple language idiom, and then
                probably a StringBuilder to pack it all into a consumable format.
                Rust is god like
            */
            contents = cli_args[1..].join(" ");
        } else {
            contents = provider.get_contents().unwrap();
        }

        if contents.contains("regional_indicator_") {
            println!("Looks like that string was alreadybiscord'd; skipping");
            return;
        }

        let mut str_buf: String = String::new();

        for c in contents.to_lowercase().chars() {
            let mut str_to_push: String = match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => format!(":{}:", number_map.get(&c).unwrap()),
                'a' => ":a:".to_string(),
                'b' => ":b:".to_string(),
                'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => format!(":regional_indicator_{}:", c),
                '?' => ":question:".to_string(),
                '!' => ":exclamation:".to_string(),
                _ => format!("{}", c),
            };

            str_to_push.push_str(" ");
            str_buf.push_str(str_to_push.as_str());
        }

        match Clipboard::new().unwrap().set_contents(str_buf.clone()) {
            Ok(_) => println!("{} => {}", contents, str_buf),
            Err(e) => println!("{}", e)
        };
    } else {
        println!("Couldn\'t acquire system clipboard.");
    }
}
