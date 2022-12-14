use crate::{Fq, Fr};
use ark_ec::{
    models::ModelParameters,
    short_weierstrass_jacobian::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    SWModelParameters,
};
use ark_ff::{field_new, Zero};

pub type SWAffine = SWGroupAffine<GrumpkinParameters>;
pub type SWProjective = SWGroupProjective<GrumpkinParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct GrumpkinParameters;

impl ModelParameters for GrumpkinParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

/// x coordinate for SW curve generator
/// value is 1
const SW_GENERATOR_X: Fq = field_new!(Fq, "1");
/// y coordinate for SW curve generator
/// value is sqrt(-15)
const SW_GENERATOR_Y: Fq = field_new!(
    Fq,
    "17631683881184975370165255887551781615748388533673675138860"
);

impl SWModelParameters for GrumpkinParameters {
    /// COEFF_A = 0
    const COEFF_A: Self::BaseField = field_new!(Fq, "0");

    /// COEFF_B = -17
    const COEFF_B: Self::BaseField = field_new!(Fq, "-17");

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[1];

    /// COFACTOR^(-1) mod r = 1
    const COFACTOR_INV: Fr = field_new!(Fr, "1");

    /// generators
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_GENERATOR_X, SW_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}
