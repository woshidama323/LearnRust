
## rust merkletree笔记

```rust
    //因为文件中增加了条件编译
#[cfg(feature = "chaincore")]

//所以编译的时候，如果需要将以上的attributes指定的部分编译进去，需要在build的时候指定feature

```

```shell
https_proxy=192.168.20.9:1089 cargo build --features 'chaincore'
```

