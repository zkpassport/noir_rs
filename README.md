# Noir.rs

Rust crate to generate and verify proofs for [Noir](https://github.com/noir-lang/noir) circuits.

This library uses
[barretenberg-rs](https://github.com/AztecProtocol/aztec-packages/tree/next/barretenberg/rust/barretenberg-rs)
to interface with Barretenberg.

To use Noir.rs on mobile, please refer to either [Swoir](https://github.com/Swoir/swoir) or
[Noirandroid](https://github.com/madztheo/noir_android) that provide a much easier interface to
generate proofs on iOS and Android. If you use React Native, you can also use the
[Noir React Native Starter](https://github.com/madztheo/noir-react-native-starter) as a base to get
started.

Also, if you work with circuits with complex inputs, we recommend you have a look at either
[Swoir](https://github.com/Swoir/Swoir/blob/6881136c86d2b6c76a5dac1db5c458e71042793c/Sources/Swoir/Circuit.swift#L123)
or
[Noirandroid](https://github.com/madztheo/noir_android/blob/644e65b04e8b24f42f5cd103f1af2fe15951f215/lib/src/main/java/com/noirandroid/lib/Circuit.kt#L119)
logic to understand how to go from the different types of input (e.g. arrays, structs, strings, ...)
to the `WitnessMap` that can only contain `FieldElement`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noir_rs = { git = "https://github.com/zkpassport/noir_rs.git", tag = "v1.0.0-beta.19-1" }
```

If you want to use `Barretenberg` as backend for proving and verifying proofs, you need to set the
`barretenberg` feature flag.

```toml
[dependencies]
noir_rs = { git = "https://github.com/zkpassport/noir_rs.git", tag = "v1.0.0-beta.19-1", features = ["barretenberg"] }
```

## Usage

Assuming a simple circuit with 3 variables, a, b and res, where res = a \* b. Here's how you would
generate a proof:

```rust
use noir_rs::{
    barretenberg::{prove::prove_ultra_honk, srs::{setup_srs_from_bytecode, setup_srs}, verify::verify_ultra_honk, utils::get_honk_verification_key},
    witness::from_vec_str_to_witness_map,
}

// The bytecode of the circuit
// You can find it in the compiled circuit json file created by running `nargo compile`
const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";

// Setup the SRS
// You can provide a path to the SRS transcript file as second argument
// Otherwise it will be downloaded automatically from Aztec's servers
setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
// Alternatively, if you know the circuit size, you can use the following function
// Assuming the circuit size is 40 here
setup_srs(40, None).unwrap();

// Set up your witness
// a = 5, b = 6, res = a * b = 30
let initial_witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();

// Start timing the proof generation
let start = std::time::Instant::now();
// Generate the proof
// It returns the proof
let proof = prove_ultra_honk(BYTECODE, initial_witness).unwrap();
// Print the time it took to generate the proof
info!("Proof generation time: {:?}", start.elapsed());

// Get the verification key
let vk = get_honk_verification_key(BYTECODE).unwrap();

// Verify the proof
let verdict = verify_ultra_honk(proof, vk).unwrap();
// Print the verdict
info!("Proof verification verdict: {}", verdict);
```

## Build

To build Noir.rs, you can use the following command:

```bash
# Debug build
cargo build
# Release build
cargo build --release
# Release build with the `barretenberg` feature
cargo build --release --features barretenberg
```

### iOS cross-compilation

_Warning: iOS cross-compilation can only be done on macOS._

Before building Noir.rs for iOS, you need to make sure you have installed XCode and its associated
command line tools.

You also need to install the iOS targets for Rust:

```bash
# Physical device
rustup target add aarch64-apple-ios
# Simulator
rustup target add aarch64-apple-ios-sim
```

Finally, to cross-compile Noir.rs for iOS, you can use the following command:

```bash
# Physical device

# Debug build
cargo build --target aarch64-apple-ios
# Release build
cargo build --target aarch64-apple-ios --release
# With the `barretenberg` feature
cargo build --target aarch64-apple-ios --features barretenberg

# Simulator

# Debug build
cargo build --target aarch64-apple-ios-sim
# Release build
cargo build --target aarch64-apple-ios-sim --release
# With the `barretenberg` feature
cargo build --target aarch64-apple-ios-sim --features barretenberg
```

### Android cross-compilation

Android cross-compilation can be done on any desktop platform as long as you have the Android SDK
and NDK installed. If you don't have it, you can get it by downloading Android Studio.

Also, make sure to install the Android target for Rust:

```bash
# ARM64
rustup target add aarch64-linux-android
# x86_64
rustup target add x86_64-linux-android
```

Once this is done, you can cross-compile Noir.rs for Android (only ARM64 and x86_64 are supported)
by running the following command:

```bash
# ARM64

# Debug
cargo build --target aarch64-linux-android
# Release
cargo build --target aarch64-linux-android --release
# With the `barretenberg` feature
cargo build --target aarch64-linux-android --features barretenberg


# x86_64

# Debug
cargo build --target x86_64-linux-android
# Release
cargo build --target x86_64-linux-android --release
# With the `barretenberg` feature
cargo build --target x86_64-linux-android --features barretenberg
```

By default, the API level is 34. You can change it by setting the `ANDROID_API_LEVEL` environment
variable.

```bash
# Set the API level to 33
ANDROID_API_LEVEL=33 cargo build --target aarch64-linux-android --features barretenberg
# Set the API level to 33
ANDROID_API_LEVEL=33 cargo build --target x86_64-linux-android --features barretenberg
```

## Use other backends

For now, Noir.rs can only use Barretenberg as backend. Teams working on other backends are welcome
to contribute to Noir.rs!
