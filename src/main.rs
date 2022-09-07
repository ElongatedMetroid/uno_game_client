use std::{net::TcpStream, io::{self, Write, BufReader, BufRead}, process};
use lib_uno_game::{Player, Game, Packet};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    handle_connection(stream);
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = String::new();
    let mut player = Player::new();

    // Get name from user
    print!("Enter a name (This will be seen by other players): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    player.set_name(&buf.trim());

    // Send initial data, like name
    let mut packet = Packet::new(&None, &Some(player.clone()));
    packet.write(&mut stream).unwrap_or_else(|error| {
        eprintln!("Write failed: {error}");
        process::exit(1);
    });
println!("WRITE 1");
    let mut buf_reader = BufReader::new(&stream);
    buf.clear();
    buf_reader.read_line(&mut buf).unwrap();
    player.set_turn(buf.trim().parse().unwrap());
println!("READ 1");

    loop {
        // Recieve Game struct
        packet = Packet::read(&mut stream).unwrap();

        // Display card
        println!("Current Card {:?}", packet.game().as_ref().unwrap().current_card());
        // Display the cards the player has
        println!("Your Hand {:?}", packet.get_player(&player).as_ref().unwrap().cards());

        loop {
            buf.clear();

            print!("Which Card Would You Like To Use: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf).unwrap();

            let index = match buf.trim().parse::<usize>() {
                Ok(index) => index,
                Err(_) => {
                    eprintln!("Please enter a number");
                    continue;
                }
            };

            let cards = packet.get_player(&player).as_ref().unwrap().cards();

            if cards.len() - 1 < index {
                eprintln!("Please enter a valid number");
                continue;
            }

            packet.set_card(&cards[index].clone());
            packet.write(&mut stream).unwrap();

            // Read response
            let (success, text) = packet.success();

            if !success {
                eprintln!("Invalid card: {:?}", text);
                continue;
            }
            
            break;
        }
    }
}
