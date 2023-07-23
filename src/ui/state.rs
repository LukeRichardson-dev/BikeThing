use defmt::info;
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, text::Text};

use super::{context::{UIContext, UIAction}, screens::{Screen, TextScreen, MenuScreen}};
use crate::{prelude::*, inputs::{rot::Direction, InputData}};


#[derive(Copy, Clone)]
pub enum UIState {
    Text(TextScreen),
    Menu(MenuScreen),
}

impl Screen for UIState {
    fn update<'a, const S: usize>(&mut self, ctx: &mut UIContext<'a, S>, inputs: InputData) -> Result<UIAction> {
        match self {
            UIState::Text(s) => s.update(ctx, inputs),
            UIState::Menu(m) => m.update(ctx, inputs),
        }
    }

    fn render<'a, const S: usize>(&self, target: &mut impl DrawTarget<Color = BinaryColor>, ctx: &UIContext<'a, S>) -> Result<()> {
        match self {
            UIState::Text(s) => s.render(target, ctx),
            UIState::Menu(m) => m.render(target, ctx),
        }
    }
}
