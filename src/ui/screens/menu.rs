use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::{TextStyle, Text};

use crate::inputs::InputData;
use crate::inputs::button::ButtonState;
use crate::inputs::rot::Direction;
use crate::ui::context::UIAction;
use crate::ui::state::UIState;
use crate::ui::{context::UIContext};
use crate::prelude::*;

use super::Screen;

#[derive(Clone, Copy)]
pub struct MenuScreen<const A: usize = 10> {
    selected: usize,
    items: [&'static MenuItem; A],
}

impl<const A: usize> MenuScreen<A> {
    pub const fn new(items: [&'static MenuItem; A]) -> Self {
        Self { 
            selected: 0, 
            items,
        }
    }

    fn up(&mut self) {
        self.selected = (self.selected + 1) % A;
    }

    fn down(&mut self) {
        self.selected = (self.selected + A - 1) % A;
    }
}


impl<const A: usize> Screen for MenuScreen<A> {
    fn update<'a, const S: usize>(&mut self, ctx: &mut UIContext<'a, S>, inputs: InputData) -> Result<UIAction> {
        if let Some(direction) = inputs.direction {
            match direction {
                Direction::Clockwise => self.up(),
                Direction::Anticlockwise => self.down(),
            }
        } else if let Some(x) = inputs.button {
            match x {
                ButtonState::Pressed => return Ok(self.items[self.selected].action.clone()),
                ButtonState::Released => (),
            }
        }

        Ok(UIAction::None)
    }

    fn render<'a, const S: usize>(
        &self, 
        target: &mut impl DrawTarget<Color = BinaryColor>, 
        ctx: &UIContext<'a, S>,
    ) -> Result<()> {
        target.clear(BinaryColor::Off).map_err(|_| BikeError::CannotDrawToTarget)?;

        for i in self.selected..10 {
            let colour = if i == self.selected {
                BinaryColor::Off
            } else {
                BinaryColor::On
            };
            self.items[self.selected]
                .draw(target, ctx.mq.dimensions, Point::new(0, (i - self.selected) as i32 * 10), &ctx.text_style, colour)?;
        }

        Ok(())
    }
}
pub struct MenuItem {
    text: &'static str,
    action: &'static UIAction,
}

impl MenuItem {
    pub const fn new(text: &'static str, action: &'static UIAction) -> Self {
        Self { text, action }
    }

    pub const fn back(text: &'static str) -> Self {
        Self { text, action: &UIAction::Pop }
    }

    pub fn draw<'a>(
        &self, 
        target: &mut impl DrawTarget<Color = BinaryColor>,
        dimensions: (u32, u32),
        at: Point,
        text_style: &MonoTextStyle<'a, BinaryColor>,
        positive: BinaryColor,
    ) -> Result<()> {
        target.fill_solid(
            &Rectangle { 
                top_left: at,
                size: Size::new(dimensions.0, text_style.font.character_size.height + 1) 
            }, 
            positive.invert(),
        ).map_err(|_| BikeError::CannotDrawToTarget)?;

        let mut style = text_style.clone();
        style.text_color = Some(positive);
        let point = Point::new(at.x + 1, at.y + style.font.character_size.height as i32 - 1);

        let text = Text::new(self.text, point, style);
        text.draw(target).map_err(|_| BikeError::CannotDrawToTarget)?;

        Ok(())
    }
}