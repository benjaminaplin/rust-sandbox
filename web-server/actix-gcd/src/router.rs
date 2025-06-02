use std::collections::HashMap;
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

pub struct Response {
    pub code: u32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

pub struct BasicRouter {
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter {
    // Create an empty router.
    pub fn new() -> BasicRouter {
        BasicRouter { routes: HashMap::new() }
    }

    // Add a route to the router.
    pub fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request)
        }
    }
}