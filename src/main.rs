mod constants;
mod telegram;

use constants::*;
use telegram::*;

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::error::Error;
use tokio::time::Duration;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut mqttoptions = MqttOptions::new("rust-mqtt-reader", "192.168.100.2", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    if let Err(e) = client.subscribe(TOPIC_BELL, QoS::AtMostOnce).await {
        eprintln!("Error al suscribirse: {:?}", e);
    }
    loop {
        while let Ok(event) = eventloop.poll().await {
            match event {
                Event::Incoming(Packet::Publish(p)) => {
                    if p.topic == TOPIC_BELL {
                        info!("topic/bell event");
                        send_telegram_message("TIMBREEEEE").await?;
                    }
                }
                Event::Outgoing(_) => {}
                _ => {
                    break;
                }
            }
        }
    }
}
