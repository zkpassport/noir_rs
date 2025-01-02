fn main(msg: [u8; 5]) {
    std::hash::keccak256(msg, 5);
}

#[test]
fn test_main() {
    main([2, 5, 10, 15, 20]);
}
