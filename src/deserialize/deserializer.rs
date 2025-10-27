// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::deserialize::utf8::read_input_to_buf;
use crate::deserialize::DeserializeError;
#[cfg(not(feature = "yyjson"))]
use crate::ffi::PyBytes_AS_STRING;
#[cfg(not(feature = "yyjson"))]
use crate::ffi::PyBytes_GET_SIZE;
#[cfg(not(feature = "yyjson"))]
use crate::isize_to_usize;
use crate::opt::{Opt, CBOR};
use crate::typeref::EMPTY_UNICODE;
use core::ptr::NonNull;

pub(crate) fn deserialize(
    ptr: *mut pyo3_ffi::PyObject,
    opts: Option<Opt>,
) -> Result<NonNull<pyo3_ffi::PyObject>, DeserializeError<'static>> {
    debug_assert!(ffi!(Py_REFCNT(ptr)) >= 1);
    if opt_enabled!(opts.unwrap_or(CBOR), 0) {
        #[cfg(not(feature = "yyjson"))]
        {
            let buffer = unsafe {
                core::slice::from_raw_parts(
                    PyBytes_AS_STRING(ptr).cast::<u8>(),
                    isize_to_usize(PyBytes_GET_SIZE(ptr)),
                )
            };
            // Pass the full buffer slice to the CBOR deserializer
            crate::deserialize::backend::deserialize_cbor(buffer)
        }
        #[cfg(feature = "yyjson")]
        {
            let buffer = crate::deserialize::utf8::read_input_to_buf(ptr)?;

            // Since we don't have deserialize_cbor with yyjson feature,
            // we'll fall back to the regular deserialize
            let buffer_str = unsafe { core::str::from_utf8_unchecked(buffer) };
            crate::deserialize::backend::deserialize(buffer_str)
        }
    } else {
        let buffer = read_input_to_buf(ptr)?;
        debug_assert!(!buffer.is_empty());

        if unlikely!(buffer.len() == 2) {
            if buffer == b"[]" {
                return Ok(nonnull!(ffi!(PyList_New(0))));
            } else if buffer == b"{}" {
                return Ok(nonnull!(ffi!(PyDict_New())));
            } else if buffer == b"\"\"" {
                unsafe { return Ok(nonnull!(use_immortal!(EMPTY_UNICODE))) }
            }
        }
        let buffer_str = unsafe { core::str::from_utf8_unchecked(buffer) };

        crate::deserialize::backend::deserialize(buffer_str)
    }
}
