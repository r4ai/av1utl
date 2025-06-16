use anyhow::Context;
use gstreamer as gst;
use gstreamer_editing_services::{self as ges, prelude::*};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use tauri::Manager;

pub enum EditorCommand {
    AddClip {
        file_path: String,
        layer_priority: u32,
        start_ns: u64,
        duration_ns: u64,
    },
}

pub struct AppState {
    pub editor_tx: Arc<Mutex<mpsc::Sender<EditorCommand>>>,
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

fn handle_editor_command(command: EditorCommand, timeline: &ges::Timeline) -> anyhow::Result<()> {
    match command {
        EditorCommand::AddClip {
            file_path,
            layer_priority,
            start_ns,
            duration_ns,
        } => {
            // レイヤーを取得または作成
            let layer = timeline
                .layer(layer_priority)
                .context("Failed to get or create layer")?;

            // URIからアセットを作成
            let uri = format!("file://{}", file_path);
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

                while let Ok(command) = rx.recv() {
                    handle_editor_command(command, &timeline)
                        .unwrap_or_else(|e| eprintln!("Error handling command: {}", e));
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, add_clip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
