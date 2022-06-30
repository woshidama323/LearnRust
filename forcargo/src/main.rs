use std::fmt;
use lazy_static::lazy_static;
use yastl::Pool;
use std::thread;
use std::time::{Duration,SystemTime};
use std::sync::{
    atomic::{AtomicU64, Ordering::SeqCst},
    Arc, MutexGuard,
};

use std::fs::OpenOptions;
use std::path::PathBuf;

use memmap::MmapMut;
use hwloc::{ObjectType, Topology, TopologyObject, Bitmap, CPUBIND_THREAD, CPUBIND_PROCESS, CPUBIND_STRICT};
use log::{debug, info, warn};
use std::sync::{Mutex};

// lazy_static! {
//     pub static ref CORE_GROUPS: Option<Vec<Mutex<CoreGroup>>> = {
//         let num_producers = &SETTINGS.multicore_sdr_producers;
//         let cores_per_unit = num_producers + 1;

//         core_groups(cores_per_unit)
//     };
// }

lazy_static!{
    static ref THREAD_POOL: Pool = Pool::new(num_cpus::get());
    pub static ref TOPOLOGY: Mutex<Topology> = Mutex::new(Topology::new());

}

pub const BASE_DEGREE: usize = 6;

fn main() {
    println!("Hello, world!");
    // #[warn(unused_variables)]
    // let _a = 123;
    // // a = "456"; 会失败，rust是请类型的语言上面的代码已经固定了 a是整型
    // // a = 4.56;  会失败，精度不同也不行
    // // a = 456;   会失败，a不可更改变量
    // let mut _b = 123i64; //下划线 函数内部使用
    // _b = 456; //b 声明成mut 即可修改变量，所以成功 

    // let mut _s = "123";
    // //s = s.len(); //可变变量 只能改值，不能改类型


    // //格式化输出
    // println!("{0}whyere{1}","harry","test");
    // let _str = format!("{}",_b);
    // println!("{}",_str);

    // //另外一种格式化输出 根据别名
    // println!("{subject} {verb} {object}",
    //             object="harry ",
    //             subject="haoqian",
    //             verb="wangna");

    // /*不同格式 用: 来表示 b二进制 
    // - ``, which uses the `Display` trait
    // - `?`, which uses the `Debug` trait
    // - `e`, which uses the `LowerExp` trait
    // - `E`, which uses the `UpperExp` trait
    // - `o`, which uses the `Octal` trait
    // - `p`, which uses the `Pointer` trait
    // - `b`, which uses the `Binary` trait
    // - `x`, which uses the `LowerHex` trait
    // - `X`, which uses the `UpperHex` trait
    // */
    // println!("{} of {:} people know binary, the other half doesn't", 1, _a);


    // //右对齐 这里number是要显示的数据，width 数据总长  xxxxx1
    // println!("{number:>width$}", number=1, width=6);

    // //特殊字符补全空格 xxxxx1
    // println!("{number:x>width$}", number=1, width=6);


    // // `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
    // // #[allow(dead_code)]
    // #[derive(Debug)]
    // struct TestDebug(i32);
    // // println!("This struct `{:}` won't print...", Structure(3)); 失败，原因是需要自己实现输出方式
    // println!("This struct `{:?}` won't print...", TestDebug(3));
    // let strtest = Structure {x: 1i32, y: 2i64};
    // println!("{}",strtest); //只打印struct实现中的数据内容
    // println!("{:?}",strtest); //打印struct本身的全定义
    // println!("{:#?}",strtest); //打印struct本身的全定义 打印的格式更为友好
    // //如果实现b trait等数据 得每一个都得自己实现


    // //测试显示list
    // let l = List(vec![1,2,3]);
    // println!("{}",l);

    // //测试city的数据
    
    // for city in [
    //     City {name : "Harry", lat: 53.3477, lon:-6.434},
    //     City {name : "qianqian",lat:23.2323,lon:27.0}, //如果类型是f32的类型，初始化的时候也要是 float格式，否认报错
    //     City {name : "nana",lat:48.56,lon:-123.4},
    // ].iter(){
    //     println!("{}",*city);
    // }

    //function使用
    // testoperate();
    // matchfunction();
    // testmapoeprate();
    // testchannel();

    //测试另外一种多线程
    // testmultiThread();
    // testSizeof();
    // optionissue();

    //利用一个线程产生一个信号量？

    // testThread();
    // testmmap();
    get_core_info();
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

fn matchfunction() -> bool {
    let number = 13;
    println!("Number..{}",number);
    match number{
        1 => println!("One !"), //注意这里是逗号 comma 
        2 | 3| 4| 5| 7 | 11 => println!("this is a prime"), 
        13..=19 => println!("A teen"),

        //handle the rest of cases
        _ => println!("Ain't special"),

    } //这里不用分号

    let boolean = true;
    let binary = match boolean {

        //the arms of a match must cover all the possible values
        false => 0,
        true  => 1,

    }; //这里需要分号

    println!("{} -> {}",boolean,binary);

    return boolean;
}



fn testmapoeprate() -> bool {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)                             // All natural numbers squared
             .take_while(|&n_squared| n_squared < upper) // Below upper limit
             .filter(|&n_squared| is_odd(n_squared))     // That are odd
             .fold(0, |acc, n_squared| acc + n_squared); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);


    return true;
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}


fn testchannel() {
    let mut databytes: Vec<u8> = vec![];

    // let(data_tx, data_rx) = channel::<(Vec<u8>,Vec<u8>)>(0);
    THREAD_POOL.scoped(|s| {
        for i in 0..100{
            s.execute(move || {
                // let id = ;
                println!("testing current id: {:?}",thread::current().id());
            });
        }

    });
}

//test 多线程
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, ParallelIterator, ParallelSliceMut,
};

fn testmultiThread() {
    println!("counting in parallel:");
    (0..100).into_par_iter()
        .for_each(|i| println!("{}", i));
}


//Struct std::slice::ChunksCopy item path

//
fn testSizeof(){
    let siz = std::mem::size_of::<u8>();
    println!("siz: {:?}",siz);
}

// 关于 option 如何修改
fn optionissue(){
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
    
    // The return value of the function is an option
    let result = divide(2.0, 3.0);
    
    // Pattern match to retrieve the value
    let why = match result {
        // The division was valid
        Some(x) => {
            println!("Result: {}", x);
            x
        }
        // The division was invalid
        None    => {
            println!("Cannot divide by 0");
            0.0
        }
    };
    println!("why: {:?}",why);
}


//(understand mmapf
fn testmmap() -> Result<MmapMut,std::io::Error>{

    let path: PathBuf = PathBuf::from("./test.txt") ;/* path to file */
    let file = OpenOptions::new()
                           .read(true)
                           .write(true)
                           .create(true)
                           .open(&path)?;
    file.set_len(13)?; //这里只设置了13字节的长度？测试结果看，只开辟了13个字节
    
    let mut mmap = unsafe { MmapMut::map_mut(&file) }?;
    
    mmap.copy_from_slice(b"Hello, world!harry");

    Ok(mmap)

}


//test thread with atomic 
fn testThread() {
    let cur_producer = AtomicU64::new(0);
    crossbeam::thread::scope(|s| {

        let newthread = s.spawn( |_| {

            loop {
                thread::sleep(Duration::from_secs(2));
                println!("starting store somethings");
                // let now = SystemTime::now();

                match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(n) => {
                        println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs());
                        // cur_producer.store(n.as_secs(),SeqCst);
                        cur_producer.fetch_add(10,SeqCst);
                    }
                    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                }
            }

        });


        // let secondthread = s.spawn( |_| {
        // });
        thread::sleep(Duration::from_secs(10));
        let output = cur_producer.load(SeqCst);
        println!("output is {}",output);

        // secondthread.join().expect("join failed");
        newthread.join().expect("join failed");
    });



    for i in 0..BASE_DEGREE { //这样是左开又闭的。
        println!("zuo kai you bi: {}",i)

    }
}
 

pub fn get_core_info()->(usize, usize, usize, usize, usize) { //Harry need to understand
    let child_topo = &TOPOLOGY;
    let topo = child_topo.lock().unwrap();

    let package_depth = topo.depth_for_type(&ObjectType::Package).unwrap();
    let package_count = topo.size_at_depth(package_depth) as usize;
    let packages = topo.objects_at_depth(package_depth);
    //if package_count>0 {
    //    info!("Package depth:{}, count:{}, object type:{:?}", package_depth, package_count, packages[0].object_type());
    //}
 
    let numa_depth = topo.depth_for_type(&ObjectType::NUMANode).unwrap_or_default();
    let numa_count = topo.size_at_depth(numa_depth) as usize;
    let numas = topo.objects_at_depth(numa_depth);
    //if numa_count>0 {
    //    info!("Numa depth:{}, count:{}, object type:{:?}", numa_depth, numa_count, numas[0].object_type());
    //}

    let cache_depth_r = topo.depth_for_type(&ObjectType::Cache); //At our dual 7F32 CPU machine, it returns error.
    let cache_depth = if cache_depth_r.is_ok() {
        cache_depth_r.unwrap() 
    }
    else {
        0
    };
    if cache_depth!=0 {
        let cache_count = topo.size_at_depth(cache_depth) as usize;
        let caches = topo.objects_at_depth(cache_depth);
        //if cache_count>0 {
        //    info!("Cache depth:{}, count:{}, object type:{:?}", cache_depth, cache_count, caches[0].object_type());
        //}
    }

    let core_depth = topo.depth_for_type(&ObjectType::Core).unwrap();
    let cores_count = topo.size_at_depth(core_depth) as usize;
    let cores = topo.objects_at_depth(core_depth);
    //if cores_count>0 {
    //    info!("Core depth:{}, count:{}, object type:{:?}", core_depth, cores_count, cores[0].object_type());
    //}

    let core_depth = match topo.depth_or_below_for_type(&ObjectType::Core) {
        Ok(depth) => depth,
        Err(_) => return (0,0,0,0,0),
    };
    let all_cores = topo.objects_with_type(&ObjectType::Core).unwrap();
    let core_count = all_cores.len();

    let mut cache_depth = core_depth;
    let mut cache_count = 0;

    while cache_depth > 0 {
        let objs = topo.objects_at_depth(cache_depth);
        let obj_count = objs.len();
        if obj_count < core_count {
            cache_count = obj_count;
            break;
        }

        cache_depth -= 1;
    }

    let mut node_count = numa_count;
    
    //Wrong info, mean this machine does not have multiple numa nodes
    if numa_count>=core_count {
        node_count = 1;
    }
    
    //If the machine has multiple numa, set group_size to 1 to disable coregroup binding
    if node_count>1 {
       cache_count = core_count; 
    }

    let mut group_size = core_count / cache_count;
    let group_count = cache_count;

    // Something is wrong
    if group_count==1 {
        group_size=1;
    }

    info!("core_count={}, cache_count={}, core_depth={}, group_size={}, group_count={}, node_count={}",
          core_count, cache_count, core_depth, group_size, group_count, node_count);
    
    (core_count, cache_count, group_size, group_count, node_count)
}