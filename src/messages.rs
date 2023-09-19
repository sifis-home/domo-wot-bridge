use serde::Serialize;

use tokio::sync::oneshot;

type AuthCredResponder = oneshot::Sender<Result<serde_json::Value, String>>;

#[derive(Debug)]
pub struct AuthCredMessage {
    pub user: String,
    pub pass: String,
    pub responder: AuthCredResponder,
}

#[derive(Debug, Clone, Serialize)]
pub enum ESP32CommandType {
    Actuator,
    Valve,
    Ping,
}
#[derive(Debug, Clone, Serialize)]
pub struct ESP32CommandMessage {
    pub command_type: ESP32CommandType,
    pub mac_address: String,
    pub payload: serde_json::Value,
    pub actuator_mac_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BleBeaconMessage {
    pub actuator: String,
    pub mac_address: String,
    pub payload: String,
    pub rssi: i64,
}

impl BleBeaconMessage {
    pub fn from(socket_string: &str, actuator: &str) -> Self {
        let split = socket_string.split(' ');
        let mut mac_address = String::new();
        let mut payload = String::new();
        let mut rssi: i64 = 0;

        for (count, part) in split.enumerate() {
            if count == 0 {
                mac_address = part.to_string();
            }
            if count == 1 {
                payload = part.to_string();
            }

            if count == 2 {
                if let Ok(r) = part.parse::<i64>() {
                    rssi = r;
                } else {
                    print!("part is {part}");
                }
            }
        }

        let act_address_with_points = actuator[0..2].to_owned()
            + ":"
            + &actuator[2..4]
            + ":"
            + &actuator[4..6]
            + ":"
            + &actuator[6..8]
            + ":"
            + &actuator[8..10]
            + ":"
            + &actuator[10..12];

        BleBeaconMessage {
            actuator: act_address_with_points,
            mac_address,
            payload,
            rssi,
        }
    }
}
