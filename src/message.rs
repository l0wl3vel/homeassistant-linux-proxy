use log::info;
use serde::Serialize;
use crate::Config;

#[derive(Serialize, Default, Debug)]
pub(crate) struct HomeassistantMessage {
    pub lock_screen_status: Option<bool>,
    pub screen_woken_up: Option<bool>,
}

impl HomeassistantMessage {
    pub(crate) async fn send(&self, config: &Config) -> Result<reqwest::Response, reqwest::Error> {
        info!("Sending: {:?}", self);

        config
            .rest_client
            .post(&config.webhook_url)
            .json(&self)
            .send()
            .await
    }
}