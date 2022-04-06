mod meter;
mod meter_new;

use crate::meter_new::{Direction, Meter, MeterHandle};
use atomic_float::AtomicF32;
use jack;
use std::sync::atomic::Ordering;
use vizia::*;

static SENT_VALUE: AtomicF32 = AtomicF32::new(0.0);

const STYLE: &str = include_str!("style.css");

#[derive(Lens)]
pub struct Data {
    input: f32,
    drop_speed: f32,
    col: String
}

impl Model for Data {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(gain_event) = event.message.downcast() {
            match gain_event {
                Events::UpdateValue(n) => {
                    self.input = *n;
                }
            }
        }
    }
}

enum Events {
    UpdateValue(f32),
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
                SENT_VALUE.store((*val).abs(), Ordering::Relaxed);
                // SENT_VALUE.store(lin2db((*val).abs()) / -80.0, Ordering::Relaxed);
            }

            // Continue as normal
            jack::Control::Continue
        },
    );

    // 4. Activate the client. Also connect the ports to the system audio.
    let _active_client = client.activate_async((), process).unwrap();

    Application::new(WindowDescription::new().with_inner_size(300, 300), |cx| {
        cx.add_theme(STYLE);
        Data {
            input: 0.0,
            drop_speed: 0.1,
            col: String::from("#ffff00")
        }
        .build(cx);
        VStack::new(cx, |cx| {
            Label::new(cx, Data::input);
            Meter::new(cx, Data::input, Direction::DownToUp)
                .smoothing_factor(0.1)
                .peak_drop_speed(0.005)
                .bar_color(Data::col)
                .left(Stretch(1.0))
                .right(Stretch(1.0));
        });
    })
    .on_idle(|cx| {
        cx.emit(Events::UpdateValue(SENT_VALUE.load(Ordering::Relaxed)));
    })
    .run();
}

pub fn lin2db(v: f32) -> f32 {
    20.0 * v.log10()
}
