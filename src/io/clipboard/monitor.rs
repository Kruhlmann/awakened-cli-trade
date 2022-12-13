use crate::io::clipboard::event::ClipboardEvent;
use std::sync::mpsc;

pub trait ClipboardMonitor {
    fn new(tx: mpsc::Sender<ClipboardEvent>) -> Self;
}
