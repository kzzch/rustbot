// src/rustdrop.rs
use std::io::net::tcp::TcpStream;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::buffered::BufferedStream;

fn main() {
  let ServerAddress = Ipv4Addr(63, 245, 216, 214); // irc.mozilla.org
  // let ServerAddress = Ipv4Addr(127, 0, 0, 1);
  let ServerPort = 6667;
  let SocketAddress = SocketAddr { ip: ServerAddress, port: ServerPort };
  
  let mut stream = BufferedStream::new(TcpStream::connect(SocketAddress).unwrap());

  stream.write(bytes!("NICK rustbot\r\n"));
  stream.write(bytes!("USER rustbot 8 * :rustbot\r\n"));
  stream.flush();

  loop {
    let r = stream.read_line();
    for value in r.iter() {
      print(*value);
      let input: ~[&str] = value.split(' ').collect();
      if input[0].contains("PING") {
        let response: &str = format!("PONG {}\r\n", input[1]);
        print(response);
        stream.write(response.as_bytes());
        stream.flush();
      }
      if input[1].contains("004") {
        foo(&mut stream);
      }
    }
  }
}

fn foo(s: &mut BufferedStream<TcpStream>) {
  loop {
    let r = s.read_line();
    for value in r.iter() {
      print(*value);
      let input: ~[&str] = value.split(' ').collect();
      if input[0].contains("PING") {
        let response: &str = format!("PONG {}\r\n", input[1]);
        print(response);
        s.write(response.as_bytes());
        s.flush();
      }
    }
  }
}
