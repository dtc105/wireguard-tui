use crate::dto::Client;

use std::{io::Result, process::Command};

pub struct Wireguard;

impl Wireguard {
    pub fn get_clients() -> Vec<Client> {
        let raw_clients = Command::new("wg")
            .arg("show")
            .arg("all")
            .arg("allowed-ips")
            .output()
            .expect("`wg` should be executable.")
            .stdout;

        let raw_clients = str::from_utf8(&raw_clients).expect("Could not parse `wg` result.");

        raw_clients
            .lines()
            .map(|line| line.trim())
            .map(|client_string| {
                let mut split_string = client_string.split_whitespace().skip(1);
                let public_key = split_string
                    .next()
                    .expect("No public key found.")
                    .trim()
                    .to_string();
                let address = split_string
                    .next()
                    .expect("No address found.")
                    .trim()
                    .to_string();

                Client {
                    name: String::new(),
                    address,
                    public_key,
                }
            })
            .collect::<Vec<Client>>()
    }

    pub fn add_client(client: Client) -> Result<()> {
        Ok(())
    }
}
