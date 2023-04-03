use sifis_dht::domocache::DomoEvent;
use std::error::Error;

use crate::command_parser;

pub enum DHTCommand {
    ActuatorCommand(serde_json::Value),
    ValveCommand(serde_json::Value),
}

pub struct DHTManager {
    pub cache: sifis_dht::domocache::DomoCache,
}

impl DHTManager {
    pub async fn new(cache_config: sifis_config::Cache) -> Result<DHTManager, Box<dyn Error>> {
        let sifis_cache = sifis_dht::domocache::DomoCache::new(cache_config).await?;

        Ok(DHTManager { cache: sifis_cache })
    }

    pub async fn get_auth_cred(
        &mut self,
        user: &str,
        password: &str,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let shelly_plus_topic_names = vec!["shelly_1plus", "shelly_1pm_plus", "shelly_2pm_plus"];

        for topic in shelly_plus_topic_names {
            let shelly_plus_topics = self.cache.get_topic_name(topic)?;

            let topics = shelly_plus_topics.as_array().unwrap();
            for t in topics.iter() {
                if let Some(value) = t.get("value") {
                    if let Some(user_login) = value.get("user_login") {
                        if let Some(user_password) = value.get("user_password") {
                            let user_login_str = user_login.as_str().unwrap();
                            let user_password_str = user_password.as_str().unwrap();
                            if user_login_str == user && user_password_str == password {
                                let mac = value.get("mac_address").unwrap().as_str().unwrap();
                                if let Some(topic_name) = t.get("topic_name") {
                                    let topic_name = topic_name.as_str().unwrap().to_owned();
                                    let json_ret = serde_json::json!({ "mac_address": mac, "topic": topic_name });
                                    return Ok(json_ret);
                                }
                            }
                        }
                    }
                }
            }
        }

        Err("cred not found".into())
    }

    pub fn get_topic(
        &mut self,
        topic_name: &str,
        mac_address: &str,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        if let Ok(actuators) = self.cache.get_topic_name(topic_name) {
            for act in actuators.as_array().unwrap() {
                if let Some(value) = act.get("value") {
                    if let Some(mac) = value.get("mac_address") {
                        if mac == mac_address {
                            return Ok(act.to_owned());
                        }
                    }
                }
            }
        }

        Err("act not found".into())
    }

    pub async fn get_actuator_from_mac_address(
        &mut self,
        mac_address_req: &str,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let actuator_topics = [
            "shelly_1",
            "shelly_1pm",
            "shelly_1plus",
            "shelly_em",
            "shelly_1pm_plus",
            "shelly_2pm_plus",
            "shelly_25",
            "shelly_dimmer",
            "shelly_rgbw",
            "domo_ble_thermometer",
            "domo_ble_valve",
            "domo_ble_contact",
        ];

        for act_type in actuator_topics {
            if let Ok(actuators) = self.cache.get_topic_name(act_type) {
                for act in actuators.as_array().unwrap() {
                    if let Some(value) = act.get("value") {
                        if let Some(mac) = value.get("mac_address") {
                            if mac == mac_address_req {
                                return Ok(act.to_owned());
                            }
                        }
                    }
                }
            }
        }

        Err("err".into())
    }

    pub async fn write_topic(
        &mut self,
        topic_name: &str,
        topic_uuid: &str,
        value: &serde_json::Value,
    ) {
        self.cache
            .write_value(topic_name, topic_uuid, value.to_owned())
            .await;
    }

    async fn handle_volatile_command(
        &self,
        command: serde_json::Value,
    ) -> Result<DHTCommand, Box<dyn Error>> {
        if let Some(command) = command.get("command") {
            if let Some(command_type) = command.get("command_type") {
                if command_type == "shelly_actuator_command" {
                    if let Some(value) = command.get("value") {
                        return Ok(DHTCommand::ActuatorCommand(value.to_owned()));
                    }
                }

                if command_type == "radiator_valve_command" {
                    if let Some(value) = command.get("value") {
                        return Ok(DHTCommand::ValveCommand(value.to_owned()));
                    }
                }

                if command_type == "turn_command" {
                    return command_parser::handle_turn_command(self, command).await;
                }

                if command_type == "valve_command" {
                    return command_parser::handle_valve_command(self, command).await;
                }

                if command_type == "dim_command" {
                    return command_parser::handle_dim_command(self, command).await;
                }

                if command_type == "rgbw_command" {
                    return command_parser::handle_rgbw_command(self, command).await;
                }

                if command_type == "shutter_command" {
                    return command_parser::handle_shutter_command(self, command).await;
                }
            }
        }

        Err("not able to parse message".into())
    }

    pub async fn wait_dht_messages(&mut self) -> Result<DHTCommand, Box<dyn Error>> {
        let data = self.cache.cache_event_loop().await?;

        if let DomoEvent::VolatileData(m) = data {
            //println!("RECEIVED COMMAND{}", m);
            return self.handle_volatile_command(m.to_owned()).await;
        }

        Err("not a volatile message".into())
    }
}
