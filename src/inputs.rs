use embassy_sync::{pubsub::PubSubChannel, blocking_mutex::raw::ThreadModeRawMutex};

use self::{rot::Direction, button::ButtonState};

pub mod rot;
pub mod button;

pub static INPUTS_CHANNEL: PubSubChannel<ThreadModeRawMutex, InputData, 2, 2, 3> = PubSubChannel::new();

#[derive(Clone)]
pub struct InputData {
    pub direction: Option<Direction>,
    pub button: Option<ButtonState>
}


impl Default for InputData {
    fn default() -> Self {
        Self { direction: None, button: None }
    }
}