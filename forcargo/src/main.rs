use std::fmt;

fn main() {
    println!("Hello, world!");
    #[warn(unused_variables)]
    let _a = 123;
    // a = "456"; 会失败，rust是请类型的语言上面的代码已经固定了 a是整型
    // a = 4.56;  会失败，精度不同也不行
    // a = 456;   会失败，a不可更改变量
    let mut _b = 123i64; //下划线 函数内部使用
    _b = 456; //b 声明成mut 即可修改变量，所以成功 

    let mut _s = "123";
    //s = s.len(); //可变变量 只能改值，不能改类型


    //格式化输出
    println!("{0}whyere{1}","harry","test");
    let _str = format!("{}",_b);
    println!("{}",_str);

    //另外一种格式化输出 根据别名
    println!("{subject} {verb} {object}",
                object="harry ",
                subject="haoqian",
                verb="wangna");

    /*不同格式 用: 来表示 b二进制 
    - ``, which uses the `Display` trait
    - `?`, which uses the `Debug` trait
    - `e`, which uses the `LowerExp` trait
    - `E`, which uses the `UpperExp` trait
    - `o`, which uses the `Octal` trait
    - `p`, which uses the `Pointer` trait
    - `b`, which uses the `Binary` trait
    - `x`, which uses the `LowerHex` trait
    - `X`, which uses the `UpperHex` trait
    */
    println!("{} of {:} people know binary, the other half doesn't", 1, _a);


    //右对齐 这里number是要显示的数据，width 数据总长  xxxxx1
    println!("{number:>width$}", number=1, width=6);

    //特殊字符补全空格 xxxxx1
    println!("{number:x>width$}", number=1, width=6);


    // `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
    // #[allow(dead_code)]
    #[derive(Debug)]
    struct TestDebug(i32);
    // println!("This struct `{:}` won't print...", Structure(3)); 失败，原因是需要自己实现输出方式
    println!("This struct `{:?}` won't print...", TestDebug(3));
    let strtest = Structure {x: 1i32, y: 2i64};
    println!("{}",strtest); //只打印struct实现中的数据内容
    println!("{:?}",strtest); //打印struct本身的全定义
    println!("{:#?}",strtest); //打印struct本身的全定义 打印的格式更为友好
    //如果实现b trait等数据 得每一个都得自己实现


    //测试显示list
    let l = List(vec![1,2,3]);
    println!("{}",l);

    //测试city的数据
    
    for city in [
        City {name : "Harry", lat: 53.3477, lon:-6.434},
        City {name : "qianqian",lat:23.2323,lon:27.0}, //如果类型是f32的类型，初始化的时候也要是 float格式，否认报错
        City {name : "nana",lat:48.56,lon:-123.4},
    ].iter(){
        println!("{}",*city);
    }

    //function使用
    testoperate();
}

#[derive(Debug)]
struct Structure{
    x: i32,
    y: i64,
} //定义的时候不需要分号结尾

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // a
        write!(f,"harry get structure data {},{}",self.x,self.y)
    }  //函数也不需要分号结尾
}


//测试显示vec 数据
struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;

        write!(f,"[")?;

        for (count,v) in vec.iter().enumerate(){
            if count != 0 {write!(f,", ")?;}
            write!(f,"{}",v)?;
        }

        write!(f,"]") //这里为啥都不用分号的 ？
    }
}

//测试formate 格式输出
struct City{
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{

        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        write!(f,"{}: {:.3}˚{} {:.3}˚{}",self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
   
}



//简单计算练习
fn testoperate() -> bool{

    println!("1 + 2 = {}",1u64 + 2);

    println!("1 -2 = {}",1i32 - 2);

    println!("1 << 5 is {}",1u32<<5);
    //下划线增强可读性
    println!("One million is written as {}", 1_000_000u32);
    return true ;
}