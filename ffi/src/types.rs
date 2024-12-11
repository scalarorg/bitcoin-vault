#[repr(C)]
pub struct ByteBuffer {
    pub data: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct TapScriptSigFFI {
    pub key_x_only: [u8; 32],
    pub leaf_hash: [u8; 32],
    pub signature: [u8; 64],
}

#[repr(C)]
pub struct TapScriptSigFFIArray {
    pub data: *mut TapScriptSigFFI,
    pub len: usize,
}
