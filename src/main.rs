use std::{
    //fs, //TODO: перетащить в другой модуль
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
//use serde::{Serialize, Deserialize}; //TODO: перетащить в другой модуль
use serde_json::{Value, Map};
use state::*;

// включение модулей (разделение кода - для удобства командной разработки); в соседних файлах .rs - включать уже через use crate::имя_модуля
mod defs;
mod query_proc;
mod state;
mod auth;
mod proc_for_admin;
mod proc_for_users;

const LISTENER_ADDRESS: &str = "0.0.0.0:8080";

fn main() {
    let listener = TcpListener::bind(LISTENER_ADDRESS).unwrap();
    let mut data_state = DataState {
        auth: Vec::<Auth>::new(),
        user: Vec::<User>::new(),
        user_in_group: Vec::<UserInGroup>::new(),
        group: Vec::<Group>::new()
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut data_state);
    }
}

fn handle_connection(mut stream: TcpStream, data_state: &mut DataState) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf_reader_ref = &mut buf_reader;
    let http_request: Vec<_> = buf_reader_ref
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // вектор из элементов: 0 - метод запроса, 1 - путь, 2 - протокол+версия
    let http_query: Vec<_> = http_request.get(0).unwrap().split(" ").collect();
    //println!("Main Header: {:#?}", http_query);
    //println!("Request: {:#?}", http_request);
    
    //TODO: на релизе убрать этот коммент
    // let contents = fs::read_to_string("Cargo.toml").unwrap(); - как читать из файла

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
        let mut status_line: String = defs::HTTP_STATUS_200.to_string();
        let mut contents: String = "".to_string();
        if qsize > 0 {
            buf_reader_ref = &mut buf_reader;
            let mut qbody = vec![0; qsize];
            buf_reader_ref.read_exact(&mut qbody).unwrap();
            let query_body_str = match String::from_utf8(qbody) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("(X) Сломанная строка UTF-8: {}", e);
                    stream.write_all(defs::HTTP_STATUS_500.as_bytes()).unwrap();
                    stream.write_all("\r\n\r\n".as_bytes()).unwrap();
                    return;
                }
            };
            //println!("POST Body: {}", query_body_str);
            // Десериализация запроса
            let req_map: Map<String, Value> = match serde_json::from_str(query_body_str.as_str()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("(X) Сломанный формат JSON-строки: {}", e);
                    stream.write_all(defs::HTTP_STATUS_500.as_bytes()).unwrap();
                    stream.write_all("\r\n\r\n".as_bytes()).unwrap();
                    return;
                }
            };
            //println!("Тело POST-запроса как JSON-объект: {:#?}", req_map);
            (status_line, contents) = query_proc::process_req(req_map, data_state);
        }
        // формирование ответа
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        stream.write_all(defs::HTTP_STATUS_405.as_bytes()).unwrap();
        stream.write_all("\r\n\r\n".as_bytes()).unwrap();
    }
}