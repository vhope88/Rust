use std::thread::sleep;
use core::borrow::Borrow;
use std::time::Duration;

// 交通灯
#[derive(Debug)]
enum TrafficLightState {
    Red { waiting_time: std::time::Duration },
    Yellow { waiting_time: std::time::Duration },
    Green { waiting_time: std::time::Duration },
}

struct TrafficLight {
    state: TrafficLightState,
}

fn change_light(state: &TrafficLightState) -> TrafficLightState {
    match state {
        TrafficLightState::Green { waiting_time } => {
            sleep(*waiting_time);
            TrafficLightState::Yellow { waiting_time: std::time::Duration::new(5, 0) }
        },
        TrafficLightState::Red { waiting_time } => {
            sleep(*waiting_time);
            TrafficLightState::Green { waiting_time: std::time::Duration::new(8, 0) }
        },
        TrafficLightState::Yellow { waiting_time } => {
            sleep(*waiting_time);
            TrafficLightState::Red { waiting_time: std::time::Duration::new(3, 0) }
        }
    }
}

fn main() {
    let mut state_machine = TrafficLight{
        state: TrafficLightState::Green { waiting_time: std::time::Duration::new(8, 0) }
    };

    loop {
        println!("{:?}", state_machine.state);
        state_machine.state = change_light(&state_machine.state)
    }
}