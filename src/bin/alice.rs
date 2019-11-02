use std::net::Ipv4Addr;
use cqc::hdr::CommHdr;
use qkd_rs::Cqc;

fn bob() -> CommHdr {
    CommHdr {
        remote_app_id: 10,
        remote_port: 8004,
        remote_node: u32::from(Ipv4Addr::new(127, 0, 0, 1)),
    }
}

fn main() {
    let cqc = Cqc::new(10, "localhost", 8001);

    for _ in 0..10 {
        let id = cqc.create_epr(bob(), false);
        let outcome = cqc.measure_qubit(id, false);
        print!("{}", outcome as u8);
    }
    println!();
}
