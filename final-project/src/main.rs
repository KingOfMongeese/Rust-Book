use std::net::TcpListener;

use final_project::handle_connection;

// TODO handle more than 1 page?

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
