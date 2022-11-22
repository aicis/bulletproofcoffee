use crate::prelude::*;

use bulletproofs::PedersenGens;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};

use jni::objects::{JClass, JObject};
use jni::signature::ReturnType;
use jni::sys::{jboolean, jlong, jobject, jclass, jobjectArray, jvalue};

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
    let value: u64 = value as u64;
    let value = Scalar::from(value);
    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let Ok(commit) = new_object(env, RISTRETTO_POINT_CLASS, commit.as_bytes()) else {
        return *JObject::null();
    };
    let mut blinding = blinding.to_bytes();
    blinding.reverse(); // from little to big endian
    let Ok(blinding) = new_object(env, SCALAR_CLASS, &blinding) else {
        return *JObject::null();
    };
    *new_pair(env, commit, blinding).unwrap_or_else(|_| JObject::null())
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_commit___3B(
    env: JNIEnv,
    _jclass: JClass,
    value: jbyteArray,
) -> jobject {
    let value = env.convert_byte_array(value).unwrap();
    let mut value: [u8; 32] = value
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    value.reverse();
    let value = Scalar::from_bytes_mod_order(value);

    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let commit = new_object(env, RISTRETTO_POINT_CLASS, commit.as_bytes())
        .expect("failed constructing Commitment object");
    let mut blinding = blinding.to_bytes();
    blinding.reverse(); // from little to big endian
    let blinding =
        new_object(env, SCALAR_CLASS, &blinding).expect("failed constructing Scalar object");
    let pair = new_pair(env, commit, blinding);
    match pair {
        Ok(obj) => *obj,
        Err(Error::Java(e)) => {
            println!("error thing happened: {}", e);
            *JObject::null()
        }
        _ => panic!("yikes"),
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
    let mut value: [u8; 32] = value
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    value.reverse();
    let value = Scalar::from_bytes_mod_order(value);

    let blinding = env.convert_byte_array(blinding).unwrap();
    let mut blinding: [u8; 32] = blinding
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    blinding.reverse(); // from big endian to little
    let blinding = Scalar::from_bytes_mod_order(blinding);

    let pc_gens = PedersenGens::default();
    let commit = pc_gens.commit(value, blinding);
    let commit = commit.compress();
    let commit = new_object(env, RISTRETTO_POINT_CLASS, commit.as_bytes())
        .expect("failed constructing RistrettoPoint object");
    let mut blinding = blinding.to_bytes();
    blinding.reverse(); // from little to big endian
    let blinding =
        new_object(env, SCALAR_CLASS, &blinding).expect("failed constructing Scalar object");
    let pair = new_pair(env, commit, blinding);
    *unwrap_or_throw(&env, pair, JObject::null())
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_verify__Ldk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_2JLdk_alexandra_bulletproofcoffee_pedersen_Scalar_2(
    env: JNIEnv,
    _class : jclass,
    object: jobject,
    value: jbyteArray,
    blinding: jobject,
) -> jboolean {
    let value = Scalar::from(value as u64);
    let object = JObject::from_raw(object);
    let blinding = JObject::from_raw(blinding);
    verify(env, object, value, blinding)
        .unwrap_or_else(|| {
            let _ = env.throw_new(
                ILLEGAL_ARGUMENT_EXCEPTION_CLASS,
                "Non-canonical commitment or blinding",
            );
            false
        })
        .into()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Committer_verify__Ldk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_2_3BLdk_alexandra_bulletproofcoffee_pedersen_Scalar_2(
    env: JNIEnv,
    _class : jclass,
    object: jobject,
    value: jbyteArray,
    blinding: jobject,
) -> jboolean {
    let value = env.convert_byte_array(value).unwrap();
    let mut value: [u8; 32] = value
        .try_into()
        .expect("should never fail as input always is 32 bytes");
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

fn verify(env: JNIEnv, object: JObject, value: Scalar, blinding: JObject) -> Option<bool> {
    let commit = lookup_bytes_as_array(env, *object).unwrap();
    let commit = CompressedRistretto::from_slice(&commit);
    let commit = commit.decompress()?;

    let blinding = lookup_bytes(env, *blinding).unwrap();
    let mut blinding: [u8; 32] = blinding
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    blinding.reverse();
    let blinding = Scalar::from_canonical_bytes(blinding)?;

    let pc_gens = PedersenGens::default();
    let check = pc_gens.commit(value, blinding) == commit;
    Some(check)
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_mul(
    env: JNIEnv,
    this: jobject,  // RistrettoPoint
    other: jobject, // Scalar
) -> jobject {
    let this = lookup_bytes_as_array(env, this).unwrap();
    let other = lookup_bytes_as_array(env, other).unwrap();

    let Some(point) = CompressedRistretto::from_slice(&this).decompress() else {
        let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Non-canonical form");
        return *JObject::null();
    };
    let Some(scalar) = Scalar::from_canonical_bytes(other) else {
        let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Non-canonical form");
        return *JObject::null();
    };

    let new = (point * scalar).compress();
    *new_object(env, RISTRETTO_POINT_CLASS, new.as_bytes()).unwrap()
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_add(
    env: JNIEnv,
    this: jobject,  // RistrettoPoint
    other: jobject, // RistrettoPoint
) -> jobject {

    let this = lookup_bytes_as_array(env, this).unwrap();
    let other = lookup_bytes_as_array(env, other).unwrap();

    let this = CompressedRistretto::from_slice(&this).decompress();
    let other = CompressedRistretto::from_slice(&other).decompress();

    match (this, other) {
        (Some(a), Some(b)) => {
            let new = (a + b).compress();
            *new_object(env, RISTRETTO_POINT_CLASS, new.as_bytes()).unwrap()
        }
        _ => {
            let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Non-canonical form");
            *JObject::null()
        }
    }
}


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_sum(
    env: JNIEnv,
    _class: jclass,
    list: jobject, // List<RistrettoPoint>
) -> jobject {
    let list = JObject::from_raw(list);

    let size = env.call_method(list, "size()", "I", &[]).unwrap();
    let size = size.i().unwrap();


    let method = env.get_method_id("java/util/List", "get", "(I)Ljava/lang/Object").unwrap();
    let get = |i: i32| -> RistrettoPoint {
        let i = jvalue{i};
        let obj = env.call_method_unchecked(list, method, ReturnType::Object, &[i]).unwrap();
        let bytes = lookup_bytes_as_array(env, *obj.l().unwrap()).unwrap();
        let point = CompressedRistretto::from_slice(&bytes);
        point.decompress().unwrap()
    };

    if size == 1 {
        let obj = env.call_method_unchecked(list, method, ReturnType::Object, &[jvalue{i:0}]).unwrap();
        return *obj.l().unwrap();
    }

    let mut sum = get(0);

    for i in 1..size {
         sum += get(i);
    }
    let sum = sum.compress();
    *new_object(env, RISTRETTO_POINT_CLASS, sum.as_bytes()).unwrap()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Scalar_add(
    env: JNIEnv,
    this: jobject,  // Commitment
    other: jobject, // Commitment
) -> jobject {
    let mut this = lookup_bytes_as_array(env, this).unwrap();
    let mut other = lookup_bytes_as_array(env, other).unwrap();


    this.reverse();
    other.reverse();

    let this = Scalar::from_canonical_bytes(this);
    let other = Scalar::from_canonical_bytes(other);
    match (this, other) {
        (Some(a), Some(b)) => {
            let new = a + b;
            let mut new = new.to_bytes();
            new.reverse();

            *new_object(env, RISTRETTO_POINT_CLASS, &new).unwrap()
        }
        _ => {
            let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Non-canonical form");
            *JObject::null()
        }
    }
}


