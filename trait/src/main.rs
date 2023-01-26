#[derive(Debug)]
struct HouseLight {
    is_on: bool,
}

impl HouseLight {
    pub fn new() -> Self {
        Self { is_on: false }
    }
    pub fn get_state(&self) -> &bool {
        &self.is_on
    }
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: "Red".to_string(),
        }
    }
    pub fn get_state(&self) -> &String {
        &self.color
    }
}
// create trait
trait Light {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &dyn std::fmt::Debug;
}

impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.is_on
    }
}

impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.color
    }
}

// use trait create function
fn print_light_state(light: &impl Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}

fn main() {
    let traffic_light = TrafficLight::new();
    let house_light = HouseLight::new();

    print_light_state(&traffic_light);
    print_light_state(&house_light);
}
