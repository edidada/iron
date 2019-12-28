extern crate iron;
extern crate router;
extern crate rustc_serialize;

mod hserver;//引用hserver文件夹，本文件文件名不是main.rs，报错
use hserver::testironfile;

mod methodsee;//引用methodsee.rs文件，本文件文件名不是main.rs，报错

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

use iron::{Iron, Request, Response, IronResult};

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

//main_iron
fn main() {

    //非pub的函数，在其他模块应用不了
    methodsee::testprint();
    methodsee::testenum();

    testironfile::testiron_method();

}