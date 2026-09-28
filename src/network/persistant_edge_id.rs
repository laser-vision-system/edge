use pnet_datalink::{interfaces, NetworkInterface};
use crate::network::NetworkError;
fn get_network_interface() -> Option<NetworkInterface> {
    interfaces()
        .iter()
        .find(
            |interface|
                interface.is_up() &&
                    !interface.is_loopback() &&
                    !interface.is_point_to_point() &&
                    !interface.mac.is_none())
        .cloned()
}
fn mac_octet_converter(mac_octets: [u8;6]) -> u64 {
    let mut mac_address_vec = Vec::<u8>::new();
    mac_octets.iter().for_each(|octets|mac_address_vec.push(octets.clone()));
    mac_address_vec.push(0u8);
    mac_address_vec.push(0u8);
    let mac_address_array = mac_address_vec.as_array();
    let mac_address =  u64::from_be_bytes(mac_address_array.unwrap().clone());
    mac_address
}
pub fn get_mac_address() -> Result<u64, NetworkError> {
    let network_interface = get_network_interface();
    match network_interface {
        Some(interface) => {
            let mac_octets = interface
                .mac
                .unwrap()
                .octets();
            let mac_address = mac_octet_converter(mac_octets);
            Ok(mac_address)
        }
        None => {
            Err(NetworkError::AutoConnect)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::persistant_edge_id::{get_mac_address, get_network_interface, mac_octet_converter};

    #[test]
    fn test_mac_address() {
        get_mac_address();
    }
    #[test]
    fn test_network_interface() {
        get_network_interface();
    }
    #[test]
    fn test_mac_octet_converter() {
        let mac_octets: [u8;6] = [0;6];
        mac_octet_converter(mac_octets);
    }
}