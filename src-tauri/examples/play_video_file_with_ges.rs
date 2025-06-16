use std::path::Path;

use gst::prelude::*;
use gstreamer as gst;

use ges::prelude::*;
use gstreamer_editing_services as ges;

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .expect("Please provide a file path as the first argument.");
    let file_path = Path::new(&file_path);
    if !file_path.exists() {
        eprintln!("File does not exist: {}", file_path.display());
        return;
    }

    ges::init().unwrap();
    println!("GStreamer Editing Services initialized.");

    let uri =
        gst::glib::filename_to_uri(file_path, None).expect("Failed to convert file path to URI");

    let timeline = ges::Timeline::new();

    let layer = timeline.append_layer();
    ges::UriClip::new(&uri)
        .and_then(|clip| layer.add_clip(&clip))
        .expect("Failed to add clip to layer");

    timeline.commit();
    println!("Clip added to timeline.");

    let pipeline = ges::Pipeline::new();
    pipeline
        .set_timeline(&timeline)
        .expect("Failed to set timeline");
    pipeline
        .set_state(gst::State::Playing)
        .expect("Failed to set pipeline to Playing state");
    println!("Pipeline set to Playing state.");

    let bus = pipeline.bus().expect("Failed to get bus from pipeline");
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                break;
            }
            gst::MessageView::Eos(..) => {
                println!("End of stream reached.");
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Failed to set pipeline to Null state");
    println!("Playback finished.");
}
