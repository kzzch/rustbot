use std::io::net::tcp::TcpStream;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::buffered::BufferedStream;

fn main() {
  let ServerAddress = Ipv4Addr(127, 0, 0, 1); // Connect to localhost
  let ServerPort = 6667;
  let SocketAddress = SocketAddr { ip: ServerAddress, port: ServerPort };
  let mut isConnected = false;
  
  let mut stream = BufferedStream::new(TcpStream::connect(SocketAddress).unwrap());

  stream.write(bytes!("NICK rustbot\r\n"));
  stream.write(bytes!("USER rustbot 8 * :rustbot\r\n"));
  stream.flush();
 
  let mut g = 1;
  while g > 0 {
    let r = stream.read_line();
    print(format!("{}", r));
    match r { 
      Some(_) => {
        if r.clone().unwrap().contains("004")  {
          isConnected = true;
        } else if r.clone().unwrap().contains("PING") {
          println("PONG.");
          stream.write(bytes!("PONG\r\n"));
          stream.flush();
        }
          
      }
      None => {
        println!("End of Stream!");
        g = 0;
      }
    }
  }
  println("Connection timed out!");
}
