use relm4::Worker;
use std::time::Duration;

pub struct TimerModel;

#[derive(Debug)]
pub enum TimerOutput {
    Tick,
}

impl Worker for TimerModel {
    type Output = TimerOutput;
    type Init = ();
    type Input = ();

    fn init(_init: Self::Init, sender: relm4::ComponentSender<Self>) -> Self {
        let output_sender = sender.output_sender().clone();
        sender.command(move |_out, shutdown| {
            shutdown
                .register(async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(1));
                    interval.tick().await;
                    loop {
                        interval.tick().await;
                        output_sender.send(TimerOutput::Tick).unwrap();
                    }
                })
                .drop_on_shutdown()
        });
        Self {}
    }

    fn update(&mut self, _message: Self::Input, _sender: relm4::ComponentSender<Self>) {}
}
