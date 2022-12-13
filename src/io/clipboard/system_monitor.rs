use crate::io::clipboard::event::ClipboardEvent;
use crate::io::clipboard::monitor::ClipboardMonitor;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

pub struct SystemClipboardMonitor {}

impl ClipboardMonitor for SystemClipboardMonitor {
    fn new(sender: mpsc::Sender<ClipboardEvent>) -> SystemClipboardMonitor {
        thread::spawn(move || {
            if env::var("WAYLAND_DISPLAY").is_ok() {
                println!("Using wayland");
                let mut last_contents = "".to_string();

                loop {
                    thread::sleep(Duration::from_millis(1000));
                    match get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text) {
                        Ok((mut pipe, mime_type)) => {
                            if !mime_type.starts_with("text/plain") {
                                return;
                            }
                            let mut new_content_buffer = vec![];
                            match pipe.read_to_end(&mut new_content_buffer) {
                                Ok(_) => {
                                    let new_contents =
                                        String::from_utf8_lossy(&new_content_buffer).to_string();
                                    if new_contents != last_contents {
                                        match sender
                                            .send(ClipboardEvent::Copied(new_contents.clone()))
                                        {
                                            Ok(_) => last_contents = new_contents,
                                            Err(_) => println!(
                                                "Unable to send clipboard message to main thread"
                                            ),
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    };
                }
            } else {
                let mut context: ClipboardContext = ClipboardProvider::new().unwrap();
                let mut last_contents = "".to_string();

                loop {
                    thread::sleep(Duration::from_millis(1000));
                    let new_contents = context.get_contents().unwrap();
                    if new_contents == last_contents {
                        return;
                    }
                    if new_contents != last_contents {
                        match sender.send(ClipboardEvent::Copied(new_contents.clone())) {
                            Ok(_) => last_contents = new_contents,
                            Err(_) => println!("Unable to send clipboard message to main thread"),
                        }
                    }
                }
            }
        });
        SystemClipboardMonitor {}
    }
}
