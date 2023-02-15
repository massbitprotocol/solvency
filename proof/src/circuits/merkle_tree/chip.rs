use super::MerkleSumInstructions;
use crate::{Field, TreeItem};
use core::marker::PhantomData;
use halo2_gadgets::poseidon::{
    primitives::ConstantLength, primitives::Spec, Hash, Pow5Chip, Pow5Config,
};
use halo2_proofs::{arithmetic::FieldExt, circuit::*, plonk::*, poly::Rotation};
#[derive(Debug, Clone)]
pub struct MerkleSumConfig<F: FieldExt, const WIDTH: usize, const RATE: usize> {
    pub pow5: Pow5Config<F, WIDTH, RATE>,
    pub sum_cur: Column<Advice>, //Current sum value
    pub sum_sis: Column<Advice>, //Sister sum value
    pub sum_agg: Column<Advice>, //Aggregated value
    pub selector: Selector,
    pub instance: Column<Instance>,
}

#[derive(Debug)]
pub struct MerkleSumChip<
    S: Spec<F, WIDTH, RATE>,
    F: FieldExt,
    const WIDTH: usize,
    const RATE: usize,
> {
    config: MerkleSumConfig<F, WIDTH, RATE>,
    pow5: Pow5Chip<F, WIDTH, RATE>,
    _marker: PhantomData<S>,
}

impl<S: Spec<F, WIDTH, RATE>, F: FieldExt, const WIDTH: usize, const RATE: usize>
    MerkleSumChip<S, F, WIDTH, RATE>
{
    pub fn construct(config: MerkleSumConfig<F, WIDTH, RATE>) -> Self {
        let pow5 = Pow5Chip::construct(config.pow5.clone());
        Self {
            config,
            pow5,
            _marker: PhantomData,
        }
    }
    pub fn configure(meta: &mut ConstraintSystem<F>) -> MerkleSumConfig<F, WIDTH, RATE> {
        //Poseidon hash chip uses 4 columns, 3 last advice column using for sum calculation
        //
        // | state0 | state1 | ... | partial_sbox | rc_a | rc_b | sum_cur | sum_sister | sum_aggregate |
        // |--------|--------|-----|--------------|------|------|---------|------------|---------------|
        // |        |        |     |              |      |      |         |            |               |

        let state = (0..WIDTH).map(|_| meta.advice_column()).collect::<Vec<_>>();
        let partial_sbox = meta.advice_column();

        let rc_a = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        let rc_b = (0..WIDTH).map(|_| meta.fixed_column()).collect::<Vec<_>>();
        meta.enable_constant(rc_b[0]);

        let pow5 = Pow5Chip::configure::<S>(
            meta,
            state.try_into().unwrap(),
            partial_sbox,
            rc_a.try_into().unwrap(),
            rc_b.try_into().unwrap(),
        );
        let selector = meta.selector();
        let sum_cur = meta.advice_column();
        let sum_sis = meta.advice_column();
        let sum_agg = meta.advice_column();

        let instance = meta.instance_column();
        meta.enable_equality(instance);
        meta.create_gate("balance check", |meta| {
            let selector = meta.query_selector(selector);
            let balance_cur = meta.query_advice(sum_cur, Rotation::cur());
            let balance_sis = meta.query_advice(sum_sis, Rotation::cur());
            let balance_agg = meta.query_advice(sum_agg, Rotation::cur());
            vec![selector * (balance_cur + balance_sis - balance_agg)]
        });

        MerkleSumConfig {
            pow5,
            sum_cur,
            sum_sis,
            sum_agg,
            selector,
            instance,
        }
    }
}

// ANCHOR: chip-impl
impl<S: Spec<F, WIDTH, RATE>, F: FieldExt, const WIDTH: usize, const RATE: usize> Chip<F>
    for MerkleSumChip<S, F, WIDTH, RATE>
{
    type Config = MerkleSumConfig<F, WIDTH, RATE>;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl<S: Spec<F, WIDTH, RATE>, F: Field, const WIDTH: usize, const RATE: usize>
    MerkleSumInstructions<F> for MerkleSumChip<S, F, WIDTH, RATE>
{
    fn load_private(
        &self,
        mut layouter: impl Layouter<F>,
        index: usize,
        leaf: Option<&TreeItem<F>>,
        path: &Vec<TreeItem<F>>,
        root: Option<&TreeItem<F>>,
    ) -> Result<AssignedCell<F, F>, Error> {
        let mut output = None;
        let (mut digest, mut sum) = leaf.map_or_else(
            || (Value::unknown(), Value::unknown()),
            |v| (Value::known(v.hash.clone()), Value::known(v.sum)),
        );
        let root = root.map_or_else(|| Value::unknown(), |v| Value::known(v.hash.clone()));
        let mut bit = index & 1;
        let mut layer_index = index >> 1;
        println!("Leaf {:?}; path: {:?}; index {:?}", &leaf, &path, &index);
        for elm in path.iter() {
            //println!("Element value {:?}; digest {:?} bit {}", elm, digest, bit);
            let sis_value = sum.map(|val| F::from_u128(val as u128));
            let elm_value = Value::known(F::from_u128(elm.sum as u128));
            sum = sum.map(|f| f + elm.sum);
            let agg_value = sum.map(|val| F::from_u128(val as u128));
            let assigned_cells = layouter.assign_region(
                || "load path elements",
                |mut region| {
                    let (left, right) = if bit == 0 {
                        (digest, Value::known(elm.hash.clone()))
                    } else {
                        (Value::known(elm.hash.clone()), digest)
                    };
                    let left_cell = region.assign_advice(
                        || format!("load left element of layer {}", layer_index),
                        self.config.pow5.state[0],
                        0,
                        || left,
                    )?;
                    let right_cell = region.assign_advice(
                        || format!("load right element of layer {}", layer_index),
                        self.config.pow5.state[1],
                        0,
                        || right,
                    )?;
                    //assign balance values
                    self.config.selector.enable(&mut region, 0)?;
                    region.assign_advice(
                        || format!("load aggregated sister balance of layer {:?}", &sum),
                        self.config.sum_sis,
                        0,
                        || sis_value,
                    )?;
                    region.assign_advice(
                        || format!("load path element balance {:?}", &elm.sum),
                        self.config.sum_cur,
                        0,
                        || elm_value,
                    )?;
                    region.assign_advice(
                        || format!("load aggregated balance {:?}", &agg_value),
                        self.config.sum_agg,
                        0,
                        || agg_value,
                    )?;
                    Ok([left_cell, right_cell])
                },
            )?;
            let chip = Pow5Chip::construct(self.config.pow5.clone());
            let hasher = Hash::<_, _, S, ConstantLength<2>, WIDTH, RATE>::init(
                chip,
                layouter.namespace(|| "init"),
            )?;
            let hash_result = hasher.hash(layouter.namespace(|| "hash"), assigned_cells);
            //println!("Hash result {:?}", &hash_result);
            output = hash_result.ok();
            digest = output
                .as_ref()
                .map(|cell| cell.value().map(|v| v.clone()))
                .unwrap();
            bit = layer_index & 1;
            layer_index = layer_index >> 1;
        }
        //println!("Last value {:?}; Last digest {:?}", &root, &digest);
        let root_cell = output.as_ref().map(|cell| cell.cell().clone());
        layouter.assign_region(
            || "constrain output",
            |mut region| {
                let expected_var = region.assign_advice(
                    || "load root hash",
                    self.config.pow5.state[0],
                    0,
                    || root,
                )?;
                region.constrain_equal(root_cell.unwrap(), expected_var.cell())
            },
        )?;
        output.ok_or_else(|| Error::Synthesis)
    }
    // fn load_public(
    //     &self,
    //     mut layouter: impl Layouter<F>,
    //     leaf: Option<&TreeItem<F>>,
    //     root: Option<&TreeItem<F>>,
    // ) -> Result<(), Error> {
    //     let (leaf_hash, leaf_balance) = leaf.map_or_else(
    //         || (Value::unknown(), Value::unknown()),
    //         |v| {
    //             (
    //                 Value::known(v.hash.clone()),
    //                 Value::known(F::from_u128(v.sum as u128)),
    //             )
    //         },
    //     );
    //     let (root_hash, root_balance) = root.map_or_else(
    //         || (Value::unknown(), Value::unknown()),
    //         |v| {
    //             (
    //                 Value::known(v.hash.clone()),
    //                 Value::known(F::from_u128(v.sum as u128)),
    //             )
    //         },
    //     );
    //     Ok(())
    // }
    // fn add(
    //     &self,
    //     mut layouter: impl Layouter<F>,
    //     a: Self::Num,
    //     b: Self::Num,
    // ) -> Result<Self::Num, Error> {
    //     todo!()
    // }

    fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error> {
        let config = self.config();

        layouter.constrain_instance(cell.cell(), config.instance, row)
    }
}
