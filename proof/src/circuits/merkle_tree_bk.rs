pub mod chip;
pub mod instruction;
use crate::{Fp, TreeItem};
use halo2_gadgets::poseidon::{primitives::ConstantLength, Hash};
use halo2_gadgets::poseidon::{primitives::Spec, Pow5Chip, Pow5Config};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    dev::MockProver,
    plonk::{Circuit, ConstraintSystem, Error},
};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct MerkleCircuit<S: Spec<Fp, WIDTH, RATE>, const WIDTH: usize, const RATE: usize> {
    // Private inputs.
    index: usize,
    leaf: Option<TreeItem>,
    // Public inputs (from the prover). The root is also a public input, but it is calculated within
    // the circuit.
    path: Vec<TreeItem>,
    root: Option<TreeItem>,
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
        leaf: Option<TreeItem>,
        path: Vec<TreeItem>,
        root: Option<TreeItem>,
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
    type Config = Pow5Config<Fp, WIDTH, RATE>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let partial_sbox = meta.advice_column();

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();

        meta.enable_constant(rc_b[0]);

        Pow5Chip::configure::<S>(
            meta,
            state.try_into().unwrap(),
            partial_sbox,
            rc_a.try_into().unwrap(),
            rc_b.try_into().unwrap(),
        )
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        let mut output = None;
        let mut digest = self
            .leaf
            .map_or_else(|| Value::unknown(), |v| Value::known(v.hash.clone()));
        let root = self
            .root
            .map_or_else(|| Value::unknown(), |v| Value::known(v.hash.clone()));
        let mut bit = self.index & 1;
        let mut layer_index = self.index >> 1;
        let mut layer = 0;
        println!(
            "Leaf {:?}; path: {:?}; index {:?}",
            &self.leaf, &self.path, &self.index
        );
        for elm in self.path.iter() {
            println!("Element value {:?}; digest {:?} bit {}", elm, digest, bit);
            let assigned_cells = layouter.assign_region(
                || "load path elements",
                |mut region| {
                    let (left, right) = if bit == 0 {
                        (digest, Value::known(elm.hash.clone()))
                    } else {
                        (Value::known(elm.hash.clone()), digest)
                    };
                    let left_cell = region.assign_advice(
                        || format!("load left element of layer {}", layer),
                        config.state[0],
                        0,
                        || left,
                    )?;
                    let right_cell = region.assign_advice(
                        || format!("load right element of layer {}", layer),
                        config.state[1],
                        0,
                        || right,
                    )?;
                    Ok([left_cell, right_cell])
                },
            )?;
            let chip = Pow5Chip::construct(config.clone());
            let hasher = Hash::<_, _, S, ConstantLength<2>, WIDTH, RATE>::init(
                chip,
                layouter.namespace(|| "init"),
            )?;
            let hash_result = hasher.hash(layouter.namespace(|| "hash"), assigned_cells);
            println!("Hash result {:?}", &hash_result);
            output = hash_result.ok();
            digest = output
                .as_ref()
                .map(|cell| cell.value().map(|v| v.clone()))
                .unwrap();
            bit = layer_index & 1;
            layer_index = layer_index >> 1;
        }
        // let message = layouter.assign_region(
        //     || "load message",
        //     |mut region| {
        //         let message_word = |i: usize| {
        //             let value = self.message.map(|message_vals| message_vals[i]);
        //             region.assign_advice(
        //                 || format!("load message_{}", i),
        //                 config.state[i],
        //                 0,
        //                 || value,
        //             )
        //         };

        //         let message: Result<Vec<_>, Error> = (0..L).map(message_word).collect();
        //         Ok(message?.try_into().unwrap())
        //     },
        // )?;

        // let output = hasher.hash(layouter.namespace(|| "hash"), message)?;
        println!("Last value {:?}; Last digest {:?}", &root, &digest);
        let root_cell = output.as_ref().map(|cell| cell.cell().clone());
        layouter.assign_region(
            || "constrain output",
            |mut region| {
                let expected_var =
                    region.assign_advice(|| "load root hash", config.state[0], 0, || root)?;
                region.constrain_equal(root_cell.unwrap(), expected_var.cell())
            },
        )?;
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
