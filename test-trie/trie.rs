use verkle_trie::proof::golang_proof_format::{
    StateDiff,
    VerkleProofGo,
    bytes32_to_element,
    hex_to_bytes32,
};
use crate::example_test_data::{ PREVIOUS_STATE_ROOT, EXECUTION_WITNESS_JSON };

pub fn deserialize_and_verify_verkle_proof() {
    let verkle_proof_go = VerkleProofGo::from_json_str(EXECUTION_WITNESS_JSON);
    let (got_verkle_proof, keys_values) = verkle_proof_go
        .from_verkle_proof_go_to_verkle_proof()
        .unwrap();
    let prestate_root = bytes32_to_element(hex_to_bytes32(PREVIOUS_STATE_ROOT)).unwrap();
    let (ok, _) = got_verkle_proof.check(
        keys_values.keys,
        keys_values.current_values,
        prestate_root
    );
    assert!(ok);
    println!("okay");
}

#[cfg(test)]
mod tests {
    use verkle_trie::proof::golang_proof_format::{
        bytes32_to_element,
        hex_to_bytes32,
        VerkleProofGo,
        EXECUTION_WITNESS_JSON,
        PREVIOUS_STATE_ROOT,
    };

    #[test]
    fn test_proof_from_json_golang_serde() {
        print!("hello1");
        let verkle_proof_go = VerkleProofGo::from_json_str(EXECUTION_WITNESS_JSON);
        let (got_verkle_proof, keys_values) = verkle_proof_go
            .from_verkle_proof_go_to_verkle_proof()
            .unwrap();
        const PREVIOUS_STATE_ROOT1: &str = "0x2cf2ab8fed2dcfe2fa77da044ab16393dbdabbc65deea5fdf272107a039f2c60";
        print!("{:?}", PREVIOUS_STATE_ROOT1);
        let prestate_root = bytes32_to_element(hex_to_bytes32(PREVIOUS_STATE_ROOT1)).unwrap();

        let (ok, _) = got_verkle_proof.check(
            keys_values.keys,
            keys_values.current_values,
            prestate_root
        );
        assert!(ok);
    }
}
