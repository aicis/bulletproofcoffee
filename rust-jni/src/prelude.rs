
use jni::objects::{JObject};
use jni::sys::jobject;
use jni::JNIEnv;
use thiserror::Error;



pub const TRANSSCRIPT_LABEL: &[u8] = b"";

pub const COMMITMENT_CLASS: &str = "dk/alexandra/bulletproofcoffee/Commitment";
pub const PROOF_CLASS: &str = "dk/alexandra/bulletproofcoffee/Proof";
pub const PAIR_CLASS: &str = "dk/alexandra/bulletproofcoffee/Pair";
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
pub fn bytes_to_jobject<'a>(
    env: JNIEnv<'a>,
    class: &str,
    bytes: &'a [u8],
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

