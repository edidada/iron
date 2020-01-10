# iron api

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


[rust 常用类型转换](https://www.jianshu.com/p/7b4a74856cdd)

rust数组数量错了，提示

```rust

        let months: [&str;12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];//mismatched types [E0308] expected `[&str; 11]`, found `[&str; 12]`
```
