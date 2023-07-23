use embassy_executor::task;
use embassy_rp::gpio::{AnyPin, Input, Pull, Level};
use super::{INPUTS_CHANNEL, InputData};

#[derive(Clone)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[task]
pub async fn button(id: u8, button: AnyPin, pull: Pull) {
    let mut button = Input::new(button, pull);

    let psc = INPUTS_CHANNEL.publisher().unwrap();
    
    loop {

        button.wait_for_any_edge().await;


        psc.publish(InputData {
            button: Some(match button.get_level() {
                Level::Low => ButtonState::Released,
                Level::High => ButtonState::Pressed,
            }),
            ..Default::default()
        }).await;
    }
}