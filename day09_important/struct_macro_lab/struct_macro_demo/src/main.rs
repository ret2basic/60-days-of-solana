use struct_macro_macros::{foo_bar_attribute, DoubleFoo};

struct Person {
    age: u8,
    name: String,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn can_drink(&self) -> bool {
        self.age >= 21
    }

    fn age_in_one_year(&self) -> u8 {
        self.age + 1
    }
}

trait Speed {
    fn get_speed_kph(&self) -> f64;
}

struct Car {
    speed_mph: f64,
}

struct Boat {
    speed_knots: f64,
}

impl Speed for Car {
    fn get_speed_kph(&self) -> f64 {
        self.speed_mph * 1.60934
    }
}

impl Speed for Boat {
    fn get_speed_kph(&self) -> f64 {
        self.speed_knots * 1.852
    }
}

#[foo_bar_attribute]
struct InsertedFieldsDemo {
    baz: i32,
}

#[derive(Debug, DoubleFoo)]
struct DerivedFoo {
    foo: i32,
    bar: i32,
}

fn main() {
    let person = Person::new(String::from("Jesserc"), 19);
    println!("can drink: {:?}", person.can_drink());
    println!("age in one year: {:?}", person.age_in_one_year());
    println!("name: {:?}", person.name);

    let car = Car { speed_mph: 60.0 };
    let boat = Boat { speed_knots: 30.0 };
    println!("Car Speed: {} km/h", car.get_speed_kph());
    println!("Boat Speed: {} km/h", boat.get_speed_kph());

    let inserted = InsertedFieldsDemo::default();
    println!("struct is {:?}", inserted);
    println!("double foo: {}", inserted.double_foo());

    let derived = DerivedFoo { foo: 3, bar: 7 };
    println!("derived struct is {:?}", derived);
    println!("derived bar: {}", derived.bar);
    println!("derived double foo: {}", derived.double_foo());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_impl_methods_work() {
        let person = Person::new(String::from("Jesserc"), 19);

        assert!(!person.can_drink());
        assert_eq!(person.age_in_one_year(), 20);
        assert_eq!(person.name, "Jesserc");
    }

    #[test]
    fn speed_trait_converts_units() {
        let car = Car { speed_mph: 60.0 };
        let boat = Boat { speed_knots: 30.0 };

        assert!((car.get_speed_kph() - 96.5604).abs() < 0.0001);
        assert!((boat.get_speed_kph() - 55.56).abs() < 0.0001);
    }

    #[test]
    fn attribute_like_macro_can_rewrite_a_struct() {
        let demo = InsertedFieldsDemo::default();

        assert_eq!(demo.double_foo(), 20);
        assert_eq!(format!("{:?}", demo), "InsertedFieldsDemo { foo: 10, bar: 20 }");
    }

    #[test]
    fn derive_macro_augments_instead_of_rewriting() {
        let derived = DerivedFoo { foo: 3, bar: 7 };

        assert_eq!(derived.double_foo(), 6);
        assert_eq!(derived.bar, 7);
    }
}
