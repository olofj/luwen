use luwen_ref::detect_chips;

/// Test reading boot fs tables from Blackhole chips
///
/// These tests verify that all standard boot FS tags can be read correctly
/// from a Blackhole chip's SPI ROM:
/// - boardcfg: board configuration information
/// - flshinfo: flash information
/// - cmfwcfg: firmware configuration
/// - origcfg: original firmware configuration
///
/// Note: These tests require physical hardware to run. By default, they are
/// annotated with #[ignore] to avoid false failures on systems without hardware.
/// To run all hardware tests:
///
///   cargo test --test boot_fs_test -- --ignored
///
/// The tests will automatically detect if compatible hardware is present;
/// if hardware is not found, the test will be skipped.

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if appropriate hardware is available
    fn blackhole_available() -> bool {
        match detect_chips() {
            Ok(devices) => {
                if devices.is_empty() {
                    println!("Test SKIPPED: No devices found");
                    return false;
                }

                if devices[0].as_bh().is_none() {
                    println!("Test SKIPPED: No Blackhole device found");
                    return false;
                }

                true
            }
            Err(e) => {
                println!("Test SKIPPED: Error detecting chips: {}", e);
                false
            }
        }
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_boardcfg_tag() {
        if !blackhole_available() {
            return;
        }

        let devices = detect_chips().unwrap();
        let bh = devices[0].as_bh().unwrap();

        let tag_read = bh.get_boot_fs_tables_spi_read("boardcfg").unwrap();
        assert!(tag_read.is_some(), "boardcfg tag should be present");

        let (_, fd) = tag_read.unwrap();
        assert_eq!(fd.image_tag_str(), "boardcfg", "Tag name should match");
        assert!(
            !unsafe { fd.flags.f.invalid() },
            "Tag should not be marked as invalid"
        );
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_flshinfo_tag() {
        if !blackhole_available() {
            return;
        }

        let devices = detect_chips().unwrap();
        let bh = devices[0].as_bh().unwrap();

        let tag_read = bh.get_boot_fs_tables_spi_read("flshinfo").unwrap();
        assert!(tag_read.is_some(), "flshinfo tag should be present");

        let (_, fd) = tag_read.unwrap();
        assert_eq!(fd.image_tag_str(), "flshinfo", "Tag name should match");
        assert!(
            !unsafe { fd.flags.f.invalid() },
            "Tag should not be marked as invalid"
        );
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_cmfwcfg_tag() {
        if !blackhole_available() {
            return;
        }

        let devices = detect_chips().unwrap();
        let bh = devices[0].as_bh().unwrap();

        let tag_read = bh.get_boot_fs_tables_spi_read("cmfwcfg").unwrap();
        assert!(tag_read.is_some(), "cmfwcfg tag should be present");

        let (_, fd) = tag_read.unwrap();
        assert_eq!(fd.image_tag_str(), "cmfwcfg", "Tag name should match");
        assert!(
            !unsafe { fd.flags.f.invalid() },
            "Tag should not be marked as invalid"
        );
    }

    #[test]
    #[ignore = "Requires hardware"]
    fn test_origcfg_tag() {
        if !blackhole_available() {
            return;
        }

        let devices = detect_chips().unwrap();
        let bh = devices[0].as_bh().unwrap();

        let tag_read = bh.get_boot_fs_tables_spi_read("origcfg").unwrap();
        assert!(tag_read.is_some(), "origcfg tag should be present");

        let (_, fd) = tag_read.unwrap();
        assert_eq!(fd.image_tag_str(), "origcfg", "Tag name should match");
        assert!(
            !unsafe { fd.flags.f.invalid() },
            "Tag should not be marked as invalid"
        );
    }
}
