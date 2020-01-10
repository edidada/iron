# readme

[测试](https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html)

tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行 cargo test 时编译这个目录中的文件。现在就运行 
`cargo test`

为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。
