use ark_ec::{AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{Field, One, PrimeField, UniformRand};
use ark_std::{rand::Rng, test_rng};

use crate::*;

use ark_algebra_test_templates::{
    curves::*, generate_bilinearity_test, generate_g1_test, generate_g2_test,
    generate_product_of_pairings_test, groups::*, msm::*,
};

use core::ops::MulAssign;

generate_g1_test!(mnt6_298; curve_tests; sw_tests;);
generate_g2_test!(mnt6_298; curve_tests; sw_tests;);
generate_bilinearity_test!(MNT6_298, Fq6);
generate_product_of_pairings_test!(MNT6_298);
