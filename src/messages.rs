use serde::{Serialize};

use tokio::sync::oneshot;

type AuthCredResponder = oneshot::Sender<Result<serde_json::Value, String>>;

#[derive(Debug)]
pub struct AuthCredMessage {
    pub user: String,
    pub pass: String,
    pub responder: AuthCredResponder
}

#[derive(Debug, Clone, Serialize)]
pub enum ESP32CommandType {
    ActuatorCommand,
    ValveCommand,
}
#[derive(Debug, Clone, Serialize)]
pub struct ESP32CommandMessage {
    pub command_type: ESP32CommandType,
    pub mac_address: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct BleBeaconMessage {
    pub mac_address: String,
    pub payload: String,
    pub rssi: i64,
}

impl BleBeaconMessage {
    pub fn from(socket_string: &str) -> Self {
        let split = socket_string.split(" ");
        let mut count = 0;
        let mut mac_address = String::from("");
        let mut payload = String::from("");
        let mut rssi: i64 = 0;

        for part in split {
            if count == 0 {
                mac_address = part.to_string();
            }
            if count == 1 {
                payload = part.to_string();
            }

            if count == 2 {
                rssi = part.parse::<i64>().unwrap();
            }

            count = count + 1;
        }
        BleBeaconMessage {
            mac_address,
            payload,
            rssi,
        }
    }
}
