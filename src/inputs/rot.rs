
use embassy_executor::task;
use embassy_rp::gpio::{AnyPin, Pull, Input};

use super::{INPUTS_CHANNEL, InputData};


#[derive(Clone)]
pub enum Direction {
    Clockwise,
    Anticlockwise,
}

#[task]
pub async fn rotenc(clk: AnyPin, dt: AnyPin) {
    let mut clk = Input::new(clk, Pull::Down);
    let dt = Input::new(dt, Pull::Down);

    let mut position = 0_i32;

    let psc = INPUTS_CHANNEL.publisher().unwrap();
    
    loop {

        clk.wait_for_any_edge().await;
        if dt.is_high() == clk.is_high() {
            position -= 1;
        } else {
            position += 1;
        }

        if position.abs() >= 2 {
            let dir = if position.is_positive() {
                Direction::Clockwise
            } else {
                Direction::Anticlockwise
            };

            position = 0;

            psc.publish(InputData { 
                direction: Some(dir),
                ..Default::default()
            }).await;
        };

    }
}
