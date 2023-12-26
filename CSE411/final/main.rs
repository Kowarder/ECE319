use futures_util::{stream::SplitSink, StreamExt, SinkExt};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use rand::Rng;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataItem {
    index: usize,
    number: f64,
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientAction {
    Add(DataItem),
    Delete(usize), // Index of the item to delete
    Modify(DataItem),
}

type ClientMap = Arc<RwLock<HashMap<usize, SplitSink<WebSocketStream<TcpStream>, Message>>>>;
type DataStorage = Arc<Mutex<Vec<DataItem>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let clients = ClientMap::new(RwLock::new(HashMap::new()));
    let data = DataStorage::new(Mutex::new(initialize_data()));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    println!("WebSocket server is running on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        println!("Accepted new connection");
        let clients = clients.clone();
        let data = data.clone();

        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                let (write, mut read) = ws_stream.split();
                let client_id = rand::random::<usize>();

                {
                    let mut clients = clients.write().await;
                    clients.insert(client_id, write);
                } // Drop the write lock here

                let initial_data = serde_json::to_string(&*data.lock().await).unwrap();
                let mut clients_guard = clients.write().await;
                if let Some(client) = clients_guard.get_mut(&client_id) {
                    let _ = client.send(Message::Text(initial_data)).await;
                }
                drop(clients_guard); // Explicitly drop the write lock
                
                // while let Some(message) = read.next().await {
                //     match message {
                //         Ok(msg) => {
                //             if msg.is_text() {
                //                 handle_client_message(&msg, &data, &clients).await;
                //             }
                //         }
                //         Err(e) => {
                //             eprintln!("WebSocket error: {:?}", e);
                //             break;
                //         }
                //     }
                while let Some(message) = read.next().await {
                    match message {
                        Ok(msg) => println!("Received message: {:?}", msg),
                        Err(e) => eprintln!("WebSocket error: {:?}", e),
                    }
                }
                println!("test after read");
            

                clients.write().await.remove(&client_id);
            }
        });
    }
}

fn initialize_data() -> Vec<DataItem> {
    let mut rng = rand::thread_rng();
    (0..100).map(|index| DataItem {
        index,
        number: rng.gen::<f64>(), // Generate a random f64 number
    }).collect()
}

async fn handle_client_message(msg: &Message, data: &DataStorage, clients: &ClientMap) {
    let text = msg.to_text().unwrap();
    let action: ClientAction = serde_json::from_str(&text).unwrap();

    let mut updated_data = String::new();
    {
        let mut data_lock = data.lock().await;
        match action {
            ClientAction::Add(item) => data_lock.push(item),
            ClientAction::Delete(index) => data_lock.retain(|item| item.index != index),
            ClientAction::Modify(modified_item) => {
                if let Some(item) = data_lock.iter_mut().find(|item| item.index == modified_item.index) {
                    *item = modified_item;
                }
            },
        }
        updated_data = serde_json::to_string(&*data_lock).unwrap();
    } // Release data lock here

    // Broadcast updated data outside the data lock scope
    let mut clients = clients.write().await;
    for (_, client) in clients.iter_mut() {
        let _ = client.send(Message::Text(updated_data.clone())).await;
    }
}
