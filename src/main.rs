#[macro_use]
extern crate serde;
extern crate log;

mod message;
use crate::message::HomeassistantMessage;

mod config;
use config::Config;

use env_logger::Env;
use log::info;

use std::error::Error;

use tokio_stream::StreamExt;
use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.gnome.ScreenSaver",
    default_service = "org.gnome.ScreenSaver",
    default_path = "/org/gnome/ScreenSaver"
)]
trait ScreenSaver {
    async fn GetActive(&self) -> Result<bool, Box<dyn Error>>;
    #[dbus_proxy(signal)]
    async fn ActiveChanged(&self, state: bool) -> Result<()>;
    #[dbus_proxy(signal)]
    async fn WakeUpScreen(&self) -> Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::init().await;

    info!("Starting DBus listeners");

    tokio::join!(
        lock_screen_event_listener(&config),
        wake_event_listener(&config)
    );

    Ok(())
}


async fn lock_screen_event_listener(config: &Config) {
    let proxy = ScreenSaverProxy::new(&config.dbus_connection)
        .await
        .unwrap();

    let mut active_changed = proxy.receive_ActiveChanged().await.unwrap();
    info!("Started lock_screen_event_listener");

    while let Some(event) = active_changed.next().await {
        let state = event.args().unwrap().state;
        println!("{:?}", state);
        let json = HomeassistantMessage {
            lock_screen_status: state.into(),
            ..HomeassistantMessage::default()
        };

        json.send(&config).await.unwrap();
    }
}

async fn wake_event_listener(config: &Config) {
    let proxy = ScreenSaverProxy::new(&config.dbus_connection)
        .await
        .unwrap();

    let mut wakeup_changed = proxy.receive_WakeUpScreen().await.unwrap();
    info!("Started wake_event_listener");
    while let Some(_event) = wakeup_changed.next().await {
        println!("Screen woken up");
        let json = HomeassistantMessage {
            screen_woken_up: true.into(),
            ..HomeassistantMessage::default()
        };

        json.send(&config).await.unwrap();
    }
}
