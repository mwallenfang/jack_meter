use vizia::*;
use vizia::vg::femtovg;
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
    /// A special round peak meter
    Radial,
}

#[derive(Debug)]
pub enum MeterEvents {
    UpdatePosition(f32),
    ChangePeakDropSpeed(f32),
    ChangeSmoothingFactor(f32),
}

#[derive(Lens)]
pub struct Meter {
    pos: f32,
    max: f32,
    max_delay_ticker: i32,
    max_drop_speed: f32,
    smoothing_factor: f32,
    direction: Direction,
}

impl Model for Meter {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(param_change_event) = event.message.downcast() {
            match param_change_event {
                MeterEvents::UpdatePosition(n) => {
                    self.pos = self.pos - self.smoothing_factor * (self.pos - (*n).abs());

                    println!("{}, {}", n, self.pos);

                    if self.max < self.pos {
                        self.max = self.pos;
                        self.max_delay_ticker = 50;
                    }
                    if self.max_delay_ticker == 0 {
                        self.max -= self.max_drop_speed;

                        if self.max < 0.0 {
                            self.max = 0.0;
                        }
                    } else {
                        self.max_delay_ticker -= 1;
                    }

                    println!("{}, {}",n, self.pos);
                }
                MeterEvents::ChangePeakDropSpeed(n) => {
                    self.max_drop_speed = *n;
                }
                MeterEvents::ChangeSmoothingFactor(n) => {
                    self.smoothing_factor = *n;
                }
            }
        }
    }
}

impl Meter {
    pub fn new<L: Lens<Target = f32>>(
        cx: &mut Context,
        lens: L,
        direction: Direction,
    ) -> Handle<Self> {
        vizia::View::build(Self {
            pos: 0.0,
            max: 0.0,
            max_delay_ticker: 0,
            max_drop_speed: 0.05,
            smoothing_factor: 0.1,
            direction: direction,
        }, cx, |cx| {
            Binding::new(cx, lens, |cx, val| {
                cx.emit(MeterEvents::UpdatePosition(val.get(cx)));
               
            });
        })
    }
}

impl View for Meter {
    fn element(&self) -> Option<String> {
        Some("meter".to_string())
    }

    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {
        let width = cx.cache.get_width(cx.current);
        let height = cx.cache.get_height(cx.current);
        let pos_x = cx.cache.get_posx(cx.current);
        let pos_y = cx.cache.get_posy(cx.current);
        let value = self.pos;

        // let bar_color = cx
        //     .style
        //     .background_color
        //     .get(cx.current)
        //     .cloned()
        //     .unwrap_or_default()
        //     .into();

        // // Create variables for the rectangle
        // let front_x;
        // let front_y;
        // let front_w;
        // let front_h;

        // // Build the start and end positions of the back and front line
        // // according to the direction the meter is going and the value the meter is showing
        // match self.direction {
        //     Direction::DownToUp => {
        //         front_x = pos_x;
        //         front_y = pos_y + (1.0 - value) * height;

        //         front_w = width;
        //         front_h = value * height;
        //     }
        //     Direction::UpToDown => {
        //         front_x = pos_x;
        //         front_y = pos_y;

        //         front_w = width;
        //         front_h = value * height;
        //     }
        //     Direction::LeftToRight => {
        //         front_x = pos_x;
        //         front_y = pos_y;

        //         front_w = value * width;
        //         front_h = height;
        //     }
        //     Direction::RightToLeft => {
        //         front_x = pos_x + (1.0 - value) * width;
        //         front_y = pos_y;

        //         front_w = value * width;
        //         front_h = height;
        //     }
        //     _ => {
        //         front_x = pos_x + (1.0 - value) * width;
        //         front_y = pos_y;

        //         front_w = value * width;
        //         front_h = height;
        //     }
        // };

        // // Draw the bar
        // if value >= 1e-3 {
        //     let mut front_path = vg::Path::new();
        //     front_path.rect(front_x, front_y, front_w, front_h);

        //     let mut front_paint = vg::Paint::color(bar_color);

        //     canvas.fill_path(&mut front_path, front_paint);
        // }

        let mut front_path = femtovg::Path::new();
        front_path.rect(pos_x, pos_y, width, height);

        let mut front_paint = femtovg::Paint::color(femtovg::Color::rgb((value * 256.0 )as u8, (value * 256.0 )as u8, (value * 256.0 )as u8));

        canvas.fill_path(&mut front_path, front_paint);
    }
}

pub trait MeterHandle {
    fn peak_drop_speed(self, val: impl Res<f32>) -> Self;
    fn smoothing_factor(self, val: impl Res<f32>) -> Self;
    fn value<L: Lens<Target = f32>>(self, lens: L) -> Self;
}

impl MeterHandle for Handle<'_, Meter> {
    fn peak_drop_speed(self, val: impl Res<f32>) -> Self {
        val.set_or_bind(self.cx, self.entity, |cx, entity, value| {
            entity.emit(cx, MeterEvents::ChangePeakDropSpeed(value));
        });

        self
    }

    fn smoothing_factor(self, val: impl Res<f32>) -> Self {
        val.set_or_bind(self.cx, self.entity, |cx, entity, value| {
            entity.emit(cx, MeterEvents::ChangeSmoothingFactor(value));
        });

        self
    }

    fn value<L: Lens<Target = f32>>(self, lens: L) -> Self {
        lens.set_or_bind(self.cx, self.entity, |cx, entity, value| {
            entity.emit(cx, MeterEvents::UpdatePosition(value));
            entity.redraw(cx);
        });

        self
    }
}
