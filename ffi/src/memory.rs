use crate::ByteBuffer;

#[no_mangle]
pub extern "C" fn free_byte_buffer(buffer: ByteBuffer) {
    if !buffer.data.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(buffer.data, buffer.len, buffer.len);
        }
    }
}
