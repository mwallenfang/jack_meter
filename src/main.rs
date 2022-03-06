mod meter;

use std::{io, thread};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use atomic_float::AtomicF32;
use vizia::*;
use crate::meter::{Direction, MeterBar};

static value: AtomicF32 = AtomicF32::new(0.2);

#[derive(Lens)]
pub struct Data {
    pos: f32,
    buffer: [f32; 128]
}

impl Model for Data {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(gain_event) = event.message.downcast() {
            match gain_event {
                Events::Yes(n) => {
                    self.pos = *n;
                    value.store(*n, Ordering::Relaxed);
                }
            }
        }
    }
}

enum Events {
    Yes(f32)
}

fn main() {
    thread::spawn(|| {
        loop {
            if let Some(n) = read_freq() {
                value.store(n, Ordering::Relaxed);
            }
        }
    });

    Application::new(WindowDescription::new().with_inner_size(100,500), |cx| {
        Data{pos: 0.2, buffer: [0.0;128]}.build(cx);
        VStack::new(cx, |cx| {
            Label::new(cx, Data::pos);
            MeterBar::new(cx, Data::pos, Direction::DownToUp)
                .background_color(Color::black())
                .width(Percentage(20.0));
            Label::new(cx, "Some text");

            MeterBar::new(cx, Data::pos, Direction::UpToDown)
                .background_color(Color::black());
            Label::new(cx, "Some text");

            MeterBar::new(cx, Data::pos, Direction::LeftToRight)
                .background_color(Color::black());
            Label::new(cx, "Some text");

            MeterBar::new(cx, Data::pos, Direction::RightToLeft)
                .background_color(Color::black());
            Label::new(cx, "Some text");
        });

    })
        .on_idle(|cx| {
            cx.emit(Events::Yes(value.load(Ordering::Relaxed)));
        }).run();
}


/// Attempt to read a frequency from standard in. Will block until there is
/// user input. `None` is returned if there was an error reading from standard
/// in, or the retrieved string wasn't a compatible u16 integer.
fn read_freq() -> Option<f32> {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => f32::from_str(user_input.trim()).ok(),
        Err(_) => None,
    }
}