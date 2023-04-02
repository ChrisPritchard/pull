use std::{env, process::exit, fs, io::{stdout, Write, self, BufRead}, net::TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("args: url [outputfilename]");
        exit(1);
    }

    let url = &args[1];
    let url = if let Some(index) = url.find("//") {
        url[index+2..].to_string()
    } else { url.to_string() };

    let (host, path) = if let Some(index) = url.find("/") {
        (url[..index].to_string(), url[index..].to_string())
    } else {
        (url.to_string(), "/".into())
    };
    let (host, port) = if let Some(index) = host.find(":") {
        (host[..index].to_string(), host[index+1..].parse::<usize>().expect("invalid url"))
    } else {
        (host.to_string(), 80 as usize)
    };

    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

    let req = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", path, host);

    stream.write(&req.as_bytes()).unwrap();
    let mut resp = io::BufReader::new(stream);

    let mut result: Vec<u8> = Vec::new();
    let mut post_headers = false;
    loop {
        let mut buf = Vec::new();
        resp.read_until(10, &mut buf).expect("read failure");
        if buf.len() == 0 {
            break;
        } else if buf.len() == 2 && buf[0] == 13 && buf[1] == 10 {
            post_headers = true;
        } else if post_headers {
            result.append(&mut buf);
        }
    }

    if args.len() > 2 {
        let path = &args[2];
        fs::write(path, &result).expect("failed to write file");
    } else {
        stdout().write(&result).expect("failed to write to stdout");
    }
}
