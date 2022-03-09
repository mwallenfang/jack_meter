mod meter;

use std::{io, thread};
use std::collections::vec_deque::VecDeque;
use std::str::FromStr;
use std::sync::atomic::Ordering;
use atomic_float::AtomicF32;
use vizia::*;
use jack;
use crate::meter::{Direction, Meter};

static SENT_VALUE: AtomicF32 = AtomicF32::new(0.0);

#[derive(Lens)]
pub struct Data {
    pos: f32,
    buffer: VecDeque<f32>,
    buffer_size: i32
}

impl Model for Data {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(gain_event) = event.message.downcast() {
            match gain_event {
                Events::UpdateValue(n) => {
                    self.buffer.push_front(*n);
                    if self.buffer.len() > self.buffer_size as usize {
                        self.buffer.pop_back();
                    }
                    let new_pos = self.buffer.iter().sum::<f32>() / self.buffer_size as f32;
                    self.pos = new_pos;
                }
            }
        }
    }
}

enum Events {
    UpdateValue(f32)
}

fn main() {
    // 1. open a client
    let (client, _status) =
        jack::Client::new("jack_meter", jack::ClientOptions::NO_START_SERVER).unwrap();

    let in_port = client
        .register_port("meter_in", jack::AudioIn::default())
        .unwrap();

    let process = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let in_p = in_port.as_slice(ps);

            // Write output
            for val in in_p {
                SENT_VALUE.store(-80.0 / lin2db((*val).abs()), Ordering::Relaxed);
            }

            // Continue as normal
            jack::Control::Continue
        },
    );

    // 4. Activate the client. Also connect the ports to the system audio.
    let _active_client = client.activate_async((), process).unwrap();

    Application::new(WindowDescription::new().with_inner_size(100,500), |cx| {
        Data{pos: 0.2, buffer: VecDeque::new(), buffer_size: 128}.build(cx);
        VStack::new(cx, |cx| {
            Label::new(cx, Data::pos);
            Meter::new(cx, Data::pos, Direction::DownToUp)
                .background_color(Color::green())
                .width(Percentage(20.0));
            Label::new(cx, "Some text");

            Meter::new(cx, Data::pos, Direction::UpToDown)
                .background_color(Color::black());
            Label::new(cx, "Some text");

            Meter::new(cx, Data::pos, Direction::LeftToRight)
                .background_color(Color::black());
            Label::new(cx, "Some text");

            Meter::new(cx, Data::pos, Direction::RightToLeft)
                .background_color(Color::black());
            Label::new(cx, "Some text");
        });

    })
        .on_idle(|cx| {
            cx.emit(Events::UpdateValue(SENT_VALUE.load(Ordering::Relaxed)));
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

pub fn lin2db(v: f32) -> f32 {
    20.0 * v.log10()
}