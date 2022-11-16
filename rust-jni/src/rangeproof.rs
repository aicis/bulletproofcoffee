use std::error::Error;

use crate::prelude::*;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jlong, jobject, jint};
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
            env.throw_new(
                BULLET_PROOF_EXCEPTION_CLASS,
                e.to_string()
            )
            .unwrap();
            *JObject::null()
        }
    }
}

fn prove(env: JNIEnv, secret: jlong, bound: jint) -> Result<JObject, Box<dyn Error>> {
    let secret = secret.unsigned_abs();
    let bound = bound.unsigned_abs() as usize;

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let blinding = Scalar::random(&mut thread_rng());
    let mut prover_transcript = Transcript::new(TRANSSCRIPT_LABEL);
    let (proof, committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        secret,
        &blinding,
        bound,
    )?;

    let proof = proof.to_bytes();
    let proof =
        bytes_to_jobject(&env, PROOF_CLASS, &proof).expect("Failed constructing Proof object");
    let commit = committed_value.to_bytes();
    let commit = bytes_to_jobject(&env, COMMITMENT_CLASS, &commit)
        .expect("Failed constructing Commitment object");
    let pair = env
        .new_object(
            PAIR_CLASS,
            "(Ljava/lang/Object;Ljava/lang/Object;)V",
            &[proof.into(), commit.into()],
        )
        .expect("Failed to construct a Pair object");
    Ok(pair)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_RangeProof_verify(
    env: JNIEnv,
    _jclass: JClass,
    proof: jobject,
    commit: jobject,
    bound: jint
) -> jboolean {
    match verify(env, proof, commit, bound) {
        Ok(res) => res,
        Err(e) => {
            env.throw_new(BULLET_PROOF_EXCEPTION_CLASS, e.to_string())
                .unwrap();
            0
        }
    }
}

fn verify(env: JNIEnv, proof: jobject, commit: jobject, bound: jint) -> Result<jboolean, Box<dyn Error>> {
    let proof = jobject_as_bytes(env, proof)?;
    let proof = RangeProof::from_bytes(&proof)?;

    let commit = jobject_as_bytes(env, commit)?;
    let commit = CompressedRistretto::from_slice(&commit);

    let bound = bound.unsigned_abs() as usize;

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);

    let mut verifier_transcript = Transcript::new(TRANSSCRIPT_LABEL);
    let check = proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &commit, bound);
    Ok(check.is_ok().into())
}
