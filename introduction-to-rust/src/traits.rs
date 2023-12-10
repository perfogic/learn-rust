use std::string::String;

pub trait Bank {
    fn print_default_balance(&self);
}

pub trait Initialize<T> where T: Bank {
    fn new(id: u32, name: String) -> T;
}

pub struct Momo {
    id: u32,
    name: String,
}

impl Bank for Momo {
    fn print_default_balance(&self) {
        println!("Momo: {} - {}", self.id, self.name);
    }
}

impl Initialize<Momo> for Momo {
    fn new(id: u32, name: String) -> Momo {
        Momo {
            id, name
        }
    }
}

pub struct VnPay {
    id: u32,
    name: String
}

impl Bank for VnPay {
    fn print_default_balance(&self) {
        println!("VnPay: {} - {}", self.id, self.name);
    }
}

impl Initialize<VnPay> for VnPay {
    fn new(id: u32, name: String) -> VnPay {
        VnPay {
            id, name
        }
    }
}

pub fn return_bank(choice: u32) -> Box<dyn Bank> {
    if choice % 2 == 0 {
        return Box::new(Momo {
            id: 0,
            name: "genesis".to_string(),
        });
    }

    Box::new(VnPay {
        id: 0,
        name: "genesis".to_string(),
    })
}