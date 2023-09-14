use serde::Serialize;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use warp::Filter;
#[tokio::main]
async fn main() {
    // x y z
    let mut robot_init = Arc::new(Mutex::new((0.0, 0.0)));

    let mut robot_init_clone = Arc::clone(&robot_init);
    let hello = warp::get()
        .and(warp::path("calc"))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |p: HashMap<String, String>| {
            let x = p.get("x").unwrap().parse::<f32>().unwrap();
            let y = p.get("y").unwrap().parse::<f32>().unwrap();

            let movement_left = calculate((x, y), WheelRollDirection::Lefty).unwrap();
            let movement_right = calculate((x, y), WheelRollDirection::Righty).unwrap();
            let movements = (movement_right, movement_left);

            warp::reply::json(&movements)
        });

    warp::serve(hello).run(([127, 0, 0, 1], 5432)).await;
}

enum WheelRollDirection {
    Righty = 1,
    Lefty = 2,
    Fallback = 0,
}

impl WheelRollDirection {
    fn value(num: isize) -> WheelRollDirection {
        match num {
            1 => WheelRollDirection::Righty,
            2 => WheelRollDirection::Lefty,
            _ => WheelRollDirection::Fallback,
        }
    }
}

#[derive(Debug, Serialize)]
struct OmniWheelMovement {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}

fn calculate(gamepad: (f32, f32), direction: WheelRollDirection) -> Option<OmniWheelMovement> {
    let freewheel: (f32, f32) = (gamepad.0, gamepad.0);

    // sets for lefty
    let gamepad_mult: (f32, f32, f32) = (gamepad.0, freewheel.0, -freewheel.1);

    let magnitude: f32 = match direction {
        WheelRollDirection::Lefty => gamepad.1 - gamepad_mult.2,
        WheelRollDirection::Righty => gamepad.1 - freewheel.1,
        WheelRollDirection::Fallback => 0.0,
    };

    return match direction {
        WheelRollDirection::Righty => Some(OmniWheelMovement {
            top_left: 0.0,
            top_right: magnitude,
            bottom_right: 0.0,
            bottom_left: magnitude,
        }),
        WheelRollDirection::Lefty => Some(OmniWheelMovement {
            top_left: magnitude,
            top_right: 0.0,
            bottom_right: magnitude,
            bottom_left: 0.0,
        }),
        WheelRollDirection::Fallback => None,
    };
}
