use gst::prelude::*;
use gstreamer as gst;

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .expect("Please provide a file path as the first argument.");

    gst::init().unwrap();

    let uri =
        gst::glib::filename_to_uri(&file_path, None).expect("Failed to convert file path to URI");

    let pipeline = gst::ElementFactory::make("playbin")
        .build()
        .expect("Failed to create playbin element");
    pipeline.set_property("uri", uri.as_str());
    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.bus().expect("Failed to get bus from pipeline");
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!(
                    "Debugging information: {:?}",
                    err.debug().unwrap_or_default()
                );
                break;
            }
            MessageView::Eos(_) => {
                println!("End-Of-Stream reached.");
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();
    println!("Playback finished.");
}
