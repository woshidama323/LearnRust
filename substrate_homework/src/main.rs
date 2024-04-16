pub mod lib;
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn time(&self) -> u8;
}

impl Duration for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,   // 红灯持续60秒
            TrafficLight::Yellow => 5, // 黄灯持续5秒
            TrafficLight::Green => 60, // 绿灯持续60秒
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light duration: {} seconds", red.time());
    println!("Yellow light duration: {} seconds", yellow.time());
    println!("Green light duration: {} seconds", green.time());

    //work 5
    lib::utils::test_sum_u32();
    lib::test_print_area();
    //work 6
}
