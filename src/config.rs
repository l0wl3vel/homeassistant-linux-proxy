#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub webhook_url: String,
    pub rest_client: reqwest::Client,
    pub dbus_connection: zbus::Connection,
}
impl Config {
    pub(crate) async fn init() -> Config {
        Config {
            webhook_url: std::env::var("HOMEASSISTANT_WEBHOOK").expect("HOMEASSISTANT_WEBHOOK environment variable not set"),
            rest_client: reqwest::Client::default(),
            dbus_connection: zbus::Connection::session().await.expect("Unable to establish DBus connection"),
        }
    }
}