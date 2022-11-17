

use crate::prelude::*;

use bulletproofs::PedersenGens;
use curve25519_dalek::ristretto::CompressedRistretto;

use jni::sys::{jobject, jlong, jboolean};
use jni::objects::{JClass, JObject};

use curve25519_dalek::scalar::Scalar;
use jni::sys::jbyteArray;
use jni::JNIEnv;
use rand::thread_rng;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_commit___3B(
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
    let commit = new_object(env, COMMITMENT_CLASS, commit.as_bytes()).expect("failed constructing Commitment object");
    let blinding = new_object(env, BLINDING_CLASS, blinding.as_bytes()).expect("failed constructing Blinding object");
    let pair = new_pair(env, commit, blinding);
    match pair {
        Ok(obj) => *obj,
        Err(Error::Java(e)) => {
            println!("error thing happened: {}", e);
            *JObject::null()
        },
        _ => panic!("yikes")
    }
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_commit__J(
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
    let commit = new_object(env, COMMITMENT_CLASS, commit.as_bytes()).expect("failed constructing Commitment object");
    let blinding = new_object(env, BLINDING_CLASS, blinding.as_bytes()).expect("failed constructing Blinding object");
    *new_pair(env, commit, blinding).expect("failed to construct Commitment object")
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Commitment_verify__JLdk_alexandra_bulletproofcoffee_pedersen_Blinding_2(
    env: JNIEnv,
    object : jobject,
    value : jbyteArray,
    blinding : jobject,
) -> jboolean {
    let value = Scalar::from(value as u64);
    let object = JObject::from_raw(object);
    let blinding = JObject::from_raw(blinding);
    verify(env, object, value, blinding).unwrap().into()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Commitment_verify___3BLdk_alexandra_bulletproofcoffee_pedersen_Blinding_2(
    env: JNIEnv,
    object : jobject,
    value : jbyteArray,
    blinding : jobject,
) -> jboolean {
    let value = env.convert_byte_array(value).unwrap();
    let value : [u8; 32] = value.try_into().expect("should never fail as input always is 32 bytes");
    let value = Scalar::from_bytes_mod_order(value);
    let blinding = JObject::from_raw(blinding);
    let object = JObject::from_raw(object);
    let Some(check) = verify(env, object, value, blinding) else {
        let _ = env.throw_new(BULLET_PROOF_EXCEPTION_CLASS, "Invalid commitment, not a canonical risretto point");
        return 0;
    };
    check.into()
}

fn verify(env: JNIEnv, object : JObject, value: Scalar, blinding: JObject) -> Option<bool> {

    let commit = jobject_as_bytes(env, "asBytes", *object).unwrap();
    // let commit = env.get_field(object, COMMITMENT_CLASS, "commitment").unwrap();
    // let commit : Vec<u8> = env.convert_byte_array(*commit.l().unwrap()).unwrap();
    let commit : [u8; 32] = commit.try_into().unwrap();
    let commit = CompressedRistretto::from_slice(&commit);
    let commit = commit.decompress()?;

    // let blinding = env.get_field(object, COMMITMENT_CLASS, "blinding").unwrap();
    // let blinding : Vec<u8> = env.convert_byte_array(*blinding.l().unwrap()).unwrap();
    let blinding = jobject_as_bytes(env, "bytes", *blinding).unwrap();
    let blinding : [u8; 32] = blinding.try_into().expect("should never fail as input always is 32 bytes");
    let blinding = Scalar::from_bytes_mod_order(blinding);

    let pc_gens = PedersenGens::default();
    let check = pc_gens.commit(value, blinding) == commit;
    Some(check)
}
