use crate::{Field, TreeItem};
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter},
    plonk::Error,
};
pub trait MerkleSumInstructions<F: Field>: Chip<F> {
    /// Loads a leaf item into the circuit as a private input.
    fn load_private(
        &self,
        layouter: impl Layouter<F>,
        index: usize,
        leaf: Option<&TreeItem<F>>,
        path: &Vec<TreeItem<F>>,
        root: Option<&TreeItem<F>>,
    ) -> Result<AssignedCell<F, F>, Error>;
    // fn load_public(
    //     &self,
    //     layouter: impl Layouter<F>,
    //     leaf: Option<&TreeItem<F>>,
    //     root: Option<&TreeItem<F>>,
    // ) -> Result<(), Error>;

    // /// Returns `c = a + b`.
    // fn add(
    //     &self,
    //     layouter: impl Layouter<F>,
    //     a: Self::Num,
    //     b: Self::Num,
    // ) -> Result<Self::Num, Error>;

    /// Exposes a number as a public input to the circuit.
    fn expose_public(
        &self,
        layouter: impl Layouter<F>,
        cell: AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error>;
}
