# Unit Test
是不是只能有一个测试文件？lib.rs 目前看是这样
https://doc.rust-lang.org/book/testing.html
https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html


大多数单元测试都会被放到一个叫 tests 的、带有 #[cfg(test)] 属性 的模块中，测试函数要加上 #[test] 属性。

当测试函数中有什么东西 panic 了，测试就失败。有一些这方面的 辅助宏：

assert!(expression) - 如果表达式的值是 false 则 panic。
assert_eq!(left, right) 和 assert_ne!(left, right) - 检验左右两边是否 相等/不等。

test vector
https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html

Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.