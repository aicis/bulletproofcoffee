#![warn(clippy::all)]
#![allow(clippy::missing_safety_doc)]

use std::error::Error;

use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jboolean, jlong, jobject, jint};
use jni::JNIEnv;
use merlin::Transcript;
use rand::thread_rng;

const TRANSSCRIPT_LABEL: &[u8] = b"";

const COMMITMENT_CLASS: &str = "dk/alexandra/bulletproofcoffee/Commitment";
const PROOF_CLASS: &str = "dk/alexandra/bulletproofcoffee/Proof";
const PAIR_CLASS: &str = "dk/alexandra/bulletproofcoffee/Pair";
const BULLET_PROOF_EXCEPTION_CLASS: &str = "dk/alexandra/bulletproofcoffee/BulletProofException";

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_FFI_proveRange(
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

/// Construct a JObject from a byte slice
///
/// * `env`: JNIEnv
/// * `class`: The type class to construct, with a constructor accepting a byte array
/// * `bytes`: byte array
fn bytes_to_jobject<'a>(
    env: &'a JNIEnv,
    class: &str,
    bytes: &'a [u8],
) -> Result<JObject<'a>, Box<dyn Error>> {
    let object = env.byte_array_from_slice(bytes)?;
    let object = unsafe { JValue::Object(JObject::from_raw(object)) };
    let object = env.new_object(class, "([B)V", &[object])?;
    Ok(object)
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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_FFI_verify(
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

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires an `asBytes` method returning a byte
/// array.
fn jobject_as_bytes(env: JNIEnv, object: jobject) -> Result<Vec<u8>, Box<dyn Error>> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.call_method(object, "asBytes", "()[B", &[])?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    Ok(bytes)
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

#[test]
fn thing() {
    let pc_gens = PedersenGens::default();

    let bp_gens = BulletproofGens::new(64, 1);

    // A secret value we want to prove lies in the range [0, 2^32)
    let secret_value = 1037578891u64;

    // The API takes a blinding factor for the commitment.
    let blinding = Scalar::random(&mut thread_rng());

    // The proof can be chained to an existing transcript.
    // Here we create a transcript with a doctest domain separator.
    let mut prover_transcript = Transcript::new(b"doctest example");

    // Create a 32-bit rangeproof.
    let (proof, committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        secret_value,
        &blinding,
        32,
    )
    .expect("A real program could handle errors");

    proof.to_bytes();

    // Verification requires a transcript with identical initial state:
    let mut verifier_transcript = Transcript::new(b"doctest example");
    assert!(proof
        .verify_single(
            &bp_gens,
            &pc_gens,
            &mut verifier_transcript,
            &committed_value,
            32
        )
        .is_ok());
}
