use pcap::Capture;

fn main() {
    for device in pcap::Device::list().unwrap() {
        if device.name.contains("vmnet8") {
            let mut cap = Capture::from_device(device)
                .unwrap()
                .promisc(true)
                .snaplen(524288)
                .timeout(1000)
                .open().unwrap();
            let mut packet_count = 0;
            while let Ok(packet) = cap.next() {
                packet_count += 1;
                println!("{:?}", packet);
                println!("{}", packet_count);
            }
        }
    }
}