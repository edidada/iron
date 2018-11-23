# iron


官方示例


Linux edidada 4.13.0-32-generic #35~16.04.1-Ubuntu SMP Thu Jan 25 10:13:43 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux


rustc -V
rustc 1.23.0 (766bd11c8 2018-01-01)

cargo -V
cargo 0.24.0 (45043115c 2017-12-05)


$HOME/.cargo/config内容如下：

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"


idea clion version 2018.2

rust plugin version 0.2.0.20

操作，在项目对应的文件夹下运行“cargo run”。
打开浏览器访问 http://localhost:3000/ 显示：This Is Rust Http Server