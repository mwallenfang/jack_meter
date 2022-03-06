use std::collections::VecDeque;
use vizia::*;
use vizia::Color as vizColor;
use femtovg::{Paint, Path, LineCap, Solidity};
use femtovg::Color as femColor;
use itertools::cloned;

/// The direction the meter bar shows the peak in.
/// The semantic is LowToHigh, so DownToUp is the standard vertical meter design
///
/// This is also used to decide the orientation of the meter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum Direction {
    /// The standard vertical meter direction
    DownToUp,
    /// The inverted direction from the standard vertical meter
    UpToDown,
    /// The standard horizontal meter direction
    LeftToRight,
    /// The inverted direction from the standard horizontal meter
    RightToLeft,
}

pub struct MeterBar<L> {
    /// The position of the meter in [0,1]
    lens: L,
    /// The directions the meter's ends are pointing in
    direction: Direction
}

impl<L: Lens<Target=f32>> MeterBar<L> {
    pub fn new(
        cx: &mut Context,
        lens: L,
        direction: Direction
    ) -> Handle<Self> {
        Self {
            lens: lens.clone(),
            direction
        }.build(cx)
    }
}

impl<L: Lens<Target=f32>> View for MeterBar<L> {
    fn element(&self) -> Option<String> {
        Some("meter_bar".to_string())
    }

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {
        let width = cx.cache.get_width(cx.current);
        let height = cx.cache.get_height(cx.current);
        let pos_x = cx.cache.get_posx(cx.current);
        let pos_y = cx.cache.get_posy(cx.current);
        let value = *self.lens.get(cx);
        println!("{}, {}, {}, {}, {}, {:?}", width, height, pos_x, pos_y, value, self.direction);

        let back_color: femColor = cx.style.background_color
            .get(cx.current).cloned().unwrap_or_default().into();
        let front_color = femColor::rgb(255,0,0);

        // Create variables for the rectangle corners
        let back_x = pos_x;
        let back_y = pos_y;
        let back_w = width;
        let back_h = height;

        let front_x;
        let front_y;
        let front_w;
        let front_h;

        // Build the start and end positions of the back and front line
        // according to the direction the meter is going and the value the meter is showing
        match self.direction {
            Direction::DownToUp => {
                front_x = pos_x;
                front_y = pos_y + (1.0-value) * height;

                front_w = width;
                front_h = value * height;
            },
            Direction::UpToDown => {
                front_x = pos_x;
                front_y = pos_y;

                front_w = width;
                front_h = value * height;
            },
            Direction::LeftToRight => {
                front_x = pos_x;
                front_y = pos_y;

                front_w = value * width;
                front_h = height;
            },
            Direction::RightToLeft => {
                front_x = pos_x + (1.0-value) * width;
                front_y = pos_y;

                front_w = value * width;
                front_h = height;
            }
        };

        // Draw the back path
        let mut back_path = Path::new();
        back_path.rect(back_x, back_y, back_w, back_h);

        let mut back_paint = Paint::color(back_color);

        canvas.fill_path(&mut back_path, back_paint);

        // Draw the front path
        let mut front_path = Path::new();
        front_path.rect(front_x, front_y, front_w, front_h);

        let mut front_paint = Paint::color(front_color);

        canvas.fill_path(&mut front_path, front_paint);
    }
}
