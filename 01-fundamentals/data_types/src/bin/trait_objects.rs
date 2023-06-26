trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}
struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}
struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

fn main() {
    // NOTE Creating a vector of trait objects
    let sales: Vec<Box<dyn Sale>> = vec![
        Box::new(FullSale(10.0)),
        Box::new(OneDollarOffCoupon(5.0)),
        Box::new(TenPercentOffPromo(10.0)),
    ];
    println!("revenue: {}", calculate_revenue(&sales));
}
