use awakened_cli_trade::data::item::Item;
use awakened_cli_trade::io::clipboard::event::ClipboardEvent;
use awakened_cli_trade::io::clipboard::monitor::ClipboardMonitor;
use awakened_cli_trade::io::clipboard::system_monitor::SystemClipboardMonitor;
use awakened_cli_trade::io::prices::checker::ItemPriceChecker;
use awakened_cli_trade::io::prices::checkers::poeprices::PoePricesItemPriceChecker;
use notify_rust::Notification;
use std::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel();
    SystemClipboardMonitor::new(sender);

    loop {
        let event = match receiver.recv() {
            Ok(e) => e,
            Err(_) => continue,
        };
        match event {
            ClipboardEvent::Copied(contents) => {
                println!("Analyzing new clipboard data");
                let clipboard_item = Item::new(contents);
                let price = PoePricesItemPriceChecker::check_item_price(
                    clipboard_item,
                    "Sanctum".to_string(),
                )
                .await;
                match price {
                    Some(p) => {
                        Notification::new()
                            .summary(
                                &format!("{:.1} -> {:.1} {}", p.min, p.max, p.currency).to_string(),
                            )
                            .body(&format!("Confidence: {:.0}%", p.confidence).to_string())
                            .timeout(5000)
                            .show()
                            .unwrap();
                    }
                    None => {}
                }
            }
            ClipboardEvent::Pasted => {}
        }
    }
}
