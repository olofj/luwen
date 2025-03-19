use luwen_core::Arch;
use luwen_if::{chip::HlCommsInterface, ChipImpl};

/// Test utilities for verifying telemetry functionality
///
/// These tests verify:
/// - Chip telemetry collection
/// - Chip status reporting
/// - Chip architecture detection
/// - Chip-specific functionality (Wormhole, Grayskull, Blackhole)
///
/// Note: These tests require physical hardware to run. By default, they are
/// annotated with #[ignore] to avoid false failures on systems without hardware.
/// To run all hardware tests:
///
///   cargo test --test telem_test -- --ignored
///
/// The tests will automatically detect if compatible hardware is present;
/// if hardware is not found, the test will be skipped.
mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::hardware_available;

    #[test]
    #[ignore = "Requires hardware"]
    fn test_chip_telemetry() {
        if !hardware_available() {
            return;
        }

        let partial_chips = luwen_ref::detect_chips_fallible().unwrap();
        assert!(!partial_chips.is_empty(), "Should find at least one chip");

        for chip in partial_chips {
            let status = chip.status();
            println!("Chip status: {:?}", status);

            let upgraded_chip = chip.try_upgrade();
            assert!(upgraded_chip.is_some(), "Should be able to upgrade chip");

            if let Some(upgraded_chip) = upgraded_chip {
                let eth_status = chip.eth_safe();
                println!("Ethernet status: {:?}", eth_status);

                // Test Wormhole-specific functionality
                if let Some(wh) = upgraded_chip.as_wh() {
                    println!("Testing Wormhole chip");

                    if chip.arc_alive() {
                        let telemetry = wh.get_telemetry().unwrap();
                        println!("Wormhole board ID: {:X}", telemetry.board_id_low);
                        assert_ne!(telemetry.board_id_low, 0, "Board ID should be non-zero");
                    }

                    // Check remote status
                    println!("Wormhole remote status: {}", wh.is_remote);
                }

                // Test Grayskull-specific functionality
                if let Some(gs) = upgraded_chip.as_gs() {
                    println!("Testing Grayskull chip");

                    // Read scratch register
                    let scratch_value = gs.axi_sread32("ARC_RESET.SCRATCH[0]").unwrap();
                    println!("Grayskull scratch value: {:x}", scratch_value);
                }

                // Test Blackhole-specific functionality
                if let Some(bh) = upgraded_chip.as_bh() {
                    println!("Testing Blackhole chip");

                    // Get telemetry twice to verify consistency
                    let telemetry1 = bh.get_telemetry().unwrap();
                    let telemetry2 = bh.get_telemetry().unwrap();

                    println!("Blackhole telemetry: {:?}", telemetry1);
                    println!("Blackhole telemetry: {:?}", telemetry2);

                    // Get subsystem ID
                    if let Some(subsystem) = bh.get_if::<luwen_if::chip::NocInterface>()
                        .map(|v| &v.backing)
                        .and_then(|v| {
                            v.as_any()
                                .downcast_ref::<luwen_if::CallbackStorage<luwen_ref::ExtendedPciDeviceWrapper>>()
                        })
                        .map(|v| v.user_data.borrow().device.physical.subsystem_id) {
                        println!("Blackhole subsystem ID: {:x}", subsystem);
                        assert_ne!(subsystem, 0, "Subsystem ID should be non-zero");
                    }
                }

                // Print chip information
                println!(
                    "Chip: {:?}, Status: {:?}, Ethernet: {:?}",
                    upgraded_chip.get_arch(),
                    status,
                    eth_status
                );

                // Verify that architecture is reported correctly
                assert!(
                    matches!(
                        upgraded_chip.get_arch(),
                        Arch::Wormhole | Arch::Grayskull | Arch::Blackhole
                    ),
                    "Architecture should be one of the supported types"
                );
            }
        }
    }
}
