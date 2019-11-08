extern crate iron;
extern crate router;
extern crate rustc_serialize;


mod hserver;
use hserver::testironfile;

mod methodsee;//引用methodsee.rs文件


use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

use iron::{Iron, Request, Response, IronResult};

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {

    //非pub的函数，在其他模块应用不了
    methodsee::testprint();
    methodsee::testenum();

    testironfile::testiron_method();

//    let mut router = Router::new();
//
//    router.get("/", hello_world,"handler");
//    router.post("/set", set_greeting,"post");
//
//
//
//    Iron::new(router).http("localhost:3000").unwrap();
//
//    fn hello_world(_: &mut Request) -> IronResult<Response> {
//        let greeting = Greeting { msg: "Hello, World".to_string() };
//        let payload = json::encode(&greeting).unwrap();
//        Ok(Response::with((status::Ok, payload)))
//    }
//
//    // Receive a message by POST and play it back.
//    fn set_greeting(request: &mut Request) -> IronResult<Response> {
//        let mut payload = String::new();
////        request.body.read_to_string(&mut payload).unwrap();
//
//        let data = "initial contents";
//
//        let payload = data.to_string();
//        let request: Greeting = json::decode(&payload).unwrap();
//        let greeting = Greeting { msg: request.msg };
//        let payload = json::encode(&greeting).unwrap();
//        Ok(Response::with((status::Ok, payload)))
//    }

    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.post("/:query", query_handler, "query_handler");

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

}