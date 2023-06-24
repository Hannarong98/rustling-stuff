struct Device {
    name: String,
    device_type: String,
    accepted_protocols: String,
    uptime_ms: u64,
}

struct NetworkInfomation(String, String);

// struct Unit;

fn main() {
    let server_1 = Device {
        name: String::from("E1"),
        device_type: String::from("Server"),
        accepted_protocols: String::from("TCP/UDP/SCTP"),
        uptime_ms: 1687523731,
    };

    let network_1 = NetworkInfomation(String::from("127.0.0.1"), String::from("80:E7:C9:CC:73:E3"));

    println!(
        "{}, is a/an {} and have the following network properties:\nIP: {}\nMAC: {}\nUptime milliseconds: {}\nProtocols: {}",
        server_1.name,
        server_1.device_type,
        network_1.0, network_1.1,
        server_1.uptime_ms,
        server_1.accepted_protocols
    );
}
