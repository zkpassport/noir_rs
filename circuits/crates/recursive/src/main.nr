use dep::std;

fn main(
    verification_key: [Field; 103],
    proof : [Field; 409], 
    public_inputs: pub [Field; 1], 
    key_hash : Field,
) {
    std::verify_proof(
        verification_key.as_slice(),
        proof.as_slice(),
        public_inputs.as_slice(),
        key_hash,
    );
}