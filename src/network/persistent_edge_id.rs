use pnet_datalink::{NetworkInterface, interfaces};

use crate::network::NetworkError;
fn get_network_interface() -> Option<NetworkInterface> {
	interfaces().into_iter().find(|interface| {
		!interface.is_loopback()
			&& !interface.is_point_to_point()
			&& interface.mac.is_some()
	})
}
#[expect(
	clippy::big_endian_bytes,
	reason = "This data is intended to be sent to the server"
)]
fn mac_octet_converter(mac_octets: [u8; 6]) -> u64 {
	let bytes: [u8; 8] = [
		mac_octets[0],
		mac_octets[1],
		mac_octets[2],
		mac_octets[3],
		mac_octets[4],
		mac_octets[5],
		0x00,
		0x00,
	];
	u64::from_be_bytes(bytes)
}
pub fn get_mac_address() -> Result<u64, NetworkError> {
	let network_interface = get_network_interface();
	match network_interface {
		Some(interface) => match interface.mac {
			Some(mac) => {
				let mac_octets = mac.octets();
				let mac_address = mac_octet_converter(mac_octets);
				Ok(mac_address)
			},
			_ => Err(NetworkError::MacAddress),
		},
		None => Err(NetworkError::MacAddress),
	}
}

#[cfg(test)]
mod tests {
	use crate::network::persistent_edge_id::{
		get_mac_address, get_network_interface, mac_octet_converter,
	};

	#[test]
	fn mac_address() {
		get_mac_address().expect(
			"expected to find a non loopback/localhost interface with a mac address",
		);
	}
	#[test]
	fn network_interface() {
		get_network_interface()
			.expect("expected to find a non loopback/localhost interface");
	}
	#[test]
	fn convert_mac_address() {
		let mac_octets = [0x12, 0x34, 0x56, 0x78, 0x90, 0x12];
		let expected: u64 = 0x1234_5678_9012_0000;
		let result = mac_octet_converter(mac_octets);
		assert_eq!(result, expected);
	}
}
