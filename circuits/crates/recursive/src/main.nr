use dep::std;

fn main(
    verification_key: [Field; 128],
    proof: [Field; 464],
    public_inputs: pub [Field; 1],
    key_hash: Field
) {
    std::verify_proof(verification_key, proof, public_inputs, key_hash);
}
