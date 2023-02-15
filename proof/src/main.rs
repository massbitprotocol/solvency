// use ethers_core::{
//     k256::{
//         ecdsa::SigningKey,
//         elliptic_curve::{rand_core::le, PrimeField},
//     },
//     utils::secret_key_to_address,
// };
// use ethers_signers::{LocalWallet, Signer};
use halo2_gadgets::poseidon::primitives::{self as poseidon, ConstantLength};
use halo2_proofs::plonk::{keygen_pk, keygen_vk, verify_proof};
use halo2_proofs::poly::kzg::commitment::{KZGCommitmentScheme, ParamsKZG};
use halo2_proofs::poly::kzg::multiopen::VerifierGWC;
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{
    Blake2bRead, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
};
use halo2_proofs::{arithmetic::Field as Halo2Field, circuit::Value, dev::MockProver};
use solvency_proof::circuits::merkle_tree::create_merkle_proof;
use solvency_proof::{CurveEngine, FieldExt, G1Affine, MerkleTree, PoseidonAlgorithm, TreeItem};
// use merkletree::{
//     merkle::{
//         get_merkle_tree_len_generic, get_merkle_tree_row_count, Element,
//         FromIndexedParallelIterator, MerkleTree,
//     },
//     store::VecStore,
// };
use rand::SeedableRng;
use rand::{rngs::OsRng, Rng};
use rand_xorshift::XorShiftRng;
use solvency_proof::{Fp, OrchardNullifier};
use solvency_proof::{HashCircuit, MerkleCircuit, MOCK_CHAIN_ID};
use std::time::Instant;
// struct Tree(Vec<Vec<Fp>>);

// impl Tree {
//     fn rand() -> Self {
//         let mut rng = thread_rng();
//         let leafs: Vec<Fr> = (0..N_LEAFS).map(|_| Fr::random(&mut rng)).collect();
//         let mut layers = vec![leafs];
//         for l in 1..TREE_LAYERS {
//             let layer: Vec<Fp> = layers[l - 1]
//                 .chunks(2)
//                 .map(|pair| mock_hash(pair[0], pair[1]))
//                 .collect();
//             layers.push(layer)
//         }
//         assert_eq!(layers.last().unwrap().len(), 1);
//         Tree(layers)
//     }

//     fn root(&self) -> Fr {
//         self.0.last().unwrap()[0]
//     }

//     fn leafs(&self) -> &[Fr] {
//         self.0.first().unwrap()
//     }

//     fn gen_path(&self, c: usize) -> Vec<Fr> {
//         let mut path = vec![];
//         let mut node_index = c;
//         for layer in 0..PATH_LEN {
//             let sib = if node_index & 1 == 0 {
//                 self.0[layer][node_index + 1].clone()
//             } else {
//                 self.0[layer][node_index - 1].clone()
//             };
//             path.push(sib);
//             node_index /= 2;
//         }
//         path
//     }
// }
// pub fn generate_vector_of_elements<E: Element>(leaves: usize) -> Vec<E> {
//     let result = (0..leaves).map(|index| {
//         // we are ok with usize -> u8 conversion problems, since we need just predictable dataset
//         let vector: Vec<u8> = (0..E::byte_len()).map(|x| (index + x) as u8).collect();
//         E::from_slice(vector.as_slice())
//     });
//     result.collect()
// }
fn main() {
    println!("Start MerkleCircuit!");
    let rng = OsRng;
    let mut thread_rng = rand::thread_rng();
    // let message = [Fp::random(rng), Fp::random(rng)];
    // let hasher = poseidon::Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init();
    // let output = hasher.hash(message);
    // println!(
    //     "Inputs {:?}; Output {:?}. Hashed in {:?}",
    //     &message,
    //     &output,
    //     now.elapsed()
    // );
    // let k = 6;
    // let circuit =
    //     HashCircuit::<OrchardNullifier, 3, 2, 2>::new(Value::known(message), Value::known(output));
    // let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    // assert_eq!(prover.verify(), Ok(()));

    let tree_size = 1 << 17;
    println!("Tree size {:?}", tree_size);
    let mut datasets = vec![];
    for _ in 0..tree_size {
        // let signer = SigningKey::random(rng);
        // let address = secret_key_to_address(&signer);
        // let item = MPTItem::from(&address);
        let val = Fp::random(rng);
        let sum = thread_rng.gen::<u32>();
        let item = TreeItem::new(val, sum as u64);
        datasets.push(item);
    }
    let now = Instant::now();
    let leaf_ind = thread_rng.gen_range(0..tree_size);
    let leaf = datasets.get(leaf_ind).map(|v| v.clone());
    let merkle_tree = MerkleTree::<PoseidonAlgorithm<Fp>>::from_leaves(datasets.as_slice());
    // let merkle_tree = MerkleTree::<MPTItem, MPTHasher, VecStore<MPTItem>>::try_from_iter(
    //     datasets.into_iter().map(Ok),
    // )
    // .expect("failed to instantiate tree [try_from_iter]");
    println!("Create merkle tree in {:?}", now.elapsed());
    let proof = merkle_tree.proof(&[leaf_ind]);
    let path = proof
        .proof_hashes()
        .iter()
        .map(|item| item.clone())
        .collect::<Vec<TreeItem<Fp>>>();
    let root = merkle_tree.root().map(|item| item.clone());
    // println!(
    //     "Generated Proof for ind {:?} with hashes {:?} in {:?}",
    //     leaf_ind,
    //     proof.proof_hashes(),
    //     now.elapsed()
    // );
    let (leaf_hash, leaf_balance) = leaf.map_or_else(
        || (Fp::zero(), Fp::zero()),
        |v| (v.hash.clone(), Fp::from_u128(v.sum as u128)),
    );
    let (root_hash, root_balance) = root.map_or_else(
        || (Fp::zero(), Fp::zero()),
        |v| (v.hash.clone(), Fp::from_u128(v.sum as u128)),
    );
    let values = vec![root_hash, root_balance, leaf_hash, leaf_balance];
    let instances = vec![values.as_slice()];
    let merkle_circuit = MerkleCircuit::<OrchardNullifier, 3, 2>::new(leaf_ind, leaf, path, root);
    let degree: u32 = 10;
    //let prover = MockProver::run(degree, &merkle_circuit, instances).unwrap();
    //println!("MockProver run in {:?}", now.elapsed());
    //assert_eq!(prover.verify(), Ok(()));
    // Initialize the polynomial commitment parameters
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);
    //let params_vec = vec![];
    //let params = Params::<EqAffine>::read(&mut BufReader::new(&params_vec[..])).unwrap();
    let params = ParamsKZG::<CurveEngine>::setup(degree, &mut rng);
    //let verifier_params: ParamsVerifierKZG<CurveEngine> = params.verifier_params().clone();
    let vk = keygen_vk(&params, &merkle_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &merkle_circuit).expect("keygen_pk should not fail");
    println!("Successfully generated proving key");
    //let instances: &[&[crate::Fp]] = &[&[]];
    //let instances = vec![];
    let now = Instant::now();
    let proof = create_merkle_proof(merkle_circuit, &params, &pk, instances.as_slice());
    println!(
        "Proof generated in {:?} with len {:?}",
        now.elapsed(),
        proof.len()
    );
    //Verify proof
    let strategy = SingleStrategy::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    //let verified = verify_merkle_proof(&params, &vk, &instances, &mut transcript);
    let verified = verify_proof::<
        KZGCommitmentScheme<CurveEngine>,
        VerifierGWC<'_, CurveEngine>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<'_, CurveEngine>,
    >(
        &params,
        pk.get_vk(),
        strategy,
        &[instances.as_slice()],
        &mut transcript,
    );
    println!("Merkle proof verified {:?}", verified);
}

#[cfg(test)]
mod tests {
    use crate::MerkleTree;
    use crate::{HashCircuit, MerkleCircuit};
    use halo2_gadgets::poseidon::primitives::{
        self as poseidon,
        ConstantLength,
        //P128Pow5T3 as OrchardNullifier,
    };
    use halo2_proofs::{arithmetic::Field as Halo2Field, circuit::Value, dev::MockProver};
    use std::time::Instant;
    // use merkletree::hash::Algorithm;
    // use merkletree::merkle::{
    //     get_merkle_tree_len_generic, get_merkle_tree_row_count, Element,
    //     FromIndexedParallelIterator, MerkleTree,
    // };
    // use merkletree::store::{
    //     DiskStore, LevelCacheStore, MmapStore, Store, StoreConfig, VecStore, SMALL_TREE_BUILD,
    // };
    //use solvency_proof::poseidon::PoseidonHasher;
    use rand::{rngs::OsRng, Rng};
    use solvency_proof::{Account, Fp, OrchardNullifier, PoseidonAlgorithm, TreeItem};
    // pub fn generate_vector_of_elements<E: Element>(leaves: usize) -> Vec<E> {
    //     let result = (0..leaves).map(|index| {
    //         // we are ok with usize -> u8 conversion problems, since we need just predictable dataset
    //         let vector: Vec<u8> = (0..E::byte_len()).map(|x| (index + x) as u8).collect();
    //         E::from_slice(vector.as_slice())
    //     });
    //     result.collect()
    // }
    #[test]
    fn poseidon_hash() {
        let rng = OsRng;

        let message = [Fp::random(rng), Fp::random(rng)];
        let output =
            poseidon::Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash(message);
        println!("Output {:?}", &output);
        let k = 6;
        let circuit = HashCircuit::<OrchardNullifier, 3, 2, 2>::new(
            Value::known(message),
            Value::known(output),
        );
        let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()))
    }
    #[test]
    fn poseidon_merkle_tree() {
        let rng = OsRng;
        let mut thread_rng = rand::thread_rng();
        let message = [Fp::random(rng), Fp::random(rng)];
        let output =
            poseidon::Hash::<_, OrchardNullifier, ConstantLength<2>, 3, 2>::init().hash(message);
        println!("Output {:?}", &output);
        let tree_size = 1 << 20;
        println!("Tree size {:?}", tree_size);
        //let now = Instant::now();
        let mut datasets = vec![];
        for _ in 0..tree_size {
            // let signer = SigningKey::random(rng);
            // let address = secret_key_to_address(&signer);
            // let item = MPTItem::from(&address);
            let val = Fp::random(rng);
            let item = TreeItem::from_hash(val);
            datasets.push(item);
        }
        // println!(
        //     "Generate elements {:?} in {:?}",
        //     datasets
        //         .iter()
        //         .map(|e| e.clone().try_into().unwrap())
        //         .collect::<Vec<Fp>>(),
        //     now.elapsed()
        // );
        let now = Instant::now();
        let leaf_ind = thread_rng.gen_range(0..tree_size);
        let leaf = datasets.get(leaf_ind).map(|v| v.clone().into());
        let merkle_tree = MerkleTree::<PoseidonAlgorithm<Fp>>::from_leaves(datasets.as_slice());
        // let merkle_tree = MerkleTree::<MPTItem, MPTHasher, VecStore<MPTItem>>::try_from_iter(
        //     datasets.into_iter().map(Ok),
        // )
        // .expect("failed to instantiate tree [try_from_iter]");
        println!("Create merkle tree in {:?}", now.elapsed());
        let now = Instant::now();
        let proof = merkle_tree.proof(&[leaf_ind]);
        let path = proof
            .proof_hashes()
            .iter()
            .map(|item| item.clone())
            .collect::<Vec<TreeItem<Fp>>>();
        let root = merkle_tree.root().and_then(|item| item.try_into().ok());
        println!(
            "Generated Proof for ind {:?} with hashes {:?} in {:?}",
            leaf_ind,
            proof.proof_hashes(),
            now.elapsed()
        );
        let k = 10;
        let merkle_circuit =
            MerkleCircuit::<OrchardNullifier, 3, 2>::new(leaf_ind, leaf, path, root);
        let prover = MockProver::run(k, &merkle_circuit, vec![]).unwrap();
        assert_eq!(prover.verify(), Ok(()));
    }
}
