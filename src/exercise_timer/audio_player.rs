use relm4::{self, gtk::gio, prelude::*};
use rodio::{self, Source};

pub struct AudioPlayerModel {
    output_stream: rodio::OutputStreamHandle,
    ping_bytes: gtk::glib::Bytes,
    volume: f64,
}

impl AudioPlayerModel {
    fn play_ping(&self, times: u32) {
        let cursor = std::io::Cursor::new(self.ping_bytes.clone());
        let decoder = rodio::Decoder::new_wav(cursor).expect("Could not decode WAV");
        let new_duration = decoder.total_duration().unwrap() * times;
        let d = decoder
            .repeat_infinite()
            .take_duration(new_duration)
            .amplify(self.volume as f32);
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
    SetVolume(f64),
}

pub struct AudioPlayerModelInit {
    pub output_stream: rodio::OutputStreamHandle,
    pub volume: f64,
}

impl relm4::Worker for AudioPlayerModel {
    type Init = AudioPlayerModelInit;
    type Input = AudioPlayerInput;
    type Output = ();

    fn init(init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        let ping_bytes = gio::resources_lookup_data(
            "/xyz/safeworlds/hiit/audio/ping.wav",
            gio::ResourceLookupFlags::NONE,
        )
        .expect("Could not open data");
        Self {
            output_stream: init.output_stream,
            volume: init.volume,
            ping_bytes,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AudioPlayerInput::Ping => {
                self.play_ping(1);
            }
            AudioPlayerInput::NextWarmup => {}
            AudioPlayerInput::NextExercise => {
                self.play_ping(2);
            }
            AudioPlayerInput::NextRest => {
                self.play_ping(2);
            }
            AudioPlayerInput::Finished => {
                self.play_ping(3);
            }
            AudioPlayerInput::SetVolume(vol) => {
                self.volume = vol;
            }
        }
    }
}
