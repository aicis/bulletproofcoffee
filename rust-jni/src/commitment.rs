use std::error::Error;

use crate::prelude::*;

use bulletproofs::PedersenGens;
use curve25519_dalek::ristretto::CompressedRistretto;
use jni::sys::{jobject, jlong, jboolean};
use jni::objects::{JClass, JObject, JValue};

use curve25519_dalek::scalar::Scalar;
use jni::sys::jbyteArray;
use jni::JNIEnv;
use rand::thread_rng;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_Commitment_newCommitmentFromBytes(
    env: JNIEnv,
    _jclass: JClass,
    value: jbyteArray,
) -> jobject {
    let value = env.convert_byte_array(value).unwrap();
    let value : [u8; 32] = value.try_into().expect("should never fail as input always is 32 bytes");
    let value = Scalar::from_bytes_mod_order(value);
    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let commit = bytes_to_jobject(&env, COMMITMENT_CLASS, commit.as_bytes()).unwrap();
    *commit
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_Commitment_newCommitmentFromLong(
    env: JNIEnv,
    _jclass: JClass,
    value: jlong,
) -> jobject {
    let value : u64 = value as u64;
    let value = Scalar::from(value);
    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    *new_commitment(&env, commit, blinding).unwrap()
}


pub fn new_commitment<'a>(
    env: &'a JNIEnv,
    commitment: CompressedRistretto,
    blinding: Scalar,
) -> Result<JObject<'a>, Box<dyn Error>> {
    let commitment = env.byte_array_from_slice(commitment.as_bytes())?;
    let commitment = unsafe { JValue::Object(JObject::from_raw(commitment)) };
    let blinding = env.byte_array_from_slice(blinding.as_bytes())?;
    let blinding = unsafe { JValue::Object(JObject::from_raw(blinding)) };
    let object = env.new_object(COMMITMENT_CLASS, "([B[B)V", &[commitment, blinding])?;
    Ok(object)
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_Commitment_verify__J(
    env: JNIEnv,
    object : jobject,
    value : jbyteArray
) -> jboolean {
    let value = Scalar::from(value as u64);
    let object = JObject::from_raw(object);
    verify(&env, object, value).unwrap().into()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_Commitment_verify___3B(
    env: JNIEnv,
    object : jobject,
    value : jbyteArray
) -> jboolean {
    let value = env.convert_byte_array(value).unwrap();
    let value : [u8; 32] = value.try_into().expect("should never fail as input always is 32 bytes");
    let value = Scalar::from_bytes_mod_order(value);

    let object = JObject::from_raw(object);
    let Some(check) = verify(&env, object, value) else {
        let _ = env.throw_new(BULLET_PROOF_EXCEPTION_CLASS, "Invalid commitment, not a canonical risretto point");
        return 0;
    };
    check.into()
}

fn verify(env: &JNIEnv, object : JObject, value: Scalar) -> Option<bool> {
    let blinding = env.get_field(object, COMMITMENT_CLASS, "blinding").unwrap();
    let blinding : Vec<u8> = env.convert_byte_array(*blinding.l().unwrap()).unwrap();
    let blinding : [u8; 32] = blinding.try_into().expect("should never fail as input always is 32 bytes");
    let blinding = Scalar::from_bytes_mod_order(blinding);

    let commit = env.get_field(object, COMMITMENT_CLASS, "commitment").unwrap();
    let commit : Vec<u8> = env.convert_byte_array(*commit.l().unwrap()).unwrap();
    let commit : [u8; 32] = commit.try_into().unwrap();
    let commit = CompressedRistretto::from_slice(&commit);
    let commit = commit.decompress()?;

    let pc_gens = PedersenGens::default();
    let check = pc_gens.commit(value, blinding) == commit;
    Some(check)
}
