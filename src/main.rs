use std::env;
use std::net::UdpSocket;
use std::io;

fn main() {
    print!("{}[2J", 27 as char);

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("use [client] or [server] as a parameter");
        return
    }

    if args[1] == "server" {
        server();
    } else if args[1] == "client" {
        if args.len() < 3 {
            println!("When starting as a client you have to pass the ip address");
            return;
        }

        client(args[2].to_owned());
    }
}

fn server() {
    // first we create a socket of udp connection binding a device selected port from a localhost
    let socket = UdpSocket::bind("127.0.0.1:0").expect("the socket was not created correctly");

    // this is for showing server info
    let local_addr = socket.local_addr().expect("Failed to get local address");
    println!("Server started at:\n - ip: {}:{}", local_addr.ip(), local_addr.port());
    println!("Messages:");

    // we create a buffer, this buffer will be the way we send data to the server 
    let mut buf = [0; 1024];
    loop {
        // Receive data from clients
        let (amt, src) = match socket.recv_from(&mut buf) {
            Ok((amt, src)) => (amt, src),
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                continue;
            }
        };

        // Convert the received data to a string and make a response
        let received = std::str::from_utf8(&buf[..amt]).expect("Invalid UTF-8 data");
        socket.send_to("Ok".as_bytes(), src).expect("Failed to send data");

        println!("  - {}: {}", src, received);
    }
}

fn client(connect_to: String) {
    // we create a socket of udp connection binding a device selected port from a localhost
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to create socket");
    
    // we just give feedback of the process
    let local_addr = socket.local_addr().expect("Failed to get local address");
    println!("Client started at:\n - ip: {}:{}", local_addr.ip(), local_addr.port());

    // a connection message that will let us know when a client connected to the server
    socket.send_to("Connected".as_bytes(), connect_to.to_owned()).expect("Failed to send data");

    // i create the buffer
    let mut buf = [0; 1024];

    loop {
        // Receive data from clients
        let (amt, src) = match socket.recv_from(&mut buf) {
            Ok((amt, src)) => (amt, src),
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                continue;
            }
        };

        // receive the data
        let received = std::str::from_utf8(&buf[..amt]).expect("Invalid UTF-8 data");
        println!("  - {}: {}", src, received);

        // send data
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .expect("Failed to read line");
        
        if input.trim() != ""{
            socket.send_to(input.trim().as_bytes(), connect_to.to_owned()).expect("Failed to send data");
        }

    }
}

