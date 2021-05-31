use crate::binary::merkle_tree::MerkleTree;
use crate::digest::Digest;
use crate::proof_set::ProofSet;

pub fn verify<D: Digest>(
    root: &[u8; 32],
    proof_set: ProofSet,
    proof_index: u64,
    num_leaves: u64,
) -> bool {
    if proof_index >= num_leaves {
        return false;
    }

    if proof_set.len() == 0 {
        return false;
    }

    let mut height = 0usize;
    let proof_data = proof_set.get(height).unwrap();
    let mut sum = MerkleTree::<D>::leaf_sum(proof_data);
    height += 1;

    let mut stable_end = proof_index;

    loop {
        let subtree_start_index = (proof_index / (1 << height)) * (1 << height);
        let subtree_end_index = subtree_start_index + (1 << height) - 1;
        if subtree_end_index >= num_leaves {
            break;
        }

        stable_end = subtree_end_index;

        if proof_set.len() <= height {
            return false;
        }

        let proof_data = proof_set.get(height).unwrap();
        if proof_index - subtree_start_index < 1 << (height - 1) {
            sum = MerkleTree::<D>::node_sum(&sum, proof_data);
        } else {
            sum = MerkleTree::<D>::node_sum(proof_data, &sum);
        }

        height += 1;
    }

    if stable_end != num_leaves - 1 {
        if proof_set.len() <= height {
            return false;
        }
        let proof_data = proof_set.get(height).unwrap();
        sum = MerkleTree::<D>::node_sum(&sum, proof_data);
        height += 1;
    }

    while height < proof_set.len() {
        let proof_data = proof_set.get(height).unwrap();
        sum = MerkleTree::<D>::node_sum(proof_data, &sum);
        height += 1;
    }

    return sum == *root;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::binary::merkle_tree::MerkleTree;
    use crate::sha::Sha256 as Hash;

    type MT = MerkleTree<Hash>;

    #[test]
    fn verify_returns_true_when_the_given_proof_set_matches_the_given_merkle_root() {
        let mut mt = MT::new();
        mt.set_proof_index(2);

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
        }

        let proof = mt.prove();
        let root = proof.0;
        let set = proof.1;

        let verification = verify::<Hash>(&root, set, 2, 5);
        assert_eq!(verification, true);
    }

    #[test]
    fn verify_returns_false_when_the_given_proof_set_does_not_match_the_given_merkle_root() {
        let root = (|| {
            let mut mt = MT::new();
            mt.set_proof_index(2);

            let leaves = [
                "Hello, World!".as_bytes(),
                "Making banana pancakes".as_bytes(),
                "What is love?".as_bytes(),
                "Bob Ross".as_bytes(),
                "The smell of napalm in the morning".as_bytes(),
            ];
            for leaf in leaves.iter() {
                mt.push(leaf);
            }

            let proof = mt.prove();
            proof.0
        })();

        let set = (|| {
            let mut mt = MT::new();
            mt.set_proof_index(2);

            let leaves = [
                "Hello, World!".as_bytes(),
                "Making banana pancakes".as_bytes(),
                "What is love?".as_bytes(),
                "Bob Ross".as_bytes(),
                "This tree is different!".as_bytes(),
            ];
            for leaf in leaves.iter() {
                mt.push(leaf);
            }

            let proof = mt.prove();
            proof.1
        })();

        let verification = verify::<Hash>(&root, set, 2, 5);
        assert_eq!(verification, false);
    }

    #[test]
    fn verify_returns_false_when_the_proof_set_is_empty() {
        let mut mt = MT::new();
        mt.set_proof_index(0);

        let proof = mt.prove();
        let root = proof.0;
        let set = proof.1;

        let verification = verify::<Hash>(&root, set, 0, 0);
        assert_eq!(verification, false);
    }

    #[test]
    fn verify_returns_false_when_the_proof_index_is_invalid() {
        let mut mt = MT::new();
        mt.set_proof_index(0);

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
        }

        let proof = mt.prove();
        let root = proof.0;
        let set = proof.1;

        let verification = verify::<Hash>(&root, set, 15, 5);
        assert_eq!(verification, false);
    }
}
