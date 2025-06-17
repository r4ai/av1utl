use anyhow::{bail, Context};
use gstreamer::{self as gst, prelude::*};
use gstreamer_editing_services::{self as ges, prelude::*};
use gstreamer_video as gst_video;
use std::{
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
};
use tauri::{Emitter, Manager};

pub enum EditorCommand {
    AddClip {
        file_path: String,
        layer_priority: u32,
        start_ns: u64,
        duration_ns: u64,
    },
    StartPreview {
        window: tauri::Window,
    },
    StopPreview,
    SeekTo {
        position_ns: u64,
    },
    PlayPause,
}

pub struct AppState {
    pub editor_tx: Arc<Mutex<mpsc::Sender<EditorCommand>>>,
}

#[derive(serde::Serialize, Debug, Clone)]
struct FramePayload {
    width: usize,
    height: usize,
    bytes: Vec<u8>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn add_clip(
    file_path: String,
    layer_priority: u32,
    start_ns: u64,
    duration_ns: u64,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    state
        .editor_tx
        .lock()
        .unwrap()
        .send(EditorCommand::AddClip {
            file_path,
            layer_priority,
            start_ns,
            duration_ns,
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn start_preview(
    window: tauri::Window,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    state
        .editor_tx
        .lock()
        .unwrap()
        .send(EditorCommand::StartPreview { window })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn stop_preview(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state
        .editor_tx
        .lock()
        .unwrap()
        .send(EditorCommand::StopPreview)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn seek_to(position_ns: u64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    state
        .editor_tx
        .lock()
        .unwrap()
        .send(EditorCommand::SeekTo { position_ns })
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn play_pause(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state
        .editor_tx
        .lock()
        .unwrap()
        .send(EditorCommand::PlayPause)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn handle_editor_command(
    command: EditorCommand,
    timeline: &ges::Timeline,
    preview_pipeline: &mut Option<ges::Pipeline>,
    is_playing: &mut bool,
) -> anyhow::Result<()> {
    match command {
        EditorCommand::AddClip {
            file_path,
            layer_priority,
            start_ns,
            duration_ns,
        } => {
            let file_path = Path::new(&file_path);
            if !file_path.exists() {
                bail!("File does not exist: {}", file_path.display());
            }

            // レイヤーを取得または作成
            let layer = timeline
                .layer(layer_priority)
                .context("Failed to get or create layer")?;

            // URIからアセットを作成
            let uri = gst::glib::filename_to_uri(&file_path, None)
                .context("Failed to convert file path to URI")?;
            let asset =
                ges::UriClipAsset::request_sync(&uri).context("Failed to create asset from URI")?;

            // レイヤーにアセットを追加してクリップを生成
            layer
                .add_asset(
                    &asset,
                    gst::ClockTime::from_nseconds(start_ns), // タイムライン上での開始位置
                    gst::ClockTime::from_seconds(0), // アセットのどの時点から使うか (in-point)
                    gst::ClockTime::from_nseconds(duration_ns), // タイムライン上での表示時間
                    ges::TrackType::VIDEO | ges::TrackType::AUDIO, // トラックタイプ
                )
                .context("Failed to add asset to layer")?;

            Ok(())
        }
        EditorCommand::StartPreview { window } => {
            // GESパイプラインを作成
            let pipeline = ges::Pipeline::new();
            pipeline.set_timeline(timeline)?;

            // プレビュー用のsinkを作成
            let appsink = gstreamer_app::AppSink::builder()
                .name("preview_sink")
                .caps(
                    &gst::Caps::builder("video/x-raw")
                        .field("format", "RGBA")
                        .field("width", 1280)
                        .field("height", 720)
                        .build(),
                )
                .build(); // プレビューsinkを設定
            pipeline.set_video_sink(Some(&appsink));

            // new-sampleシグナルに対するコールバックを設定 [11, 46]
            appsink.set_callbacks(
                gstreamer_app::AppSinkCallbacks::builder()
                    .new_sample(move |sink| {
                        // サンプルをプル [47, 48]
                        let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                        let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
                        let caps = sample.caps().ok_or(gst::FlowError::Error)?;
                        let info = gst_video::VideoInfo::from_caps(caps).unwrap();

                        // バッファをマップしてバイトスライスを取得 [49]
                        let map = buffer.map_readable().unwrap();
                        let frame_data = map.as_slice();

                        // Tauriイベントを発行 [15]
                        let payload = FramePayload {
                            width: info.width() as usize,
                            height: info.height() as usize,
                            bytes: frame_data.to_vec(),
                        };
                        tauri::Window::emit(&window, "new-frame", payload).unwrap_or_else(|e| {
                            eprintln!("Failed to emit new-frame event: {}", e);
                        });

                        Ok(gst::FlowSuccess::Ok)
                    })
                    .build(),
            );
            pipeline.set_state(gst::State::Playing).unwrap();
            *preview_pipeline = Some(pipeline);

            Ok(())
        }
        EditorCommand::StopPreview => {
            if let Some(pipeline) = preview_pipeline.take() {
                pipeline.set_state(gst::State::Null)?;
            }
            *is_playing = false;
            Ok(())
        }
        EditorCommand::SeekTo { position_ns } => {
            if let Some(pipeline) = preview_pipeline {
                pipeline.seek_simple(
                    gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                    gst::ClockTime::from_nseconds(position_ns),
                )?;
            }
            Ok(())
        }
        EditorCommand::PlayPause => {
            if let Some(pipeline) = preview_pipeline {
                if *is_playing {
                    pipeline.set_state(gst::State::Paused)?;
                    *is_playing = false;
                } else {
                    pipeline.set_state(gst::State::Playing)?;
                    *is_playing = true;
                }
            }
            Ok(())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    gstreamer::init().expect("Failed to initialize GStreamer.");
    ges::init().expect("Failed to initialize GES.");

    tauri::Builder::default()
        .setup(|app| {
            let (tx, rx) = mpsc::channel::<EditorCommand>();

            app.manage(AppState {
                editor_tx: Arc::new(Mutex::new(tx)),
            });
            thread::spawn(move || {
                let timeline = ges::Timeline::new_audio_video();
                timeline.append_layer();

                let mut preview_pipeline: Option<ges::Pipeline> = None;
                let mut is_playing = false;

                while let Ok(command) = rx.recv() {
                    handle_editor_command(
                        command,
                        &timeline,
                        &mut preview_pipeline,
                        &mut is_playing,
                    )
                    .unwrap_or_else(|e| eprintln!("Error handling command: {}", e));
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_clip,
            start_preview,
            stop_preview,
            seek_to,
            play_pause
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
