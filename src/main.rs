use std::env;
use rand::Rng;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use arboard::Clipboard;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "new" => generate_password(args[2].clone(), args[0].clone()),
        "get" => get_password(args[2].clone(), args[0].clone()),
        &_ => println!("Command not found."),
    }
}


fn generate_password(value: String, mut path: String) {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    
    let password: String = (0..14)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char 
        })
        .collect();
    path.push_str(".data");
    let mut file = OpenOptions::new().append(true).open(path).expect("cannot open file");
    file.write_all(format!("{} {}\n", value, password).as_bytes()).expect("write failed");
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(password).unwrap();
    println!("Password is in your clipboard.");
}


fn get_password(value: String, mut path: String) {
    path.push_str(".data");
    println!("{}", path);
    let passwords = read_to_string(path).expect("failed to open data.txt");
    let mut content: Vec<&str>;
    for line in passwords.lines() {
        content = line.split_whitespace().collect();
        if content[0] == value {
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(content[1].to_owned()).unwrap();
            println!("Password is in your clipboard.");
            return
        }
    }
    println!("Password not found.")
}
