use keccak256::keccak256;

fn main(msg: [u8; 5]) {
    for i in 0..150 {
        keccak256(
            [msg[0] + i, msg[1] + i, msg[2] + i, msg[3] + i, msg[4] + i],
            5,
        );
    }
}

#[test]
fn test_main() {
    main([2, 5, 10, 15, 20]);
}
