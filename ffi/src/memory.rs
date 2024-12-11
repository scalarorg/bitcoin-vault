use crate::{ByteBuffer, TapScriptSigFFIArray};

#[no_mangle]
pub extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(buffer.data, buffer.len, buffer.len);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_tap_script_sig_array(array: TapScriptSigFFIArray) {
    if !array.data.is_null() {
        let _ = Vec::from_raw_parts(array.data, array.len, array.len);
    }
}
