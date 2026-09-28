use pnet_datalink::{interfaces, MacAddr};
use crate::network::NetworkError;

fn identify_network_mac_address() -> Result<u64, NetworkError> {
    let network_interface = interfaces()
        .iter()
        .find(|interface| interface.is_up() && !interface.is_loopback() && !interface.is_point_to_point());
    match network_interface {
        Some(interface) => {
            return Ok(u64::from_le_bytes(interface.mac.unwrap().octets().a))
        }
        None => {

        }
    }
}