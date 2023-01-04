use std::{
    //fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const LISTENER_ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let listener = TcpListener::bind(LISTENER_ADDRESS).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf_reader_ref = &mut buf_reader;
    let http_request: Vec<_> = buf_reader_ref
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // вектор из элементов: 0 - метод запроса, 1 - путь, 2 - протокол+версия
    let http_query: Vec<_> = http_request.get(0).unwrap().split(" ").collect();
    println!("Main Header: {:#?}", http_query);
    println!("Request: {:#?}", http_request);
    
    // let contents = fs::read_to_string("Cargo.toml").unwrap(); - как читать из файла //TODO: на релизе убрать этот коммент

    // принимать только POST-запросы (рассчитываем, что JSON будет в теле запроса)
    if http_query.get(0).unwrap().to_ascii_uppercase() == "POST" {
        // TODO: убрать на релизе эти комменты
        /*
        let http_query_path = http_query.get(1).unwrap().to_string();
        let (status_line, contents) = match &http_query_path[..] {
            "/" => {
                println!("Root Path Queried");
                ("HTTP/1.1 200 OK", "Корень\n")
            },
            "/some_other_path" => {
                ("HTTP/1.1 200 OK", "Вы обратились к известному вложенному пути\r\n")
            },
            _ => {
                ("HTTP/1.1 404 Not Found", "Это кто?\r\n")
            }
        };
        */
        let mut qsize: usize = 0;
        for reqline in http_request {
            if reqline.starts_with("Content-Length") {
                qsize = reqline.split(":").collect::<Vec<_>>()
                    .get(1).unwrap().trim()
                    .parse::<usize>().unwrap();
                break;
            }
        }
        let mut status_line: String = "HTTP/1.1 200 OK".to_string();
        let mut contents: String = "".to_string();
        if qsize > 0 {
            buf_reader_ref = &mut buf_reader;
            let mut qbody = vec![0; qsize];
            buf_reader_ref.read_exact(&mut qbody).unwrap();
            let query_body_str = match String::from_utf8(qbody) {
                Ok(v) => v,
                Err(e) => panic!("Broken UTF-8 sequence: {}", e)
            };
            println!("POST Body: {}", query_body_str);
            // TODO: убедиться, что строка в query_body_str - корректная JSON-строка
            // TODO: если да, то десериализовать ее в объект и обработать; по результатам обработки изменить contents и status_line
        }
        // ответ
        let length = contents.len();
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}