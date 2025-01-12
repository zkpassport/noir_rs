# Noir.rs

Rust crate to generate and verify proofs for [Noir](https://github.com/noir-lang/noir) circuits.

This library uses
[bb.rs](https://github.com/zkpassport/aztec-packages/tree/v0.58.0/barretenberg/bb_rs) to interface
with Barretenberg, which is optimized to run on mobile platforms (i.e. iOS and Android ARM64), but
can also be used on desktop platforms.

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
noir_rs = { git = "https://github.com/zkpassport/noir_rs.git", branch = "v1.0.0-beta.1" }
```

If you want to use `Barretenberg` as backend for proving and verifying proofs, you need to set the
`barretenberg` feature flag.

```toml
[dependencies]
noir_rs = { git = "https://github.com/zkpassport/noir_rs.git", branch = "v1.0.0-beta.1", features = ["barretenberg"] }
```

## Usage

Assuming a simple circuit with 3 variables, a, b and res, where res = a \* b. Here's how you would
generate a proof:

```rust
use noir_rs::{
    barretenberg::{prove::prove_ultra_honk, srs::setup_srs, verify::verify_ultra_honk},
    witness::from_vec_str_to_witness_map,
}

// The bytecode of the circuit
// You can find it in the compiled circuit json file created by running `nargo compile`
const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";

// Setup the SRS
// You can provide a path to the SRS transcript file as second argument
// Otherwise it will be downloaded automatically from Aztec's servers
setup_srs(BYTECODE, None).unwrap();

// Set up your witness
// a = 5, b = 6, res = a * b = 30
let initial_witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();

// Start timing the proof generation
let start = std::time::Instant::now();
// Generate the proof
// It returns the proof and the verification key
let (proof, vk) = prove_ultra_honk(BYTECODE, initial_witness).unwrap();
// Print the time it took to generate the proof
info!("Proof generation time: {:?}", start.elapsed());

// Verify the proof
let verdict = verify_ultra_honk(proof, vk).unwrap();
// Print the verdict
info!("Proof verification verdict: {}", verdict);
```

## Build

To build Noir.rs, you can use the following command (with the `-vvvv` flag to get more information):

```bash
# We recommend using the `-vvvv` flag to see the progress of the build as it can take several minutes
# if you enable `barretenberg`
cargo build -vvvv
# Release build
cargo build -vvvv --release
# Release build with the `barretenberg` feature
cargo build -vvvv --release --features barretenberg
```

### iOS cross-compilation

_Warning: iOS cross-compilation can only be done on macOS._

Before building Noir.rs for iOS, you need to make sure you have installed XCode and its associated
command line tools.

You also need to install the iOS target for Rust:

```bash
rustup target add aarch64-apple-ios
```

Finally, to cross-compile Noir.rs for iOS (only ARM64 is supported), you can use the following
command:

```bash
# Without setting the deployment target explicitly, the build may fail
IPHONEOS_DEPLOYMENT_TARGET=15.0 cargo build -vvvv --target aarch64-apple-ios
# Release build
IPHONEOS_DEPLOYMENT_TARGET=15.0 cargo build -vvvv --target aarch64-apple-ios --release
# Build with the `barretenberg` feature
IPHONEOS_DEPLOYMENT_TARGET=15.0 cargo build -vvvv --target aarch64-apple-ios --features barretenberg
# Release build with the `barretenberg` feature
IPHONEOS_DEPLOYMENT_TARGET=15.0 cargo build -vvvv --target aarch64-apple-ios --features barretenberg --release
```

### Android cross-compilation

Android cross-compilation can be done on any desktop platform as long as you have the Android SDK
and NDK installed. If you don't have it, you can get it by downloading Android Studio.

Before building Noir.rs for Android, you need to set up a few environment variables:

```bash
# Set the ANDROID_HOME environment variable to the path to your Android SDK
export ANDROID_HOME=/path/to/your/android-sdk # e.g. /Users/<username>/Library/Android/sdk
# Set the NDK_VERSION environment variable to the version of the Android NDK you have installed
export NDK_VERSION=<number>.<number>.<number> # e.g. 26.3.11579264
# Set the HOST_TAG environment variable to the host tag of your platform
export HOST_TAG=your-host-tag # e.g. darwin-x86_64 (for macOS)
# Then you just copy paste the ones below
export TARGET=aarch64-linux-android
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/$NDK_VERSION
export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$HOST_TAG
export API=33
export AR="$TOOLCHAIN/bin/llvm-ar"
export CC="$TOOLCHAIN/bin/$TARGET$API-clang"
export AS="$TOOLCHAIN/bin/$TARGET$API-clang"
export CXX="$TOOLCHAIN/bin/$TARGET$API-clang++"
export LD="$TOOLCHAIN/bin/ld"
export RANLIB="$TOOLCHAIN/bin/llvm-ranlib"
export STRIP="$TOOLCHAIN/bin/llvm-strip"
export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin"
export PATH="$PATH:$TOOLCHAIN/bin"
export CMAKE_TOOLCHAIN_FILE_aarch64_linux_android="$ANDROID_NDK_HOME/build/cmake/android.toolchain.cmake"
export ANDROID_ABI="arm64-v8a"
```

Also, make sure to install the Android target for Rust:

```bash
rustup target add aarch64-linux-android
```

Once this is done, you can cross-compile Noir.rs for Android (only ARM64) by running the following
command:

```bash
cargo build -vvvv --target aarch64-linux-android --features android-compat
# Release build
cargo build -vvvv --target aarch64-linux-android --release --features android-compat
# Build with the `barretenberg` feature
cargo build -vvvv --target aarch64-linux-android --features barretenberg --features android-compat
# Release build with the `barretenberg` feature
cargo build -vvvv --target aarch64-linux-android --features barretenberg --features android-compat --release
```

## Use other backends

For now, Noir.rs can only use Barretenberg as backend. Teams working on other backends are welcome
to contribute to Noir.rs!
