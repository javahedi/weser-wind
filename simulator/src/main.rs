mod physics;

fn main() {
    //println!("Hello, world!");
    println!("{:>6} | {:>5}", "v m/s", "P MW");
    for i in 0..=30 {
        let v = i as f64;
        println!("{:>6.1} | {:>5.2}", v, physics::power_curve(v));
    }
}
