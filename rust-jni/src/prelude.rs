use jni::objects::{JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use thiserror::Error;



pub const TRANSSCRIPT_LABEL: &[u8] = b"";

pub const COMMITMENT_CLASS: &str = "dk/alexandra/bulletproofcoffee/pedersen/Commitment";
pub const BLINDING_CLASS: &str = "dk/alexandra/bulletproofcoffee/pedersen/Blinding";
pub const PROOF_CLASS: &str = "dk/alexandra/bulletproofcoffee/Proof";
pub const PAIR_CLASS: &str = "dk/alexandra/bulletproofcoffee/Pair";
pub const TRIPLE_CLASS: &str = "dk/alexandra/bulletproofcoffee/Triple";
pub const BULLET_PROOF_EXCEPTION_CLASS: &str = "dk/alexandra/bulletproofcoffee/BulletProofException";

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
pub fn new_object<'a>(
    env: JNIEnv<'a>,
    class: &str,
    bytes: &[u8],
) -> Result<JObject<'a>> {
    let object = env.byte_array_from_slice(bytes)?;
    let object = unsafe {JObject::from_raw(object) };
    let object = env.new_object(class, "([B)V", &[object.into()])?;
    Ok(object)
}

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires an `asBytes` method returning a byte
/// array.
pub fn jobject_as_bytes(env: JNIEnv, method: &str, object: jobject) -> Result<Vec<u8>> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.call_method(object, method, "()[B", &[])?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    Ok(bytes)
}

/// Construct a pair
///
/// * `env`: JNIEnv
/// * `fst` first object for the pair
/// * `snd` second object for the pair
pub fn new_pair<'a>(env: JNIEnv<'a>, fst: JObject<'a>, snd: JObject<'a>) -> Result<JObject<'a>> {
    let pair = env
        .new_object(
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
pub fn new_triple<'a>(env: JNIEnv<'a>, fst: JObject<'a>, snd: JObject<'a>, trd: JObject<'a>) -> Result<JObject<'a>> {
    let pair = env
        .new_object(
            TRIPLE_CLASS,
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
            &[JValue::from(fst), JValue::from(snd), JValue::from(trd)],
        )?;
    Ok(pair)
}
