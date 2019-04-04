use clipboard::{
    windows_clipboard::WindowsClipboardContext as Clipboard,
    ClipboardProvider
};

fn main() {
    if let Ok(mut provider) = Clipboard::new() {
        let contents = provider.get_contents().unwrap();

        if contents.contains("regional_indicator_") {
            println!("Looks like that string was already Biscord'd; skipping");
            return;
        }

        let mut str_buf: String = String::new();

        for c in contents.to_lowercase().chars() {
            let mut str_to_push: String = match c {
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
