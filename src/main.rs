mod rusty_maths;

fn main() {
    for x in 1..10 {
        let added = rusty_maths::add(x as f64, (x+1) as f64);
        let subtracted = rusty_maths::subtract(x as f64, (x+1) as f64);
        let multiplied = rusty_maths::multiply(x as f64, (x+1) as f64);
        let divided = rusty_maths::divide(x as f64, (x+1) as f64);
        println!("\n\n{} + {} = {}", x, x+1, added);
        println!("{} - {} = {}", x, x+1, subtracted);
        println!("{} * {} = {}", x, x+1, multiplied);
        println!("{} / {} = {}", x, x+1, divided);
    }
}