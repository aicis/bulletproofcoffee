use jni::objects::{JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use thiserror::Error;

pub const TRANSSCRIPT_LABEL: &[u8] = b"";

pub const RISTRETTO_POINT_CLASS: &str = "dk/alexandra/bulletproofcoffee/pedersen/RistrettoPoint";
pub const SCALAR_CLASS: &str = "dk/alexandra/bulletproofcoffee/pedersen/Scalar";
pub const PROOF_CLASS: &str = "dk/alexandra/bulletproofcoffee/Proof";
pub const PAIR_CLASS: &str = "dk/alexandra/bulletproofcoffee/Pair";
pub const TRIPLE_CLASS: &str = "dk/alexandra/bulletproofcoffee/Triple";
pub const BULLET_PROOF_EXCEPTION_CLASS: &str =
    "dk/alexandra/bulletproofcoffee/BulletProofException";
pub const ILLEGAL_ARGUMENT_EXCEPTION_CLASS: &str = "java/lang/IllegalArgumentException";
pub const RUNTIME_EXCEPTION_CLASS: &str = "java/lang/RuntimeException";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Bulletproof failed: {0}")]
    Bulletproof(#[from] bulletproofs::ProofError),
    #[error("Failed java thing: {0}")]
    Java(#[from] jni::errors::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Construct a JObject from a byte slice
///
/// * `env`: JNIEnv
/// * `class`: The type class to construct, with a constructor accepting a byte array
/// * `bytes`: byte array
pub fn new_object<'a>(env: JNIEnv<'a>, class: &str, bytes: &[u8]) -> Result<JObject<'a>> {
    let object = env.byte_array_from_slice(bytes)?;
    let object = unsafe { JObject::from_raw(object) };
    let object = env.new_object(class, "([B)V", &[object.into()])?;
    Ok(object)
}

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires a `bytes` field with type byte[]
pub fn lookup_bytes(env: JNIEnv, object: jobject) -> Result<Vec<u8>> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.get_field(object, "bytes", "[B")?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    Ok(bytes)
}

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires an `asBytes` method returning a byte
/// array.
pub fn lookup(env: JNIEnv, field: &str, object: jobject) -> Result<Vec<u8>> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.get_field(object, field, "[B")?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    Ok(bytes)
}

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires an `asBytes` method returning a byte
/// array.
pub fn lookup_bytes_as_array(env: JNIEnv, object: jobject) -> Result<[u8; 32]> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.get_field(object, "bytes", "[B")?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    let bytes: [u8; 32] = bytes
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    Ok(bytes)
}

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires an `asBytes` method returning a byte
/// array.
pub fn lookup_as_array(env: JNIEnv, field: &str, object: jobject) -> Result<[u8; 32]> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.get_field(object, field, "[B")?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    let bytes: [u8; 32] = bytes
        .try_into()
        .expect("should never fail as input always is 32 bytes");
    Ok(bytes)
}

/// Construct a pair
///
/// * `env`: JNIEnv
/// * `fst` first object for the pair
/// * `snd` second object for the pair
pub fn new_pair<'a>(env: JNIEnv<'a>, fst: JObject<'a>, snd: JObject<'a>) -> Result<JObject<'a>> {
    let pair = env.new_object(
        PAIR_CLASS,
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::from(fst), JValue::from(snd)],
    )?;
    Ok(pair)
}

/// Construct a triple
///
/// * `env`: JNIEnv
/// * `fst` first object for the pair
/// * `snd` second object for the pair
pub fn new_triple<'a>(
    env: JNIEnv<'a>,
    fst: JObject<'a>,
    snd: JObject<'a>,
    trd: JObject<'a>,
) -> Result<JObject<'a>> {
    let pair = env.new_object(
        TRIPLE_CLASS,
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::from(fst), JValue::from(snd), JValue::from(trd)],
    )?;
    Ok(pair)
}

pub fn unwrap_or_throw<T>(env: &JNIEnv, res: Result<T>, error_val : T) -> T {
    match res {
        Ok(res) => res,
        Err(e) => {
            let _ = env.throw_new(RUNTIME_EXCEPTION_CLASS, e.to_string());
            error_val
        }
    }
}
