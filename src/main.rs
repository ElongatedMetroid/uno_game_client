use std::{net::TcpStream, io::stdin, process};
use lib_uno_game::{Player, Game, Packet};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    handle_connection(stream);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = String::new();
    let mut player = Player::new();

    // Get name from user
    stdin().read_line(&mut buf).unwrap();
    player.set_name(&buf.trim());

    // Send initial data, like name
    let mut packet = Packet::new(&None, &Some(player));
    packet.write(&mut stream).unwrap_or_else(|error| {
        eprintln!("Write failed: {error}");
        process::exit(1);
    });

    // Recieve initial cards
    let packet = Packet::read(&mut stream).unwrap();

    loop {
        // Recieve Game struct

        // Display card

        // Send card
    }
}
