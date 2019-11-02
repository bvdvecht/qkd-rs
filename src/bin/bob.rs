use qkd_rs::Cqc;

fn main() {
    let cqc = Cqc::new(10, "localhost", 8004);

    for _ in 0..10 {
        let id = cqc.recv_epr(false);
        let outcome = cqc.measure_qubit(id, false);
        print!("{}", outcome as u8);
    }
    println!();
}
