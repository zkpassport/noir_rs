use keccak256::keccak256;

fn main(msg: [u8; 5]) {
    keccak256(msg, 5);
}

#[test]
fn test_main() {
    main([2, 5, 10, 15, 20]);
}
