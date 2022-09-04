use std::{net::TcpStream, collections::VecDeque, io::stdin};
use serde_derive::Serialize;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    deck: VecDeque<Card>,
    current_card: Card,
}

#[derive(Debug, Serialize)]
struct Player {
    name: String,
    turn: usize,
    cards: Vec<Card>,
}

#[derive(Clone, Debug, Serialize)]
enum CardKind {
    WildCard,
    DrawFour,
    DrawTwo,
    Cancel,
    Reverse,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, Serialize)]
struct Card {
    /// Color is None if it is a wildcard, or any card can be placed on it
    color: Color,
    kind: CardKind,
}

#[derive(Clone, Debug, Serialize)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,
}

impl Player {
    fn new() -> Player {
        Player { name: String::new(), turn: 0, cards: Vec::new() }
    }   
    fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    handle_connection(stream);
}

fn handle_connection(stream: TcpStream) {
    let mut buf = String::new();

    let mut player = Player::new();

    stdin().read_line(&mut buf).unwrap();
    player.set_name(&buf);

    // Send initial data, like name
    let send = bincode::serialize(&player);

    // Recieve initial cards

    loop {
        // Recieve Game struct

        // Display card

        // Send card
    }
}
