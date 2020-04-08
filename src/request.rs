use std::collections::{HashMap};

pub enum Method {
    GET,
    POST,
    UNKNOWN
}

impl Method {
    pub fn from(string: &String) -> Method {
        let str = string.as_str();
        let input = String::from(str);
        return if input == String::from("GET") {
            Method::GET
        } else if input == String::from("POST") {
            Method::POST
        } else {
            Method::UNKNOWN
        }
    }

    pub fn as_string(&self) -> String {
        return match self {
            Method::GET => String::from("GET"),
            Method::POST => String::from("POST"),
            Method::UNKNOWN => String::from("Unknown"),
        }
    }
}

pub struct Request {
    method: Method,
    path: String,
    body: HashMap<String, String>,
}

impl Request {
    pub fn new(request: String) -> Request{
        let mut split_request: Vec<String> = Vec::new();
        for s in request.split("\r\n") {
            split_request.push(format!("{}",s ));
        };

        let mut request: Vec<String> = Vec::new();
        for s in split_request[0].split(" ") {
            request.push(format!("{}", s));
        };
        let method = Method::from(&request[0]);
        let path = format!("{}", request[1]);

        split_request.remove(0);

        let mut body = HashMap::new();

        for s in split_request {
            let k_v: Vec<&str> = s.split(": ").collect();
            if k_v.len() > 1 {
                body.insert(format!("{}", k_v[0]), format!("{}", k_v[1]));
            }
        }

        Request {
            method,
            path,
            body
        }
    }

    pub fn get_method(&self) -> String {
        self.method.as_string()
    }

    pub fn get_path(&self) -> String {
        let path = &self.path;
        let path = path.as_str();
        String::from(path)
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        let token = self.body.get(key);
        let ok: &str;
        match token {
            Some(s) => {ok = s.as_str()},
            None => return None
        };
        return Some(String::from(ok))
    }

    pub fn get_body(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        for (key, token) in &self.body {
            map.insert(format!("{}", key), format!("{}", token));
        };
        map
    }
}
