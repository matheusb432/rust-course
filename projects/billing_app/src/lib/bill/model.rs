#[derive(Debug)]
pub struct Bill {
    name: String,
    price: f64,
}

impl Bill {
    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            Err("Name cannot be empty!".to_owned())
        } else {
            Ok(())
        }
    }

    pub fn validate_price(price: &f64) -> Result<(), String> {
        if !price.is_sign_positive() {
            Err("Price must be greater than 0!".to_owned())
        } else {
            Ok(())
        }
    }

    pub fn new(name: String, price: f64) -> Result<Self, String> {
        Self::validate_name(name.as_str())?;
        Self::validate_price(&price)?;

        // NOTE fields are private when out of this module so a Bill will only exist if it's valid
        Ok(Self { name, price })
    }

    pub fn set_price(&mut self, price: f64) -> Result<(), String> {
        Self::validate_price(&price)?;
        self.price = price;
        Ok(())
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn key(&self) -> String {
        Self::create_key(&self.name)
    }

    pub fn create_key(name: &str) -> String {
        name.to_lowercase()
    }

    pub fn print(&self) {
        println!("{}\t|\t${}", self.name, self.price)
    }
}

pub struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn add(&mut self, bill: Bill) {
        self.inner.push(bill)
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}
