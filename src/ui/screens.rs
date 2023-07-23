use embedded_graphics::{prelude::DrawTarget, pixelcolor::BinaryColor};
use crate::{prelude::*, inputs::InputData};
use super::context::{UIContext, UIAction};


mod text;
pub use text::*;

mod menu;
pub use menu::*;

pub trait Screen {
    fn update<'a, const S: usize>(&mut self, ctx: &mut UIContext<'a, S>, inputs: InputData) -> Result<UIAction>;

    fn render<'a, const S: usize>(
        &self, 
        target: &mut impl DrawTarget<Color = BinaryColor>, 
        ctx: &UIContext<'a, S>,
    ) -> Result<()>;
}
