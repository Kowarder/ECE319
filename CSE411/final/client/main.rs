use eframe::{egui, Frame, NativeOptions, run_native};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use std::thread;
use serde::{Deserialize, Serialize};
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use std::sync::mpsc;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataItem {
    index: usize,
    number: f64,
}
#[derive(Serialize, Deserialize, Debug)]
enum ClientAction {
    Add(DataItem),
    Delete(usize),
    Modify(DataItem),
}
struct MyApp {
    data_items: Vec<DataItem>,
    input_index: String,
    input_number: String,
    selected_index: Option<usize>,
    ws_sender: UnboundedSender<Message>,
    // ws_receiver: UnboundedReceiver<Message>,
    ui_receiver: mpsc::Receiver<DataItem>,
}

impl MyApp {
    fn new(ws_sender: UnboundedSender<Message>, ui_receiver: mpsc::Receiver<DataItem>) -> Self {
        Self {
            ws_sender,
            ui_receiver,
            data_items: Vec::new(),
            input_index: String::new(),
            input_number: String::new(),
            selected_index: None,
        }
    }

    fn send_message(&self, message: String) {
        let _ = self.ws_sender.unbounded_send(Message::Text(message));
    }

    fn update_data_items(&mut self, message: Message) {
        if let Ok(text) = message.to_text() {
            if let Ok(data_item) = serde_json::from_str::<DataItem>(text) {
                self.data_items.push(data_item);
            }
        }
    }

    // fn sort_data_default(&mut self) {
        
    // }
    // fn sort_data_htol(&mut self) {
        
    // }
    // fn sort_data_ltoh(&mut self) {
        
    // }
    // fn sort_new(&mut self) {
        
    // }

    // fn visual(&mut self, ui:&mut egui::Ui) {

    // }

    fn add_data(&mut self) {
        if let (Ok(index), Ok(number)) = (self.input_index.parse::<usize>(), self.input_number.parse::<f64>()) {
            let message = format!(r#"{{"Add":{{"index":{},"number":{}}}}}"#, index, number);
            println!("add {:?}", message);
            self.send_message(message);
        }
    }

    fn modify_data(&mut self) {
        if let Some(selected_index) = self.selected_index {
            if let Ok(number) = self.input_number.parse::<f64>() {
                let message = format!(r#"{{"Modify":{{"index":{},"number":{}}}}}"#, selected_index, number);
                println!("modify {:?}", message);
                self.send_message(message);
            }
        }
    }

    fn delete_data(&mut self) {
        if let Some(selected_index) = self.selected_index {
            let message = format!(r#"{{"Delete":{}}}"#, selected_index);
            println!("delete {:?}", message);
            self.send_message(message);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        egui::SidePanel::left("side panel").show(ctx, |ui| {
            ui.heading("Sort method");
            if ui.button("From H to L").clicked() {
                self.data_items.sort_by(|a, b| b.number.partial_cmp(&a.number).unwrap());
            }
            if ui.button("From L to H").clicked() {
                self.data_items.sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
            }
            if ui.button("Lastest Data").clicked() {
                self.data_items.sort_by(|a, b| a.index.cmp(&b.index));
            }
            ui.separator();
            ui.heading("Data Operation");
            if ui.button("Modify").clicked() {
                self.modify_data();
            }
            if ui.button("Add").clicked() {
                self.add_data();
            }
            if ui.button("Delete").clicked() {
                self.delete_data();
            }
            if ui.button("Clear").clicked() {
                self.input_index.clear();
                self.input_number.clear();
            }
            while let Ok(data_item) = self.ui_receiver.try_recv() {
                self.data_items.push(data_item);
                ctx.request_repaint();
                println!("updated data_items: {:?}", self.data_items);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, data_item) in self.data_items.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.set_min_width(available_width);
                        if ui.selectable_label(self.selected_index == Some(idx), format!("{}: {}", data_item.index, data_item.number)).clicked() {
                            self.selected_index = Some(idx);
                            self.input_index = data_item.index.to_string();
                            self.input_number = data_item.number.to_string();
                        }
                    });
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            
                ui.label("Index:");
                ui.text_edit_singleline(&mut self.input_index);
                ui.label("Number:");
                ui.text_edit_singleline(&mut self.input_number);
            
            
        });
        while let Ok(data_item) = self.ui_receiver.try_recv() {
            println!("Received data in UI:{:?}", data_item);
            self.data_items.push(data_item);
            ctx.request_repaint();
        }
    }
}

fn main() {
    let (ws_sender, mut ws_receiver) = unbounded();
    let (ui_sender, ui_receiver) = mpsc::channel();

    // WebSocket communication thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await.expect("Failed to connect");
            let (write, mut read) = ws_stream.split();

            // Read incoming WebSocket messages and send them to the eframe thread
            while let Some(message) = read.next().await {
                if let Ok(msg) = message {
                    if msg.is_text() {
                        let data_text = msg.to_text().unwrap();
                        println!("Rceived text:{}", data_text);
                        match serde_json::from_str::<Vec<DataItem>>(&data_text) {
                            Ok(data_items) => {
                                for data_item in data_items {
                                    ui_sender.send(data_item).unwrap();
                                }
                            }
                            Err(e) => {
                                println!("Failed to parse data_items: {}", e);
                            }
                        }
                        
                        
                    }
                }
            }
        });
    });

    let app = MyApp::new(ws_sender, ui_receiver);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My App", native_options, Box::new(|_cc| Box::new(app)));
}


// Test the sending message and the server receiver
// use eframe::{egui, Frame, NativeOptions, run_native};
// use serde::{Serialize, Deserialize};

// use futures_util::{stream::StreamExt, sink::SinkExt}; // For splitting the stream and sending messages
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
// use tokio::net::TcpStream;
// use futures_util::stream::SplitSink;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct DataItem {
//     index: usize,
//     number: f64,
// }

// #[derive(Serialize, Deserialize, Debug)]
// enum ClientAction {
//     Add(DataItem),
//     Delete(usize),
//     Modify(DataItem),
// }

// struct Client {
//     ws_sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
// }

// impl Client {
//     async fn new(url: &str) -> Self {
//         let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
//         let (write, _read) = ws_stream.split();
//         Self {
//             ws_sink: write,
//         }
//     }

//     async fn send_action(&mut self, action: ClientAction) {
//         let message = serde_json::json!(action).to_string();
//         self.ws_sink.send(Message::Text(message)).await.expect("Failed to send message");
//     }
// }

// #[tokio::main]
// async fn main() {
//     let mut client = Client::new("ws://127.0.0.1:8080").await;
//     // Example usage
//     let add_item = DataItem {
//         index: 101,
//         number: 0.5,
//     };
//     client.send_action(ClientAction::Add(add_item)).await;

//     let delete_index = 50;
//     client.send_action(ClientAction::Delete(delete_index)).await;

//     let modify_item = DataItem {
//         index: 30,
//         number: 0.75,
//     };
//     client.send_action(ClientAction::Modify(modify_item)).await;
// }
