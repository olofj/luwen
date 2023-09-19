use std::sync::Arc;

use crate::DeviceInfo;

use super::eth_addr::EthAddr;

/// This trait is used to abstract the interface to the lowest level
/// chip communication primatives. These primatives are defined to be a chip resource
/// which requires exclusive access to safely use.
///
/// The implementation makes no attempt to use the borrow checker to enforce this exclusive access rule
/// because the primary implementors of this trait will be C++ and Python code.
pub trait ChipInterface {
    /// Access information about the underlying chip.
    fn get_device_info(&self) -> Option<DeviceInfo>;

    /// Read and write to the noc endpoint on the chip in implementation
    /// this may take the form of a direct pci bar read/write or a noc read/write
    fn axi_read(&self, addr: u32, data: &mut [u8]);
    fn axi_write(&self, addr: u32, data: &[u8]);

    /// Read and write to a noc endpoint on the chip.
    fn noc_read(&self, noc_id: u8, x: u8, y: u8, addr: u64, data: &mut [u8]);
    fn noc_write(&self, noc_id: u8, x: u8, y: u8, addr: u64, data: &[u8]);
    fn noc_broadcast(&self, noc_id: u8, addr: u64, data: &[u8]);

    /// Read and write to a noc endpoint via ethernet on a local or remote chip.
    fn eth_noc_read(&self, eth_addr: EthAddr, noc_id: u8, x: u8, y: u8, addr: u64, data: &mut [u8]);
    fn eth_noc_write(&self, eth_addr: EthAddr, noc_id: u8, x: u8, y: u8, addr: u64, data: &[u8]);
    fn eth_noc_broadcast(&self, eth_addr: EthAddr, noc_id: u8, addr: u64, data: &[u8]);
}

impl ChipInterface for Arc<dyn ChipInterface + Send + Sync> {
    fn get_device_info(&self) -> Option<DeviceInfo> {
        self.as_ref().get_device_info()
    }

    fn axi_read(&self, addr: u32, data: &mut [u8]) {
        self.as_ref().axi_read(addr, data)
    }

    fn axi_write(&self, addr: u32, data: &[u8]) {
        self.as_ref().axi_write(addr, data)
    }

    fn noc_read(&self, noc_id: u8, x: u8, y: u8, addr: u64, data: &mut [u8]) {
        self.as_ref().noc_read(noc_id, x, y, addr, data)
    }

    fn noc_write(&self, noc_id: u8, x: u8, y: u8, addr: u64, data: &[u8]) {
        self.as_ref().noc_write(noc_id, x, y, addr, data)
    }

    fn noc_broadcast(&self, noc_id: u8, addr: u64, data: &[u8]) {
        self.as_ref().noc_broadcast(noc_id, addr, data)
    }

    fn eth_noc_read(
        &self,
        eth_addr: EthAddr,
        noc_id: u8,
        x: u8,
        y: u8,
        addr: u64,
        data: &mut [u8],
    ) {
        self.as_ref()
            .eth_noc_read(eth_addr, noc_id, x, y, addr, data)
    }

    fn eth_noc_write(&self, eth_addr: EthAddr, noc_id: u8, x: u8, y: u8, addr: u64, data: &[u8]) {
        self.as_ref()
            .eth_noc_write(eth_addr, noc_id, x, y, addr, data)
    }

    fn eth_noc_broadcast(&self, eth_addr: EthAddr, noc_id: u8, addr: u64, data: &[u8]) {
        self.as_ref()
            .eth_noc_broadcast(eth_addr, noc_id, addr, data)
    }
}
