pub mod light {
    #[derive(Debug)]
    pub struct HouseLight {
        state: bool,
    }
    impl HouseLight {
        pub fn new() -> Self {
            Self { state: false }
        }
        pub fn get_name(&self) -> &str {
            "House light"
        }
        pub fn set_state(&mut self, new_state: bool) {
            self.state = new_state
        }
    }
    #[derive(Debug)]
    pub struct TrafficLight {
        state: String,
    }

    impl TrafficLight {
        pub fn new() -> Self {
            Self {
                state: "Red".to_string(),
            }
        }
        pub fn get_name(&self) -> &str {
            "Traffic light"
        }
        pub fn set_state(&mut self, new_state: String) {
            self.state = new_state
        }
    }
}
