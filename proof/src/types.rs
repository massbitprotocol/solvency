//Pasta curve
//pub type Fp = halo2_proofs::pasta::Fp;
//pub type Fq = halo2_proofs::pasta::Fq;
//pub type Fr = halo2_proofs::pasta::Fp;

pub type MerkleTree<H> = crate::merkle_sum_tree::MerkleSumTree<H>;
// pub type Fp = goldilocks_ntt::Field;
// pub type Fr = goldilocks_ntt::Field;

// pub type Fp = halo2curves::pasta::Fp;
// pub type Fq = halo2curves::pasta::Fq;

pub type Fp = halo2_proofs::halo2curves::bn256::Fr;
pub type Fq = halo2_proofs::halo2curves::bn256::Fq;
