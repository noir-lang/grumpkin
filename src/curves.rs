use crate::{Fq, Fr};
use ark_ec::{
    models::ModelParameters,
    short_weierstrass_jacobian::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    SWModelParameters,
};
use ark_ff::field_new;

// Note: All parameters have been commented in non-montgomery form

pub type SWAffine = SWGroupAffine<GrumkinParameters>;
pub type SWProjective = SWGroupProjective<GrumkinParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct GrumkinParameters;

impl ModelParameters for GrumkinParameters {
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
    "8005362797447496392714541620568246214488611886064243306355082849781966593384"
);

impl SWModelParameters for GrumkinParameters {
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
}
