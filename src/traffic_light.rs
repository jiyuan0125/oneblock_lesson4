pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait Time {
    fn time(&self) -> u8;
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match *self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time() {
        let red = TrafficLight::Red;
        let yellow = TrafficLight::Yellow;
        let green = TrafficLight::Green;

        assert_eq!(red.time(), 30);
        assert_eq!(yellow.time(), 5);
        assert_eq!(green.time(), 45);
    }
}
