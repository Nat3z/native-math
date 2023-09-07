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
            let vector_change_speed = p
                .get("vector_change_speed")
                .unwrap()
                .parse::<f32>()
                .unwrap();
            let mut robot_init_guard = robot_init_clone.lock().unwrap();

            let movement = calculate((x, y), vector_change_speed, 45.0, -45.0, -135.0, 135.0);

            format!("{:?}", movement)
        });

    warp::serve(hello).run(([127, 0, 0, 1], 5432)).await;
}

#[derive(Debug)]
struct OmniWheelMovement {
    top_left: f32,
    top_right: f32,
    bottom_right: f32,
    bottom_left: f32,
}
fn calculate(
    gamepad: (f32, f32),
    vector_change_speed: f32,
    current_angle_top_left: f32,
    current_angle_top_right: f32,
    current_angle_bottom_right: f32,
    current_angle_bottom_left: f32,
) -> OmniWheelMovement {
    let theta = gamepad.1.atan2(gamepad.0);
    let theta = theta.to_degrees();
    // take stand still state, convert through wheel movement into the correct end position
    // first, compare each current angle value
    // to theta and find the difference
    println!("Theta: {}", theta);
    let angle_difference_top_left = theta - current_angle_top_left;
    let angle_difference_top_right = theta - current_angle_top_right;
    let angle_difference_bottom_right = theta - current_angle_bottom_right;
    let angle_difference_bottom_left = theta - current_angle_bottom_left;

    let ng_diff_vect_top_left = angle_difference_top_left * vector_change_speed;
    let ng_diff_vect_top_right = angle_difference_top_right * vector_change_speed;
    let ng_diff_vect_bottom_right = angle_difference_bottom_right * vector_change_speed;
    let ng_diff_vect_bottom_left = angle_difference_bottom_left * vector_change_speed;

    // TODO: Deal with exceptions where wheels cant produce angle
    OmniWheelMovement {
        top_left: ng_diff_vect_top_left,
        top_right: ng_diff_vect_top_right,
        bottom_right: ng_diff_vect_bottom_right,
        bottom_left: ng_diff_vect_bottom_left,
    }
}
