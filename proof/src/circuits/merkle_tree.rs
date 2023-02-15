pub mod chip;
pub mod instruction;

use self::chip::MerkleSumConfig;
use crate::{CurveEngine, Fp, G1Affine, TreeItem};
use chip::MerkleSumChip;
use halo2_gadgets::poseidon::primitives::Spec;
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    dev::MockProver,
    plonk::{create_proof, verify_proof, Circuit, ConstraintSystem, Error, ProvingKey},
    poly::{
        commitment::{CommitmentScheme, ParamsVerifier, Prover, Verifier},
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::{ProverGWC, VerifierGWC},
        },
    },
    transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer},
};
use instruction::MerkleSumInstructions;
use rand::rngs::OsRng;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct MerkleCircuit<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize> {
    // Private inputs.
    index: usize,
    leaf: Option<TreeItem<Fp>>,
    // Public inputs (from the prover). The root is also a public input, but it is calculated within
    // the circuit.
    path: Vec<TreeItem<Fp>>,
    root: Option<TreeItem<Fp>>,
    _spec: PhantomData<S>,
}
impl<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize>
    MerkleCircuit<S, WIDTH, RATE>
{
    pub fn default() -> Self {
        MerkleCircuit {
            index: 0,
            leaf: None,
            path: vec![],
            root: None,
            _spec: PhantomData,
        }
    }
    pub fn new(
        index: usize,
        leaf: Option<TreeItem<Fp>>,
        path: Vec<TreeItem<Fp>>,
        root: Option<TreeItem<Fp>>,
    ) -> Self {
        Self {
            index,
            leaf,
            path,
            root,
            _spec: PhantomData::default(),
        }
    }
}
impl<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize> Circuit<Fp>
    for MerkleCircuit<S, WIDTH, RATE>
{
    type Config = MerkleSumConfig<Fp, WIDTH, RATE>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        MerkleSumChip::<S, Fp, WIDTH, RATE>::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let merkle_chip = MerkleSumChip::<S, Fp, WIDTH, RATE>::construct(config);
        // merkle_chip.load_public(
        //     layouter.namespace(|| "load public"),
        //     self.leaf.as_ref(),
        //     self.root.as_ref(),
        // )?;
        let root_cell = merkle_chip.load_private(
            layouter.namespace(|| "load private"),
            self.index,
            self.leaf.as_ref(),
            &&self.path,
            self.root.as_ref(),
        )?;
        //
        // Public Inputs: user ID i; claimed balance b, merkle sum balance-root
        // (R,B)
        //
        merkle_chip.expose_public(layouter.namespace(|| "expose public"), root_cell, 0)?;
        // let merkle_chip = MerkleChip::new(config);
        // let mut layer_digest = merkle_chip.hash_leaf_layer(
        //     &mut layouter,
        //     self.leaf.as_ref().unwrap().clone(),
        //     self.path.as_ref().unwrap()[0],
        //     self.c_bits.as_ref().unwrap()[0].clone(),
        // )?;
        // for layer in 1..PATH_LEN {
        //     layer_digest = merkle_chip.hash_non_leaf_layer(
        //         &mut layouter,
        //         layer_digest,
        //         self.path.as_ref().unwrap()[layer].clone(),
        //         self.c_bits.as_ref().unwrap()[layer].clone(),
        //         layer,
        //     )?;
        // }
        Ok(())
    }
}

pub fn create_merkle_proof<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize>(
    circuit: MerkleCircuit<S, WIDTH, RATE>,
    params: &ParamsKZG<CurveEngine>,
    pk: &ProvingKey<<KZGCommitmentScheme<CurveEngine> as CommitmentScheme>::Curve>,
    //instances: &Vec<&[Fp]>,
    instances: &[&[Fp]],
) -> Vec<u8>
// -> Blake2bWrite<
//     Vec<u8>,
//     G1Affine,
//     Challenge255<<KZGCommitmentScheme<CurveEngine> as CommitmentScheme>::Curve>,
// >
{
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    // Create a proof
    let result = create_proof::<
        KZGCommitmentScheme<CurveEngine>,
        ProverGWC<'_, CurveEngine>,
        Challenge255<G1Affine>,
        _,
        Blake2bWrite<Vec<u8>, G1Affine, Challenge255<_>>,
        _,
    >(params, pk, &[circuit], &[instances], OsRng, &mut transcript);
    match result {
        Ok(_) => transcript.finalize(),
        Err(err) => {
            panic!("Error while create proof {:?}", &err);
        }
    }
    // let proof: Vec<u8> = transcript.finalize();
    // proof
}

// pub fn verify_merkle_proof<
//     'params,
//     Scheme: CommitmentScheme,
//     V: Verifier<'params, Scheme>,
//     E: EncodedChallenge<Scheme::Curve>,
//     T: TranscriptRead<Scheme::Curve, E>,
//     Strategy: VerificationStrategy<'params, Scheme, V>,
// >(
//     params: ParamsKZG<CurveEngine>,
//     vk: &VerifyingKey<Scheme::Curve>,
//     instances: &[&[&[Scheme::Scalar]]],
//     transcript: &mut T,
// ) -> Result<Strategy::Output, Error> {
//     let strategy = SingleStrategy::<CurveEngine>::new(&params);
//     verify_proof::<Scheme, V, E, T, Strategy>(&params, vk, strategy, instances, transcript)
// }
