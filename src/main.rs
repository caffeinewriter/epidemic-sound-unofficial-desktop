#![windows_subsystem = "windows"]

extern crate web_view;

use std::fs::File;
use std::io::prelude::*;
use std::{thread, time::Duration};
use web_view::*;

fn write_track(track_data: &str) -> std::io::Result<()> {
    let mut f = File::create("epidemic_current_song.txt")?;
    f.write_all(track_data.as_bytes())?;

    f.sync_all()?;
    Ok(())
}

fn check_song(webview: &mut WebView<()>) -> WVResult {
    webview.set_color((51, 51, 51));
    webview.eval(&"
    function checkSongInfo() {
        try {
            const titleElement = document.querySelector('[class*=TrackInfo__title]');
            if (!titleElement) return;
            const trackTitle = titleElement ? titleElement.textContent.trim() : 'Title Not Found';
            const artistElement = document.querySelector('[class*=CreativesLabel__container]');
            const trackArtist = artistElement ? artistElement.textContent.trim() : 'Artist Not Found';
            external.invoke(`${trackArtist} - ${trackTitle}`);
        } catch (e) {
            /**/
        }
    }
    checkSongInfo();
    ")
}

fn main() {
    let webview = web_view::builder()
        .title("Epidemic Sound")
        .content(Content::Url("https://epidemicsound.com"))
        .size(960, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            write_track(arg);
            webview.set_title(&format!("Epidemic Sound | {}", arg).to_string());
            Ok(())
        })
        .build()
        .unwrap();

    let handle = webview.handle();

    thread::spawn(move || loop {
        {
            handle.dispatch(move |webview| check_song(webview));
        }
        thread::sleep(Duration::from_secs(1));
    });

    webview.run().unwrap();
}
