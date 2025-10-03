use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{Read, Write};

const addr: &str = "10.216.184.25:3000";
// serversida, startar först, Tcplistener för att vänta på anslutningar
pub fn start_server() -> std::io::Result<TcpStream> {
    let listener = TcpListener::bind(addr)?;

    let (stream, client_addr) = listener.accept()?; // blocks until a client connects
    println!("client connected");
    return Ok(stream);
}

// klientsidan, startar efter servern, använder Tcpstream::connect för att ansluta
pub fn start_client() -> std::io::Result<TcpStream> {
    let mut stream = TcpStream::connect(addr)?;
    stream.set_nonblocking(true)?;
    return Ok(stream);
}

pub fn send_message(stream: &mut TcpStream, msg: String) -> std::io::Result<()> {
    stream.write(msg.as_bytes())?;
    return Ok(());
}

pub fn try_receive_message(stream: &mut TcpStream) -> Option<String> {
    let mut msg_buf = [0u8; 128];
    match stream.read(&mut msg_buf) {
        Ok(0) => return None, // connection closed
        Ok(n) => return Some(String::from_utf8_lossy(&msg_buf[..n]).to_string()),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return None,
        Err(e) => { panic!("IO error: {e}"); }
    };
}


/*
Tips: För att testa era program lokalt kan ni använda er av loopbackadressen 127.0.0.1. 
*/