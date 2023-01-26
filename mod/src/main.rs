pub mod object {
    pub mod light;
}

fn main() {
    let mut traffic = crate::object::light::發光體::TrafficLight::new();
    let mut house = crate::object::light::發光體::HouseLight::new();
    println!("{:?}", traffic);
    println!("{:?}", house);
    traffic.set_state("green".to_string());
    house.set_state(true);
    println!("{:?}", traffic.get_name());
    println!("{:?}", traffic);
    println!("{:?}", house.get_name());
    println!("{:?}", house);
}
