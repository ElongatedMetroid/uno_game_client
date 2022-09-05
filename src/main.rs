use std::{net::TcpStream, io::stdin};
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
    packet.write(&mut stream).unwrap();

    // Recieve initial cards

    loop {
        // Recieve Game struct

        // Display card

        // Send card
    }
}
