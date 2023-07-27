use std::time::Instant;

use indy_blssignatures::*;

fn main() {
    let rounds = 10000;

    let gen = Generator::new().unwrap();
    let sign_key: SignKey = SignKey::new(None).unwrap();
    let ver_key = VerKey::new(&gen, &sign_key).unwrap();
    let message = vec![1, 2, 3, 4, 5];
    let mut start;

    start = Instant::now();
    for _ in 0..rounds {
        Bls::sign(&message, &sign_key).unwrap();
    }
    println!("Sign time is {:?}", start.elapsed() / rounds);

    let sig = Bls::sign(&message, &sign_key).unwrap();
    start = Instant::now();
    for _ in 0..rounds {
        Bls::verify(&sig, &message, &ver_key, &gen).unwrap();
    }
    println!("Verify time is {:?}", start.elapsed() / rounds);
}
