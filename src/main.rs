use std::time::Duration;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::{
    adw,
    gtk::{self},
    Component, ComponentParts, ComponentSender, RelmApp, Worker, WorkerController, RelmWidgetExt,
};

struct TimerModel;

#[derive(Debug)]
enum TimerOutput {
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

struct AppModel {
    start: usize,
    counter: usize,
    running: bool,
    timer: Option<WorkerController<TimerModel>>,
}

#[derive(Debug)]
enum AppInput {
    Tick,
    StartStop,
    Reset,
}

fn build_timer(sender: &ComponentSender<AppModel>) -> Option<WorkerController<TimerModel>> {
    Some(
        TimerModel::builder()
            .detach_worker(())
            .forward(sender.input_sender(), |timer_output| match timer_output {
                TimerOutput::Tick => AppInput::Tick,
            }),
    )
}

#[relm4::component]
impl Component for AppModel {
    type Init = usize;
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::Window {
            gtk::Box {
                set_spacing: 5,
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_class_active: ("timer-running", model.running),
                adw::HeaderBar {},
                adw::Clamp {
                    gtk::Box {
                        set_spacing: 5,
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 20,
                        append: label = &gtk::Label {
                            #[watch]
                            set_label: &format!("{}", model.counter)
                        },
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_halign: gtk::Align::Center,
                            set_spacing: 10,
                            append: start_stop_btn = &gtk::Button {
                                #[watch]
                                set_label: if model.running { "Stop" } else { "Start" },
                                #[watch]
                                set_sensitive: model.counter != 0,
                                #[watch]
                                set_class_active: ("suggested-action", !model.running),
                                connect_clicked => AppInput::StartStop,
                            },
                            gtk::Button {
                                set_label: "Reset",
                                connect_clicked => AppInput::Reset,
                            },
                        }
                    }
                }
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        relm4::set_global_css(".timer-running { background: #2b8724; }");
        let model = AppModel {
            start: init,
            counter: init,
            running: true,
            timer: build_timer(&sender),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInput::StartStop => {
                if self.running {
                    self.timer = None;
                } else {
                    self.timer = build_timer(&sender);
                }
                self.running = !self.running;
            }
            AppInput::Tick => {
                assert!(self.running);
                self.counter -= 1;
                if self.counter == 0 {
                    sender.input_sender().send(AppInput::StartStop).unwrap();
                }
            },
            AppInput::Reset => {
                self.counter = self.start;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(10usize);
}
