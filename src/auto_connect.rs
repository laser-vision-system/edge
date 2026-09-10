use pnet_datalink::interfaces;
use reqwest::Client;
use crate::error::EdgeError;


pub async fn identify_network() -> Result<Vec<String>, String> {
    let all_interfaces = interfaces();
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    match default_interface {

        Some(interface) => {
            let mut network_addresses = Vec::new();
            for networks in interface.ips.iter() {
                if networks.is_ipv4() == true {
                    for raw_address in networks.iter() {
                        if raw_address.is_ipv4() == true {
                            let mut url = "http://".to_string();
                            url.push_str(raw_address.to_string().as_str());
                            url.push_str(":8080/is_api");
                            network_addresses.push(url);
                        }
                    }
                }
            }
            Ok(network_addresses)
        },
        None => {
            println!("Error while finding the default interface.");
            Err("Error while finding the default interface.".to_string())
        },

    }
}

pub async fn websocket_connect_automatic() -> Result<String,Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let urls: Vec<String> = identify_network().await?;

    let mut pairs = Vec::new();

    for url in urls {
        let a = tokio::spawn(get_response(url, client.clone()));
        pairs.push(a);
    }

    async fn get_response(url: String, client: Client) -> Result<(String, String), Box<reqwest::Error>> {
        let result = client.get(&url[..]).send().await;
        match result {
            Ok(a) => Ok((url, a.text().await?)),
            Err(e) => Err(Box::new(e))
        }
    }

    for pair in pairs {
        let result = pair.await;
        match result {
            Ok(Ok(a)) =>{
                if a.1 == "This is an API for the laser intensity analysis project" {
                    return Ok(a.0.clone());
                }
            }

            _ => {

            }
        }
    }
    Err(Box::new(EdgeError::AutoConnect))
}