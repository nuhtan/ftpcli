use std::{io::{BufRead, BufReader, Error, ErrorKind, Write}, net::IpAddr, process::{Command, Stdio}, sync::{Arc, Mutex}};

#[derive(Clone)]
pub struct SharedState {
    pub name: String,
    pub password: String,
    pub address: IpAddr,
    pub port: u16,
    pub selected_files: Arc<Mutex<Vec<String>>>,
    pub currently_downloading: Arc<Mutex<Vec<(String, u8)>>>,
}

impl SharedState {
    pub fn new(
        name: String,
        password: String,
        address: IpAddr,
        port: u16,
    ) -> Result<SharedState, Error> {
        match try_connection() {
            ConnectionState::Success => println!("Connected Fully"),
            ConnectionState::LoginError => println!("Login Error"),
            ConnectionState::ConnectionError => println!("Connection Failure"),
        }
        Err(Error::new(ErrorKind::Other, "Sample"))
    }

    pub fn new_auto_port(
        name: String,
        password: String,
        address: IpAddr,
    ) -> Result<SharedState, Error> {
        try_connection();
        Err(Error::new(ErrorKind::Other, "Sample 2"))
    }

}

// Try to connect with the given credentials.
// If the process exits without input then the address or port is incorrect.
// Otherwise 
fn try_connection() -> ConnectionState {
    let mut child = Command::new("ssh")
        .arg("nuhtan@192.168.1.7")
        .arg("-T")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start ssh");
    let mut output = BufReader::new(
        child.stdout.take().expect("Failed to open ssh output")
    );
    let mut inputed = false;
    loop {
        let mut buf = Vec::new();
        output.read_until(b'\n', &mut buf).unwrap();
        let line = String::from_utf8(buf).unwrap();
        if line == "Linux\n" && inputed {
            return ConnectionState::Success;
        } else if line == "" && inputed {
            println!("{:?}", line);
            return ConnectionState::LoginError;
        }
        if line != "" {
            // println!("{:?}", line);
            let input = child.stdin.as_mut().unwrap();
            input.write_all(b"uname\n").unwrap();
            inputed = true;
        }
        match child.try_wait() {
            Ok(status) => match status {
                Some(_) => return ConnectionState::ConnectionError,
                None => {},
            },
            Err(_) => {},
        }
    }
}

enum ConnectionState {
    Success,
    LoginError,
    ConnectionError
}