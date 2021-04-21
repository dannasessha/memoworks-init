fn package_memo(s: String) -> String {
    s
}

#[cfg(test)]
mod unit {
    use super::*;
    #[test]
    fn freedom() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn package_memo_sanity() {
        let test_message = "is sanity statistical?".to_string();
        let expected = test_message.clone().to_string();
        let result = package_memo(test_message);
        assert_eq!(result, expected);
    }

    #[test]
    fn package_memo_breadcrumbs1() {
        let test_message =
            "If the first byte (byte 0) has a value of 0xF4 or smaller, then the reader MUST:
        strip any trailing zero bytes
        decode it as a UTF-8 string (if decoding fails then report an error)."
                .to_string();
        println!("{}", &test_message);
        let expected = test_message.clone().to_string();
        let result = package_memo(test_message);
        assert_eq!(result, expected);
    }

    #[test]
    fn package_memo_breadcrumbs2() {
        let test_message = "If the first byte has a value of 0xF6, and the remaining 511 bytes are 0x00, then the user supplied no memo, and the encrypted memo field is to be treated as empty.".to_string();
        println!("{}", &test_message);
        let expected = test_message.clone().to_string();
        let result = package_memo(test_message);
        assert_eq!(result, expected);
    }

    #[test]
    fn package_memo_breadcrumbs3() {
        let test_message = "If the memo matches any of these patterns, then this memo is from the future, because these ranges are reserved for future updates to this specification: - The first byte has a value of 0xF5. - The first byte has a value of 0xF6, and the remaining 511 bytes are not all 0x00. - The first byte has a value between 0xF7 and 0xFE inclusive.".to_string();
        println!("{}", &test_message);
        let expected = test_message.clone().to_string();
        let result = package_memo(test_message);
        assert_eq!(result, expected);
    }

    #[test]
    fn package_memo_breadcrumbs4() {
        let test_message = "If the first byte has a value of 0xFF then the reader should not make any other assumption about the memo. In order to put arbitrary data into a memo field (that might have some private non-standard structure), the value of the first byte SHOULD be set to 0xFF; the remaining 511 bytes are then unconstrained.".to_string();
        println!("{}", &test_message);
        let expected = test_message.clone().to_string();
        let result = package_memo(test_message);
        assert_eq!(result, expected);
    }
}
