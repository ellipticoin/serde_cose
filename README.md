<h1 align="center">Serde COSE</h1>

<br />

<div align="center">
  <a href="https://crates.io/crates/serde_cose">
    <img src="https://img.shields.io/crates/v/serde_cose.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/serde_cose">
    <img src="https://img.shields.io/crates/d/serde_cose.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/serde_cose">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/serde_cose">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/http-rs/serde_cose/blob/main/.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

[COSE (RFC #8152)](https://tools.ietf.org/html/rfc8152) support for [Serde](https://serde.rs/)
## Project Status

Currently serde_cose only supports decoding `ed25519` `Sign1` messsages. No future work is planned but adding signature formats should be fairly straightfoward.


## Usage

Add this to your Cargo.toml:
```toml
serde_cose = "0.1.0"
```
Use `serde_cose::from_slice` to decode COSE messages:
```rust
use ed25519_dalek::PublicKey;

struct User {
    public_key: ed25519_dalek::PublicKey,
}

fn main() {
    let cose_message = hex::decode("D28445A201270300A10442313154546869732069732074686520636F6E74656E742E58407142FD2FF96D56DB85BEE905A76BA1D0B7321A95C8C4D3607C5781932B7AFB8711497DFA751BF40B58B3BCC32300B1487F3DB34085EEF013BF08F4A44D6FEF0D").unwrap();

    // First decode the `Sign1` message type
    // https://tools.ietf.org/html/rfc8152#section-4.2
    let sign1 = serde_cose::from_slice(&cose_message);

    // Next Lookup the user using the key id (`kid`) field
    // https://tools.ietf.org/html/rfc8152#section-3.1
    let user = lookup_user(&sign1.kid());

    // Convert the users public key into a COSE key
    let key: serde_cose::Key = user.public_key.into();
    
    // Verify the signature
    if key.verify(&sign1) {
        println!("Valid Signature!")
    } else {
        println!("Invalid Signature :(")
    }
}

fn lookup_user(user_id: &[u8]) -> User {
    match std::str::from_utf8(&user_id).unwrap() {
        "11" => User {
            public_key: PublicKey::from_bytes(
                &hex::decode(&"d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
                    .unwrap(),
            )
            .unwrap(),
        },
        id => panic!(format!("user {} not found", id)),
    }
}
```


## Contributing
Want to join us? Check out our [The "Contributing" section of the
guide][contributing] and take a look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

#### Conduct

The Serde COSE project adheres to the [Contributor Covenant Code of
Conduct](https://github.com/masonforest/serde_cose/blob/main/.github/CODE_OF_CONDUCT.md).
This describes the minimum behavior expected from all contributors.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[releases]: https://github.com/masonforest/serde_cose/releases
[contributing]: https://github.com/masonforest/serde_cose/blob/main/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/masonforest/serde_cose/labels/good%20first%20issue
[help-wanted]: https://github.com/masonforest/serde_cose/labels/help%20wanted
