use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::BinaryColor, prelude::*};

use crate::{ds::Stack, inputs::InputData};
use crate::prelude::*;
use super::{state::UIState, screens::Screen};

pub struct UI<'a, const S: usize = 5> {
    current: UIState,
    ctx: UIContext<'a, S>,
}

pub struct UIContext<'a, const S: usize = 5> {
    layers: Stack<UIState, S>,
    pub text_style: MonoTextStyle<'a, BinaryColor>,
    pub mq: MediaQuery,
}

#[derive(Clone)]
pub enum UIAction {
    None,
    Pop,
    Push(UIState),
}

impl<'a, const S: usize> UI<'a, S> {
    pub fn new(top: UIState, style: MonoTextStyle<'a, BinaryColor>, mq: MediaQuery) -> Self {
        Self {
            current: top,
            ctx: UIContext::new(style, mq),
        }
    }
}

impl<'a, const S: usize> UIContext<'a, S> {
    pub fn new(style: MonoTextStyle<'a, BinaryColor>, mq: MediaQuery) -> Self {
        Self {
            layers: Stack::new(),
            text_style: style,
            mq,
        }
    }
}

impl<'a, const S: usize> UI<'a, S> {
    pub fn update(&mut self, inputs: InputData) -> Result<()> {
        let action = self.current.update(&mut self.ctx, inputs)?;

        match action {
            UIAction::None => (),
            UIAction::Pop => self.current = self.ctx.layers.pop().ok_or(BikeError::StackEmpty)?,
            UIAction::Push(x) => {
                self.ctx.layers.push(self.current.clone())?;
                self.current = x;
            },
        }

        Ok(())
    }

    pub fn render(&mut self, target: &mut impl DrawTarget<Color = BinaryColor>) -> Result<()> {
        self.current.render(target, &self.ctx)
    }
}

pub struct MediaQuery {
    pub dimensions: (u32, u32),
}