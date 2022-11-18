

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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_commit__J(
    env: JNIEnv,
    _jclass: JClass,
    value: jlong,
) -> jobject {
    let value : u64 = value as u64;
    let value = Scalar::from(value);
    println!("Rust Value: {:?}", value);
    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let commit = new_object(env, COMMITMENT_CLASS, commit.as_bytes()).expect("failed constructing Commitment object");
    let mut blinding = blinding.to_bytes();
    blinding.reverse(); // from little to big endian
    let blinding = new_object(env, BLINDING_CLASS, &blinding).expect("failed constructing Blinding object");
    *new_pair(env, commit, blinding).expect("failed to construct Commitment object")
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_commit___3B(
    env: JNIEnv,
    _jclass: JClass,
    value: jbyteArray,
) -> jobject {
    let value = env.convert_byte_array(value).unwrap();
    let mut value : [u8; 32] = value.try_into().expect("should never fail as input always is 32 bytes");
    value.reverse();
    let value = Scalar::from_bytes_mod_order(value);
    println!("Rust Value: {:?}", value);

    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let commit = new_object(env, COMMITMENT_CLASS, commit.as_bytes()).expect("failed constructing Commitment object");
    let mut blinding = blinding.to_bytes();
    blinding.reverse(); // from little to big endian
    let blinding = new_object(env, BLINDING_CLASS, &blinding).expect("failed constructing Blinding object");
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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_commit___3B_3B(
    env: JNIEnv,
    _jclass: JClass,
    value: jbyteArray,
    blinding: jbyteArray,
) -> jobject {
    let value = env.convert_byte_array(value).unwrap();
    let mut value : [u8; 32] = value.try_into().expect("should never fail as input always is 32 bytes");
    value.reverse();
    let value = Scalar::from_bytes_mod_order(value);

    let blinding = env.convert_byte_array(blinding).unwrap();
    let mut blinding : [u8; 32] = blinding.try_into().expect("should never fail as input always is 32 bytes");
    blinding.reverse(); // from big endian to little
    let blinding = Scalar::from_bytes_mod_order(blinding);
    println!("Commit Value: {:?}", value);

    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let commit = new_object(env, COMMITMENT_CLASS, commit.as_bytes()).expect("failed constructing Commitment object");
    let mut blinding = blinding.to_bytes();
    blinding.reverse(); // from little to big endian
    let blinding = new_object(env, BLINDING_CLASS, &blinding).expect("failed constructing Blinding object");
    let pair = new_pair(env, commit, blinding);
    match pair {
        Ok(obj) => *obj,
        _ => *JObject::null()
    }
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
    verify(env, object, value, blinding).unwrap_or_else(|| {
        let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Non-canonical commitment or blinding");
        false
    }).into()
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
    let mut value : [u8; 32] = value.try_into().expect("should never fail as input always is 32 bytes");
    value.reverse();
    let value = Scalar::from_bytes_mod_order(value);
    let blinding = JObject::from_raw(blinding);
    let object = JObject::from_raw(object);
    let Some(check) = verify(env, object, value, blinding) else {
        let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Invalid commitment, not a canonical risretto point");
        return 0;
    };
    check.into()
}

fn verify(env: JNIEnv, object : JObject, value: Scalar, blinding: JObject) -> Option<bool> {

    let commit = lookup_bytes_as_array(env, *object).unwrap();
    // let commit = env.get_field(object, COMMITMENT_CLASS, "commitment").unwrap();
    // let commit : Vec<u8> = env.convert_byte_array(*commit.l().unwrap()).unwrap();
    let commit = CompressedRistretto::from_slice(&commit);
    let commit = commit.decompress()?;

    // let blinding = env.get_field(object, COMMITMENT_CLASS, "blinding").unwrap();
    // let blinding : Vec<u8> = env.convert_byte_array(*blinding.l().unwrap()).unwrap();
    let blinding = lookup_bytes(env, *blinding).unwrap();
    let mut blinding : [u8; 32] = blinding.try_into().expect("should never fail as input always is 32 bytes");
    blinding.reverse();
    let blinding = Scalar::from_canonical_bytes(blinding)?;
    println!("Verify Value: {:?}", value);

    let pc_gens = PedersenGens::default();
    let check = pc_gens.commit(value, blinding) == commit;
    Some(check)
}



#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Commitment_add(
    env: JNIEnv,
    this : jobject, // Commitment
    other : jobject, // Commitment
) -> jobject {
    let thing = env.get_field(JObject::from_raw(this), "bytes","[B").unwrap();
    let bytes = env.convert_byte_array(*thing.l().unwrap()).unwrap();
    let this : [u8; 32] = bytes.try_into().expect("should never fail as input always is 32 bytes");

    let thing = env.get_field(JObject::from_raw(other), "bytes","[B").unwrap();
    let bytes = env.convert_byte_array(*thing.l().unwrap()).unwrap();
    let other : [u8; 32] = bytes.try_into().expect("should never fail as input always is 32 bytes");

    let this = CompressedRistretto::from_slice(&this).decompress().unwrap();
    let other = CompressedRistretto::from_slice(&other).decompress().unwrap();

    let new = (this + other).compress();
    *new_object(env, COMMITMENT_CLASS, new.as_bytes()).unwrap()
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Blinding_add(
    env: JNIEnv,
    this : jobject, // Commitment
    other : jobject, // Commitment
) -> jobject {
    let thing = env.get_field(JObject::from_raw(this), "bytes","[B").unwrap();
    let bytes = env.convert_byte_array(*thing.l().unwrap()).unwrap();
    let mut this : [u8; 32] = bytes.try_into().expect("should never fail as input always is 32 bytes");

    let thing = env.get_field(JObject::from_raw(other), "bytes","[B").unwrap();
    let bytes = env.convert_byte_array(*thing.l().unwrap()).unwrap();
    let mut other : [u8; 32] = bytes.try_into().expect("should never fail as input always is 32 bytes");

    this.reverse();
    other.reverse();

    let this = Scalar::from_canonical_bytes(this).expect("Scalar should be canonical");
    let other = Scalar::from_canonical_bytes(other).expect("Scalar should be canoncial");

    let new = this + other;
    let mut new = new.to_bytes();
    new.reverse();

    *new_object(env, COMMITMENT_CLASS, &new).unwrap()
}
