

use iron::prelude::*;
use iron::status;
use router::Router;

pub fn testiron_method(){
    fn mian_page(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "This Is Rust Http Server")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    let mut router = Router::new();
    router.get("/", mian_page, "handler");
    router.post("/:query", query_handler, "query_handler");

//    let _server = Iron::new(mian_page).http("127.0.0.1:3000").unwrap();

    Iron::new(router).http("localhost:3000").unwrap();
}
