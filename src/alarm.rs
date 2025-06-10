use tokio::time::{Duration, sleep};

use rumqttc::AsyncClient;
use std::error::Error;
use tracing::info;

use crate::constants::{TOPIC_ALARM_EXTERIOR, TOPIC_ALARM_INTERIOR};

pub struct Alarm<'a> {
    armed: bool,
    client: &'a AsyncClient,
}

impl<'a> Alarm<'a> {
    pub fn new(client: &'a AsyncClient) -> Self {
        Self {
            armed: false,
            client,
        }
    }

    pub async fn arm(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.is_armed() {
            self.client
                .publish(TOPIC_ALARM_INTERIOR, rumqttc::QoS::AtLeastOnce, false, "ON")
                .await?;
            sleep(Duration::from_millis(100)).await;
            self.client
                .publish(
                    TOPIC_ALARM_INTERIOR,
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    "OFF",
                )
                .await?;
        }
        info!("alarm armed");
        self.armed = true;
        Ok(())
    }

    pub async fn disarm(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_armed() {
            self.client
                .publish(TOPIC_ALARM_INTERIOR, rumqttc::QoS::AtLeastOnce, false, "ON")
                .await?;
            sleep(Duration::from_millis(100)).await;
            self.client
                .publish(
                    TOPIC_ALARM_INTERIOR,
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    "OFF",
                )
                .await?;
        }
        info!("alarm disarmed");
        self.armed = false;
        Ok(())
    }

    pub fn is_armed(&self) -> bool {
        self.armed
    }

    pub async fn activate(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_armed() {
            self.client
                .publish(
                    TOPIC_ALARM_EXTERIOR,
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    "OFF",
                )
                .await?;
        }
        Ok(())
    }

    pub async fn desactivate(&mut self) -> Result<(), Box<dyn Error>> {
        self.disarm().await?;

        self.client
            .publish(
                TOPIC_ALARM_EXTERIOR,
                rumqttc::QoS::AtLeastOnce,
                false,
                "OFF",
            )
            .await?;

        Ok(())
    }
}
