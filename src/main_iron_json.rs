extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

use iron::{Iron, Request, Response, IronResult};

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

//main_iron_json

fn main() {

    let mut router = Router::new();

    router.get("/", hello_world,"handler");
    router.post("/set", set_greeting,"post");

    Iron::new(router).http("localhost:3000").unwrap();

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let hello_string : & str = "Hello, World";
        let greeting = Greeting { msg: hello_string.to_string() };
        let payload : String = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload : String = String::new();
//        request.body.read_to_string(&mut payload).unwrap();

        let datas : & str = "initial contents";

        let payload : String = datas.to_string();
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload : String = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

}