### rust基础只是学习与代码样例

#### Debug程序方法
+ Linux上使用rust-gdb 
+ MacOs上使用rust-lldb
[lldb]

#### rust官方资料
https://doc.rust-lang.org/stable/rust-by-example/hello.html

lldb 


### 编译器选择

cargo vs rustc

You might note that cargo uses rustc. It is not a compiler by itself. Instead, it could be seen as a convenience wrapper, providing shortcuts to the commands used in almost every project. Then, one obvious (but extremely rare) case when you'd prefer rustc over cargo is if you need something which is easier to achieve by manually instructing the compiler then by describing your goal to cargo.

cargo 是上一层，最终调用rustc
推荐cargo

#### cargo 资源
https://doc.rust-lang.org/cargo/getting-started/index.html

1. build release version with cargo but still keep debug info 

https://stackoverflow.com/questions/38803760/how-to-get-a-release-build-with-debugging-information-when-using-cargo



1. [文件锁在rust中的使用](notes/lockfile.md#rust中的样例)
2. [merkletree样例](notes/merkletree_notes.md)



### attributes
1. dead_code 编译器会报warning unused 如果加上这个，会过滤掉


## cargo new 用于创建一个新的工程
```shell
cargo new [new path]
cargo init ## 用于当前的目录
```