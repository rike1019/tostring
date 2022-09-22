mod smartphone;
use smartphone::Smartphone;

fn main() {
    let linemo3 = Smartphone::new(900, 1.1, 3);
    let rakuten20 = Smartphone::new(1980, 1.1, 3);
    let rakuten_unlimited = Smartphone::new(2980, 1.1, 3);
    println!("{}", linemo3);
    println!("{}", rakuten20);
    println!("{}", rakuten_unlimited);
    let linemo3 = linemo3.calc();
    let rakuten20 = rakuten20.calc();
    let rakuten_unlimited = rakuten_unlimited.calc();
    println!("linemo3+rakuten20={}", linemo3 + rakuten20);
    println!("linemo3+rakuten_unlimited={}", linemo3 + rakuten_unlimited);
}
