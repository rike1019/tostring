struct Smartphone {
    price: i64,
    tax: f64,
    service: i64
}

impl Smartphone {
    fn calc(&self) -> i64 {
        // 暗黙的な型変換は行われない
        let price = self.price as f64;
        let tax = self.tax;
        let service = self.service as f64;
        return (price * tax + service) as i64;
    }
}

impl std::fmt::Display for Smartphone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.price, self.tax, self.service)
    }
}

fn main() {
    let linemo3 = Smartphone { price: 900, tax: 1.1, service: 3 };
    let rakuten20 = Smartphone { price: 1980, tax: 1.1, service: 3 };
    let rakuten_unlimited = Smartphone { price: 2980, tax: 1.1, service: 3 };
    println!("{}", linemo3);
    println!("{}", rakuten20);
    println!("{}", rakuten_unlimited);
    let linemo3 = linemo3.calc();
    let rakuten20 = rakuten20.calc();
    let rakuten_unlimited = rakuten_unlimited.calc();
    println!("linemo3+rakuten20={}", linemo3 + rakuten20);
    println!("linemo3+rakuten_unlimited={}", linemo3 + rakuten_unlimited);
}
