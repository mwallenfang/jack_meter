use vizia::*;

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

#[derive(Lens)]
pub struct MeterData {
    pos: f32,
    max: f32,
    max_delay_ticker: i32,
    max_drop_speed: f32,
    smoothing_factor: f32,
}

impl Model for MeterData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(param_change_event) = event.message.downcast() {
            match param_change_event {
                MeterEvents::UpdatePosition(n) => {
                    self.pos = self.pos - self.smoothing_factor * (self.pos - (*n).abs());

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
                }
                MeterEvents::ChangePeakDropSpeed(n) => {
                    println!("B");
                    self.max_drop_speed = *n;
                }
            }
        }
    }
}

pub enum MeterEvents {
    UpdatePosition(f32),
    ChangePeakDropSpeed(f32),
}

pub struct Meter<L> {
    /// The position of the meter in [0,1]
    lens: L,
    /// The directions the meter's ends are pointing in
    direction: Direction,
}

impl<L: Lens<Target = f32>> Meter<L> {
    pub fn new(
        cx: &mut Context,
        lens: L,
        direction: Direction,
        smoothing_factor: f32,
    ) -> Handle<Self> {
        Self {
            lens: lens.clone(),
            direction,
        }
        .build2(cx, move |cx| {
            MeterData {
                pos: 0.1,
                max: 0.0,
                max_delay_ticker: 0,
                max_drop_speed: 0.0,
                smoothing_factor,
            }
            .build(cx);

            Binding::new(cx, lens, |cx, value| {
                cx.emit(MeterEvents::UpdatePosition(value.get(cx)));
            });
            ZStack::new(cx, |cx| {
                MeterBar::new(cx)
                    .height(MeterData::pos.map(|val| Percentage(val * 100.0)))
                    .top(Stretch(1.0))
                    .width(Stretch(1.0))
                    .background_color(Color::red());

                MeterLine::new(cx)
                    .width(Stretch(1.0))
                    .height(Pixels(2.0))
                    .top(Stretch(1.0))
                    .bottom(MeterData::max.map(|val| Percentage(val * 100.0)))
                    .background_color(Color::black());
            });
        })
    }
}

impl<L: Lens<Target = f32>> View for Meter<L> {
    fn element(&self) -> Option<String> {
        Some("meter".to_string())
    }
}

pub trait MeterHandle {
    fn peak_drop_speed<L: Lens<Target = f32>>(self, lens: L) -> Self
    where
        L: Lens<Target = f32>;

    fn peak_drop_speed_const(self, val: f32) -> Self;
}

impl<T> MeterHandle for Handle<'_, Meter<T>> {
    fn peak_drop_speed<L: Lens<Target = f32>>(self, lens: L) -> Self {
        Binding::new(self.cx, lens, move |cx, value| {
            let value = value.get(cx);
            println!("A");
            cx.emit(MeterEvents::ChangePeakDropSpeed(value));
        });

        self
    }

    fn peak_drop_speed_const(self, val: f32) -> Self {
        self.cx.emit(MeterEvents::ChangePeakDropSpeed(val));
        println!("A");
        self
    }
}

pub struct MeterBar {}

impl MeterBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx)
    }
}

impl View for MeterBar {}

pub struct MeterLine {}

impl MeterLine {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx)
    }
}

impl View for MeterLine {
    fn element(&self) -> Option<String> {
        Some("meter_line".to_string())
    }
}
