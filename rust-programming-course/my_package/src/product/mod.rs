use category::Category;
#[derive(Debug, PartialEq, Clone)]
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: Category
}

pub mod category;

impl Product {
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }

    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {
            id,
            name,
            price,
            category
        }
    }
}