use crate::prelude::*;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jint, jlong, jobject};
use jni::JNIEnv;
use merlin::Transcript;
use rand::thread_rng;
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_RangeProof_proveRange(
    env: JNIEnv,
    _jclass: JClass,
    secret: jlong,
    bound: jint,
) -> jobject {
    match prove(env, secret, bound) {
        Ok(res) => *res,
        Err(e) => {
            let _ = env.throw_new(BULLET_PROOF_EXCEPTION_CLASS, e.to_string());
            *JObject::null()
        }
    }
}

fn prove(env: JNIEnv, secret: jlong, bound: jint) -> Result<JObject> {
    let secret = secret.unsigned_abs();
    let bound = bound.unsigned_abs() as usize;

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let blinding = Scalar::random(&mut thread_rng());
    let mut prover_transcript = Transcript::new(TRANSSCRIPT_LABEL);
    let (proof, commitment) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        secret,
        &blinding,
        bound,
    )?;

    let proof = proof.to_bytes();
    let proof = new_object(env, PROOF_CLASS, &proof)?;
    let commit = new_object(env, COMMITMENT_CLASS, commitment.as_bytes())?;
    let blinding = new_object(env, BLINDING_CLASS, blinding.as_bytes())?;
    new_triple(env, proof, commit, blinding)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_RangeProof_verify(
    env: JNIEnv,
    _jclass: JClass,
    proof: jobject,
    commit: jobject,
    bound: jint,
) -> jboolean {
    match verify(env, proof, commit, bound) {
        Ok(res) => res,
        Err(e) => {
            let _ = env.throw_new(BULLET_PROOF_EXCEPTION_CLASS, e.to_string());
            0
        }
    }
}

fn verify(env: JNIEnv, proof: jobject, commit: jobject, bound: jint) -> Result<jboolean> {
    let proof = lookup_bytes(env, proof)?;
    let proof = RangeProof::from_bytes(&proof)?;

    let commit = lookup_bytes(env, commit)?;
    let commit = CompressedRistretto::from_slice(&commit);

    let bound = bound.unsigned_abs() as usize;

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);

    let mut verifier_transcript = Transcript::new(TRANSSCRIPT_LABEL);
    let check = proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &commit, bound);
    Ok(check.is_ok().into())
}
