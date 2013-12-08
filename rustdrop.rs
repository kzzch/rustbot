use std::str;
use std::vec;
use std::rt::io::{Reader, Writer};
use std::rt::io::net::tcp::TcpStream;
use std::rt::io::net::ip::{Ipv4Addr, SocketAddr};
use std::rt::io::buffered::BufferedStream;

fn main() {
  let ServerAddress = Ipv4Addr(127, 0, 0, 1); // Connect to localhost
  let ServerPort = 6667;
  let SocketAddress = SocketAddr { ip: ServerAddress, port: ServerPort };
  
  let tcp_stream = TcpStream::connect(SocketAddress).unwrap();
  
  let mut buf: ~[u8] = vec::from_elem(1024, 0u8);
  let mut stream = BufferedStream::new(tcp_stream);

  stream.write(bytes!("NICK rustbot\r\n"));
  stream.write(bytes!("USER rustbot 8 * :rustbot\r\n"));
  stream.flush();
 
  let mut g = 1;
  while g > 0 {
    let r = stream.read(buf);
    let s = str::from_utf8_slice(buf);
    match r { 
      Some(nread) => {
        if s.contains("004")  { 
          bot_loop()                        // This indicates we've connected to the server
        } else {                            // successfully.
          match s {
            _ => {
              ()
              /* println(fmt!("Read %u bytes", nread));
               * println(fmt!("%s", s));
               */
            }
          }
        }
      },
      None => {
        println!("End of Stream!");
        g = 0;
      }
    }
  }

  println("Connection timed out!");
}

fn bot_loop() {
  println!("You made it, yay!");
  loop {};
}

/*
 * ^(?:[:@]([^\\s]+) )?([^\\s]+)(?: ((?:[^:\\s][^\\s]* ?)*))?(?: ?:(.*))?$
 * regex for parsing irc messages
 */
