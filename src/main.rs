
struct SimpleMath {

}

impl SimpleMath {

    fn area(&self, width :f32, height : f32) -> f32 {
        return width * height
    }

    fn add(&self, a :f32, b :f32 ) -> f32 {
        return a + b;
    }

    fn subtract(&self, a: i32, b: i32) -> i32 {
        return  a - b;
    }

    fn divide(&self, a: i32, b: i32) -> i32 {
        return a / b;
    }

    fn is_odd(&self, a_int: i32) -> bool {
        let mod_result = a_int % 2;

        if mod_result == 1 {
            return true;
        }
        
        return false;
    }

    fn is_prime(&self, prime_suspect: u32) -> bool {
        
        let mut index :u32 = 2;

        if prime_suspect <= 1 {
            return false
        }

        while index < prime_suspect {

            if  prime_suspect % index == 0{
                return false;
            }
            index += 1;
        }
        
        return true;
    }

}



fn main() {

    let math_obj = SimpleMath{};

    let area_result: f32 = math_obj.area(5.0, 6.0);
    let add_result: f32 = math_obj.add(5.0, 6.0);
    let subtract_result: i32 = math_obj.subtract(3, 6);
    let divide_result: i32 = math_obj.divide(10, 5);
    let is_odd_result: bool = math_obj.is_odd(10);
    let is_prime_result: bool = math_obj.is_prime(11);

    println!("{}", area_result);
    println!("{}", add_result);
    println!("{}", subtract_result);
    println!("{}", divide_result);
    println!("{}", is_odd_result);
    println!("{}", is_prime_result);

}
