//use crate::Fr;
use crate::Fp;
use halo2_gadgets::poseidon::primitives::Spec;
use halo2_proofs::arithmetic::Field;

/// The type used to hold the MDS matrix and its inverse.
pub type Mds<F, const T: usize> = [[F; T]; T];
/// Poseidon-128 using the $x^5$ S-box, with a width of 3 field elements, and the
/// standard number of rounds for 128-bit security "with margin".
///
/// The standard specification for this set of parameters (on either of the Pasta
/// fields) uses $R_F = 8, R_P = 56$. This is conveniently an even number of
/// partial rounds, making it easier to construct a Halo 2 circuit.
#[derive(Clone, Debug)]
pub struct P128Pow5T3;

impl Spec<Fp, 3, 2> for P128Pow5T3 {
    fn full_rounds() -> usize {
        8
    }

    fn partial_rounds() -> usize {
        56
    }

    fn sbox(val: Fp) -> Fp {
        val.pow_vartime([5])
    }

    fn secure_mds() -> usize {
        3
    }
    // fn constants() -> (Vec<[Fr; 3]>, Mds<Fr, 3>, Mds<Fr, 3>) {
    //     constants.clone()
    // }
    // /// Generates `(round_constants, mds, mds^-1)` corresponding to this specification.
    // fn constants() -> (Vec<[Fp; T]>, Mds<Fp, T>, Mds<Fp, T>) {
    //     let r_f = Self::full_rounds();
    //     let r_p = Self::partial_rounds();

    //     let mut grain = grain::Grain::new(SboxType::Pow, T as u16, r_f as u16, r_p as u16);

    //     let round_constants = (0..(r_f + r_p))
    //         .map(|_| {
    //             let mut rc_row = [F::zero(); T];
    //             for (rc, value) in rc_row
    //                 .iter_mut()
    //                 .zip((0..T).map(|_| grain.next_field_element()))
    //             {
    //                 *rc = value;
    //             }
    //             rc_row
    //         })
    //         .collect();

    //     let (mds, mds_inv) = mds::generate_mds::<F, T>(&mut grain, Self::secure_mds());

    //     (round_constants, mds, mds_inv)
    // }
    // fn constants() -> (Vec<[Fr; 3]>, Mds<Fr, 3>, Mds<Fr, 3>) {
    //     (
    //         super::fr::ROUND_CONSTANTS[..].to_vec(),
    //         super::fr::MDS,
    //         super::fr::MDS_INV,
    //     )
    // }
}
