use reqwest;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct ClientLegacy {
    id: i32,
    first_name: String,
    last_name: String,
    age: i32,
    address: String,
    country: String,
    items_purchased: i32,
    is_updated: bool,
}

#[derive(Serialize, Deserialize)]
struct Client {
    name: String,
    age: i32,
    address: String,
    items_purchased: i32,
}

#[tokio::main]
async fn main()-> Result<(), reqwest::Error>  {
    let pipe_name = "/tmp/legacy";

    // Create the named pipe (if it doesn't exist)
    //create_named_pipe(pipe_name);

    // Sender application
    let message = "Hello, Luiz Felipe Escandiuzzi!";
    send_data(pipe_name, message);

    let data = receive_data(pipe_name);

    let clients = convert_data(data);

    let json = serde_json::to_string(&clients).expect("Failed to serialize data");

    println!("Making the request {}", json.clone());

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/clients")
        .header("Content-Type", "application/json")
        .body(json.clone())
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        // Request was successful
        let response_text = response.text().await?;
        println!("Response: {}", response_text);
    } else {
        // Request failed
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

fn receive_data(pipe_name: &str) -> Vec<ClientLegacy> {
    // Open the named pipe for reading
    let mut pipe = OpenOptions::new()
        .read(true)
        .open(pipe_name)
        .expect("Failed to open named pipe for reading");

    // Read the data from the pipe
    let mut buffer = String::new();
    pipe.read_to_string(&mut buffer)
        .expect("Failed to read from named pipe");

    let data: Vec<ClientLegacy> = serde_json::from_str(&buffer).expect("Failed to deserialize");

    data
}

fn convert_data(data: Vec<ClientLegacy>) -> Vec<Client> {
    let mut result = Vec::new();

    for item in data {
        result.push(Client {
            name: format!("{} {}", item.first_name, item.last_name),
            age: item.age,
            address: format!("{}- {}", item.address, item.country),
            items_purchased: item.items_purchased,
        })
    }

    result
}

fn send_data(pipe_name: &str, message: &str) {
    // Open the named pipe for writing
    let mut pipe = OpenOptions::new()
        .write(true)
        .open(pipe_name)
        .expect("Failed to open named pipe for writing");

    // Write the message to the pipe
    pipe.write_all(message.as_bytes())
        .expect("Failed to write to named pipe");
}
