### 必要的记录

资料来源：
https://doc.rust-lang.org/stable/rust-by-example/hello.html


### debug 方式 
lldb 
https://www.nesono.com/sites/default/files/lldb%20cheat%20sheet.pdf

### 编译器选择

cargo vs rustc

You might note that cargo uses rustc. It is not a compiler by itself. Instead, it could be seen as a convenience wrapper, providing shortcuts to the commands used in almost every project. Then, one obvious (but extremely rare) case when you'd prefer rustc over cargo is if you need something which is easier to achieve by manually instructing the compiler then by describing your goal to cargo.

cargo 是上一层，最终调用rustc
推荐cargo

#### cargo 资源
https://doc.rust-lang.org/cargo/getting-started/index.html

1. build release version with cargo but still keep debug info 

https://stackoverflow.com/questions/38803760/how-to-get-a-release-build-with-debugging-information-when-using-cargo



[代码样例](notes/lockfile.md#rust中的样例)

