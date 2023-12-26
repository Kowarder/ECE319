// #![feature(test)]
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::{str, thread};
// extern crate test;
// use std::time::{Instant};

// fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
//     loop {
//         println!("Please input the request:");
//         let mut input_request = String::new();
//         io::stdin().read_line(&mut input_request).expect("failed to read");
//         if input_request == "put\n" {
//             println!("Please enter the key-len:");
//             let mut input_kl = String::new();
//             io::stdin().read_line(&mut input_kl).expect("failed to read");
//             println!("Please enter the key:");
//             let mut input_key = String::new();
//             io::stdin().read_line(&mut input_key).expect("failed to read");
//             println!("Please enter the value-len:");
//             let mut input_vl = String::new();
//             io::stdin().read_line(&mut input_vl).expect("failed to read");
//             println!("Please enter the value:");
//             let mut input_value = String::new();
//             io::stdin().read_line(&mut input_value).expect("failed to read");
//             let mut input = String::new();
//             input = "PUT\n".to_owned() + &input_kl + &input_key + &input_vl + &input_value;

//             stream.write(input.as_bytes()).expect("fail to write");
//         } else if input_request == "get\n" {
//             println!("Please enter the key-len:");
//             let mut input_kl = String::new();
//             io::stdin().read_line(&mut input_kl).expect("failed to read");
//             println!("Please enter the key:");
//             let mut input_key = String::new();
//             io::stdin().read_line(&mut input_key).expect("failed to read");
//             let mut input = String::new();
//             input = "GET\n".to_owned() + &input_kl + &input_key;

//             stream.write(input.as_bytes()).expect("fail to write");
//         } else if input_request == "del\n" {
//             println!("Please enter the key-len:");
//             let mut input_kl = String::new();
//             io::stdin().read_line(&mut input_kl).expect("failed to read");
//             println!("Please enter the key:");
//             let mut input_key = String::new();
//             io::stdin().read_line(&mut input_key).expect("failed to read");
//             let mut input = String::new();
//             input = "DEL\n".to_owned() + &input_kl + &input_key;

//             stream.write(input.as_bytes()).expect("fail to write");
//         } else {
//             break
//         }
//         let mut reader = BufReader::new(&stream);
//         let mut buffer: Vec<u8> = Vec::new();
//         reader.read_until(b'\r', &mut buffer).expect("fail to read into buffer");
//         println!("{}", str::from_utf8(&buffer).unwrap());
//     };
//     Ok(())
// }


fn main() -> io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:1895").unwrap();
    // let mut thread_vec : Vec<thread::JoinHandle<()>> = Vec::new();

    // thread::spawn(move || {
    //     println!("come in");
    //     handle_connection(stream);
    //     });

    // for handle in thread_vec {
    //         handle.join().unwrap();
    //     }
    loop {
        // let start_time = Instant::now();
        println!("Please input the request:");
        let mut input_request = String::new();
        io::stdin().read_line(&mut input_request).expect("failed to read");
        if input_request == "put\n" {
            println!("Please enter the key-len:");
            let mut input_kl = String::new();
            io::stdin().read_line(&mut input_kl).expect("failed to read");
            println!("Please enter the key:");
            let mut input_key = String::new();
            io::stdin().read_line(&mut input_key).expect("failed to read");
            println!("Please enter the value-len:");
            let mut input_vl = String::new();
            io::stdin().read_line(&mut input_vl).expect("failed to read");
            println!("Please enter the value:");
            let mut input_value = String::new();
            io::stdin().read_line(&mut input_value).expect("failed to read");
            let mut input = String::new();
            input = "PUT\n".to_owned() + &input_kl + &input_key + &input_vl + &input_value;

            stream.write(input.as_bytes()).expect("fail to write");
        } else if input_request == "get\n" {
            println!("Please enter the key-len:");
            let mut input_kl = String::new();
            io::stdin().read_line(&mut input_kl).expect("failed to read");
            println!("Please enter the key:");
            let mut input_key = String::new();
            io::stdin().read_line(&mut input_key).expect("failed to read");
            let mut input = String::new();
            input = "GET\n".to_owned() + &input_kl + &input_key;

            stream.write(input.as_bytes()).expect("fail to write");
        } else if input_request == "del\n" {
            println!("Please enter the key-len:");
            let mut input_kl = String::new();
            io::stdin().read_line(&mut input_kl).expect("failed to read");
            println!("Please enter the key:");
            let mut input_key = String::new();
            io::stdin().read_line(&mut input_key).expect("failed to read");
            let mut input = String::new();
            input = "DEL\n".to_owned() + &input_kl + &input_key;

            stream.write(input.as_bytes()).expect("fail to write");
        } else {
            break
        }
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\r', &mut buffer).expect("fail to read into buffer");
        println!("{}", str::from_utf8(&buffer).unwrap());
        // let elapsed_time = start_time.elapsed();
        // println!("{:?}", elapsed_time);
    };
        
    Ok(())
}
