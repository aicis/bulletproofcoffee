use crate::prelude::*;

use bulletproofs::PedersenGens;
use curve25519_dalek::ristretto::CompressedRistretto;

use jni::objects::JObject;

use jni::sys::{jboolean, jlong, jobject};

use curve25519_dalek::scalar::Scalar;
use jni::sys::jbyteArray;
use jni::JNIEnv;
use rand::thread_rng;

fn get_pc_gens(env: JNIEnv, obj: JObject) -> PedersenGens {
    let use_default = env.get_field(obj, "useDefault", "Z").unwrap().z().unwrap();
    if use_default {
        PedersenGens::default()
    } else {
        println!("Using something new");
        let field_type = "L".to_owned() + RISTRETTO_POINT_CLASS + ";";
        let basepoint = env
            .get_field(obj, "basePoint", &field_type)
            .unwrap()
            .l()
            .unwrap();
        let blinding_basepoint = env
            .get_field(obj, "basePoint", &field_type)
            .unwrap()
            .l()
            .unwrap();
        let basepoint = lookup_bytes_as_array(env, *basepoint).unwrap();
        let blinding_basepoint = lookup_bytes_as_array(env, *blinding_basepoint).unwrap();

        let Some(basepoint) = CompressedRistretto::from_slice(&basepoint).decompress() else {
            let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Basepoint malformed");
            return PedersenGens::default();
        };
        let Some(blinding_basepoint) = CompressedRistretto::from_slice(&blinding_basepoint).decompress() else {
            let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Blinding basepoint malformed");
            return PedersenGens::default();
        };

        PedersenGens {
            B: basepoint,
            B_blinding: blinding_basepoint,
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Generator_commit__J(
    env: JNIEnv,
    this: jobject,
    value: jlong,
) -> jobject {
    let value: u64 = value as u64;
    let value = Scalar::from(value);
    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = get_pc_gens(env, JObject::from_raw(this));
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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Generator_commit___3B(
    env: JNIEnv,
    this: jobject,
    value: jbyteArray,
) -> jobject {
    let value = env.convert_byte_array(value).unwrap();
    let mut value: [u8; 32] = value
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    value.reverse();
    let value = Scalar::from_bytes_mod_order(value);

    let blinding = Scalar::random(&mut thread_rng());
    let pc_gens = get_pc_gens(env, JObject::from_raw(this));
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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Generator_commit___3B_3B(
    env: JNIEnv,
    this: jobject,
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

    let pc_gens = get_pc_gens(env, JObject::from_raw(this));
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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Generator_verify__Ldk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_2JLdk_alexandra_bulletproofcoffee_pedersen_Scalar_2(
    env: JNIEnv,
    this: jobject,
    object: jobject,
    value: jbyteArray,
    blinding: jobject,
) -> jboolean {
    let value = Scalar::from(value as u64);
    let object = JObject::from_raw(object);
    let blinding = JObject::from_raw(blinding);
    let this = JObject::from_raw(this);
    verify(env, this, object, value, blinding)
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
pub unsafe extern "system" fn Java_dk_alexandra_bulletproofcoffee_pedersen_Generator_verify__Ldk_alexandra_bulletproofcoffee_pedersen_RistrettoPoint_2_3BLdk_alexandra_bulletproofcoffee_pedersen_Scalar_2(
    env: JNIEnv,
    this: jobject,
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
    let this = JObject::from_raw(this);
    let Some(check) = verify(env, this, object, value, blinding) else {
        let _ = env.throw_new(ILLEGAL_ARGUMENT_EXCEPTION_CLASS, "Invalid commitment, not a canonical risretto point");
        return 0;
    };
    check.into()
}

fn verify(
    env: JNIEnv,
    this: JObject,
    object: JObject,
    value: Scalar,
    blinding: JObject,
) -> Option<bool> {
    let commit = lookup_bytes_as_array(env, *object).unwrap();
    let commit = CompressedRistretto::from_slice(&commit);
    let commit = commit.decompress()?;

    let blinding = lookup_bytes(env, *blinding).unwrap();
    let mut blinding: [u8; 32] = blinding
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    blinding.reverse();
    let blinding = Scalar::from_canonical_bytes(blinding)?;

    let pc_gens = get_pc_gens(env, this);
    let check = pc_gens.commit(value, blinding) == commit;
    Some(check)
}
