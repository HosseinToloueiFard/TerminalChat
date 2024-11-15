use std::io::{self, Write, BufReader, BufRead};
use std::net::TcpStream;
use std::thread;

fn main() {
    println!("Welcome to the Terminal Chat!");

    let mut username = String::new();
    let mut password = String::new();
    let mut chat_host = String::new();

    println!("Enter a username: ");
    io::stdin().read_line(&mut username).expect("Failed to read username");

    println!("Enter a password: ");
    io::stdin().read_line(&mut password).expect("Failed to read password");

    println!("Enter chat host domain (e.g., 127.0.0.1:8080): ");
    io::stdin().read_line(&mut chat_host).expect("Failed to read host domain");

    let username = username.trim().to_string();
    let password = password.trim().to_string(); 
    let chat_host = chat_host.trim().to_string();

    let mut stream = TcpStream::connect(chat_host).expect("Failed to connect to server");
    println!("Successfully connected to the server!");

    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));

    writeln!(stream, "{}", username).expect("Failed to send username");

    writeln!(stream, "{}", password).expect("Failed to send password");

    thread::spawn(move || {
        let mut server_response = String::new();
        loop {
            server_response.clear(); 
            if let Ok(_) = reader.read_line(&mut server_response) {
                if !server_response.trim().is_empty() {
                    println!("{}", server_response.trim());
                }
            }
        }
    });

    loop {
        let mut message = String::new();
        println!("Enter a message to send (or type 'exit' to quit):");
        io::stdin().read_line(&mut message).expect("Failed to read message");

        if !message.trim().is_empty() {
            writeln!(stream, "{}", message.trim()).expect("Failed to send message");
        }

        if message.trim() == "exit" {
            break;
        }
    }
}
