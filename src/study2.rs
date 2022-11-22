#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32)
}
#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }
#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = (Age::New, miles);
    return quality
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

//////////////////////////////////////////////////

fn main() {

    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut car: Car;
    let mut engine = Transmission::Manual;

    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
    

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}

fn arrayTest () {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
  

    let three_nums = vec![15, 3, 24];
    let three = three_nums[1];
    println!("Vector: {:?}, three = {}", three_nums, three); 
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes); 

    let mut fruit = Vec::<&str>::new();
    fruit.push("Apple");
    fruit.push("Apple");
    fruit.push("Apple");
    println!("Fruits: {:?}", fruit[3]); 
}