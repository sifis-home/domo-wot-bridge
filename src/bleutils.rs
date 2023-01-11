use aead::{generic_array::GenericArray, Aead, KeyInit, Payload};
use ccm::{
    consts::{U11, U12, U4},
    Ccm,
};
use hex_literal::hex;
use std::error::Error;

#[derive(Debug)]
pub struct AtcResult {
    pub temperature: f32,
    pub humidity: f32,
    pub battery: f32,
}

#[derive(PartialEq, Debug)]
pub enum ContactStatus {
    Open,
    Close,
}

pub struct ContactResult {
    pub state: ContactStatus,
}

pub fn decrypt_atc(
    payload: &Vec<u8>,
    key: &Vec<u8>,
    nonce: &Vec<u8>,
) -> Result<AtcResult, Box<dyn Error>> {
    // 4 bytes di mac len + 11 bytes di nonce
    type Cipher = Ccm<aes::Aes128, U4, U11>;
    let key = GenericArray::from_slice(&key);
    let nonce = GenericArray::from_slice(&nonce);
    let c = Cipher::new(key);

    let aad = hex!("11");

    let res;
    let res_r = c.decrypt(
        nonce,
        Payload {
            aad: &aad,
            msg: &payload,
        },
    );

    match res_r {
        Ok(r) => res = r,
        Err(_e) => return Err("error".into()),
    }

    let res0: f32 = res[0] as f32;
    let res1: f32 = res[1] as f32;
    let res2: u8 = res[2] as u8;

    let temp = res0 / f32::from(2 as u8) - f32::from(40 as u16);
    let humi = res1 / f32::from(2 as u8);
    let batt = res2 & 0x7F;

    let res = AtcResult {
        temperature: temp,
        humidity: humi,
        battery: f32::from(batt),
    };

    Ok(res)
}

pub fn decrypt_contact(
    payload: &Vec<u8>,
    key: &Vec<u8>,
    nonce: &Vec<u8>,
) -> Result<ContactResult, Box<dyn Error>> {
    /*
    println!(
        "payload {}, key {}, nonce {}",
        hex::encode(payload),
        hex::encode(key),
        hex::encode(nonce)
    );
    */

    // 4 bytes di mac len + 12 bytes di nonce
    type Cipher2 = Ccm<aes::Aes128, U4, U12>;
    let key = GenericArray::from_slice(&key);
    let nonce = GenericArray::from_slice(&nonce);
    let c = Cipher2::new(key);

    let aad = hex!("11");

    let res = c.decrypt(
        nonce,
        Payload {
            aad: &aad,
            msg: &payload,
        },
    );

    return match res {
        Ok(r) => {
            //println!("{:?}", r);
            let re = hex::encode(r);

            let chars: Vec<_> = re.chars().collect();

            if chars[chars.len() - 1] == 48 as char {
                Ok(ContactResult {
                    state: ContactStatus::Open,
                })
            } else {
                Ok(ContactResult {
                    state: ContactStatus::Close,
                })
            }
        }
        Err(_e) => {
            //println!("{:?}", e);
            Err("Bad request".into())
        }
    };
}

fn decrypt_aes_ccm(
    token: &str,
    mac_str_inverted: &str,
    decrypt_data: &str,
) -> Result<AtcResult, Box<dyn Error>> {
    let token_decoded = hex::decode(token).expect("Decoding failed");
    let mac_decoded = hex::decode(mac_str_inverted).expect("Decoding failed");
    let data_decoded = hex::decode(decrypt_data).expect("Decoding failed");

    let adslength: u8 = data_decoded.len() as u8;
    if adslength > 8
        && data_decoded[0] <= adslength
        && data_decoded[0] > 7
        && data_decoded[1] == 0x16
        && data_decoded[2] == 0x1a
        && data_decoded[3] == 0x18
    {
        let len: usize = data_decoded[0] as usize + 1;
        let pkt = &data_decoded[0..len];

        let mut nonce = mac_decoded.clone();

        for i in 0..5 {
            nonce.push(pkt[i]);
        }

        let payload = &pkt[5..pkt.len()];
        /*
        println!(
            "payload {:?}, key {:?}, nonce {:?}, mac {:?} ",
            payload, token_decoded, nonce, mac_decoded
        );
         */

        return decrypt_atc(&payload.to_vec(), &token_decoded, &nonce);
    }

    Err("Parsing Error".into())
}

pub fn parse_atc(mac: &str, data: &str, token: &str) -> Result<AtcResult, Box<dyn Error>> {
    let preamble = "161a18";
    let packet_start = data.find(preamble);

    let pkt_start: usize;
    match packet_start {
        Some(start) => {
            pkt_start = start;
        }
        _ => {
            return Err("error".into());
        }
    }

    let offset = pkt_start + preamble.len();
    let stripped_data_str = &data[offset..];
    let mac_str = mac.replace(":", "");

    /*
    println!(
        "mac_str {}, stripped_data_str {}",
        mac_str, stripped_data_str
    );
    */

    let mut mac_str_inverted: String = String::from("");

    for i in (0..mac_str.len()).step_by(2) {
        mac_str_inverted.insert_str(0, &mac_str[i..i + 2]);
    }

    let length_hex = &data[offset - 8..offset - 6];

    let decrypt_data = length_hex.to_owned() + "161a18" + stripped_data_str;

    let ret = decrypt_aes_ccm(token, &mac_str_inverted, &decrypt_data)?;

    Ok(ret)
}

pub fn parse_contact_sensor(
    mac: &str,
    data: &str,
    key: &str,
) -> Result<ContactResult, Box<dyn Error>> {
    //println!("data {}", data);

    let xiaomi_preamble = "1695fe";
    let packet_start = data.find(xiaomi_preamble);

    let pkt_start: usize;
    match packet_start {
        Some(start) => {
            //println!("start {}", start);
            pkt_start = start / 2;
        }
        _ => {
            return Err("error".into());
        }
    }

    let data = hex::decode(data).expect("Decoding failed");
    let key = hex::decode(key).expect("Decoding failed");

    let packet_start = pkt_start;

    //println!("packet_start {}", packet_start);

    let mac_str = mac.replace(":", "");

    let mut mac_str_inverted: String = String::from("");

    for i in (0..mac_str.len()).step_by(2) {
        mac_str_inverted.insert_str(0, &mac_str[i..i + 2]);
    }

    if packet_start + 5 > data.len() || packet_start + 7 > data.len() {
        return Err("Packet length not sufficient".into());
    }

    let device_type = &data[(packet_start + 5)..(packet_start + 7)];

    //println!("Device Type {}", hex::encode(device_type));
    let mac_inverted = hex::decode(mac_str_inverted).expect("Decoding failed");

    let mut nonce: Vec<u8> = vec![];
    nonce.extend_from_slice(&mac_inverted);
    nonce.extend_from_slice(device_type);

    let app_nonce = &data[(packet_start + 7)..(packet_start + 8)];

    nonce.extend_from_slice(app_nonce);

    let encrypted_payload = &data[(packet_start + 14)..data.len() - 1];

    //println!("Encrypted payload {}", hex::encode(encrypted_payload));

    let token = &encrypted_payload[(encrypted_payload.len() - 4)..encrypted_payload.len()];

    //println!("token {}", hex::encode(token));

    let payload_counter =
        &encrypted_payload[encrypted_payload.len() - 7..encrypted_payload.len() - 4];

    //println!("Payload counter {}", hex::encode(payload_counter));

    nonce.extend_from_slice(payload_counter);

    //println!("Nonce {}", hex::encode(&nonce));

    let cypher_payload = &encrypted_payload[0..encrypted_payload.len() - 7];

    //println!("Cypher Payload {}", hex::encode(cypher_payload));

    let mut total: Vec<u8> = cypher_payload.to_vec().clone();

    //println!("To decrypt {}", hex::encode(&total));

    total.extend_from_slice(token);

    let ret = decrypt_contact(&total, &key, &nonce)?;

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use crate::bleutils::parse_contact_sensor;

    #[cfg(test)]
    #[test]
    fn test_contact_parse() {
        use super::ContactResult;

        let mac = "e4:aa:ec:53:9e:2b";
        let key = "6b1db353566f01c6d3585100b9d348f4";
        let data = "1d020106191695fe58588b09482b9e53ecaae46db81e190d00007d32b33ccb";

        let ContactResult { state } = parse_contact_sensor(&mac, &data, &key).unwrap();

        assert_eq!(state, crate::bleutils::ContactStatus::Close);
    }
}
