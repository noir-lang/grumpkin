use crate::{Fq, Fr};
use ark_ec::models::{
    short_weierstrass::{Affine as SWGroupAffine, Projective as SWGroupProjective, SWCurveConfig},
    CurveConfig,
};
use ark_ff::{Field, MontFp, Zero};

pub type SWAffine = SWGroupAffine<GrumpkinParameters>;
pub type SWProjective = SWGroupProjective<GrumpkinParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct GrumpkinParameters;

impl CurveConfig for GrumpkinParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[1];

    /// COFACTOR^(-1) mod r = 1
    const COFACTOR_INV: Fr = Fr::ONE;
}

/// x coordinate for SW curve generator
/// value is 1
const SW_GENERATOR_X: Fq = Fq::ONE;
/// y coordinate for SW curve generator
/// value is sqrt(-15)
const SW_GENERATOR_Y: Fq = MontFp!("17631683881184975370165255887551781615748388533673675138860");

impl SWCurveConfig for GrumpkinParameters {
    /// COEFF_A = 0
    const COEFF_A: Self::BaseField = Fq::ZERO;

    /// COEFF_B = -17
    const COEFF_B: Self::BaseField = MontFp!("-17");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: SWAffine = SWAffine::new_unchecked(SW_GENERATOR_X, SW_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}
