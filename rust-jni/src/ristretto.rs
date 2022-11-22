use crate::prelude::*;


use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};

use jni::objects::JObject;
use jni::signature::ReturnType;
use jni::sys::{jclass, jobject, jvalue};

use curve25519_dalek::scalar::Scalar;

use jni::JNIEnv;


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

    let method = env
        .get_method_id("java/util/List", "get", "(I)Ljava/lang/Object")
        .unwrap();
    let get = |i: i32| -> RistrettoPoint {
        let i = jvalue { i };
        let obj = env
            .call_method_unchecked(list, method, ReturnType::Object, &[i])
            .unwrap();
        let bytes = lookup_bytes_as_array(env, *obj.l().unwrap()).unwrap();
        let point = CompressedRistretto::from_slice(&bytes);
        point.decompress().unwrap()
    };

    if size == 1 {
        let obj = env
            .call_method_unchecked(list, method, ReturnType::Object, &[jvalue { i: 0 }])
            .unwrap();
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
