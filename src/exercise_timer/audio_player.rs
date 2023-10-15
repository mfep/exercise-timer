use relm4::gtk::gio;
use relm4::{self, prelude::*, Worker};
use rodio::{self, Decoder, Source};

pub struct AudioPlayerModel {
    output_stream: rodio::OutputStreamHandle,
    ping_bytes: gtk::glib::Bytes,
}

impl AudioPlayerModel {
    fn play_ping(&self, times: u32) {
        let cursor = std::io::Cursor::new(self.ping_bytes.clone());
        let decoder = Decoder::new_wav(cursor).expect("Could not decode WAV");
        let new_duration = decoder.total_duration().unwrap() * times;
        let d = decoder.repeat_infinite().take_duration(new_duration);
        self.output_stream
            .play_raw(d.convert_samples())
            .expect("Could not play audio");
    }
}

#[derive(Debug)]
pub enum AudioPlayerInput {
    Ping,
    NextWarmup,
    NextExercise,
    NextRest,
    Finished,
}

impl Worker for AudioPlayerModel {
    type Init = rodio::OutputStreamHandle;
    type Input = AudioPlayerInput;
    type Output = ();

    fn init(output_stream: Self::Init, _sender: ComponentSender<Self>) -> Self {
        let ping_bytes = gio::resources_lookup_data(
            "/xyz/safeworlds/hiit/audio/ping.wav",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Could not open data");
        Self {
            output_stream,
            ping_bytes,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AudioPlayerInput::Ping => {
                self.play_ping(1);
            }
            AudioPlayerInput::NextWarmup => {
                self.play_ping(2);
            }
            AudioPlayerInput::NextExercise => {
                self.play_ping(2);
            }
            AudioPlayerInput::NextRest => {
                self.play_ping(2);
            }
            AudioPlayerInput::Finished => {
                self.play_ping(3);
            }
        }
    }
}
