use std::collections::HashMap;
use std::hash::Hash;
use std::vec;


#[derive(Debug, Clone, Eq, PartialEq)]
struct Car {
    name: String,
    year: u32,
    fuel_level: u32,
    price: u32,
}

impl Car {
    fn print_price(&self) {
        println!("Price: {}", self.price);
    }

    fn add_price(&mut self, val: u32) {
        self.price = self.price + val; 
    }
}

#[derive(Debug)]
struct Point2D(u32, u32);

#[derive(Debug)]
enum Status {
    FALSE,
    TRUE
}

fn get_car_price(car_name: &String, car_db: &Vec<Car>) -> Result<u32, String> {
    for car in car_db {
        if (&car.name == car_name) {
            return Ok(car.price);   
        }
    }
    Err(String::from("Not found car"))
} 

struct S {
    x: i32,
}

fn main() {
    let mut my_car = Car {
        name: String::from("Mercedes"),
        year: 2024,
        fuel_level: 5,
        price: 500000,
    };
    my_car.fuel_level = 6;
    println!("Car: {:?}", my_car);

    let another_car = Car {
        name: String::from("BMW"),
        ..my_car
    };

    let cars_db = vec![
        my_car.clone(), another_car.clone()
    ];

    let price = get_car_price(&"Lucas".to_string(), &cars_db);

    match price {
        Ok(u32) => {
            println!("Price: {u32}");
        },
        Err(err) => {
            println!("Err: {:?}", err);
        }
    }

    /**
     * Car: Car { name: "Mercedes", year: 2024, fuel_level: 6, price: 500000 }
     * Another Car: Car { name: "BMW", year: 2024, fuel_level: 6, price: 500000 }
     */
    println!("Car: {:?}", my_car);
    println!("Another Car: {:?}", another_car);

    // Additional
    my_car.print_price();
    my_car.add_price(5);
    my_car.print_price();

    // Structs as tuples
    let point_2D_1 = Point2D(3,4);
    println!("Point 2D: {:?}", point_2D_1);

    // Enum
    let status = Status::TRUE;
    println!("Status: {:?}", status);


    // Hash map
    let mut car_owners: HashMap<String, Car> = HashMap::new();
    car_owners.insert("Pham Minh Dang".to_string(), my_car.clone());
    car_owners.insert("Dinh Quoc Bao".to_string(), another_car.clone());

    match car_owners.get("Pham Minh Dang") {
        Some(car) => {
            println!("Found {:?}", car);
        },
        None => {
            println!("Not found this car");
        }
    }

    for (name, car) in &car_owners {
        println!("Name: {}, car: {:?}", name, car);
    }

    let some_vec = vec![5,5,1,0,8,8,3,2];
    let mut freq_vec: HashMap<u32, u32> = HashMap::new();
    for i in some_vec {
        let freq = freq_vec.entry(i).or_insert(0);
        *freq += 1;
    }
    println!("Freq vec {:?}", freq_vec);

    let mut s1 = S { x: 2 };
    let v = &mut s1;
    v.x += 1;
    print!("{}", v.x);


}
