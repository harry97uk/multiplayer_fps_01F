use bevy::prelude::*;
use renet::{
    ClientAuthentication,
    RenetClient,
    RenetConnectionConfig,
    RenetError,
    NETCODE_USER_DATA_BYTES,
};
use std::{ net::UdpSocket, time::SystemTime };
use store::game::{ GameEvent, GameState };
use dotenv::dotenv;
use std::env;

// This id needs to be the same that the server is using
const PROTOCOL_ID: u64 = 1208;

pub fn new_renet_client(username: &String) -> anyhow::Result<RenetClient> {
    // Load environment variables from the .env file
    dotenv().ok();

    // Retrieve the values using std::env::var
    let host = env::var("HOST").expect("HOST not set in .env");
    let port = env::var("PORT").expect("PORT not set in .env");

    let server_addr = format!("{}:{}", host, port).parse()?;
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;

    // Place username in user data
    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too big");
    }
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let client = RenetClient::new(
        current_time,
        socket,
        client_id,
        RenetConnectionConfig::default(),
        ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: Some(user_data),
        }
    )?;

    Ok(client)
}

pub fn receive_events_from_server(
    mut client: ResMut<RenetClient>,
    mut game_state: ResMut<GameState>,
    mut game_events: EventWriter<GameEvent>
) {
    while let Some(message) = client.receive_message(0) {
        // Whenever the server sends a message we know that it must be a game event
        let event: GameEvent = bincode::deserialize(&message).unwrap();
        trace!("{:#?}", event);

        // We trust the server - It's always been good to us!
        // No need to validate the events it is sending us
        game_state.consume(&event);

        // Send the event into the bevy event system so systems can react to it
        game_events.send(event);
    }
}

// If there's any error network we just panic 🤷‍♂️
pub fn handle_renet_error(mut renet_error: EventReader<RenetError>) {
    for err in renet_error.iter() {
        panic!("{}", err);
    }
}
