use std::io::{Read, BufReader};
use std::net::TcpStream;
use std::str;


fn calculate_center_and_angle(top_left: (f64, f64), _bottom_left: (f64, f64), bottom_right: (f64, f64), top_right: (f64, f64)) -> ((f64, f64), f64) {
    // 计算中心点
    let center_x = (top_left.0 + bottom_right.0) / 2.0;
    let center_y = (top_left.1 + bottom_right.1) / 2.0;
    let center = (center_x, center_y);

    // 计算夹角
    let delta_x = top_right.0 - top_left.0;
    let delta_y = top_right.1 - top_left.1;
    let angle = delta_y.atan2(delta_x).to_degrees();

    (center, angle)
}

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
                    // 清洗和解析接收到的数据
                    let received_data = str::from_utf8(&buffer).unwrap()
                        .trim_matches(char::from(0)) // 清除空字符
                        .trim(); // 清除首尾空白

                    println!("Received: {}", received_data);

                    let points: Vec<f64> = received_data.split(',')
                        .filter_map(|s| s.parse::<f64>().ok())
                        .collect();

                    if points.len() == 8 {
                        let (center, angle) = calculate_center_and_angle(
                            (points[0], points[1]), // top_left
                            (points[2], points[3]), // bottom_left
                            (points[4], points[5]), // bottom_right
                            (points[6], points[7]), // top_right
                        );
                        println!("Center: {:?}", center);
                        println!("Angle: {:.2} degrees", angle);
                    } else {
                        println!("Invalid data format.");
                    }
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
