fn main() {
    println!("Hello, world!");
    println!("{}", add(12.0, 12.0));

    println!("Hello, {}!", "Takashi");
    println!(
        "r {:.1}, pi {:.3}, area {:.3}",
        3.2,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI
    )
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}
