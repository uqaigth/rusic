// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod freq;

use rodio::{OutputStream, Sink};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();
    //
    // let mut notes: Vec<Note> = Vec::new();
    // notes.push(Note::new(freq("C", 5), 0.4));
    // notes.push(Note::new(freq("Ab", 4), 0.2));
    // notes.push(Note::new(freq("Bb", 4), 0.8));
    // notes.push(Note::new(freq("Ab", 4), 0.1));
    // notes.push(Note::new(freq("Bb", 4), 0.3));
    // notes.push(Note::new(freq("Bb", 4), 0.1));
    // notes.push(Note::new(freq("Bb", 4), 0.3));
    // notes.push(Note::new(freq("Bb", 4), 0.1));
    // notes.push(Note::new(freq("Bb", 4), 0.2));
    // notes.push(Note::new(freq("Ab", 4), 0.6));
    //
    // for note in notes {
    //     sink.append(note.create());
    // }
    //
    // sink.sleep_until_end();
}
