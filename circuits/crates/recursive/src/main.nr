use dep::std;

fn main(
    verification_key: [Field; 128],
    proof: [Field; 463],
    public_inputs: pub [Field; 1],
    key_hash: Field,
) {
    std::verify_proof_with_type(verification_key, proof, public_inputs, key_hash, 1);
}
