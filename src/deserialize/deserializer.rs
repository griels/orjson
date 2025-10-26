// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::deserialize::utf8::read_input_to_buf;
use crate::deserialize::DeserializeError;
use crate::opt::{Opt, CBOR};
use crate::typeref::EMPTY_UNICODE;
use core::ptr::NonNull;

pub(crate) fn deserialize(
    ptr: *mut pyo3_ffi::PyObject,
    opts: Option<Opt>,
) -> Result<NonNull<pyo3_ffi::PyObject>, DeserializeError<'static>> {
    debug_assert!(ffi!(Py_REFCNT(ptr)) >= 1);
    let buffer = read_input_to_buf(ptr)?;
    debug_assert!(!buffer.is_empty());
    if  opt_enabled!(opts.unwrap_or(CBOR), 0){
        #[cfg(not(feature = "yyjson"))]
        {
            crate::deserialize::backend::deserialize_cbor(str::from_utf8(buffer).unwrap())
        }
        #[cfg(feature = "yyjson")]
        {
            // Since we don't have deserialize_cbor with yyjson feature,
            // we'll fall back to the regular deserialize
            let buffer_str = unsafe { core::str::from_utf8_unchecked(buffer) };
            crate::deserialize::backend::deserialize(buffer_str)
        }
    }
    else {
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
