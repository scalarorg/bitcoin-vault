#[macro_export]
macro_rules! hex_to_vec {
    ($hex:expr) => {{
        let hex_str = $hex.replace("0x", "").replace(" ", "");
        let mut vec = Vec::new();
        
        for i in (0..hex_str.len()).step_by(2) {
            if i + 2 > hex_str.len() {
                panic!("Invalid hex string length");
            }
            let byte_str = &hex_str[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16)
                .expect("Invalid hex string");
            vec.push(byte);
        }
        vec
    }};
}

// Optional: Add a test module
#[cfg(test)]
mod tests {
    #[test]
    fn test_hex_to_vec() {
        assert_eq!(hex_to_vec!("0123"), vec![0x01, 0x23]);
        assert_eq!(hex_to_vec!("0x0123"), vec![0x01, 0x23]);
        assert_eq!(hex_to_vec!("01 23"), vec![0x01, 0x23]);
    }
}

