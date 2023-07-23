use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, text::Text};

use crate::{ui::{context::{UIContext, UIAction}}, prelude::*, inputs::{InputData, rot::Direction, button::ButtonState}};

use super::{Screen, MenuScreen, MenuItem};


#[derive(Copy, Clone)]
pub struct TextScreen {
    text: &'static str,
}

impl TextScreen {
    pub fn new(text: &'static str) -> Self {
        Self { text }
    }
}

const BACK_ACTION: MenuItem = MenuItem::back("BACK");
impl Screen for TextScreen {

    fn update<'a, const S: usize>(&mut self, _ctx: &mut UIContext<'a, S>, inputs: InputData) -> Result<UIAction> { 
        if let Some(x) = inputs.direction {
            self.text = match x {
                Direction::Anticlockwise => "Anticlockwise",
                Direction::Clockwise => "Clockwise",
            }
        } else if let Some(x) = inputs.button {
            match x {
                ButtonState::Released => self.text = "Clicked",
                ButtonState::Pressed => return Ok(
                UIAction::Push(
                    crate::ui::state::UIState::Menu(MenuScreen::new([&BACK_ACTION; 10]))
                )),
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
        
        let text = Text::new(self.text, Point::new(0, 6), ctx.text_style);
        text.draw(target).map_err(|_| BikeError::CannotDrawToTarget)?;

        Ok(())
    }
}