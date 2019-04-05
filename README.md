# Biscord
Maps characters in string to their respective :regional_indicator:s for use in discord

# Usage

Simply copy whatever text you want to map to your clipboard, run biscord in CMD, and paste it to whatever channel you want. 

If the text in your clipboard already contains the text "regional_indicator_" then the program doesn't do anything.

# How to compile

Compiling third party rust projects is easy. Install Cargo, cargo init a new project, put main.rs in the src folder and replace Cargo.toml
with the one in this repository, and run cargo build --release. 

I took it a step further and put the folder I put the binary in into my path variable so I can use it from any directory.

# Supported platforms

I think this only works on Windows because it only tries to access the clipboard through a WindowsClipboardContext.
You need to install X11 on Linux for the clipboard crate to work on Linux anyway, and that's just too much work for the
scope of this project.

# Is it fast?

Rust makes everything fast.
