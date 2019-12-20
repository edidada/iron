# iron

[Rust std](https://dev.kriry.com/www/rust/std/)

[Rust Crates 镜像使用帮助](https://lug.ustc.edu.cn/wiki/mirrors/help/rust-crates)

官方示例

### Linux
Linux edidada 4.13.0-32-generic #35~16.04.1-Ubuntu SMP Thu Jan 25 10:13:43 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux

```jshelllanguage
rustc -V
cargo -V
rustup -V
```

```shell

rustc -V
rustc 1.23.0 (766bd11c8 2018-01-01)

cargo -V
cargo 0.24.0 (45043115c 2017-12-05)

rustup -V
rustup 1.14.0 (1e51b07cc 2018-10-04)

```

### Windows

```jshelllanguage
rustc -V
rustc 1.34.0 (91856ed52 2019-04-10)

cargo -V
cargo 1.34.0 (6789d8a0a 2019-04-01)

rustup -V
rustup 1.20.2 (13979c968 2019-10-16)

```

### Mac

```

cargo test
cargo doc
```

$HOME/.cargo/config内容如下：

```shell
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```


idea clion version 2018.2

rust plugin version 0.2.0.20

操作，在项目对应的文件夹下运行“cargo run”。
打开浏览器访问 http://localhost:3000/ 显示：This Is Rust Http Server


```shell
/usr/local/include/pistache/description.h:332:13: error: no match for ‘operator->*’ (operand types are ‘const Accountsss::Accounts’ and ‘void (Accountsss::Accounts::* const)(const Pistache::Rest::Request&, Pistache::Http::ResponseWriter)’)
             CALL_MEMBER_FN(obj, func)(request, std::move(response));
```

关键字
pub

[rust](https://www.ibm.com/developerworks/cn/opensource/os-know-rust/index.html?lnk=hm)

[极客学院](http://wiki.jikexueyuan.com/project/rust-primer/match/match.html)


这里的 b 变量，绑定了 2.0f32。这是 Rust 里面值类型显式标记的语法，规定为value+type的形式。

例如： 固定大小类型：

1u8 1i8
1u16 1i16
1u32 1i32
1u64 1i64

可变大小类型：

1usize 1isize

浮点类型：

1f32 1f64


Rust内置的原生类型 (primitive types) 有以下几类：

布尔类型：有两个值true和false。
字符类型：表示单个Unicode字符，存储为4个字节。
数值类型：分为有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 以及浮点数 (f32, f64)。
字符串类型：最底层的是不定长类型str，更常用的是字符串切片&str和堆分配字符串String， 其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
数组：具有固定大小，并且元素都是同种类型，可表示为[T; N]。
切片：引用一个数组的部分数据并且不需要拷贝，可表示为&[T]。
元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
指针：最底层的是裸指针*const T和*mut T，但解引用它们是不安全的，必须放到unsafe块里。
函数：具有函数类型的变量实质上是一个函数指针。
元类型：即()，其唯一的值也是()。

有几点是需要特别注意的：

数值类型可以使用_分隔符来增加可读性。
Rust还支持单字节字符b'H'以及单字节字符串b"Hello"，仅限制于ASCII字符。 此外，还可以使用r#"..."#标记来表示原始字符串，不需要对特殊字符进行转义。
使用&符号将String类型转换成&str类型很廉价， 但是使用to_string()方法将&str转换到String类型涉及到分配内存， 除非很有必要否则不要这么做。
数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏vec!创建。
元组可以使用==和!=运算符来判断是否相同。
不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的。
Rust不提供原生类型之间的隐式转换，只能使用as关键字显式转换。
可以使用type关键字定义某个类型的别名，并且应该采用驼峰命名法。


Rust 里面有两种字符串类型。String 和 str。
str 类型基本上不怎么使用，通常使用 &str 类型，它其实是 [u8] 类型的切片形式 &[u8]。这是一种固定大小的字符串类型。 常见的的字符串字面值就是 &'static str 类型。这是一种带有 'static 生命周期的 &str 类型。

to_string();

String
String 是一个带有的 vec:Vec<u8> 成员的结构体，你可以理解为 str 类型的动态形式。 它们的关系相当于 [T] 和 Vec<T> 的关系。 显然 String 类型也有压入和弹出。


编译

```shell

/home/edidada/.cargo/bin/cargo run

```

iron-0.6.0

cargo install 是把可执行文件复制到cargo/bin文件夹下面

[rust使用外部函数](https://blog.csdn.net/teamlet/article/details/50923682)

iron依赖hyper router

[rust doc](https://doc.rust-lang.org/rustc/what-is-rustc.html)

切片只能通过某种指针来处理，因此有多种形式，例如：
&[T] - 共享切片
&mut [T] - 可变切片
Box<[T]> - 拥有切片

pub	使其可见


https://rustwiki.org/zh-CN/rust-by-example/mod/visibility.html

https://github.com/rust-lang/book

集成测试
'cargo test --test integration_test'


[包和模块](https://wiki.jikexueyuan.com/project/rust-primer/module/module.html)

`curl 127.0.0.1:3000`

`mod.rs`


### crate
在main.rs中引用，不要在其他mod中引用,在其他mod中引用，会找不到自定义的mod

main.rs、lib.rs、mod.rs中的mod xxx; 默认优先查找同级目录下的 xxx.rs 文件。

单元测试，目前只有一个文件夹

 个人感觉最大的问题在于没能搞懂 Rust 的所有权、引用借用和生命周期三个方面。
`cargo run --color=always --package testiron --bin testiron`

iron 0.4.0 0.6.0两个版本 多包依赖冲突


rust 常用类型转换
https://www.jianshu.com/p/7b4a74856cdd

# todo

- 自定义类型 enum struct
- 跨文件调用 搞定
- map,unit test
- 跨mod调用
- 多线程
- rustc


https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html



Compiling rockettest v0.1.0 (D:\git\github\testiron\rockettest)
error[E0658]: use of unstable library feature 'rustc_private': this crate is being loaded from the sysroot, an unstable location; did you mean to load this crate from crates.io via `Cargo.toml` instead?

