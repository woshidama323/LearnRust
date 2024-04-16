pub mod utils {

    pub fn sum_u32(values: &[u32]) -> Option<u32> {
        let mut sum: u32 = 0;

        for &value in values {
            match sum.checked_add(value) {
                Some(s) => sum = s,
                None => return None, // 在发生溢出时提前返回 None
            }
        }

        Some(sum) // 如果没有溢出，返回求和结果
    }

    pub fn test_sum_u32() {
        let numbers = vec![1, 2, 3, 4, 5];
        println!("The sum is: {:?}", sum_u32(&numbers)); // 应该打印出 Some(15)

        let large_numbers = vec![u32::MAX, 1];
        println!("The sum is: {:?}", sum_u32(&large_numbers)); // 应该打印出 None
    }
}

// 定义一个 Shape 特征，包含计算面积的方法
pub trait Shape {
    fn area(&self) -> f64;
}

// 为 Circle 结构体实现 Shape 特征
pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159265358979323846264338327950288 * self.radius * self.radius
    }
}

// 为 Triangle 结构体实现 Shape 特征
pub struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 为 Square 结构体实现 Shape 特征
pub struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义一个打印 Shape 面积的泛型函数
pub fn print_area<T: Shape>(shape: &T) {
    println!("The area of the shape is {}", shape.area());
}

pub fn test_print_area() {
    let circle = Circle { radius: 1.0 };
    print_area(&circle); // 打印出 The area of the shape is 3.141592653589793

    let triangle = Triangle {
        base: 2.0,
        height: 3.0,
    };
    print_area(&triangle); // 打印出 The area of the shape is 3.0

    let square = Square { side: 2.0 };
    print_area(&square); // 打印出 The area of the shape is 4.0
}
