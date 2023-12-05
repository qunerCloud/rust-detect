use std::io::{Read, BufReader};
use std::net::TcpStream;
use std::str;

fn main() {
    // 服务器的地址和端口
    let addr = "127.0.0.1:65535";
    
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("Successfully connected.");

            let mut reader = BufReader::new(stream);

                let mut buffer = vec![0; 1024]; // 创建一个缓冲区来接收数据
                match reader.read(&mut buffer) {
                    Ok(_) => {
                        let received_data = str::from_utf8(&buffer).unwrap();
                        println!("Received: {}", received_data);
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
