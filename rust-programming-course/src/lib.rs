mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn safe_new(radius: f32) -> Result<Circle, String> {
            if radius >= 0.0 {
                Ok(Circle { radius })
            } else {
                Err(String::from("Radius has to be greater than 0"))
            }
        }

        // Not use in realworld
        pub fn panic_new(radius: f32) -> Circle {
            match radius {
                ..=0.0 => panic!("Radius has to be greater than 0"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(3.0);
        assert_eq!(larger_circle.contains(&smaller_circle), true);
    }

    #[test]
    fn smaller_should_not_contain_larger_circle() {
        let larger_circle = shapes::Circle::new(5.0);
        let smaller_circle = shapes::Circle::new(3.0);
        assert_eq!(smaller_circle.contains(&larger_circle), false);
    }

    #[test]
    #[should_panic(expected="Radius has to be greater than 0")]
    fn should_not_create_and_panic() {
        shapes::Circle::panic_new(-1.0);
    }
}
