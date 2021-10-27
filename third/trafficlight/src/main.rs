enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait LightUpTime {
    fn light_up_time_in_second(&self) -> u32;
}

impl LightUpTime for TrafficLight {
    fn light_up_time_in_second(&self) -> u32 {
        match self {
            TrafficLight::Red => {
                return 30;
            }
            TrafficLight::Yellow => {
                return 2;
            }
            TrafficLight::Green => {
                return 28;
            }
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    println!(
        "The red light will light up {:?} seconds",
        red_light.light_up_time_in_second()
    );
    let yellow_light = TrafficLight::Yellow;
    println!(
        "The yellow light will light up {:?} seconds",
        yellow_light.light_up_time_in_second()
    );
    let green_light = TrafficLight::Green;
    println!(
        "The green light will light up {:?} seconds",
        green_light.light_up_time_in_second()
    );
}
