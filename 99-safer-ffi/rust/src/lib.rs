use ::safer_ffi::prelude::*;

#[derive_ReprC]
#[repr(C)]
pub struct MyCallback {
    cb: fn(),
}

#[ffi_export]
fn call(it: MyCallback) {
    (it.cb)()
}

/*

Ref. https://www.ditto.live/blog/introducing-safer-ffi

#[require_unsafe_in_body]
#[no_mangle]
pub unsafe extern "C" fn ditto_document_cbor(
    document: *const Document,
    out_cbor_len: *mut usize,
) -> *const c_uchar {
    let value = unsafe { &*document }.to_value();
    let cbor_bytes: Box<[u8]> = ::serde_cbor::to_vec(&value).unwrap().into_boxed_slice();
    unsafe {
        *out_cbor_len = cbor_bytes.len();
    }
    Box::into_raw(cbor_bytes) as *const _
}

// Bring types with a C layout (such as `c_slice::Box`) in scope
use ::safer_ffi::prelude::*;

#[ffi_export]
pub fn ditto_document_cbor(document: &'_ Document) -> c_slice::Box<u8> {
    let value = document.to_value();
    let cbor_bytes: Box<[u8]> = ::serde_cbor::to_vec(&value).unwrap().into_boxed_slice();
    cbor_bytes.into()
}

*/
