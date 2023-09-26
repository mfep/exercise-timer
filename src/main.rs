mod timer;

use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use relm4::{
    adw,
    gtk::{self},
    Component, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, WorkerController,
};
use timer::{TimerModel, TimerOutput};

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
                adw::HeaderBar {},
                adw::Clamp {
                    set_orientation: gtk::Orientation::Horizontal,
                    gtk::Box {
                        set_class_active: ("timer", true),
                        #[watch]
                        set_class_active: ("timer-running", model.running),
                        #[watch]
                        set_class_active: ("timer-stopped", !model.running),
                        set_spacing: 5,
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::Center,
                        set_margin_all: 20,
                        set_vexpand: true,
                        append: label = &gtk::Label {
                            set_class_active: ("timer-label", true),
                            #[watch]
                            set_label: &format!("{}", model.counter)
                        },
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_halign: gtk::Align::Center,
                            set_spacing: 10,
                            append: start_stop_btn = &gtk::Button {
                                #[watch]
                                set_label: if model.running { "Pause" } else if model.counter == model.start { "Start" } else { "Resume" },
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
        relm4::set_global_css(
            ".timer {
                border-radius: 8px;
                padding: 20px;
            }
            .timer-running {
                background: #2b8724;
            }
            .timer-stopped {
                background: #346d9a;
            }
            .timer-label {
                font-size: 48px;
            }
            ",
        );
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
            }
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
