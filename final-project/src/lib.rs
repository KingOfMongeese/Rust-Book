use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

// TODO print client specific info or something cool
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer).unwrap();

    let response = GeneratedResponse::generate_response(&buffer);
    let response = response.as_http_response();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

struct GeneratedResponse {
    status_line: String,
    file_to_render: String,
}

impl GeneratedResponse {
    fn generate_response(request: &[u8]) -> Self {

        let get = b"GET / HTTP/1.1\r\n";
        let (status_line, filename) = if request.starts_with(get) {
            ("HTTP/1.1 200 OK".to_string(), "index.html".to_string())
        } else {
            ("HTTP/1.1 404 NOT FOUND".to_string(), "404.html".to_string())
        };

        Self {
            status_line,
            file_to_render: filename,
        }

    }

    fn as_http_response(&self) -> String {
        let contents = fs::read_to_string(&self.file_to_render).unwrap();
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line,
            contents.len(),
            contents
        )
    }
}
