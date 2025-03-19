use luwen_ref::detect_chips;

/// Test utilities for verifying boot filesystem protobuf decoding
///
/// These tests verify:
/// - Boot filesystem protobuf binary decoding functionality
/// - Access to different protobuf tables (boardcfg, flshinfo, cmfwcfg, origcfg)
/// - Successful deserialization of protobuf messages
///
/// Note: These tests require physical hardware to run. By default, they are
/// annotated with #[ignore] to avoid false failures on systems without hardware.
/// To run all hardware tests:
///
///   cargo test --test decode_proto_bin_test -- --ignored
///
/// The tests will automatically detect if compatible hardware is present;
/// if hardware is not found, the test will be skipped.
mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::has_chip_type;

    #[test]
    #[ignore = "Requires hardware"]
    fn test_decode_boardcfg() {
        // Skip if no Blackhole chips available
        if !has_chip_type(|chip| chip.as_bh().is_some()) {
            return;
        }

        let devices = detect_chips().unwrap();
        for device in devices {
            if let Some(bh) = device.as_bh() {
                // Test decoding the boardcfg table from the boot fs
                let decode_msg = bh.decode_boot_fs_table("boardcfg");
                println!("Decoded boardcfg: {:#?}", decode_msg);

                // Verify the decoding was successful
                assert!(decode_msg.is_ok(), "Failed to decode boardcfg table");
                let boardcfg = decode_msg.unwrap();

                // Verify the boardcfg message contains valid data
                assert!(!boardcfg.is_empty(), "Boardcfg message should not be empty");
            }
        }
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_decode_flshinfo() {
        // Skip if no Blackhole chips available
        if !has_chip_type(|chip| chip.as_bh().is_some()) {
            return;
        }

        let devices = detect_chips().unwrap();
        for device in devices {
            if let Some(bh) = device.as_bh() {
                // Test decoding the flshinfo table from the boot fs
                let decode_msg = bh.decode_boot_fs_table("flshinfo");
                println!("Decoded flshinfo: {:#?}", decode_msg);

                // Verify the decoding was successful
                assert!(decode_msg.is_ok(), "Failed to decode flshinfo table");
                let flshinfo = decode_msg.unwrap();

                // Verify the flshinfo message contains valid data
                assert!(!flshinfo.is_empty(), "Flshinfo message should not be empty");
            }
        }
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_decode_cmfwcfg() {
        // Skip if no Blackhole chips available
        if !has_chip_type(|chip| chip.as_bh().is_some()) {
            return;
        }

        let devices = detect_chips().unwrap();
        for device in devices {
            if let Some(bh) = device.as_bh() {
                // Test decoding the cmfwcfg table from the boot fs
                let decode_msg = bh.decode_boot_fs_table("cmfwcfg");
                println!("Decoded cmfwcfg: {:#?}", decode_msg);

                // Verify the decoding was successful
                assert!(decode_msg.is_ok(), "Failed to decode cmfwcfg table");
                let cmfwcfg = decode_msg.unwrap();

                // Verify the cmfwcfg message contains valid data
                assert!(!cmfwcfg.is_empty(), "Cmfwcfg message should not be empty");
            }
        }
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_decode_origcfg() {
        // Skip if no Blackhole chips available
        if !has_chip_type(|chip| chip.as_bh().is_some()) {
            return;
        }

        let devices = detect_chips().unwrap();
        for device in devices {
            if let Some(bh) = device.as_bh() {
                // Test decoding the origcfg table from the boot fs
                let decode_msg = bh.decode_boot_fs_table("origcfg");
                println!("Decoded origcfg: {:#?}", decode_msg);

                // Verify the decoding was successful
                assert!(decode_msg.is_ok(), "Failed to decode origcfg table");
                let origcfg = decode_msg.unwrap();

                // Verify the origcfg message contains valid data
                assert!(!origcfg.is_empty(), "Origcfg message should not be empty");
            }
        }
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_decode_nonexistent_table() {
        // Skip if no Blackhole chips available
        if !has_chip_type(|chip| chip.as_bh().is_some()) {
            return;
        }

        let devices = detect_chips().unwrap();
        for device in devices {
            if let Some(bh) = device.as_bh() {
                // Test decoding a non-existent table
                let decode_msg = bh.decode_boot_fs_table("nonexistent_table");

                // Verify the operation fails as expected
                assert!(
                    decode_msg.is_err(),
                    "Decoding non-existent table should fail"
                );
                println!(
                    "Expected error for non-existent table: {:?}",
                    decode_msg.err()
                );
            }
        }
    }
}
