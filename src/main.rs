use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use structs::ChallengeResolve;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::structs::{Challenge, MD5HashInput, Message, PublicLeaderBoard, Player};
use crate::structs::ChallengeAnswer;
use crate::structs::MD5HashOutput;

use md5_hash::MD5Hash;

mod structs;
mod md5_hash;

pub struct Game {
    leader_board: Vec<Player>,
}

fn main() {
    // Connect to local server
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            let message = Message::Hello;
            send_message(&stream, message);
            let mut game: Game = Game::create_game();
            game.get_messages(&stream);
            println!("Successfully connected to server, waiting to start");
        },
        Err(_) => panic!("Unable to connect to server, is it running ?"),
    }

}

fn send_message(mut stream: &TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let binary_message = message.as_bytes();
        let message_length = binary_message.len() as u32;
        let biinary_message_length = message_length.to_be_bytes();

        stream.write(&biinary_message_length).unwrap(); // message size
        stream.write(binary_message).unwrap(); // binary message
    }
}


impl Game {
    fn create_game () -> Game{
        const GAME: Game = Game{ leader_board: vec![] };
        return GAME
    }

    fn get_messages(&mut self, mut stream: &TcpStream){
        loop {
            let mut buf_size = [0; 4];
            stream.read(&mut buf_size);
            let res_size = u32::from_be_bytes(buf_size);
            if res_size == 0 {
                continue
            }
            println!("{}", res_size);

            let mut buf = vec![0; res_size as usize];
            stream.read(&mut buf);
            let string_receive = String::from_utf8_lossy(&buf);
            println!("{:?}", string_receive);

            match serde_json::from_str(&string_receive) {
                Ok(message) => self.parse_message(stream, message),
                Err(err) => println!("Error while parsing message = {}", err),
            }
        }
    }

    fn parse_message(&mut self, mut stream: &TcpStream, message: Message) {
        match message {
            Message::Welcome { version } => { // Notify server
                println!("{}", version);
                let mut rng = rand::thread_rng();
                let number: u8 = rng.gen();
                let answer = Message::Subscribe { name: "Player".to_string() + &number.to_string() };
                send_message(&stream, answer);
            }

            Message::SubscribeResult(subscribeResult) => { // server notify connection is ok
                println!("Subscribe: {:?}", subscribeResult);
            }

            Message::PublicLeaderBoard(leaderBoard ) => { // server sent us the leaderBoard
                for player in leaderBoard{
                    self.leader_board.push(Player{
                        name: player.name,
                        score: 0,
                        steps: 0,
                        stream_id: "0".to_string(),
                        total_used_time: 0.0,
                        is_active: false,
                    });
                }
                println!("LeaderBoard = {:?}", self.leader_board.get(0).unwrap().name);
            },

            Message::ChallengeTimeout(message) => {
                println!("Timeout = {}", message);
            },

            Message::RoundSummary{ challenge, chain} => {
                println!("Challenge = {} | Chain = {:?}", challenge, chain);
            }

            // _________________________________________________CHALLENGE _____________________________________
            Message::Challenge(challenge) => {
                let mut message: Message;
                let next_player: Option<&Player> = self.leader_board.get(0);
                let answer = match challenge {
                    Challenge::MD5Hash(md5) => {
                        let solver = MD5Hash::new(md5);
                        ChallengeAnswer::MD5Hash(solver.solve())
                    }
                };

                match next_player {
                    None => {
                        message = Message::ChallengeResult {
                            answer,
                            next_target: "".to_string()
                        };
                        send_message(&stream, message);
                    }
                    Some(target) => {

                        message = Message::ChallengeResult {
                            answer,
                            next_target: target.clone().name
                        };
                        send_message(&stream, message);
                    }
                }
            }
            // _____________________________________________FIN_CHALLENGE _____________________________________

            Message::EndOfGame { leader_board } => {
                println!("leader_board = {:?}", leader_board);
                stream.flush();
                exit(0);
            }
            _ => {print!("Error")}
        }

    }
}
