#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::{AtomicU32, self};

use defmt::{*};
use embassy_executor::{Spawner, task, _export::StaticCell};
use embassy_rp::{i2c::{I2c, self}, bind_interrupts, peripherals::I2C0, gpio::{Input, Pull, AnyPin}};
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Timer, Instant};
use embedded_graphics::{prelude::DrawTarget, pixelcolor::BinaryColor, mono_font::{MonoTextStyleBuilder, MonoFont, ascii::{FONT_4X6, FONT_6X10, FONT_10X20}}};
use ssd1306::{Ssd1306, size::DisplaySize128x64, I2CDisplayInterface, prelude::{I2CInterface, DisplayConfig}, mode::BufferedGraphicsMode};

use {defmt_rtt as _, panic_probe as _};

mod spedometer;
mod inputs;
mod ui;
pub mod ds;
pub mod prelude;

use ui::state::UIState;

use crate::{ui::{context::{UI, MediaQuery}, screens::TextScreen}, inputs::{button::button, INPUTS_CHANNEL, rot::rotenc, InputData}};

// #[task]
// pub async fn rotenc(clk: AnyPin, dt: AnyPin) {
//     let mut clk = Input::new(clk, Pull::Down);
//     let dt = Input::new(dt, Pull::Down);
    
//     loop {

//         clk.wait_for_rising_edge().await;

//         if dt.is_high() == clk.is_high() {
//             let p = x.load(atomic::Ordering::Relaxed);
//             x.store((p + 1) % 128, atomic::Ordering::Relaxed);
//             info!("CW");
//         } else {
//             let p = y.load(atomic::Ordering::Relaxed);
//             y.store((p + 1) % 64, atomic::Ordering::Relaxed);
//             info!("ACW");
//         }
        
//     }
// }

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

pub fn init_display<'d, T: i2c::Instance>(i2c: I2c<'d, T, i2c::Async>) -> Ssd1306<I2CInterface<I2c<'d, T, i2c::Async>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>> {

    let intf = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(intf, DisplaySize128x64, ssd1306::rotation::DisplayRotation::Rotate0).into_buffered_graphics_mode();
    display.init().unwrap();

    display
}



#[embassy_executor::main]
async fn main(spawner: Spawner) {

    let p = embassy_rp::init(Default::default());

    spawner.spawn(rotenc(p.PIN_2.into(), p.PIN_3.into())).unwrap();
    spawner.spawn(button(0, p.PIN_4.into(), Pull::Down)).unwrap();

    // let button = Input::new(p.PIN_4, Pull::Up);

    let i2c = I2c::new_async(p.I2C0, p.PIN_21, p.PIN_20, Irqs, i2c::Config::default());
    let mut display = init_display(i2c);

    let ts = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut ui: UI<'_, 5> = UI::new(
        UIState::Text(TextScreen::new("Hello, World!")), 
        ts, 
        MediaQuery { dimensions: (128, 64) },
    );

    let mut dtime = Instant::now();

    info!("Started PN532");
    
    let mut sub = INPUTS_CHANNEL.subscriber().unwrap();

    loop {
        let inputs = sub.next_message_pure().await;
    
        let now = Instant::now();

        ui.update(inputs).unwrap();
        
        if now.as_millis() - dtime.as_millis() > 50 {
            ui.render(&mut display).unwrap();

            display.flush().unwrap();

            dtime = now;
        }

        // Timer::after(Duration::from_millis(10)).await;
    }
}
