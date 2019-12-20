# api

SocketAddr


pub enum Method {
    /// OPTIONS
    Options,
    /// GET
    Get,
    /// POST
    Post,
    /// PUT
    Put,
    /// DELETE
    Delete,
    /// HEAD
    Head,
    /// TRACE
    Trace,
    /// CONNECT
    Connect,
    /// PATCH
    Patch,
    /// Method extensions. An example would be `let m = Extension("FOO".to_string())`.
    Extension(String)
}

Method::GET

Headers::new()


use typemap::TypeMap;

TypeMap::new()



## syntax

#[derive(Debug)]
