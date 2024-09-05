#!/usr/bin/env sh

set -e
mkdir -p proofs
circuit=${1:-recursive}

# Execute the circuit to generate a witness
nargo execute --package $circuit ${circuit}_witness

# Generate proof for the circuit using the generated witness
bb prove_ultra_honk -v -b target/$circuit.json -w target/${circuit}_witness.gz -o ./proofs/${circuit}_honk.proof
# Save a JSON version of the proof
bb proof_as_fields_honk -p proofs/${circuit}_honk.proof -o proofs/${circuit}_honk.proof.json

# Write verification keys
bb write_vk_ultra_honk -v -b target/${circuit}.json -o target/${circuit}_honk.vkey
# Save a JSON version of the verification keys
bb vk_as_fields_ultra_honk -v -k target/${circuit}_honk.vkey -o target/${circuit}_honk.vkey.json
# Verify proof using verification keys
bb verify_ultra_honk -v -p proofs/${circuit}_honk.proof -k target/${circuit}_honk.vkey