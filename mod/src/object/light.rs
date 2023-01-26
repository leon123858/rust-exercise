pub mod 發光體 {
    #[derive(Debug)]
    pub struct HouseLight {
        state: bool,
    }

    impl Default for HouseLight {
        fn default() -> Self {
            Self::new()
        }
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

    impl Default for TrafficLight {
        fn default() -> Self {
            Self::new()
        }
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
