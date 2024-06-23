use serde_derive::Serialize;

// String::from()
//                           ... 文字列リテラルからString型の値を作成
// serde_json::to_string()
//                           ... JSON形式の文字列に変換
// serde_cbor::to_vec()
//                           ... CBOR形式のバイト列に変換
// bincode::serialize()
//                           ... Bincode形式のバイト列に変換
// as_bytes()
//                           ... 文字列をバイト列に変換
// String::from_utf8_lossy()
//                           ... バイト列を文字列に変換(エラーだった場合は?マークに置き換え)

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
