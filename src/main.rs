use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        // We don’t want to stop the server if an error occurs, so just ignore it and continue.
        let _ = handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    use book_web_server::{Method, Request, Response, Status, Uri, Version};
    use std::{
        fs,
        io::{Read, Write},
    };

    let mut request = [0; 512];
    let _ = stream.read(&mut request)?;

    let request = String::from_utf8_lossy(&request);
    let request = Request::new(&request)?;

    let (status, filename) = if request.method == Method::Get && request.uri == Uri::root() {
        (Status::Ok, "hello.html")
    } else {
        (Status::NotFound, "404.html")
    };

    let body = fs::read_to_string(filename)?;

    let response = Response {
        version: Version::OneDotOne,
        status,
        headers: Vec::new(),
        body: &body,
    };

    stream.write_all(response.to_string().as_bytes())?;
    stream.flush()?;

    Ok(())
}
