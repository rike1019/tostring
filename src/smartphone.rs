pub struct Smartphone {
    price: i64,
    tax: f64,
    service: i64
}

impl Smartphone {
    pub fn new(price: i64, tax: f64, service: i64) -> Self {
        return Smartphone { price: price, tax: tax, service: service };
    }
    pub fn calc(&self) -> i64 {
        // 暗黙的な型変換は行われない
        let price = self.price as f64;
        let tax = self.tax;
        let service = self.service as f64;
        return (price * tax + service) as i64;
    }
}

impl std::fmt::Display for Smartphone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}, {}, {}", self.price, self.tax, self.service);
    }
}
