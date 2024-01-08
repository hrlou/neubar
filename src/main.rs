use polodb_core as db;
use db::Database;
use serde::{Serialize, Deserialize};
use std::{fs, path::PathBuf};
use symphonia::core::{
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
    errors::Error,
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};

/* TODO:
 *
 * Some sort of unified audio stream structure which can both play sound but also provide infastructure for other processes
 * i.e. Oscilliscope, Wave Form Visualiser, VU etc.
 * 
 * Tag formating lanaguage
 * ```
 * $if2(%albumartist% %artist%) - %album%/%track% - %title%
 * ```
 * 
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub artist: String,
    pub album: String,
    pub track: i32,
    pub title: String,
}

/* PROTOTYPES:
 *
 * fn Track::format(FormatStr) -> String
 * 
 * fn Player::get_track(PathBuf) -> Track
 * fn Player::enqueue_track(Track) -> Result
 * fn Player::playpause() -> bool
 * fn Player::seek(Timestamp) -> Result
 * fn Player::library() -> Library
 * 
 * fn Library::add(Track) -> bool
 * 
 */

fn read_track(path: PathBuf) {
    let src = std::fs::File::open(path).expect("failed to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3");
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");
    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");
    let dec_opts: DecoderOptions = Default::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");
    let track_id = track.id;
}

fn main() {
    // let db = Database::open_file("./profile/neubar.db").unwrap();

    // Using `./profile/tracks` as a temporary testing directory before proper implementation through gtk interface
    let track_paths = fs::read_dir("./profile/tracks").unwrap();
    for track in track_paths {
        /* TODO:
         *
         * Check if file is in a compatible format
         * Enqueue and play file
         */
        println!("{:?}", track.unwrap().path().display());
    }
}
