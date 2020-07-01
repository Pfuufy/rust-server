// We bring prelude into scope to get access to certain
// traits that let us read and write to streams.
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // TcpListener::bind() is basically a new() function, but
    // its called bind because in networking you "bind" to a 
    // specific port. bind() returns a Result<T, E>.
    //
    // unwrap() is an error handler. There might be an issue
    // binding to the specified port. It requires administrative
    // privelages to listen to a port from 1024 or below.
    // unwrap() stops the program if an error happens.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listener.incoming() gives us an iterator of a sequence of
    // streams. A stream is an open connection between the client
    // and the server. A connection is the name for the whole
    // process of the client opening a connection with the server,
    // the server generates a response, and the server closes
    // the connection.
    //
    // listener.incoming() actually iterates over connection attempts,
    // meaning it's possible that they'll fail. They might fail
    // for many reasons, many of them OS specific. For example,
    // OS's only have so many connections the can have open at once.
    // If too many connections are trying to be made then some
    // will be dropped until others are closed.
    for stream in listener.incoming() {
        // unwrap() here just ends the program if there's an error.
        // For a real server, it is important to handle the errors
        // gracefully.
        let stream = stream.unwrap();

        handle_connnection(stream);
    }
}

// TcpStream needs to be mutable because it keeps internal state
// of what data has been accessed and that needs to be able
// to change.
fn handle_connnection(mut stream: TcpStream) {
    // The buffer is 512 bytes. This is enough to hold data
    // for a basic request. If we needed a buffer of an arbitrary
    // size, we would have to make buffer management more complex.
    let mut buffer = [0; 512];

    // This reads bytes from TcpStream and puts them in the buffer.
    stream.read(&mut buffer).unwrap();

    // String::from_utf8_lossy() takes &[u8] as input and produces
    // a String from it. The "lossy" part refers to how it
    // handles invalid UTF-8 sequences. It will print �.
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // b"" syntax creates a byte string. Byte string necessary
    // because we're reading raw bytes into the buffer.
    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("200 OK", "html/hello.html")
    } else {
        ("404 NOT FOUND", "html/404.html")
    };

    let status_line = format!("HTTP/1.1 {}\r\n\r\n", status);
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // if buffer.starts_with(get) {
    //     let file = fs::read_to_string("hello.html").unwrap();

    //     // \r\n is written twice because theres an empty
    //     // headers line after the first \r\n
    //     // Concatenate the file to the response.
    //     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", file);
    
    //     // stream.write() takes &[u8]
    //     stream.write(response.as_bytes()).unwrap();
    //     // stream.flush() will wait and prevent the program from
    //     // continuing until all bytes have been written to the
    //     // connection.
    //     stream.flush().unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();

    //     let response = format!("{}{}", status_line, contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
}

// When we recieve a request, the first line is the request line.
// Method Request-URI HTTP-Version CRLF
// I already know what all of those are except CRLF
// CRLF stands for carriage return and line feed. It's a vestige
// from the typewriter days. Basically all it is is \r\n, which
// just starts a new line when printed.

// When we send back a response, they have this format:
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body