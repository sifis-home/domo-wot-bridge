use std::error::Error;

pub fn mangle_domo_power_energy_sensor(
    source_topic: &mut serde_json::Value,
    channel_number_str: &str,
    actuator_topic: &serde_json::Value,
) -> Result<(), Box<dyn Error>> {
    let updated_props = actuator_topic["updated_properties"].as_array().unwrap();

    let mut found = false;
    for prop in updated_props {
        let prop = prop.as_str().unwrap();
        if prop == "power_data" {
            found = true;
        }
    }

    if !found {
        return Err("not update".into());
    }

    source_topic["value"]["power"] = actuator_topic["power_data"]
        ["channel".to_owned() + channel_number_str]["active_power"]
        .clone();

    let old_ene = source_topic["value"]["energy"].as_f64();

    let mut old_value: f64 = 0.0;
    if let Some(old_ene) = old_ene {
        old_value = old_ene;
    }

    let current_ene = actuator_topic["power_data"]["channel".to_owned() + channel_number_str]
        ["energy"]
        .as_f64()
        .unwrap();

    let total_ene = old_value + current_ene;

    source_topic["value"]["energy"] = serde_json::Value::from(total_ene);

    let props = vec![
        serde_json::Value::String("power".to_owned()),
        serde_json::Value::String("energy".to_owned()),
    ];

    source_topic["value"]["updated_properties"] = serde_json::Value::Array(props);

    Ok(())
}

pub fn mangle_domo_light_dimmable_shelly_dimmer(
    source_topic: &mut serde_json::Value,
    actuator_topic: &serde_json::Value,
) {
    source_topic["value"]["status"] = actuator_topic["dimmer_status"].clone();
    source_topic["value"]["power"] = actuator_topic["power1"].clone();

    let old_ene = source_topic["value"]["energy"].as_f64();

    let mut old_value: f64 = 0.0;
    if let Some(old_ene) = old_ene {
        old_value = old_ene;
    }

    let current_ene = actuator_topic["energy1"].as_f64().unwrap();

    let total_ene = old_value + current_ene;

    source_topic["value"]["energy"] = serde_json::Value::from(total_ene);

    let updated_props = actuator_topic["updated_properties"].as_array().unwrap();

    let mut props = Vec::new();

    for prop in updated_props {
        if prop == "power1" {
            props.push(serde_json::Value::String("power".to_owned()));
        }
        if prop == "energy1" {
            props.push(serde_json::Value::String("energy".to_owned()));
        }
    }

    source_topic["value"]["updated_properties"] = serde_json::Value::Array(props);
}

pub fn mangle_domo_light_dimmable_shelly_rgbw(
    source_topic: &mut serde_json::Value,
    channel_number: u64,
    actuator_topic: &serde_json::Value,
) {
    if channel_number == 1 {
        source_topic["value"]["status"] = actuator_topic["rgbw_status"]["r"].clone();
    }
    if channel_number == 2 {
        source_topic["value"]["status"] = actuator_topic["rgbw_status"]["g"].clone();
    }

    if channel_number == 3 {
        source_topic["value"]["status"] = actuator_topic["rgbw_status"]["b"].clone();
    }

    if channel_number == 4 {
        source_topic["value"]["status"] = actuator_topic["rgbw_status"]["w"].clone();
    }
}

pub fn mangle_domo_rgbw_light(
    source_topic: &mut serde_json::Value,
    actuator_topic: &serde_json::Value,
) {
    source_topic["value"]["r"] = actuator_topic["rgbw_status"]["r"].clone();
    source_topic["value"]["g"] = actuator_topic["rgbw_status"]["g"].clone();
    source_topic["value"]["b"] = actuator_topic["rgbw_status"]["b"].clone();
    source_topic["value"]["w"] = actuator_topic["rgbw_status"]["w"].clone();
}

pub fn mangle_domo_light_siren_switch(
    source_topic: &mut serde_json::Value,
    channel_number_str: &str,
    actuator_topic: &serde_json::Value,
    target_topic_name: &str,
) {
    source_topic["value"]["status"] =
        actuator_topic["output".to_owned() + channel_number_str].clone();

    if target_topic_name != "shelly_1" && target_topic_name != "shelly_1plus" {
        source_topic["value"]["power"] =
            actuator_topic["power".to_owned() + channel_number_str].clone();

        let old_ene = source_topic["value"]["energy"].as_f64();

        let mut old_value: f64 = 0.0;
        if let Some(old_ene) = old_ene {
            old_value = old_ene;
        }

        let current_ene = actuator_topic["energy".to_owned() + channel_number_str]
            .as_f64()
            .unwrap();

        let total_ene = old_value + current_ene;

        source_topic["value"]["energy"] = serde_json::Value::from(total_ene);
    }

    let updated_props = actuator_topic["updated_properties"].as_array().unwrap();

    //println!("UPDATED PROPS {:?}", updated_props);

    let mut props = Vec::new();

    for prop in updated_props {
        let prop_str = prop.as_str().unwrap();

        //println!("prop_str {}", prop_str);

        if prop_str == ("power".to_owned() + channel_number_str) {
            //println!("pushing power {}", channel_number_str);
            props.push(serde_json::Value::String("power".to_owned()));
        }

        if prop_str == ("energy".to_owned() + channel_number_str) {
            //println!("pushing power {}", channel_number_str);
            props.push(serde_json::Value::String("energy".to_owned()));
        }
    }

    source_topic["value"]["updated_properties"] = serde_json::Value::Array(props);
}

pub fn mangle_domo_pir_sensor_radar_sensor_button_bistable_button(
    source_topic: &mut serde_json::Value,
    channel_number_str: &str,
    actuator_topic: &serde_json::Value,
) -> Result<(), &'static str> {
    let updated_props = actuator_topic["updated_properties"].as_array().unwrap();

    let mut found = false;
    for prop in updated_props {
        let prop = prop.as_str().unwrap();
        if prop == ("input".to_owned() + channel_number_str) {
            source_topic["value"]["status"] =
                actuator_topic["input".to_owned() + channel_number_str].clone();
            found = true;
        }
    }

    if found {
        Ok(())
    } else {
        Err("not update")
    }
}

pub fn mangle_domo_floor_valve(
    source_topic: &mut serde_json::Value,
    channel_number_str: &str,
    actuator_topic: &serde_json::Value,
) {
    source_topic["value"]["status"] =
        actuator_topic["output".to_owned() + channel_number_str].clone();
}

pub fn mangle_domo_roller_shutter_garage_gate(
    source_topic: &mut serde_json::Value,
    actuator_topic: &serde_json::Value,
) {
    source_topic["value"]["shutter_status"] = actuator_topic["shutter_status"].clone();
}

pub fn somo_window_sensor_door_sensor(
    source_topic: &mut serde_json::Value,
    channel_number_str: &str,
    actuator_topic: &serde_json::Value,
    target_topic_name: &str,
) {
    if target_topic_name == "domo_ble_contact" {
        source_topic["value"]["status"] = actuator_topic["status"].clone();
    } else {
        source_topic["value"]["status"] =
            actuator_topic["input".to_owned() + channel_number_str].clone();
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn light_siren_switch_with_energy() {
        let actuator_topic = json!({
            "output7": "my_status",
            "power7": "my_power",
            "energy7": 42.5,
            "updated_properties": ["prop1", "prop2", "power7", "energy7"],
        });

        let mut source_topic = json!({
            "value": {
                "status": "old_status",
                "energy": 7.5,
            }
        });

        mangle_domo_light_siren_switch(&mut source_topic, "7", &actuator_topic, "dummy");

        assert_eq!(
            source_topic,
            json!({
                "value": {
                    "status": "my_status",
                    "power": "my_power",
                    "energy": 50.,
                    "updated_properties": [
                        "power", "energy",
                    ]
                }
            })
        );
    }

    #[test]
    fn light_siren_switch_without_energy() {
        let actuator_topic = json!({
            "output7": "my_status",
            "power7": "my_power",
            "energy7": 42.5,
            "updated_properties": ["prop1", "prop2", "power7", "energy7"],
        });

        let mut source_topic = json!({
            "value": {
                "status": "old_status",
            }
        });

        mangle_domo_light_siren_switch(&mut source_topic, "7", &actuator_topic, "dummy");

        assert_eq!(
            source_topic,
            json!({
                "value": {
                    "status": "my_status",
                    "power": "my_power",
                    "energy": 42.5,
                    "updated_properties": [
                        "power", "energy",
                    ]
                }
            })
        );
    }

    #[test]
    fn light_siren_switch_empty_updated_properties() {
        let actuator_topic = json!({
            "output7": "my_status",
            "power7": "my_power",
            "energy7": 42.5,
            "updated_properties": ["prop1"],
        });

        let mut source_topic = json!({
            "value": {
                "status": "old_status",
            }
        });

        mangle_domo_light_siren_switch(&mut source_topic, "7", &actuator_topic, "dummy");

        assert_eq!(
            source_topic,
            json!({
                "value": {
                    "status": "my_status",
                    "power": "my_power",
                    "energy": 42.5,
                    "updated_properties": [],
                }
            })
        );
    }

    #[test]
    fn light_siren_switch_with_shelly_1() {
        let actuator_topic = json!({
            "output7": "my_status",
            "power7": "my_power",
            "energy7": 42.5,
            "updated_properties": ["prop1", "prop2", "power7", "energy7"],
        });

        let mut source_topic = json!({
            "value": {
                "status": "old_status",
                "energy": 7.5,
            }
        });

        mangle_domo_light_siren_switch(&mut source_topic, "7", &actuator_topic, "shelly_1");

        assert_eq!(
            source_topic,
            json!({
                "value": {
                    "status": "my_status",
                    "energy": 7.5,
                    "updated_properties": [
                        "power", "energy",
                    ]
                }
            })
        );
    }
}
