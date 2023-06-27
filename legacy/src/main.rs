use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct Client {
    id: i32,
    first_name: String,
    last_name: String,
    age: i32,
    address: String,
    country: String,
    items_purchased: i32,
    is_updated: bool,
}

fn main() -> Result<()> {
    let conn = Connection::open("legacy.db")?;

    /// Create Table
    //create_table(&conn)?;

    // Add temp data
    //insert_client(&conn, "John", "Doe", 30, "Temp Street", "England", 7, false)?;
    //insert_client(&conn, "Jane", "Smith", 25, "Temp Street", "England", 0, false)?;

    let clients = get_clients(&conn)?;

    // for client in clients {
    //     println!("ID: {}, Name: {}, Age: {}", client.id, client.first_name, client.age);
    // }

    let pipe_name = "/tmp/legacy";

    // Create the named pipe (if it doesn't exist)
    create_named_pipe(pipe_name);

    // Receiver application
    receive_data(pipe_name);
    send_response(clients);

    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clients (
            id               INTEGER PRIMARY KEY,
            first_name       TEXT NOT NULL,
            last_name        TEXT NOT NULL,
            age              INTEGER NOT NULL,
            address           TEXT NOT NULL,
            country          TEXT NOT NULL,
            items_purchased INTEGER,
            is_updated      BOOLEAN NOT NULL DEFAULT 0
            )",
        [],
    )?;
    Ok(())
}

fn insert_client(
    conn: &Connection,
    first_name: &str,
    last_name: &str,
    age: i32,
    address: &str,
    country: &str,
    items_purchased: i32,
    is_updated: bool,
) -> Result<()> {
    conn.execute(
        "INSERT INTO clients (first_name,
            last_name,
            age,
            address,
            country,
            items_purchased,
            is_updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[
            first_name,
            last_name,
            &age.to_string(),
            address,
            country,
            &items_purchased.to_string(),
            &is_updated.to_string(),
        ],
    )?;
    Ok(())
}

fn get_clients(conn: &Connection) -> Result<Vec<Client>> {
    let mut stmt = conn.prepare(
        "SELECT
                id, 
                first_name,
                last_name,
                age,
                address,
                country,
                items_purchased 
            FROM clients",
    )?;
    let client_iter = stmt.query_map([], |row| {
        Ok(Client {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            age: row.get(3)?,
            address: row.get(4)?,
            country: row.get(5)?,
            items_purchased: row.get(6)?,
            is_updated: true,
        })
    })?;

    let mut clients = Vec::new();
    for client_result in client_iter {
        clients.push(client_result?);
    }

    Ok(clients)
}

fn create_named_pipe(pipe_name: &str) {
    // Remove the pipe if it already exists
    fs::remove_file(pipe_name).ok();

    // Create the named pipe
    match nix::unistd::mkfifo(
        pipe_name,
        nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR,
    ) {
        Ok(_) => println!("Created named pipe: {}", pipe_name),
        Err(err) => eprintln!("Failed to create named pipe: {}", err),
    }
}

fn receive_data(pipe_name: &str) -> String {
    // Open the named pipe for reading
    let mut pipe = OpenOptions::new()
        .read(true)
        .open(pipe_name)
        .expect("Failed to open named pipe for reading");

    // Read the data from the pipe
    let mut buffer = String::new();
    pipe.read_to_string(&mut buffer)
        .expect("Failed to read from named pipe");

    buffer
}

fn send_response(clients: Vec<Client>) {
    let json = serde_json::to_string(&clients).expect("Failed to serialize data");

    let pipe_name = "/tmp/legacy";

    // Open the named pipe for writing
    let mut pipe = OpenOptions::new()
        .write(true)
        .open(pipe_name)
        .expect("Failed to open named pipe for writing");

    // Write the message to the pipe
    pipe.write_all(json.as_bytes())
        .expect("Failed to write to named pipe");
}
