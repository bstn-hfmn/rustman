use std::collections::HashMap;
use std::io;

use rustman::app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear().expect("Failed to clear the terminal.");

    let result = App::new().run(terminal);
    ratatui::restore();

    result
}

/*
struct Request {
    uri: String,
    query: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String
}

impl Request {
    pub fn new() -> Request {
        let mut headers = HashMap::new();
        headers.insert(
            String::from("User-Agent"),
            String::from("Rustman/ 1.0.0"));
        headers.insert(
            String::from("Accept"),
            String::from("application/json"));
        headers.insert(
            String::from("Host"),
            String::from("localhost"));

        Request {
            uri: String::new(),
            query: HashMap::new(),
            headers,
            body: String::new()
        }
    }
}

struct Response {
    time: u32,
    status: u32,
    size: u64,

    body: String,
    headers: HashMap<String, String>
}

impl Response {
    pub fn new() -> Response {
        Response {
            time: 0,
            status: 0,
            size: 0,

            body: String::new(),
            headers: HashMap::new()
        }
    }
}
*/
