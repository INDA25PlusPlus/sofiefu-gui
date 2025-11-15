use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write};

// serversida, startar först, Tcplistener för att vänta på anslutningar
pub fn start_server() -> std::io::Result<TcpStream> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)?;

    let (mut stream, client_addr) = listener.accept()?; // blocks until a client connects
    println!("client connected");
    stream.set_nonblocking(true)?;
    return Ok(stream);
}

// klientsidan, startar efter servern, använder Tcpstream::connect för att ansluta
pub fn start_client() -> std::io::Result<TcpStream> {
    let addr =  "127.0.0.1:8080";
    let mut stream = TcpStream::connect(addr)?;
    stream.set_nonblocking(true)?;
    return Ok(stream);
}

pub fn send_message(stream: &mut TcpStream, msg: String) {
    println!("The message is {} with length {}", msg, msg.len());
    match stream.write(msg.as_bytes()) {
        Ok(_) => {println!("Move sent to opponent"); return; },
        Err(e) => {println!("Failed to send message: {}", e); return;}
    }
}

pub fn try_receive_message(stream: &mut TcpStream) -> std::io::Result<Option<String>> {
    let mut msg_buf = [0u8; 128];
    match stream.read_exact(&mut msg_buf) {
        Ok(_) => return Ok(Some(String::from_utf8_lossy(&msg_buf).to_string())),  // received exactly 128 bytes 
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(None), // no full packet, but on nonblocking
        Err(e) => panic!("IO error: {}", e),    
    };

}


/*
Tips: För att testa era program lokalt kan ni använda er av loopbackadressen 127.0.0.1. 
*/