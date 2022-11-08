#![warn(clippy::all)]
#![allow(clippy::missing_safety_doc)]

use curve25519_dalek::edwards::CompressedEdwardsY;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use jni::sys::{jlong, jobject, jboolean};
use jni::{JNIEnv, sys::jbyteArray};
use jni::objects::{JClass, JValue, JObject};
use merlin::Transcript;
use curve25519_dalek::scalar::Scalar;
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use rand::thread_rng;

fn wrap_to_object<'a>(env : JNIEnv<'a>, bytes : &[u8], class : &str) -> JObject<'a> {
    let commit = env.byte_array_from_slice(bytes).unwrap();
    let commit = unsafe {JValue::Object(JObject::from_raw(commit))};
    env.new_object(class, "", &[commit]).unwrap()
}


const TRANSSCRIPT_LABEL : &[u8] = b"";

const COMMITMENT_CLASS : &str = "dk/alexandra/bulletproofcoffee/Commitment";
const PROOF_CLASS : &str = "dk/alexandra/bulletproofcoffee/Proof";
const PAIR_CLASS : &str = "dk/alexandra/bulletproofcoffee/Pair";
const BULLET_PROOF_EXCEPTION_CLASS : &str = "dk/alexandra/bulletproofcoffee/BulletProofException";

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_FFI_prove(
    env : JNIEnv,
    _jclass : JClass,
    secret : jlong,
) -> jobject {
    let secret = secret.unsigned_abs();

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
        32,
    ).unwrap();

    // let _ = env.throw_new("dk.alexandra.bulletproofcoffee.BulletProofException", "Could not create proof.");

    let proof = proof.to_bytes();
    let proof = env.byte_array_from_slice(&proof).unwrap();

    let commit = committed_value.to_bytes();
    let commit = env.byte_array_from_slice(&commit).unwrap();

    let commit = JValue::Object(JObject::from_raw(commit));
    let commit = env.new_object(COMMITMENT_CLASS, "([B)V", &[commit]).unwrap();
    let proof = JValue::Object(JObject::from_raw(proof));
    let proof = env.new_object(PROOF_CLASS, "([B)V", &[proof]).unwrap();
    *env.new_object(PAIR_CLASS, "(Ljava/lang/Object;Ljava/lang/Object;)V", &[proof.into(), commit.into()]).unwrap()
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_FFI_verify(
    env : JNIEnv,
    _jclass : JClass,
    proof : jobject,
    commit : jobject,
) -> jboolean {
    let proof = JObject::from_raw(proof);
    let proof = env.call_method(proof, "asBytes", "()[B", &[]).unwrap();
    let proof = env.convert_byte_array(*proof.l().unwrap()).unwrap();
    let proof = RangeProof::from_bytes(&proof).unwrap();

    let commit = JObject::from_raw(commit);
    let commit = env.call_method(commit, "asBytes", "()[B", &[]).unwrap();
    let commit = env.convert_byte_array(*commit.l().unwrap()).unwrap();
    let commit = CompressedRistretto::from_slice(&commit);

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);

    let mut verifier_transcript = Transcript::new(TRANSSCRIPT_LABEL);
    proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &commit, 32).is_ok().into()
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
    ).expect("A real program could handle errors");


    proof.to_bytes();

    // Verification requires a transcript with identical initial state:
    let mut verifier_transcript = Transcript::new(b"doctest example");
    assert!(
    proof
        .verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 32)
        .is_ok()
    );
}
