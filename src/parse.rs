//! Util functions
use bellman_ce::{
    groth16::{self, PreparedVerifyingKey, Proof, VerifyingKey},
    pairing::{
        ff::{PrimeField, PrimeFieldDecodingError, ScalarEngine},
        CurveAffine, EncodedPoint, Engine, GroupDecodingError,
    },
};

/// Generate G point from bytes
pub fn curve_affine<C: CurveAffine>(input: &[u8]) -> C::Uncompressed {
    let mut p = <C::Uncompressed as EncodedPoint>::empty();
    p.as_mut().copy_from_slice(&input);
    p
}

/// Generate VerifyingKey from bytes
pub fn verifying_key<G1Affine, G2Affine, E>(
    alpha_g1: G1Affine,
    beta_g1: G1Affine,
    beta_g2: G2Affine,
    gamma_g2: G2Affine,
    delta_g1: G1Affine,
    delta_g2: G2Affine,
    ic: Vec<G1Affine>,
) -> Result<PreparedVerifyingKey<E>, GroupDecodingError>
where
    G1Affine: AsRef<[u8]>,
    G2Affine: AsRef<[u8]>,
    E: Engine,
{
    let mut icv = vec![];
    for i in ic {
        icv.push(curve_affine::<E::G1Affine>(i.as_ref()).into_affine()?);
    }

    Ok(groth16::prepare_verifying_key::<E>(&VerifyingKey {
        alpha_g1: curve_affine::<E::G1Affine>(alpha_g1.as_ref()).into_affine()?,
        beta_g1: curve_affine::<E::G1Affine>(beta_g1.as_ref()).into_affine()?,
        beta_g2: curve_affine::<E::G2Affine>(beta_g2.as_ref()).into_affine()?,
        gamma_g2: curve_affine::<E::G2Affine>(gamma_g2.as_ref()).into_affine()?,
        delta_g1: curve_affine::<E::G1Affine>(delta_g1.as_ref()).into_affine()?,
        delta_g2: curve_affine::<E::G2Affine>(delta_g2.as_ref()).into_affine()?,
        ic: icv,
    }))
}

/// Parse proof
pub fn proof<G1Affine, G2Affine, E>(
    a: G1Affine,
    b: G2Affine,
    c: G1Affine,
) -> Result<Proof<E>, GroupDecodingError>
where
    G1Affine: AsRef<[u8]>,
    G2Affine: AsRef<[u8]>,
    E: Engine,
{
    Ok(Proof {
        a: curve_affine::<E::G1Affine>(a.as_ref()).into_affine()?,
        b: curve_affine::<E::G2Affine>(b.as_ref()).into_affine()?,
        c: curve_affine::<E::G1Affine>(c.as_ref()).into_affine()?,
    })
}

/// Parse Vec<Fr> from bytes
pub fn vector_fr<E: Engine + ScalarEngine>(
    input: &[u64],
) -> Result<Vec<E::Fr>, PrimeFieldDecodingError> {
    let ch = <E::Fr as PrimeField>::char();
    let elem = ch.as_ref().len();

    let len = input.len();
    if len % elem != 0 {
        return Err(PrimeFieldDecodingError::NotInField(format!(
            "Wrong length of Fr vector input: {:?}",
            input
        )));
    }

    let mut vfr: Vec<E::Fr> = vec![];
    for i in 0..len {
        let mut repr = <E::Fr as PrimeField>::char();
        repr.as_mut().copy_from_slice(&input[i..elem + i]);
        vfr.push(<E::Fr as PrimeField>::from_repr(repr)?);
    }
    Ok(vfr)
}
