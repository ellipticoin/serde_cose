use ed25519_zebra::SigningKey;
use std::convert::TryFrom;

struct User {
    public_key: ed25519_zebra::SigningKey,
}

fn main() {
    let cose_bytes = hex::decode("D28445A201270300A10442313154546869732069732074686520636F6E74656E742E58407142FD2FF96D56DB85BEE905A76BA1D0B7321A95C8C4D3607C5781932B7AFB8711497DFA751BF40B58B3BCC32300B1487F3DB34085EEF013BF08F4A44D6FEF0D").unwrap();

    let sign1 = serde_cose::from_slice(&cose_bytes).unwrap();
    let user = lookup_user(&sign1.kid());
    let key: serde_cose::Key = user.public_key.into();
    if key.verify(&sign1).is_ok() {
        println!("Valid Signature!")
    } else {
        println!("Invalid Signature :(")
    }
}

fn lookup_user(user_id: &[u8]) -> User {
    match std::str::from_utf8(&user_id).unwrap() {
        "11" => User {
            public_key: SigningKey::try_from(
                <[u8; 32]>::try_from(
                    &hex::decode(
                        &"d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
                    )
                    .unwrap()[..],
                )
                .unwrap(),
            )
            .unwrap(),
        },
        id => panic!(format!("user {} not found", id)),
    }
}
