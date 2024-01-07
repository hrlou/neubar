/* use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("net.hrlou.neubar")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("neubar")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
} */
use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/* TODO
 * Some sort of unified audio stream structure which can both play sound but also provide infastructure for other processes
 * i.e. Oscilliscope, Wave Form Visualiser, VU etc.
 */

fn enqueue_file(track: PathBuf) {
   todo!() 
}

fn main() {
    // Using `./profile/tracks` as a temporary testing directory before proper implementation through gtk interface
    let track_paths = fs::read_dir("./profile/tracks").unwrap();
    for track in track_paths {
        /* TODO
         * Check if file is in a compatible format
         * Enqueue and play file
         */
        println!("{:?}", track.unwrap().path().display());
    }

    let file = todo!();
    // let file = Box::new(File::open(Path::new(&args[1])).unwrap());

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate. In this
    // example we'll leave it empty.
    let hint = Hint::new();

    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    // Probe the media source stream for a format.
    let probed =
        symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts).unwrap();

    // Get the format reader yielded by the probe operation.
    let mut format = probed.format;

    // Get the default track.
    let track = format.default_track().unwrap();

    // Create a decoder for the track.
    let mut decoder =
        symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts).unwrap();

    // Store the track identifier, we'll use it to filter packets.
    let track_id = track.id;

    let mut sample_count = 0;
    let mut sample_buf = None;

    loop {
        // Get the next packet from the format reader.
        let packet = format.next_packet().unwrap();

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    sample_count += buf.samples().len();
                    print!("\rDecoded {} samples", sample_count);
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }
}