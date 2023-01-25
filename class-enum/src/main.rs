#[derive(Debug)]
struct TrafficLight {
    color: TrafficLightColor,
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }
    pub fn ref_state(&self) -> &TrafficLightColor {
        &self.color
    }
    pub fn get_state(&self) -> String {
        self.color.to_string()
    }
    pub fn turn_light(&mut self, light: TrafficLightColor) {
        self.color = light
    }
}

impl std::fmt::Display for TrafficLightColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_string = match self {
            TrafficLightColor::Green => "green",
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
        };
        write!(f, "{}", color_string)
    }
}

fn main() {
    let mut light = TrafficLight::new();
    println!("{:?}", light);
    println!("{}", light.get_state());
    light.turn_light(TrafficLightColor::Green);
    println!("{:?}", light);
    println!("{}", light.ref_state());
    light.turn_light(TrafficLightColor::Yellow);
    println!("{:?}", light);
}
