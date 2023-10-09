use crate::util;
use crate::util::{decode_hash, encode_hash, hash_internal, read_merkle_proof, MerkleProof};
use std::collections::VecDeque;

pub fn run(proof_file: &String) {
    let merkle_proof = read_merkle_proof(proof_file);

    verify_merkle_proof(merkle_proof)
}

fn verify_merkle_proof(merkle_proof: Box<MerkleProof>) {
    let root = decode_hash("rWSy+vjSP6b5d73u6QyoRCo8HKHQ3EpvlYC09qgTxmA=");
    let computed_root = compute_merkle_root_from_merkle_proof(merkle_proof);
    // let computed_root_base64 = encode_hash(computed_root);
    // println!("computed_root: {:?}", computed_root);
    // println!("computed_root: {:?}", computed_root_base64);
    // println!("root: {:?}", root);
    assert_eq!(computed_root, root);
}

fn compute_merkle_root_from_merkle_proof(merkle_proof: Box<MerkleProof>) -> util::Hash32Bytes {
    let mut pos = merkle_proof.leaf_position;
    let mut hashes = VecDeque::from(merkle_proof.proof_hash_values.unwrap());
    let mut cur_root = util::hash_leaf(merkle_proof.leaf_value);
    while let Some(hash) = hashes.pop_front() {
        let (left, right);
        if pos % 2 == 0 {
            left = cur_root;
            right = hash;
        } else {
            left = hash;
            right = cur_root;
        }
        cur_root = hash_internal(left, right);
        pos >>= 1;
    }
    cur_root
}
