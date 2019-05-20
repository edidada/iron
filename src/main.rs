extern crate iron;

use iron::prelude::*;
use iron::status;



fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "This Is Rust Http Server")))
    }

    let _server = Iron::new(hello_world).http("127.0.0.1:3000").unwrap();
}

