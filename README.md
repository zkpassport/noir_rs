# Noir.rs
Rust crate to generate and verify proofs for [Noir](https://github.com/noir-lang/noir) circuits. 

This library uses [bb.rs](https://github.com/zkpassport/aztec-packages/tree/v0.58.0/barretenberg/bb_rs) to interface with Barretenberg, which is optimized to run on mobile platforms (i.e. iOS and Android ARM64), but can also be used on desktop platforms.

To use Noir.rs on mobile, please refer to either [Swoir](https://github.com/Swoir/swoir) or [Noirandroid](https://github.com/madztheo/noir_android) that provide a much easier interface to generate proofs on iOS and Android. If you use React Native, you can also use the [Noir React Native Starter](https://github.com/madztheo/noir-react-native-starter) as a base to get started.

Also, if you work with circuits with complex inputs, we recommend you have a look at either [Swoir](https://github.com/Swoir/Swoir/blob/6881136c86d2b6c76a5dac1db5c458e71042793c/Sources/Swoir/Circuit.swift#L123) or [Noirandroid](https://github.com/madztheo/noir_android/blob/644e65b04e8b24f42f5cd103f1af2fe15951f215/lib/src/main/java/com/noirandroid/lib/Circuit.kt#L119) logic to understand how to go from the different types of input (e.g. arrays, structs, strings, ...) to the `WitnessMap` that can only contain `FieldElement`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noir_rs = { git = "https://github.com/zkpassport/noir_rs.git", branch = "v0.36.0" }
```

## Usage

Assuming a simple circuit with 3 variables, a, b and res, where res = a * b. Here's how you would generate a proof:

```rust
use noir_rs::{prove::prove_honk, srs::setup_srs, utils::WitnessMap, verify::verify_honk};

// The bytecode of the circuit
// You can find it in the compiled circuit json file created by running `nargo compile`
const BYTECODE: &str = "H4sIAAAAAAAA/62QQQ6AMAgErfFBUKCFm1+xsf3/E9TYxka96SQEwmGyWTecjPu44aLdc93wDWzOu5cgMOfoMxIu4C2pAEsKioqisnolysoaLVkEQ6aMRYxKFY//ZYQj29T10XfhXv4PNvD4VlxNAQAA";

// Setup the SRS
// You can provide a path to the SRS transcript file as second argument
// Otherwise it will be downloaded automatically from Aztec's servers
setup_srs(String::from(BYTECODE), None).unwrap();

// Set up your witness
let mut initial_witness = WitnessMap::new();
// The first argument is the variable index, the second is the value
// The indexing starts from 0 and the values must be FieldElement
// You can use the `FieldElement::try_from_str` method to convert a string to a FieldElement
// The strings can be decimal or hexadecimal numbers

// a = 5
initial_witness.insert(Witness(0), FieldElement::from(5u128));
// b = 6
initial_witness.insert(Witness(1), FieldElement::try_from_str("6").unwrap());
// res = a * b = 30
initial_witness.insert(Witness(2), FieldElement::try_from_str("0x1e").unwrap());

// Start timing the proof generation
let start = std::time::Instant::now();
// Generate the proof
// It returns the proof and the verification key
let (proof, vk) = prove_honk(String::from(BYTECODE), initial_witness).unwrap();
// Print the time it took to generate the proof
info!("Proof generation time: {:?}", start.elapsed());

// Verify the proof
let verdict = verify_honk(proof, vk).unwrap();
// Print the verdict
info!("Proof verification verdict: {}", verdict);
```

**Note:** Noir.rs currently supports both Plonk and Honk proofs. Plonk proofs will soon be deprecated from Barretenberg, so we recommend using Honk for new projects as you should expect Plonk support to be removed in the next release of Noir.rs.

## Build

To build Noir.rs, you can use the following command (with the `-vvvv` flag to get more information):

```bash
# We recommend using the `-vvvv` flag to see the progress of the build as it can take several minutes
cargo build -vvvv
```

## Use other backends

For now, Noir.rs can only use Barretenberg as backend. It is however planned to add support for other backends. The work of generating proofs and verifying them is already pretty much on the side of `bb.rs`, so decoupling it further from Noir.rs should not be too hard.

Teams working on other backends are welcome to contribute to Noir.rs!