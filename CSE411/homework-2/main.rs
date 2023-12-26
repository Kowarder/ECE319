use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};
use std::thread;
use std::io;
extern crate lazy_static;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::Arc;
extern crate threadpool;
use threadpool::ThreadPool; 

lazy_static! {
    static ref global_vec: Mutex<Vec<node>> = Mutex::new(Vec::new());
}
struct node {
    key_len: String,
    key: String,
    value_len: String,
    value: String
}

// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Job>,
// }

// impl ThreadPool {
//     fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);
//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));
//         let mut workers = Vec::with_capacity(size);
//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }
//         ThreadPool { workers, sender }
//     }

//     fn execute<F>(&self, f:F) 
//     where
//         F:FnOnce() + Send + 'static,
//         {
//             let job = Box::new(f);
//             self.sender.send(job).unwrap();
//         }
// }

// struct Worker {
//     id: usize,
//     thread: thread::JoinHandle<()>,
// }
// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             while let Ok(job) = receiver.lock().unwrap().recv(){
//                 job.call_box();
//             }
//         });
//         Worker { id, thread }
//     }
    
// }
// type Job = Box<dyn FnBox + Send + 'static>;
// trait FnBox {
//     fn call_box(self: Box<Self>);
// }
// impl <F: FnOnce()>FnBox for F {
//     fn call_box(self:Box<F>) {
//         (*self)()
//     }
// }

fn main() -> io::Result<()>{
    
    let listener = TcpListener::bind("127.0.0.1:1895").unwrap();
    // let mut thread_vec : Vec<thread::JoinHandle<()>> = Vec::new();

    // for stream in listener.incoming() {
    //     let _stream = stream.unwrap();
        
    //     let handle = thread::spawn(move || {
    //         handle_connection(_stream).unwrap_or_else(|error| eprintln!("{:?}", error));
    //     });
    //     thread_vec.push(handle);
    // }

    // for handle in thread_vec {
    //     handle.join().unwrap();
    // }
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()>  {
    let mut buffer = [0; 512];
    let mut store = global_vec.lock().unwrap();

    for _ in 0..1000 {
        let bytes = stream.read(&mut buffer).unwrap();
        if bytes == 0 {
          return Ok(());  
        }
        let data_string = String::from_utf8_lossy(&buffer);
        // println!("{}", data_string);

        let mut parts = data_string.split("\n");
        let part1 = parts.next().unwrap_or_default();
        let mut flag = 0;
        if part1 == "PUT" {
            println!("Now is put");
            let part2 = parts.next().unwrap_or_default();
            let part3 = parts.next().unwrap_or_default();
            let part4 = parts.next().unwrap_or_default();
            let part5 = parts.next().unwrap_or_default();

            let index = store.iter().position(|n| n.key == part3);
            if let Some(index) = index {
                store.drain(index..index+1);
            }
        
            let Node_temp = node {
                key_len: part2.to_string(),
                key: part3.to_string(),
                value_len: part4.to_string(),
                value: part5.to_string(),
            };
            store.push(Node_temp);
            println!("already push");
            stream.write(b"OK\n\r").unwrap();

        } else if part1 == "GET" {
            println!("Now is get");
            let part2 = parts.next().unwrap_or_default();
            let part3 = parts.next().unwrap_or_default();
            let mut flag = 0;
            for item in store.iter() {
                if item.key == part3 {
                    let mut rst = String::new();
                    rst = ("OK\n").to_string() + &item.value_len + "\n" + &item.value + "\n\r";
                    stream.write(rst.as_bytes()).unwrap();
                    flag = 1;
                }
            }
            if flag == 0 {
                stream.write(b"ERROR\n\r").unwrap();
            }
        } else if part1 == "DEL" {
            println!("Now is del");
            let part2 = parts.next().unwrap_or_default();
            let part3 = parts.next().unwrap_or_default();
            let index = store.iter().position(|n| n.key == part3);
            if let Some(index) = index {
                store.drain(index..index+1);
                stream.write(b"OK\n\r").unwrap();
            } else {
                stream.write(b"ERROR\n\r").unwrap();
            }
        }
        // stream.write(&buffer[..bytes]).unwrap();
    }
    
    Ok(())
}
