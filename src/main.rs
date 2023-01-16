use hex;
use tiny_keccak::keccak256;
use curve25519_dalek::scalar::Scalar;

// derive private view key from private spend key
fn main() {
    let key = hex::decode("dd62d51183f6208cf4d1b9af523f2c80bf534c2694279fc6e46d38f21a3f6e03");
    println!("{}", hex::encode(keccak256(&key.as_ref().unwrap())));
    println!("{}", hex::encode(Scalar::from_bytes_mod_order(keccak256(&key.unwrap())).to_bytes()));
}
