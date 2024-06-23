use serde_derive::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    popu: usize,
}

fn main() {
    let japan = City {
        name: String::from("Japan"),
        popu: 126_000_000,
    };

    let json = serde_json::to_string(&japan).unwrap();
    let cbor = serde_cbor::to_vec(&japan).unwrap();
    let bin = bincode::serialize(&japan).unwrap();

    println!("\nJSON: {}\nCBOR: {:?}\nBincode: {:?}\n", json, cbor, bin);
    println!(
        "JSON UTF-8: {}\nCBOR UTF-8: {:?}\nBincode UTF-8: {:?}\n",
        String::from_utf8_lossy(json.as_bytes()),
        String::from_utf8_lossy(&cbor),
        String::from_utf8_lossy(&bin)
    )
}
