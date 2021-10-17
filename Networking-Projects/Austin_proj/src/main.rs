//Get access to traits that allow us to read and write to stream.
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    //Listener listens to TCP connections at the given address. Bind returns the TCPListener instance. Binding to this port could fail so we use
    //unwrap to deal with errors. 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //Incoming returns an iterator that gives a sequence of streams (of TcpStream). A stream is a connection between client and server.
    //TcpStream read from what the client sent and then allows the server to write a response to the stream.
    //The for loop processes each connection produces a series of streams to handle.
    //We need to unwrap the stream to check for failed connection attempts, to handle these faile attempts appropriately (for now shutting down).
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

//Handles the connections to the TCP port.
//We take in a mutable TcpStream type. We do this because the TcpStream keeps track of what data it returns to us. 
//It may read data we had not asked for and will save that data for the next time we ask for data.
fn handle_connection(mut stream: TcpStream) {

    //Creates a buffer of 1024 bytes.
    let mut buffer = [0; 1024];

    //Reads bytes from the TcpStream and puts them in the buffer.
    stream.read(&mut buffer).unwrap();

    //converts and prints bytes from buffer into a string. Will replace characters that aren't filled with request data.
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //b converts the string into a byte string.
    let get = b"GET / HTTP/1.1\r\n";

    let(status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    /* A LOT OF REPETITIVE CODE HERE THERE IS A BETTER WAY!!!!!
    //Check if buffer starts with get, if it does we recieved a request to /
    if buffer.starts_with(get) {
        //Read the contents of the html file as a string.
        let contents = fs::read_to_string("hello.html").unwrap();

        //Holds a success message's data
        //Adds the files contents as the body of the sucess response
        //For valid HTTP response we add content length as the header and set it as the size of our response body.
        //Currently ignores the request data of buffer and sends back the HTML response no matter what.
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        //Convert our response from a string to bytes and send those bytes directly down the connection stream using write.
        stream.write(response.as_bytes()).unwrap();

        //Flush prevents the program from continuing until all the bytes are written to the connection.
        stream.flush().unwrap();

    } else {
        //We have recieved some other request.
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    */
}
