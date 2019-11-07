extern crate iron;

mod hserver;
use hserver::testironfile;

mod methodsee;//引用foo.rs文件

fn main() {

    //非pub的函数，在其他模块应用不了
    methodsee::testprint();
    methodsee::testenum();

    testironfile::testiron_method();
}

//extern crate iron;
//extern crate router;
//extern crate rustc_serialize;
//
//use iron::prelude::*;
//use iron::status;
//use router::Router;
//use rustc_serialize::json;
//
//#[derive(RustcEncodable, RustcDecodable)]
//struct Greeting {
//    msg: String
//}
//
//fn main() {
//    let mut router = Router::new();
//
//    router.get("/", hello_world,router);
//    router.post("/set", set_greeting,router);
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
//        request.body.read_to_string(&mut payload).unwrap();
//        let request: Greeting = json::decode(payload).unwrap();
//        let greeting = Greeting { msg: request.msg };
//        let payload = json::encode(&greeting).unwrap();
//        Ok(Response::with((status::Ok, payload)))
//    }
//
//    Iron::new(router).http("localhost:3000").unwrap();
//}