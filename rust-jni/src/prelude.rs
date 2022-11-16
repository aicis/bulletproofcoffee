use std::error::Error;




use jni::objects::{JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;


pub const TRANSSCRIPT_LABEL: &[u8] = b"";

pub const COMMITMENT_CLASS: &str = "dk/alexandra/bulletproofcoffee/Commitment";
pub const PROOF_CLASS: &str = "dk/alexandra/bulletproofcoffee/Proof";
pub const PAIR_CLASS: &str = "dk/alexandra/bulletproofcoffee/Pair";
pub const BULLET_PROOF_EXCEPTION_CLASS: &str = "dk/alexandra/bulletproofcoffee/BulletProofException";


/// Construct a JObject from a byte slice
///
/// * `env`: JNIEnv
/// * `class`: The type class to construct, with a constructor accepting a byte array
/// * `bytes`: byte array
pub fn bytes_to_jobject<'a>(
    env: &'a JNIEnv,
    class: &str,
    bytes: &'a [u8],
) -> Result<JObject<'a>, Box<dyn Error>> {
    let object = env.byte_array_from_slice(bytes)?;
    let object = unsafe { JValue::Object(JObject::from_raw(object)) };
    let object = env.new_object(class, "([B)V", &[object])?;
    Ok(object)
}

/// Jobject to bytes
///
/// * `env`: JNIEnv
/// * `object`: Object for which to extract bytes, requires an `asBytes` method returning a byte
/// array.
pub fn jobject_as_bytes(env: JNIEnv, method: &str, object: jobject) -> Result<Vec<u8>, Box<dyn Error>> {
    let object = unsafe { JObject::from_raw(object) };
    let bytes = env.call_method(object, method, "()[B", &[])?;
    let bytes = env.convert_byte_array(*bytes.l()?)?;
    Ok(bytes)
}

