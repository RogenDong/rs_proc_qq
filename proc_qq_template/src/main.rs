use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use proc_qq::Authentication::QRCode;
use proc_qq::ClientBuilder;

mod imglib;
mod menu;
mod misc;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    ClientBuilder::new()
        .priority_session("session.token")
        .authentication(QRCode)
        .build(vec![menu::module(), imglib::module(), misc::module()])
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
        .unwrap();
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("rs_qq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG),
        )
        .init();
}
