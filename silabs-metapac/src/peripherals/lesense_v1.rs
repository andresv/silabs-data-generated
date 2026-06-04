#[doc = "LESENSE peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lesense {
    ptr: *mut u8,
}
unsafe impl Send for Lesense {}
unsafe impl Sync for Lesense {}
impl Lesense {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IPVERSION."]
    #[inline(always)]
    pub const fn ipversion(self) -> crate::common::Reg<regs::Ipversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Global Enable of LESENSE functions."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn swrst(self) -> crate::common::Reg<regs::Swrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Timing Control Register."]
    #[inline(always)]
    pub const fn timctrl(self) -> crate::common::Reg<regs::Timctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Peripheral Control Register."]
    #[inline(always)]
    pub const fn perctrl(self) -> crate::common::Reg<regs::Perctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Decoder control Register."]
    #[inline(always)]
    pub const fn decctrl(self) -> crate::common::Reg<regs::Decctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LESENSE evaluation control."]
    #[inline(always)]
    pub const fn evalctrl(self) -> crate::common::Reg<regs::Evalctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "PRS control register."]
    #[inline(always)]
    pub const fn prsctrl(self) -> crate::common::Reg<regs::Prsctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel enable Register."]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Scan result register."]
    #[inline(always)]
    pub const fn scanres(self) -> crate::common::Reg<regs::Scanres, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Result FIFO Count."]
    #[inline(always)]
    pub const fn rescount(self) -> crate::common::Reg<regs::Rescount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Result Fifo."]
    #[inline(always)]
    pub const fn resfifo(self) -> crate::common::Reg<regs::Resfifo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Current channel index."]
    #[inline(always)]
    pub const fn curch(self) -> crate::common::Reg<regs::Curch, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Current decoder state."]
    #[inline(always)]
    pub const fn decstate(self) -> crate::common::Reg<regs::Decstate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Decoder input register."]
    #[inline(always)]
    pub const fn sensorstate(self) -> crate::common::Reg<regs::Sensorstate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GPIO Idle phase configuration."]
    #[inline(always)]
    pub const fn idleconf(self) -> crate::common::Reg<regs::Idleconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Synchronization Busy Register."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt Flags."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Interrupt Enables."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch0_timing(self) -> crate::common::Reg<regs::Ch0Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch0_interact(self) -> crate::common::Reg<regs::Ch0Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch0_evalcfg(self) -> crate::common::Reg<regs::Ch0Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch0_evalthres(self) -> crate::common::Reg<regs::Ch0Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch1_timing(self) -> crate::common::Reg<regs::Ch1Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch1_interact(self) -> crate::common::Reg<regs::Ch1Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch1_evalcfg(self) -> crate::common::Reg<regs::Ch1Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch1_evalthres(self) -> crate::common::Reg<regs::Ch1Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch2_timing(self) -> crate::common::Reg<regs::Ch2Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch2_interact(self) -> crate::common::Reg<regs::Ch2Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch2_evalcfg(self) -> crate::common::Reg<regs::Ch2Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch2_evalthres(self) -> crate::common::Reg<regs::Ch2Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch3_timing(self) -> crate::common::Reg<regs::Ch3Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch3_interact(self) -> crate::common::Reg<regs::Ch3Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch3_evalcfg(self) -> crate::common::Reg<regs::Ch3Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch3_evalthres(self) -> crate::common::Reg<regs::Ch3Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch4_timing(self) -> crate::common::Reg<regs::Ch4Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch4_interact(self) -> crate::common::Reg<regs::Ch4Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch4_evalcfg(self) -> crate::common::Reg<regs::Ch4Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch4_evalthres(self) -> crate::common::Reg<regs::Ch4Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch5_timing(self) -> crate::common::Reg<regs::Ch5Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch5_interact(self) -> crate::common::Reg<regs::Ch5Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch5_evalcfg(self) -> crate::common::Reg<regs::Ch5Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch5_evalthres(self) -> crate::common::Reg<regs::Ch5Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch6_timing(self) -> crate::common::Reg<regs::Ch6Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch6_interact(self) -> crate::common::Reg<regs::Ch6Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch6_evalcfg(self) -> crate::common::Reg<regs::Ch6Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch6_evalthres(self) -> crate::common::Reg<regs::Ch6Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch7_timing(self) -> crate::common::Reg<regs::Ch7Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch7_interact(self) -> crate::common::Reg<regs::Ch7Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch7_evalcfg(self) -> crate::common::Reg<regs::Ch7Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch7_evalthres(self) -> crate::common::Reg<regs::Ch7Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch8_timing(self) -> crate::common::Reg<regs::Ch8Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch8_interact(self) -> crate::common::Reg<regs::Ch8Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch8_evalcfg(self) -> crate::common::Reg<regs::Ch8Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch8_evalthres(self) -> crate::common::Reg<regs::Ch8Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch9_timing(self) -> crate::common::Reg<regs::Ch9Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch9_interact(self) -> crate::common::Reg<regs::Ch9Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch9_evalcfg(self) -> crate::common::Reg<regs::Ch9Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch9_evalthres(self) -> crate::common::Reg<regs::Ch9Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch10_timing(self) -> crate::common::Reg<regs::Ch10Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch10_interact(self) -> crate::common::Reg<regs::Ch10Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch10_evalcfg(self) -> crate::common::Reg<regs::Ch10Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch10_evalthres(self) -> crate::common::Reg<regs::Ch10Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch11_timing(self) -> crate::common::Reg<regs::Ch11Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch11_interact(self) -> crate::common::Reg<regs::Ch11Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch11_evalcfg(self) -> crate::common::Reg<regs::Ch11Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch11_evalthres(self) -> crate::common::Reg<regs::Ch11Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch12_timing(self) -> crate::common::Reg<regs::Ch12Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch12_interact(self) -> crate::common::Reg<regs::Ch12Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch12_evalcfg(self) -> crate::common::Reg<regs::Ch12Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch12_evalthres(self) -> crate::common::Reg<regs::Ch12Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch13_timing(self) -> crate::common::Reg<regs::Ch13Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch13_interact(self) -> crate::common::Reg<regs::Ch13Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch13_evalcfg(self) -> crate::common::Reg<regs::Ch13Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch13_evalthres(self) -> crate::common::Reg<regs::Ch13Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch14_timing(self) -> crate::common::Reg<regs::Ch14Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch14_interact(self) -> crate::common::Reg<regs::Ch14Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch14_evalcfg(self) -> crate::common::Reg<regs::Ch14Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch14_evalthres(self) -> crate::common::Reg<regs::Ch14Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch15_timing(self) -> crate::common::Reg<regs::Ch15Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch15_interact(self) -> crate::common::Reg<regs::Ch15Interact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch15_evalcfg(self) -> crate::common::Reg<regs::Ch15Evalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch15_evalthres(self) -> crate::common::Reg<regs::Ch15Evalthres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn st_arc(self, n: usize) -> crate::common::Reg<regs::StArc, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Global Enable of LESENSE functions. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn swrst_set(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Timing Control Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timctrl_set(self) -> crate::common::Reg<regs::Timctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Peripheral Control Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn perctrl_set(self) -> crate::common::Reg<regs::Perctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Decoder control Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn decctrl_set(self) -> crate::common::Reg<regs::Decctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "LESENSE evaluation control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn evalctrl_set(self) -> crate::common::Reg<regs::Evalctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "PRS control register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prsctrl_set(self) -> crate::common::Reg<regs::Prsctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Command Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "Channel enable Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chen_set(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "GPIO Idle phase configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn idleconf_set(self) -> crate::common::Reg<regs::Idleconf, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "Interrupt Enables. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_timing_set(self) -> crate::common::Reg<regs::Ch0Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_interact_set(self) -> crate::common::Reg<regs::Ch0Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_evalcfg_set(self) -> crate::common::Reg<regs::Ch0Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_evalthres_set(self) -> crate::common::Reg<regs::Ch0Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x110cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_timing_set(self) -> crate::common::Reg<regs::Ch1Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1110usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_interact_set(self) -> crate::common::Reg<regs::Ch1Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_evalcfg_set(self) -> crate::common::Reg<regs::Ch1Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_evalthres_set(self) -> crate::common::Reg<regs::Ch1Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_timing_set(self) -> crate::common::Reg<regs::Ch2Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_interact_set(self) -> crate::common::Reg<regs::Ch2Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_evalcfg_set(self) -> crate::common::Reg<regs::Ch2Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_evalthres_set(self) -> crate::common::Reg<regs::Ch2Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_timing_set(self) -> crate::common::Reg<regs::Ch3Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_interact_set(self) -> crate::common::Reg<regs::Ch3Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_evalcfg_set(self) -> crate::common::Reg<regs::Ch3Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_evalthres_set(self) -> crate::common::Reg<regs::Ch3Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_timing_set(self) -> crate::common::Reg<regs::Ch4Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_interact_set(self) -> crate::common::Reg<regs::Ch4Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_evalcfg_set(self) -> crate::common::Reg<regs::Ch4Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_evalthres_set(self) -> crate::common::Reg<regs::Ch4Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_timing_set(self) -> crate::common::Reg<regs::Ch5Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_interact_set(self) -> crate::common::Reg<regs::Ch5Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_evalcfg_set(self) -> crate::common::Reg<regs::Ch5Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1158usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_evalthres_set(self) -> crate::common::Reg<regs::Ch5Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_timing_set(self) -> crate::common::Reg<regs::Ch6Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_interact_set(self) -> crate::common::Reg<regs::Ch6Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_evalcfg_set(self) -> crate::common::Reg<regs::Ch6Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_evalthres_set(self) -> crate::common::Reg<regs::Ch6Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_timing_set(self) -> crate::common::Reg<regs::Ch7Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_interact_set(self) -> crate::common::Reg<regs::Ch7Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1174usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_evalcfg_set(self) -> crate::common::Reg<regs::Ch7Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1178usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_evalthres_set(self) -> crate::common::Reg<regs::Ch7Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_timing_set(self) -> crate::common::Reg<regs::Ch8Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_interact_set(self) -> crate::common::Reg<regs::Ch8Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_evalcfg_set(self) -> crate::common::Reg<regs::Ch8Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_evalthres_set(self) -> crate::common::Reg<regs::Ch8Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x118cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_timing_set(self) -> crate::common::Reg<regs::Ch9Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1190usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_interact_set(self) -> crate::common::Reg<regs::Ch9Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_evalcfg_set(self) -> crate::common::Reg<regs::Ch9Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1198usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_evalthres_set(self) -> crate::common::Reg<regs::Ch9Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_timing_set(self) -> crate::common::Reg<regs::Ch10Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_interact_set(self) -> crate::common::Reg<regs::Ch10Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_evalcfg_set(self) -> crate::common::Reg<regs::Ch10Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_evalthres_set(self) -> crate::common::Reg<regs::Ch10Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11acusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_timing_set(self) -> crate::common::Reg<regs::Ch11Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_interact_set(self) -> crate::common::Reg<regs::Ch11Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_evalcfg_set(self) -> crate::common::Reg<regs::Ch11Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_evalthres_set(self) -> crate::common::Reg<regs::Ch11Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_timing_set(self) -> crate::common::Reg<regs::Ch12Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_interact_set(self) -> crate::common::Reg<regs::Ch12Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_evalcfg_set(self) -> crate::common::Reg<regs::Ch12Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_evalthres_set(self) -> crate::common::Reg<regs::Ch12Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_timing_set(self) -> crate::common::Reg<regs::Ch13Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_interact_set(self) -> crate::common::Reg<regs::Ch13Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_evalcfg_set(self) -> crate::common::Reg<regs::Ch13Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_evalthres_set(self) -> crate::common::Reg<regs::Ch13Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11dcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_timing_set(self) -> crate::common::Reg<regs::Ch14Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_interact_set(self) -> crate::common::Reg<regs::Ch14Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_evalcfg_set(self) -> crate::common::Reg<regs::Ch14Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_evalthres_set(self) -> crate::common::Reg<regs::Ch14Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_timing_set(self) -> crate::common::Reg<regs::Ch15Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_interact_set(self) -> crate::common::Reg<regs::Ch15Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_evalcfg_set(self) -> crate::common::Reg<regs::Ch15Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_evalthres_set(self) -> crate::common::Reg<regs::Ch15Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11fcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn st_arc_set(self, n: usize) -> crate::common::Reg<regs::StArc, crate::common::W> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize + n * 4usize) as _) }
    }
    #[doc = "Global Enable of LESENSE functions. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn swrst_clr(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "Timing Control Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timctrl_clr(self) -> crate::common::Reg<regs::Timctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "Peripheral Control Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn perctrl_clr(self) -> crate::common::Reg<regs::Perctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "Decoder control Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn decctrl_clr(self) -> crate::common::Reg<regs::Decctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "LESENSE evaluation control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn evalctrl_clr(self) -> crate::common::Reg<regs::Evalctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "PRS control register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prsctrl_clr(self) -> crate::common::Reg<regs::Prsctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Command Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "Channel enable Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chen_clr(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "GPIO Idle phase configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn idleconf_clr(self) -> crate::common::Reg<regs::Idleconf, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "Interrupt Enables. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_timing_clr(self) -> crate::common::Reg<regs::Ch0Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_interact_clr(self) -> crate::common::Reg<regs::Ch0Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_evalcfg_clr(self) -> crate::common::Reg<regs::Ch0Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2108usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_evalthres_clr(self) -> crate::common::Reg<regs::Ch0Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x210cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_timing_clr(self) -> crate::common::Reg<regs::Ch1Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2110usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_interact_clr(self) -> crate::common::Reg<regs::Ch1Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2114usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_evalcfg_clr(self) -> crate::common::Reg<regs::Ch1Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2118usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_evalthres_clr(self) -> crate::common::Reg<regs::Ch1Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x211cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_timing_clr(self) -> crate::common::Reg<regs::Ch2Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_interact_clr(self) -> crate::common::Reg<regs::Ch2Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_evalcfg_clr(self) -> crate::common::Reg<regs::Ch2Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_evalthres_clr(self) -> crate::common::Reg<regs::Ch2Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x212cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_timing_clr(self) -> crate::common::Reg<regs::Ch3Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2130usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_interact_clr(self) -> crate::common::Reg<regs::Ch3Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2134usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_evalcfg_clr(self) -> crate::common::Reg<regs::Ch3Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2138usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_evalthres_clr(self) -> crate::common::Reg<regs::Ch3Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x213cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_timing_clr(self) -> crate::common::Reg<regs::Ch4Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_interact_clr(self) -> crate::common::Reg<regs::Ch4Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2144usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_evalcfg_clr(self) -> crate::common::Reg<regs::Ch4Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2148usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_evalthres_clr(self) -> crate::common::Reg<regs::Ch4Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x214cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_timing_clr(self) -> crate::common::Reg<regs::Ch5Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2150usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_interact_clr(self) -> crate::common::Reg<regs::Ch5Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2154usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_evalcfg_clr(self) -> crate::common::Reg<regs::Ch5Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2158usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_evalthres_clr(self) -> crate::common::Reg<regs::Ch5Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x215cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_timing_clr(self) -> crate::common::Reg<regs::Ch6Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_interact_clr(self) -> crate::common::Reg<regs::Ch6Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2164usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_evalcfg_clr(self) -> crate::common::Reg<regs::Ch6Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2168usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_evalthres_clr(self) -> crate::common::Reg<regs::Ch6Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x216cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_timing_clr(self) -> crate::common::Reg<regs::Ch7Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_interact_clr(self) -> crate::common::Reg<regs::Ch7Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2174usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_evalcfg_clr(self) -> crate::common::Reg<regs::Ch7Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2178usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_evalthres_clr(self) -> crate::common::Reg<regs::Ch7Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x217cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_timing_clr(self) -> crate::common::Reg<regs::Ch8Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_interact_clr(self) -> crate::common::Reg<regs::Ch8Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2184usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_evalcfg_clr(self) -> crate::common::Reg<regs::Ch8Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2188usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_evalthres_clr(self) -> crate::common::Reg<regs::Ch8Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x218cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_timing_clr(self) -> crate::common::Reg<regs::Ch9Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2190usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_interact_clr(self) -> crate::common::Reg<regs::Ch9Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2194usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_evalcfg_clr(self) -> crate::common::Reg<regs::Ch9Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2198usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_evalthres_clr(self) -> crate::common::Reg<regs::Ch9Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x219cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_timing_clr(self) -> crate::common::Reg<regs::Ch10Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_interact_clr(self) -> crate::common::Reg<regs::Ch10Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_evalcfg_clr(self) -> crate::common::Reg<regs::Ch10Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_evalthres_clr(self) -> crate::common::Reg<regs::Ch10Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21acusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_timing_clr(self) -> crate::common::Reg<regs::Ch11Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_interact_clr(self) -> crate::common::Reg<regs::Ch11Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_evalcfg_clr(self) -> crate::common::Reg<regs::Ch11Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_evalthres_clr(self) -> crate::common::Reg<regs::Ch11Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21bcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_timing_clr(self) -> crate::common::Reg<regs::Ch12Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_interact_clr(self) -> crate::common::Reg<regs::Ch12Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_evalcfg_clr(self) -> crate::common::Reg<regs::Ch12Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_evalthres_clr(self) -> crate::common::Reg<regs::Ch12Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_timing_clr(self) -> crate::common::Reg<regs::Ch13Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_interact_clr(self) -> crate::common::Reg<regs::Ch13Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_evalcfg_clr(self) -> crate::common::Reg<regs::Ch13Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_evalthres_clr(self) -> crate::common::Reg<regs::Ch13Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21dcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_timing_clr(self) -> crate::common::Reg<regs::Ch14Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_interact_clr(self) -> crate::common::Reg<regs::Ch14Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_evalcfg_clr(self) -> crate::common::Reg<regs::Ch14Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_evalthres_clr(self) -> crate::common::Reg<regs::Ch14Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21ecusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_timing_clr(self) -> crate::common::Reg<regs::Ch15Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_interact_clr(self) -> crate::common::Reg<regs::Ch15Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_evalcfg_clr(self) -> crate::common::Reg<regs::Ch15Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_evalthres_clr(self) -> crate::common::Reg<regs::Ch15Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21fcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn st_arc_clr(self, n: usize) -> crate::common::Reg<regs::StArc, crate::common::W> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize + n * 4usize) as _) }
    }
    #[doc = "Global Enable of LESENSE functions. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn swrst_tgl(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "Timing Control Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timctrl_tgl(self) -> crate::common::Reg<regs::Timctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "Peripheral Control Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn perctrl_tgl(self) -> crate::common::Reg<regs::Perctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "Decoder control Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn decctrl_tgl(self) -> crate::common::Reg<regs::Decctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "LESENSE evaluation control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn evalctrl_tgl(self) -> crate::common::Reg<regs::Evalctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "PRS control register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prsctrl_tgl(self) -> crate::common::Reg<regs::Prsctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Command Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "Channel enable Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chen_tgl(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "GPIO Idle phase configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn idleconf_tgl(self) -> crate::common::Reg<regs::Idleconf, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "Interrupt Enables. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_timing_tgl(self) -> crate::common::Reg<regs::Ch0Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_interact_tgl(self) -> crate::common::Reg<regs::Ch0Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch0Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3108usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_evalthres_tgl(self) -> crate::common::Reg<regs::Ch0Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x310cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_timing_tgl(self) -> crate::common::Reg<regs::Ch1Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3110usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_interact_tgl(self) -> crate::common::Reg<regs::Ch1Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3114usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch1Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3118usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_evalthres_tgl(self) -> crate::common::Reg<regs::Ch1Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x311cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_timing_tgl(self) -> crate::common::Reg<regs::Ch2Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_interact_tgl(self) -> crate::common::Reg<regs::Ch2Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch2Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_evalthres_tgl(self) -> crate::common::Reg<regs::Ch2Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x312cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_timing_tgl(self) -> crate::common::Reg<regs::Ch3Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3130usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_interact_tgl(self) -> crate::common::Reg<regs::Ch3Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3134usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch3Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3138usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_evalthres_tgl(self) -> crate::common::Reg<regs::Ch3Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x313cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_timing_tgl(self) -> crate::common::Reg<regs::Ch4Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3140usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_interact_tgl(self) -> crate::common::Reg<regs::Ch4Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3144usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch4Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3148usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_evalthres_tgl(self) -> crate::common::Reg<regs::Ch4Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x314cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_timing_tgl(self) -> crate::common::Reg<regs::Ch5Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3150usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_interact_tgl(self) -> crate::common::Reg<regs::Ch5Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3154usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch5Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3158usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_evalthres_tgl(self) -> crate::common::Reg<regs::Ch5Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x315cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_timing_tgl(self) -> crate::common::Reg<regs::Ch6Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3160usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_interact_tgl(self) -> crate::common::Reg<regs::Ch6Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3164usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch6Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3168usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_evalthres_tgl(self) -> crate::common::Reg<regs::Ch6Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x316cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_timing_tgl(self) -> crate::common::Reg<regs::Ch7Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3170usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_interact_tgl(self) -> crate::common::Reg<regs::Ch7Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3174usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch7Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3178usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_evalthres_tgl(self) -> crate::common::Reg<regs::Ch7Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x317cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_timing_tgl(self) -> crate::common::Reg<regs::Ch8Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3180usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_interact_tgl(self) -> crate::common::Reg<regs::Ch8Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3184usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch8Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3188usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_evalthres_tgl(self) -> crate::common::Reg<regs::Ch8Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x318cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_timing_tgl(self) -> crate::common::Reg<regs::Ch9Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3190usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_interact_tgl(self) -> crate::common::Reg<regs::Ch9Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3194usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch9Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3198usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_evalthres_tgl(self) -> crate::common::Reg<regs::Ch9Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x319cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_timing_tgl(self) -> crate::common::Reg<regs::Ch10Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_interact_tgl(self) -> crate::common::Reg<regs::Ch10Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch10Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_evalthres_tgl(self) -> crate::common::Reg<regs::Ch10Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31acusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_timing_tgl(self) -> crate::common::Reg<regs::Ch11Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_interact_tgl(self) -> crate::common::Reg<regs::Ch11Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch11Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_evalthres_tgl(self) -> crate::common::Reg<regs::Ch11Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31bcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_timing_tgl(self) -> crate::common::Reg<regs::Ch12Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_interact_tgl(self) -> crate::common::Reg<regs::Ch12Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch12Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_evalthres_tgl(self) -> crate::common::Reg<regs::Ch12Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_timing_tgl(self) -> crate::common::Reg<regs::Ch13Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_interact_tgl(self) -> crate::common::Reg<regs::Ch13Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch13Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_evalthres_tgl(self) -> crate::common::Reg<regs::Ch13Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31dcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_timing_tgl(self) -> crate::common::Reg<regs::Ch14Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_interact_tgl(self) -> crate::common::Reg<regs::Ch14Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch14Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_evalthres_tgl(self) -> crate::common::Reg<regs::Ch14Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31ecusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_timing_tgl(self) -> crate::common::Reg<regs::Ch15Timing, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_interact_tgl(self) -> crate::common::Reg<regs::Ch15Interact, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_evalcfg_tgl(self) -> crate::common::Reg<regs::Ch15Evalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_evalthres_tgl(self) -> crate::common::Reg<regs::Ch15Evalthres, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31fcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn st_arc_tgl(self, n: usize) -> crate::common::Reg<regs::StArc, crate::common::W> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Configure scan mode."]
        #[must_use]
        #[inline(always)]
        pub const fn scanmode(&self) -> super::vals::Scanmode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Scanmode::from_bits(val as u8)
        }
        #[doc = "Configure scan mode."]
        #[inline(always)]
        pub const fn set_scanmode(&mut self, val: super::vals::Scanmode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Select scan configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn scanconf(&self) -> super::vals::Scanconf {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Scanconf::from_bits(val as u8)
        }
        #[doc = "Select scan configuration."]
        #[inline(always)]
        pub const fn set_scanconf(&mut self, val: super::vals::Scanconf) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Enable dual sample mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dualsample(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable dual sample mode."]
        #[inline(always)]
        pub const fn set_dualsample(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable storing of SCANRES."]
        #[must_use]
        #[inline(always)]
        pub const fn strscanres(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable storing of SCANRES."]
        #[inline(always)]
        pub const fn set_strscanres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA wake-up from EM2."]
        #[must_use]
        #[inline(always)]
        pub const fn dmawu(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA wake-up from EM2."]
        #[inline(always)]
        pub const fn set_dmawu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Result FIFO level."]
        #[must_use]
        #[inline(always)]
        pub const fn resfidl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Result FIFO level."]
        #[inline(always)]
        pub const fn set_resfidl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Debug Mode Run Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn debugrun(&self) -> super::vals::Debugrun {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Debugrun::from_bits(val as u8)
        }
        #[doc = "Debug Mode Run Enable."]
        #[inline(always)]
        pub const fn set_debugrun(&mut self, val: super::vals::Debugrun) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("scanmode", &self.scanmode())
                .field("scanconf", &self.scanconf())
                .field("dualsample", &self.dualsample())
                .field("strscanres", &self.strscanres())
                .field("dmawu", &self.dmawu())
                .field("resfidl", &self.resfidl())
                .field("debugrun", &self.debugrun())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ scanmode: {:?}, scanconf: {:?}, dualsample: {=bool:?}, strscanres: {=bool:?}, dmawu: {=bool:?}, resfidl: {=u8:?}, debugrun: {:?} }}",
                self.scanmode(),
                self.scanconf(),
                self.dualsample(),
                self.strscanres(),
                self.dmawu(),
                self.resfidl(),
                self.debugrun()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Evalcfg(pub u32);
    impl Ch0Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch0EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch0EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch0EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch0EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch0EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch0EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch0EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch0EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch0EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch0Evalcfg {
        #[inline(always)]
        fn default() -> Ch0Evalcfg {
            Ch0Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch0Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Evalthres(pub u32);
    impl Ch0Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch0Evalthres {
        #[inline(always)]
        fn default() -> Ch0Evalthres {
            Ch0Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch0Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch0Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Interact(pub u32);
    impl Ch0Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch0InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch0InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch0InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch0InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch0InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch0InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch0InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch0InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch0InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch0InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch0InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch0InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch0InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch0InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch0InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch0Interact {
        #[inline(always)]
        fn default() -> Ch0Interact {
            Ch0Interact(0)
        }
    }
    impl core::fmt::Debug for Ch0Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Timing(pub u32);
    impl Ch0Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch0Timing {
        #[inline(always)]
        fn default() -> Ch0Timing {
            Ch0Timing(0)
        }
    }
    impl core::fmt::Debug for Ch0Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Evalcfg(pub u32);
    impl Ch10Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch10EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch10EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch10EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch10EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch10EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch10EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch10EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch10EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch10EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch10Evalcfg {
        #[inline(always)]
        fn default() -> Ch10Evalcfg {
            Ch10Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch10Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Evalthres(pub u32);
    impl Ch10Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch10Evalthres {
        #[inline(always)]
        fn default() -> Ch10Evalthres {
            Ch10Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch10Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch10Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Interact(pub u32);
    impl Ch10Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch10InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch10InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch10InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch10InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch10InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch10InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch10InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch10InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch10InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch10InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch10InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch10InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch10InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch10InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch10InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch10Interact {
        #[inline(always)]
        fn default() -> Ch10Interact {
            Ch10Interact(0)
        }
    }
    impl core::fmt::Debug for Ch10Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Timing(pub u32);
    impl Ch10Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch10Timing {
        #[inline(always)]
        fn default() -> Ch10Timing {
            Ch10Timing(0)
        }
    }
    impl core::fmt::Debug for Ch10Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Evalcfg(pub u32);
    impl Ch11Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch11EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch11EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch11EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch11EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch11EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch11EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch11EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch11EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch11EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch11Evalcfg {
        #[inline(always)]
        fn default() -> Ch11Evalcfg {
            Ch11Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch11Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Evalthres(pub u32);
    impl Ch11Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch11Evalthres {
        #[inline(always)]
        fn default() -> Ch11Evalthres {
            Ch11Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch11Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch11Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Interact(pub u32);
    impl Ch11Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch11InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch11InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch11InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch11InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch11InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch11InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch11InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch11InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch11InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch11InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch11InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch11InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch11InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch11InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch11InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch11Interact {
        #[inline(always)]
        fn default() -> Ch11Interact {
            Ch11Interact(0)
        }
    }
    impl core::fmt::Debug for Ch11Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Timing(pub u32);
    impl Ch11Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch11Timing {
        #[inline(always)]
        fn default() -> Ch11Timing {
            Ch11Timing(0)
        }
    }
    impl core::fmt::Debug for Ch11Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Evalcfg(pub u32);
    impl Ch12Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch12EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch12EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch12EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch12EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch12EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch12EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch12EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch12EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch12EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch12Evalcfg {
        #[inline(always)]
        fn default() -> Ch12Evalcfg {
            Ch12Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch12Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Evalthres(pub u32);
    impl Ch12Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch12Evalthres {
        #[inline(always)]
        fn default() -> Ch12Evalthres {
            Ch12Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch12Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch12Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Interact(pub u32);
    impl Ch12Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch12InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch12InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch12InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch12InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch12InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch12InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch12InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch12InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch12InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch12InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch12InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch12InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch12InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch12InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch12InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch12Interact {
        #[inline(always)]
        fn default() -> Ch12Interact {
            Ch12Interact(0)
        }
    }
    impl core::fmt::Debug for Ch12Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Timing(pub u32);
    impl Ch12Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch12Timing {
        #[inline(always)]
        fn default() -> Ch12Timing {
            Ch12Timing(0)
        }
    }
    impl core::fmt::Debug for Ch12Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Evalcfg(pub u32);
    impl Ch13Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch13EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch13EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch13EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch13EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch13EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch13EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch13EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch13EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch13EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch13Evalcfg {
        #[inline(always)]
        fn default() -> Ch13Evalcfg {
            Ch13Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch13Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Evalthres(pub u32);
    impl Ch13Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch13Evalthres {
        #[inline(always)]
        fn default() -> Ch13Evalthres {
            Ch13Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch13Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch13Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Interact(pub u32);
    impl Ch13Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch13InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch13InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch13InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch13InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch13InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch13InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch13InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch13InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch13InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch13InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch13InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch13InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch13InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch13InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch13InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch13Interact {
        #[inline(always)]
        fn default() -> Ch13Interact {
            Ch13Interact(0)
        }
    }
    impl core::fmt::Debug for Ch13Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Timing(pub u32);
    impl Ch13Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch13Timing {
        #[inline(always)]
        fn default() -> Ch13Timing {
            Ch13Timing(0)
        }
    }
    impl core::fmt::Debug for Ch13Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Evalcfg(pub u32);
    impl Ch14Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch14EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch14EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch14EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch14EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch14EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch14EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch14EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch14EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch14EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch14Evalcfg {
        #[inline(always)]
        fn default() -> Ch14Evalcfg {
            Ch14Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch14Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Evalthres(pub u32);
    impl Ch14Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch14Evalthres {
        #[inline(always)]
        fn default() -> Ch14Evalthres {
            Ch14Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch14Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch14Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Interact(pub u32);
    impl Ch14Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch14InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch14InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch14InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch14InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch14InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch14InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch14InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch14InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch14InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch14InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch14InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch14InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch14InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch14InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch14InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch14Interact {
        #[inline(always)]
        fn default() -> Ch14Interact {
            Ch14Interact(0)
        }
    }
    impl core::fmt::Debug for Ch14Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Timing(pub u32);
    impl Ch14Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch14Timing {
        #[inline(always)]
        fn default() -> Ch14Timing {
            Ch14Timing(0)
        }
    }
    impl core::fmt::Debug for Ch14Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Evalcfg(pub u32);
    impl Ch15Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch15EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch15EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch15EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch15EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch15EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch15EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch15EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch15EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch15EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch15Evalcfg {
        #[inline(always)]
        fn default() -> Ch15Evalcfg {
            Ch15Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch15Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Evalthres(pub u32);
    impl Ch15Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch15Evalthres {
        #[inline(always)]
        fn default() -> Ch15Evalthres {
            Ch15Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch15Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch15Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Interact(pub u32);
    impl Ch15Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch15InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch15InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch15InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch15InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch15InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch15InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch15InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch15InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch15InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch15InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch15InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch15InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch15InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch15InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch15InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch15Interact {
        #[inline(always)]
        fn default() -> Ch15Interact {
            Ch15Interact(0)
        }
    }
    impl core::fmt::Debug for Ch15Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Timing(pub u32);
    impl Ch15Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch15Timing {
        #[inline(always)]
        fn default() -> Ch15Timing {
            Ch15Timing(0)
        }
    }
    impl core::fmt::Debug for Ch15Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Evalcfg(pub u32);
    impl Ch1Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch1EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch1EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch1EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch1EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch1EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch1EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch1EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch1EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch1EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch1Evalcfg {
        #[inline(always)]
        fn default() -> Ch1Evalcfg {
            Ch1Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch1Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Evalthres(pub u32);
    impl Ch1Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch1Evalthres {
        #[inline(always)]
        fn default() -> Ch1Evalthres {
            Ch1Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch1Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Interact(pub u32);
    impl Ch1Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch1InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch1InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch1InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch1InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch1InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch1InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch1InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch1InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch1InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch1InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch1InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch1InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch1InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch1InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch1InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch1Interact {
        #[inline(always)]
        fn default() -> Ch1Interact {
            Ch1Interact(0)
        }
    }
    impl core::fmt::Debug for Ch1Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Timing(pub u32);
    impl Ch1Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch1Timing {
        #[inline(always)]
        fn default() -> Ch1Timing {
            Ch1Timing(0)
        }
    }
    impl core::fmt::Debug for Ch1Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Evalcfg(pub u32);
    impl Ch2Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch2EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch2EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch2EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch2EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch2EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch2EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch2EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch2EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch2EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch2Evalcfg {
        #[inline(always)]
        fn default() -> Ch2Evalcfg {
            Ch2Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch2Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Evalthres(pub u32);
    impl Ch2Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch2Evalthres {
        #[inline(always)]
        fn default() -> Ch2Evalthres {
            Ch2Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch2Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Interact(pub u32);
    impl Ch2Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch2InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch2InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch2InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch2InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch2InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch2InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch2InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch2InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch2InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch2InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch2InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch2InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch2InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch2InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch2InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch2Interact {
        #[inline(always)]
        fn default() -> Ch2Interact {
            Ch2Interact(0)
        }
    }
    impl core::fmt::Debug for Ch2Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Timing(pub u32);
    impl Ch2Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch2Timing {
        #[inline(always)]
        fn default() -> Ch2Timing {
            Ch2Timing(0)
        }
    }
    impl core::fmt::Debug for Ch2Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Evalcfg(pub u32);
    impl Ch3Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch3EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch3EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch3EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch3EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch3EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch3EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch3EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch3EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch3EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch3Evalcfg {
        #[inline(always)]
        fn default() -> Ch3Evalcfg {
            Ch3Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch3Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Evalthres(pub u32);
    impl Ch3Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch3Evalthres {
        #[inline(always)]
        fn default() -> Ch3Evalthres {
            Ch3Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch3Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Interact(pub u32);
    impl Ch3Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch3InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch3InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch3InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch3InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch3InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch3InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch3InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch3InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch3InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch3InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch3InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch3InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch3InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch3InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch3InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch3Interact {
        #[inline(always)]
        fn default() -> Ch3Interact {
            Ch3Interact(0)
        }
    }
    impl core::fmt::Debug for Ch3Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Timing(pub u32);
    impl Ch3Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch3Timing {
        #[inline(always)]
        fn default() -> Ch3Timing {
            Ch3Timing(0)
        }
    }
    impl core::fmt::Debug for Ch3Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Evalcfg(pub u32);
    impl Ch4Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch4EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch4EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch4EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch4EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch4EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch4EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch4EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch4EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch4EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch4Evalcfg {
        #[inline(always)]
        fn default() -> Ch4Evalcfg {
            Ch4Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch4Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Evalthres(pub u32);
    impl Ch4Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch4Evalthres {
        #[inline(always)]
        fn default() -> Ch4Evalthres {
            Ch4Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch4Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch4Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Interact(pub u32);
    impl Ch4Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch4InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch4InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch4InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch4InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch4InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch4InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch4InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch4InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch4InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch4InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch4InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch4InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch4InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch4InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch4InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch4Interact {
        #[inline(always)]
        fn default() -> Ch4Interact {
            Ch4Interact(0)
        }
    }
    impl core::fmt::Debug for Ch4Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Timing(pub u32);
    impl Ch4Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch4Timing {
        #[inline(always)]
        fn default() -> Ch4Timing {
            Ch4Timing(0)
        }
    }
    impl core::fmt::Debug for Ch4Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Evalcfg(pub u32);
    impl Ch5Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch5EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch5EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch5EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch5EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch5EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch5EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch5EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch5EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch5EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch5Evalcfg {
        #[inline(always)]
        fn default() -> Ch5Evalcfg {
            Ch5Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch5Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Evalthres(pub u32);
    impl Ch5Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch5Evalthres {
        #[inline(always)]
        fn default() -> Ch5Evalthres {
            Ch5Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch5Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch5Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Interact(pub u32);
    impl Ch5Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch5InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch5InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch5InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch5InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch5InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch5InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch5InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch5InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch5InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch5InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch5InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch5InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch5InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch5InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch5InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch5Interact {
        #[inline(always)]
        fn default() -> Ch5Interact {
            Ch5Interact(0)
        }
    }
    impl core::fmt::Debug for Ch5Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Timing(pub u32);
    impl Ch5Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch5Timing {
        #[inline(always)]
        fn default() -> Ch5Timing {
            Ch5Timing(0)
        }
    }
    impl core::fmt::Debug for Ch5Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Evalcfg(pub u32);
    impl Ch6Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch6EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch6EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch6EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch6EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch6EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch6EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch6EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch6EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch6EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch6Evalcfg {
        #[inline(always)]
        fn default() -> Ch6Evalcfg {
            Ch6Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch6Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Evalthres(pub u32);
    impl Ch6Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch6Evalthres {
        #[inline(always)]
        fn default() -> Ch6Evalthres {
            Ch6Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch6Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch6Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Interact(pub u32);
    impl Ch6Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch6InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch6InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch6InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch6InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch6InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch6InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch6InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch6InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch6InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch6InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch6InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch6InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch6InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch6InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch6InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch6Interact {
        #[inline(always)]
        fn default() -> Ch6Interact {
            Ch6Interact(0)
        }
    }
    impl core::fmt::Debug for Ch6Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Timing(pub u32);
    impl Ch6Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch6Timing {
        #[inline(always)]
        fn default() -> Ch6Timing {
            Ch6Timing(0)
        }
    }
    impl core::fmt::Debug for Ch6Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Evalcfg(pub u32);
    impl Ch7Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch7EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch7EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch7EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch7EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch7EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch7EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch7EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch7EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch7EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch7Evalcfg {
        #[inline(always)]
        fn default() -> Ch7Evalcfg {
            Ch7Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch7Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Evalthres(pub u32);
    impl Ch7Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch7Evalthres {
        #[inline(always)]
        fn default() -> Ch7Evalthres {
            Ch7Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch7Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch7Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Interact(pub u32);
    impl Ch7Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch7InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch7InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch7InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch7InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch7InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch7InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch7InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch7InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch7InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch7InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch7InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch7InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch7InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch7InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch7InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch7Interact {
        #[inline(always)]
        fn default() -> Ch7Interact {
            Ch7Interact(0)
        }
    }
    impl core::fmt::Debug for Ch7Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Timing(pub u32);
    impl Ch7Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch7Timing {
        #[inline(always)]
        fn default() -> Ch7Timing {
            Ch7Timing(0)
        }
    }
    impl core::fmt::Debug for Ch7Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Evalcfg(pub u32);
    impl Ch8Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch8EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch8EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch8EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch8EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch8EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch8EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch8EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch8EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch8EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch8Evalcfg {
        #[inline(always)]
        fn default() -> Ch8Evalcfg {
            Ch8Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch8Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Evalthres(pub u32);
    impl Ch8Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch8Evalthres {
        #[inline(always)]
        fn default() -> Ch8Evalthres {
            Ch8Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch8Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch8Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Interact(pub u32);
    impl Ch8Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch8InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch8InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch8InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch8InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch8InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch8InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch8InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch8InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch8InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch8InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch8InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch8InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch8InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch8InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch8InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch8Interact {
        #[inline(always)]
        fn default() -> Ch8Interact {
            Ch8Interact(0)
        }
    }
    impl core::fmt::Debug for Ch8Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Timing(pub u32);
    impl Ch8Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch8Timing {
        #[inline(always)]
        fn default() -> Ch8Timing {
            Ch8Timing(0)
        }
    }
    impl core::fmt::Debug for Ch8Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Evalcfg(pub u32);
    impl Ch9Evalcfg {
        #[doc = "Send result to decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Send result to decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select mode for threshold comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> super::vals::Ch9EvalcfgComp {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ch9EvalcfgComp::from_bits(val as u8)
        }
        #[doc = "Select mode for threshold comparison."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: super::vals::Ch9EvalcfgComp) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[must_use]
        #[inline(always)]
        pub const fn strsample(&self) -> super::vals::Ch9EvalcfgStrsample {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ch9EvalcfgStrsample::from_bits(val as u8)
        }
        #[doc = "Enable storing of sensor sample in resul."]
        #[inline(always)]
        pub const fn set_strsample(&mut self, val: super::vals::Ch9EvalcfgStrsample) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Enable inversion of result."]
        #[must_use]
        #[inline(always)]
        pub const fn scanresinv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable inversion of result."]
        #[inline(always)]
        pub const fn set_scanresinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Configure evaluation mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Ch9EvalcfgMode {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ch9EvalcfgMode::from_bits(val as u8)
        }
        #[doc = "Configure evaluation mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Ch9EvalcfgMode) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Ch9Evalcfg {
        #[inline(always)]
        fn default() -> Ch9Evalcfg {
            Ch9Evalcfg(0)
        }
    }
    impl core::fmt::Debug for Ch9Evalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Evalcfg")
                .field("decode", &self.decode())
                .field("comp", &self.comp())
                .field("strsample", &self.strsample())
                .field("scanresinv", &self.scanresinv())
                .field("mode", &self.mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Evalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Evalcfg {{ decode: {=bool:?}, comp: {:?}, strsample: {:?}, scanresinv: {=bool:?}, mode: {:?} }}",
                self.decode(),
                self.comp(),
                self.strsample(),
                self.scanresinv(),
                self.mode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Evalthres(pub u32);
    impl Ch9Evalthres {
        #[doc = "Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn evalthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Threshold."]
        #[inline(always)]
        pub const fn set_evalthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ch9Evalthres {
        #[inline(always)]
        fn default() -> Ch9Evalthres {
            Ch9Evalthres(0)
        }
    }
    impl core::fmt::Debug for Ch9Evalthres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Evalthres")
                .field("evalthres", &self.evalthres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Evalthres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch9Evalthres {{ evalthres: {=u16:?} }}", self.evalthres())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Interact(pub u32);
    impl Ch9Interact {
        #[doc = "ACMP threshold or DAC data."]
        #[must_use]
        #[inline(always)]
        pub const fn thres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ACMP threshold or DAC data."]
        #[inline(always)]
        pub const fn set_thres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Set GPIO mode."]
        #[must_use]
        #[inline(always)]
        pub const fn exmode(&self) -> super::vals::Ch9InteractExmode {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch9InteractExmode::from_bits(val as u8)
        }
        #[doc = "Set GPIO mode."]
        #[inline(always)]
        pub const fn set_exmode(&mut self, val: super::vals::Ch9InteractExmode) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Use alternative excite pin."]
        #[must_use]
        #[inline(always)]
        pub const fn altex(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Use alternative excite pin."]
        #[inline(always)]
        pub const fn set_altex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Select clock used for timing of sample d."]
        #[must_use]
        #[inline(always)]
        pub const fn sampleclk(&self) -> super::vals::Ch9InteractSampleclk {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ch9InteractSampleclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for timing of sample d."]
        #[inline(always)]
        pub const fn set_sampleclk(&mut self, val: super::vals::Ch9InteractSampleclk) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Select clock used for excitation timing."]
        #[must_use]
        #[inline(always)]
        pub const fn exclk(&self) -> super::vals::Ch9InteractExclk {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch9InteractExclk::from_bits(val as u8)
        }
        #[doc = "Select clock used for excitation timing."]
        #[inline(always)]
        pub const fn set_exclk(&mut self, val: super::vals::Ch9InteractExclk) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> super::vals::Ch9InteractSetif {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Ch9InteractSetif::from_bits(val as u8)
        }
        #[doc = "Enable interrupt generation."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: super::vals::Ch9InteractSetif) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OFFSET for IADC/ACMP interaction."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Sample mode Selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample(&self) -> super::vals::Ch9InteractSample {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch9InteractSample::from_bits(val as u8)
        }
        #[doc = "Sample mode Selection."]
        #[inline(always)]
        pub const fn set_sample(&mut self, val: super::vals::Ch9InteractSample) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ch9Interact {
        #[inline(always)]
        fn default() -> Ch9Interact {
            Ch9Interact(0)
        }
    }
    impl core::fmt::Debug for Ch9Interact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Interact")
                .field("thres", &self.thres())
                .field("exmode", &self.exmode())
                .field("altex", &self.altex())
                .field("sampleclk", &self.sampleclk())
                .field("exclk", &self.exclk())
                .field("setif", &self.setif())
                .field("offset", &self.offset())
                .field("sample", &self.sample())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Interact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Interact {{ thres: {=u16:?}, exmode: {:?}, altex: {=bool:?}, sampleclk: {:?}, exclk: {:?}, setif: {:?}, offset: {=u8:?}, sample: {:?} }}",
                self.thres(),
                self.exmode(),
                self.altex(),
                self.sampleclk(),
                self.exclk(),
                self.setif(),
                self.offset(),
                self.sample()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Timing(pub u32);
    impl Ch9Timing {
        #[doc = "Set excitation time."]
        #[must_use]
        #[inline(always)]
        pub const fn extime(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Set excitation time."]
        #[inline(always)]
        pub const fn set_extime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Set sample delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sampledly(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0xff;
            val as u8
        }
        #[doc = "Set sample delay."]
        #[inline(always)]
        pub const fn set_sampledly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 6usize)) | (((val as u32) & 0xff) << 6usize);
        }
        #[doc = "Set measure delay."]
        #[must_use]
        #[inline(always)]
        pub const fn measuredly(&self) -> u16 {
            let val = (self.0 >> 14usize) & 0x03ff;
            val as u16
        }
        #[doc = "Set measure delay."]
        #[inline(always)]
        pub const fn set_measuredly(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
        }
    }
    impl Default for Ch9Timing {
        #[inline(always)]
        fn default() -> Ch9Timing {
            Ch9Timing(0)
        }
    }
    impl core::fmt::Debug for Ch9Timing {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Timing")
                .field("extime", &self.extime())
                .field("sampledly", &self.sampledly())
                .field("measuredly", &self.measuredly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Timing {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Timing {{ extime: {=u8:?}, sampledly: {=u8:?}, measuredly: {=u16:?} }}",
                self.extime(),
                self.sampledly(),
                self.measuredly()
            )
        }
    }
    #[doc = "Channel enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chen(pub u32);
    impl Chen {
        #[doc = "Enable scan channel."]
        #[must_use]
        #[inline(always)]
        pub const fn chen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Enable scan channel."]
        #[inline(always)]
        pub const fn set_chen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Chen {
        #[inline(always)]
        fn default() -> Chen {
            Chen(0)
        }
    }
    impl core::fmt::Debug for Chen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chen").field("chen", &self.chen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chen {{ chen: {=u16:?} }}", self.chen())
        }
    }
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Start scanning of sensors."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start scanning of sensors."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop scanning of sensors."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop scanning of sensors."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Start decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Start decoder."]
        #[inline(always)]
        pub const fn set_decode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear result buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn clearbuf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear result buffer."]
        #[inline(always)]
        pub const fn set_clearbuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    impl core::fmt::Debug for Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd")
                .field("start", &self.start())
                .field("stop", &self.stop())
                .field("decode", &self.decode())
                .field("clearbuf", &self.clearbuf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ start: {=bool:?}, stop: {=bool:?}, decode: {=bool:?}, clearbuf: {=bool:?} }}",
                self.start(),
                self.stop(),
                self.decode(),
                self.clearbuf()
            )
        }
    }
    #[doc = "Current channel index."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Curch(pub u32);
    impl Curch {
        #[doc = "Shows the index of the current channel."]
        #[must_use]
        #[inline(always)]
        pub const fn curch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Shows the index of the current channel."]
        #[inline(always)]
        pub const fn set_curch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Curch {
        #[inline(always)]
        fn default() -> Curch {
            Curch(0)
        }
    }
    impl core::fmt::Debug for Curch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Curch").field("curch", &self.curch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Curch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Curch {{ curch: {=u8:?} }}", self.curch())
        }
    }
    #[doc = "Decoder control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Decctrl(pub u32);
    impl Decctrl {
        #[doc = "Disable the decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn decdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable the decoder."]
        #[inline(always)]
        pub const fn set_decdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable decoder to channel interrupt map."]
        #[must_use]
        #[inline(always)]
        pub const fn intmap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable decoder to channel interrupt map."]
        #[inline(always)]
        pub const fn set_intmap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable decoder hysteresis on PRS0 output."]
        #[must_use]
        #[inline(always)]
        pub const fn hystprs0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable decoder hysteresis on PRS0 output."]
        #[inline(always)]
        pub const fn set_hystprs0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable decoder hysteresis on PRS1 output."]
        #[must_use]
        #[inline(always)]
        pub const fn hystprs1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable decoder hysteresis on PRS1 output."]
        #[inline(always)]
        pub const fn set_hystprs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable decoder hysteresis on PRS2 output."]
        #[must_use]
        #[inline(always)]
        pub const fn hystprs2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable decoder hysteresis on PRS2 output."]
        #[inline(always)]
        pub const fn set_hystprs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable decoder hysteresis on interrupt r."]
        #[must_use]
        #[inline(always)]
        pub const fn hystirq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable decoder hysteresis on interrupt r."]
        #[inline(always)]
        pub const fn set_hystirq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable count mode on decoder PRS channel."]
        #[must_use]
        #[inline(always)]
        pub const fn prscnt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable count mode on decoder PRS channel."]
        #[inline(always)]
        pub const fn set_prscnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Decctrl {
        #[inline(always)]
        fn default() -> Decctrl {
            Decctrl(0)
        }
    }
    impl core::fmt::Debug for Decctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Decctrl")
                .field("decdis", &self.decdis())
                .field("intmap", &self.intmap())
                .field("hystprs0", &self.hystprs0())
                .field("hystprs1", &self.hystprs1())
                .field("hystprs2", &self.hystprs2())
                .field("hystirq", &self.hystirq())
                .field("prscnt", &self.prscnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Decctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Decctrl {{ decdis: {=bool:?}, intmap: {=bool:?}, hystprs0: {=bool:?}, hystprs1: {=bool:?}, hystprs2: {=bool:?}, hystirq: {=bool:?}, prscnt: {=bool:?} }}",
                self.decdis(),
                self.intmap(),
                self.hystprs0(),
                self.hystprs1(),
                self.hystprs2(),
                self.hystirq(),
                self.prscnt()
            )
        }
    }
    #[doc = "Current decoder state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Decstate(pub u32);
    impl Decstate {
        #[doc = "Shows the current decoder state."]
        #[must_use]
        #[inline(always)]
        pub const fn decstate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Shows the current decoder state."]
        #[inline(always)]
        pub const fn set_decstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Decstate {
        #[inline(always)]
        fn default() -> Decstate {
            Decstate(0)
        }
    }
    impl core::fmt::Debug for Decstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Decstate").field("decstate", &self.decstate()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Decstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Decstate {{ decstate: {=u8:?} }}", self.decstate())
        }
    }
    #[doc = "Global Enable of LESENSE functions."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Disabling."]
        #[must_use]
        #[inline(always)]
        pub const fn disabling(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Disabling."]
        #[inline(always)]
        pub const fn set_disabling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for En {
        #[inline(always)]
        fn default() -> En {
            En(0)
        }
    }
    impl core::fmt::Debug for En {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("En")
                .field("en", &self.en())
                .field("disabling", &self.disabling())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for En {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "En {{ en: {=bool:?}, disabling: {=bool:?} }}",
                self.en(),
                self.disabling()
            )
        }
    }
    #[doc = "LESENSE evaluation control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evalctrl(pub u32);
    impl Evalctrl {
        #[doc = "Sliding window and step detection size."]
        #[must_use]
        #[inline(always)]
        pub const fn winsize(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sliding window and step detection size."]
        #[inline(always)]
        pub const fn set_winsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Evalctrl {
        #[inline(always)]
        fn default() -> Evalctrl {
            Evalctrl(0)
        }
    }
    impl core::fmt::Debug for Evalctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evalctrl").field("winsize", &self.winsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evalctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Evalctrl {{ winsize: {=u16:?} }}", self.winsize())
        }
    }
    #[doc = "GPIO Idle phase configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idleconf(pub u32);
    impl Idleconf {
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle0(&self) -> super::vals::Chidle0 {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Chidle0::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle0(&mut self, val: super::vals::Chidle0) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle1(&self) -> super::vals::Chidle1 {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Chidle1::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle1(&mut self, val: super::vals::Chidle1) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle2(&self) -> super::vals::Chidle2 {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Chidle2::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle2(&mut self, val: super::vals::Chidle2) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle3(&self) -> super::vals::Chidle3 {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Chidle3::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle3(&mut self, val: super::vals::Chidle3) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle4(&self) -> super::vals::Chidle4 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Chidle4::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle4(&mut self, val: super::vals::Chidle4) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle5(&self) -> super::vals::Chidle5 {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Chidle5::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle5(&mut self, val: super::vals::Chidle5) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle6(&self) -> super::vals::Chidle6 {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Chidle6::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle6(&mut self, val: super::vals::Chidle6) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle7(&self) -> super::vals::Chidle7 {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Chidle7::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle7(&mut self, val: super::vals::Chidle7) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle8(&self) -> super::vals::Chidle8 {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Chidle8::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle8(&mut self, val: super::vals::Chidle8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle9(&self) -> super::vals::Chidle9 {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Chidle9::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle9(&mut self, val: super::vals::Chidle9) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle10(&self) -> super::vals::Chidle10 {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Chidle10::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle10(&mut self, val: super::vals::Chidle10) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle11(&self) -> super::vals::Chidle11 {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Chidle11::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle11(&mut self, val: super::vals::Chidle11) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle12(&self) -> super::vals::Chidle12 {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Chidle12::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle12(&mut self, val: super::vals::Chidle12) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle13(&self) -> super::vals::Chidle13 {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Chidle13::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle13(&mut self, val: super::vals::Chidle13) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle14(&self) -> super::vals::Chidle14 {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Chidle14::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle14(&mut self, val: super::vals::Chidle14) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Channel IDLE configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn chidle15(&self) -> super::vals::Chidle15 {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Chidle15::from_bits(val as u8)
        }
        #[doc = "Channel IDLE configuration."]
        #[inline(always)]
        pub const fn set_chidle15(&mut self, val: super::vals::Chidle15) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Idleconf {
        #[inline(always)]
        fn default() -> Idleconf {
            Idleconf(0)
        }
    }
    impl core::fmt::Debug for Idleconf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idleconf")
                .field("chidle0", &self.chidle0())
                .field("chidle1", &self.chidle1())
                .field("chidle2", &self.chidle2())
                .field("chidle3", &self.chidle3())
                .field("chidle4", &self.chidle4())
                .field("chidle5", &self.chidle5())
                .field("chidle6", &self.chidle6())
                .field("chidle7", &self.chidle7())
                .field("chidle8", &self.chidle8())
                .field("chidle9", &self.chidle9())
                .field("chidle10", &self.chidle10())
                .field("chidle11", &self.chidle11())
                .field("chidle12", &self.chidle12())
                .field("chidle13", &self.chidle13())
                .field("chidle14", &self.chidle14())
                .field("chidle15", &self.chidle15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idleconf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idleconf {{ chidle0: {:?}, chidle1: {:?}, chidle2: {:?}, chidle3: {:?}, chidle4: {:?}, chidle5: {:?}, chidle6: {:?}, chidle7: {:?}, chidle8: {:?}, chidle9: {:?}, chidle10: {:?}, chidle11: {:?}, chidle12: {:?}, chidle13: {:?}, chidle14: {:?}, chidle15: {:?} }}",
                self.chidle0(),
                self.chidle1(),
                self.chidle2(),
                self.chidle3(),
                self.chidle4(),
                self.chidle5(),
                self.chidle6(),
                self.chidle7(),
                self.chidle8(),
                self.chidle9(),
                self.chidle10(),
                self.chidle11(),
                self.chidle12(),
                self.chidle13(),
                self.chidle14(),
                self.chidle15()
            )
        }
    }
    #[doc = "Interrupt Enables."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Scan Complete."]
        #[must_use]
        #[inline(always)]
        pub const fn scandone(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Complete."]
        #[inline(always)]
        pub const fn set_scandone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn dec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder."]
        #[inline(always)]
        pub const fn set_dec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Result Watermark Level."]
        #[must_use]
        #[inline(always)]
        pub const fn reswl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Result Watermark Level."]
        #[inline(always)]
        pub const fn set_reswl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Result Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn resof(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Result Overflow."]
        #[inline(always)]
        pub const fn set_resof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Counter Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn cntof(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Counter Overflow."]
        #[inline(always)]
        pub const fn set_cntof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Result Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn resuf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Result Underflow."]
        #[inline(always)]
        pub const fn set_resuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Ien {
        #[inline(always)]
        fn default() -> Ien {
            Ien(0)
        }
    }
    impl core::fmt::Debug for Ien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ien")
                .field("ch0", &self.ch0())
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .field("ch5", &self.ch5())
                .field("ch6", &self.ch6())
                .field("ch7", &self.ch7())
                .field("ch8", &self.ch8())
                .field("ch9", &self.ch9())
                .field("ch10", &self.ch10())
                .field("ch11", &self.ch11())
                .field("ch12", &self.ch12())
                .field("ch13", &self.ch13())
                .field("ch14", &self.ch14())
                .field("ch15", &self.ch15())
                .field("scandone", &self.scandone())
                .field("dec", &self.dec())
                .field("reswl", &self.reswl())
                .field("resof", &self.resof())
                .field("cntof", &self.cntof())
                .field("resuf", &self.resuf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ ch0: {=bool:?}, ch1: {=bool:?}, ch2: {=bool:?}, ch3: {=bool:?}, ch4: {=bool:?}, ch5: {=bool:?}, ch6: {=bool:?}, ch7: {=bool:?}, ch8: {=bool:?}, ch9: {=bool:?}, ch10: {=bool:?}, ch11: {=bool:?}, ch12: {=bool:?}, ch13: {=bool:?}, ch14: {=bool:?}, ch15: {=bool:?}, scandone: {=bool:?}, dec: {=bool:?}, reswl: {=bool:?}, resof: {=bool:?}, cntof: {=bool:?}, resuf: {=bool:?} }}",
                self.ch0(),
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4(),
                self.ch5(),
                self.ch6(),
                self.ch7(),
                self.ch8(),
                self.ch9(),
                self.ch10(),
                self.ch11(),
                self.ch12(),
                self.ch13(),
                self.ch14(),
                self.ch15(),
                self.scandone(),
                self.dec(),
                self.reswl(),
                self.resof(),
                self.cntof(),
                self.resuf()
            )
        }
    }
    #[doc = "Interrupt Flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Channel."]
        #[inline(always)]
        pub const fn set_ch15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Scan Done."]
        #[must_use]
        #[inline(always)]
        pub const fn scandone(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Done."]
        #[inline(always)]
        pub const fn set_scandone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Decoder."]
        #[must_use]
        #[inline(always)]
        pub const fn dec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder."]
        #[inline(always)]
        pub const fn set_dec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Result Watermark Level."]
        #[must_use]
        #[inline(always)]
        pub const fn reswl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Result Watermark Level."]
        #[inline(always)]
        pub const fn set_reswl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Result Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn resof(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Result Overflow."]
        #[inline(always)]
        pub const fn set_resof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Counter Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn cntof(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Counter Overflow."]
        #[inline(always)]
        pub const fn set_cntof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Result Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn resuf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Result Underflow."]
        #[inline(always)]
        pub const fn set_resuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for If {
        #[inline(always)]
        fn default() -> If {
            If(0)
        }
    }
    impl core::fmt::Debug for If {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("If")
                .field("ch0", &self.ch0())
                .field("ch1", &self.ch1())
                .field("ch2", &self.ch2())
                .field("ch3", &self.ch3())
                .field("ch4", &self.ch4())
                .field("ch5", &self.ch5())
                .field("ch6", &self.ch6())
                .field("ch7", &self.ch7())
                .field("ch8", &self.ch8())
                .field("ch9", &self.ch9())
                .field("ch10", &self.ch10())
                .field("ch11", &self.ch11())
                .field("ch12", &self.ch12())
                .field("ch13", &self.ch13())
                .field("ch14", &self.ch14())
                .field("ch15", &self.ch15())
                .field("scandone", &self.scandone())
                .field("dec", &self.dec())
                .field("reswl", &self.reswl())
                .field("resof", &self.resof())
                .field("cntof", &self.cntof())
                .field("resuf", &self.resuf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ ch0: {=bool:?}, ch1: {=bool:?}, ch2: {=bool:?}, ch3: {=bool:?}, ch4: {=bool:?}, ch5: {=bool:?}, ch6: {=bool:?}, ch7: {=bool:?}, ch8: {=bool:?}, ch9: {=bool:?}, ch10: {=bool:?}, ch11: {=bool:?}, ch12: {=bool:?}, ch13: {=bool:?}, ch14: {=bool:?}, ch15: {=bool:?}, scandone: {=bool:?}, dec: {=bool:?}, reswl: {=bool:?}, resof: {=bool:?}, cntof: {=bool:?}, resuf: {=bool:?} }}",
                self.ch0(),
                self.ch1(),
                self.ch2(),
                self.ch3(),
                self.ch4(),
                self.ch5(),
                self.ch6(),
                self.ch7(),
                self.ch8(),
                self.ch9(),
                self.ch10(),
                self.ch11(),
                self.ch12(),
                self.ch13(),
                self.ch14(),
                self.ch15(),
                self.scandone(),
                self.dec(),
                self.reswl(),
                self.resof(),
                self.cntof(),
                self.resuf()
            )
        }
    }
    #[doc = "IPVERSION."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IPVERSION."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IPVERSION."]
        #[inline(always)]
        pub const fn set_ipversion(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipversion {
        #[inline(always)]
        fn default() -> Ipversion {
            Ipversion(0)
        }
    }
    impl core::fmt::Debug for Ipversion {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipversion")
                .field("ipversion", &self.ipversion())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipversion {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipversion {{ ipversion: {=u32:?} }}", self.ipversion())
        }
    }
    #[doc = "Peripheral Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perctrl(pub u32);
    impl Perctrl {
        #[doc = "DAC CH0 data selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dacch0data(&self) -> super::vals::Dacch0data {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Dacch0data::from_bits(val as u8)
        }
        #[doc = "DAC CH0 data selection."]
        #[inline(always)]
        pub const fn set_dacch0data(&mut self, val: super::vals::Dacch0data) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "DAC startup configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn dacstartup(&self) -> super::vals::Dacstartup {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Dacstartup::from_bits(val as u8)
        }
        #[doc = "DAC startup configuration."]
        #[inline(always)]
        pub const fn set_dacstartup(&mut self, val: super::vals::Dacstartup) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "DAC conversion trigger configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn dacconvtrig(&self) -> super::vals::Dacconvtrig {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Dacconvtrig::from_bits(val as u8)
        }
        #[doc = "DAC conversion trigger configuration."]
        #[inline(always)]
        pub const fn set_dacconvtrig(&mut self, val: super::vals::Dacconvtrig) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "ACMP0 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0mode(&self) -> super::vals::Acmp0mode {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Acmp0mode::from_bits(val as u8)
        }
        #[doc = "ACMP0 mode."]
        #[inline(always)]
        pub const fn set_acmp0mode(&mut self, val: super::vals::Acmp0mode) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "ACMP1 mode."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1mode(&self) -> super::vals::Acmp1mode {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Acmp1mode::from_bits(val as u8)
        }
        #[doc = "ACMP1 mode."]
        #[inline(always)]
        pub const fn set_acmp1mode(&mut self, val: super::vals::Acmp1mode) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Invert analog comparator 0 output."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0inv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Invert analog comparator 0 output."]
        #[inline(always)]
        pub const fn set_acmp0inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Invert analog comparator 1 output."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1inv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Invert analog comparator 1 output."]
        #[inline(always)]
        pub const fn set_acmp1inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Perctrl {
        #[inline(always)]
        fn default() -> Perctrl {
            Perctrl(0)
        }
    }
    impl core::fmt::Debug for Perctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perctrl")
                .field("dacch0data", &self.dacch0data())
                .field("dacstartup", &self.dacstartup())
                .field("dacconvtrig", &self.dacconvtrig())
                .field("acmp0mode", &self.acmp0mode())
                .field("acmp1mode", &self.acmp1mode())
                .field("acmp0inv", &self.acmp0inv())
                .field("acmp1inv", &self.acmp1inv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Perctrl {{ dacch0data: {:?}, dacstartup: {:?}, dacconvtrig: {:?}, acmp0mode: {:?}, acmp1mode: {:?}, acmp0inv: {=bool:?}, acmp1inv: {=bool:?} }}",
                self.dacch0data(),
                self.dacstartup(),
                self.dacconvtrig(),
                self.acmp0mode(),
                self.acmp1mode(),
                self.acmp0inv(),
                self.acmp1inv()
            )
        }
    }
    #[doc = "PRS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prsctrl(pub u32);
    impl Prsctrl {
        #[doc = "Decoder state compare value."]
        #[must_use]
        #[inline(always)]
        pub const fn deccmpval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Decoder state compare value."]
        #[inline(always)]
        pub const fn set_deccmpval(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Decoder state compare value mask."]
        #[must_use]
        #[inline(always)]
        pub const fn deccmpmask(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Decoder state compare value mask."]
        #[inline(always)]
        pub const fn set_deccmpmask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Enable PRS output DECCMP."]
        #[must_use]
        #[inline(always)]
        pub const fn deccmpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable PRS output DECCMP."]
        #[inline(always)]
        pub const fn set_deccmpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Prsctrl {
        #[inline(always)]
        fn default() -> Prsctrl {
            Prsctrl(0)
        }
    }
    impl core::fmt::Debug for Prsctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prsctrl")
                .field("deccmpval", &self.deccmpval())
                .field("deccmpmask", &self.deccmpmask())
                .field("deccmpen", &self.deccmpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prsctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prsctrl {{ deccmpval: {=u8:?}, deccmpmask: {=u8:?}, deccmpen: {=bool:?} }}",
                self.deccmpval(),
                self.deccmpmask(),
                self.deccmpen()
            )
        }
    }
    #[doc = "Result FIFO Count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rescount(pub u32);
    impl Rescount {
        #[doc = "Result Fifo Count."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Result Fifo Count."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Rescount {
        #[inline(always)]
        fn default() -> Rescount {
            Rescount(0)
        }
    }
    impl core::fmt::Debug for Rescount {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rescount").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rescount {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rescount {{ count: {=u8:?} }}", self.count())
        }
    }
    #[doc = "Result Fifo."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resfifo(pub u32);
    impl Resfifo {
        #[doc = "Result data and source."]
        #[must_use]
        #[inline(always)]
        pub const fn bufdatasrc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Result data and source."]
        #[inline(always)]
        pub const fn set_bufdatasrc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Resfifo {
        #[inline(always)]
        fn default() -> Resfifo {
            Resfifo(0)
        }
    }
    impl core::fmt::Debug for Resfifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Resfifo")
                .field("bufdatasrc", &self.bufdatasrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Resfifo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Resfifo {{ bufdatasrc: {=u32:?} }}", self.bufdatasrc())
        }
    }
    #[doc = "Scan result register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scanres(pub u32);
    impl Scanres {
        #[doc = "Scan results."]
        #[must_use]
        #[inline(always)]
        pub const fn scanres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Scan results."]
        #[inline(always)]
        pub const fn set_scanres(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Direction of previous step detection."]
        #[must_use]
        #[inline(always)]
        pub const fn stepdir(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Direction of previous step detection."]
        #[inline(always)]
        pub const fn set_stepdir(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Scanres {
        #[inline(always)]
        fn default() -> Scanres {
            Scanres(0)
        }
    }
    impl core::fmt::Debug for Scanres {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scanres")
                .field("scanres", &self.scanres())
                .field("stepdir", &self.stepdir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scanres {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scanres {{ scanres: {=u16:?}, stepdir: {=u16:?} }}",
                self.scanres(),
                self.stepdir()
            )
        }
    }
    #[doc = "Decoder input register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sensorstate(pub u32);
    impl Sensorstate {
        #[doc = "Sensor State."]
        #[must_use]
        #[inline(always)]
        pub const fn sensorstate(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Sensor State."]
        #[inline(always)]
        pub const fn set_sensorstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Sensorstate {
        #[inline(always)]
        fn default() -> Sensorstate {
            Sensorstate(0)
        }
    }
    impl core::fmt::Debug for Sensorstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sensorstate")
                .field("sensorstate", &self.sensorstate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sensorstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sensorstate {{ sensorstate: {=u8:?} }}", self.sensorstate())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StArc(pub u32);
    impl StArc {
        #[doc = "Sensor compare value."]
        #[must_use]
        #[inline(always)]
        pub const fn scomp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Sensor compare value."]
        #[inline(always)]
        pub const fn set_scomp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Sensor mask."]
        #[must_use]
        #[inline(always)]
        pub const fn smask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Sensor mask."]
        #[inline(always)]
        pub const fn set_smask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Current State."]
        #[must_use]
        #[inline(always)]
        pub const fn curstate(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Current State."]
        #[inline(always)]
        pub const fn set_curstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Configure transition action in normal mode."]
        #[must_use]
        #[inline(always)]
        pub const fn prsact(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Configure transition action in normal mode."]
        #[inline(always)]
        pub const fn set_prsact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Next state index."]
        #[must_use]
        #[inline(always)]
        pub const fn nextstate(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Next state index."]
        #[inline(always)]
        pub const fn set_nextstate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Set interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn setif(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Set interrupt flag."]
        #[inline(always)]
        pub const fn set_setif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for StArc {
        #[inline(always)]
        fn default() -> StArc {
            StArc(0)
        }
    }
    impl core::fmt::Debug for StArc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StArc")
                .field("scomp", &self.scomp())
                .field("smask", &self.smask())
                .field("curstate", &self.curstate())
                .field("prsact", &self.prsact())
                .field("nextstate", &self.nextstate())
                .field("setif", &self.setif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StArc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "StArc {{ scomp: {=u8:?}, smask: {=u8:?}, curstate: {=u8:?}, prsact: {=u8:?}, nextstate: {=u8:?}, setif: {=bool:?} }}",
                self.scomp(),
                self.smask(),
                self.curstate(),
                self.prsact(),
                self.nextstate(),
                self.setif()
            )
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Result fifo valid."]
        #[must_use]
        #[inline(always)]
        pub const fn resfifov(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Result fifo valid."]
        #[inline(always)]
        pub const fn set_resfifov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Result fifo full."]
        #[must_use]
        #[inline(always)]
        pub const fn resfifofull(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Result fifo full."]
        #[inline(always)]
        pub const fn set_resfifofull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LESENSE scan active."]
        #[must_use]
        #[inline(always)]
        pub const fn scanactive(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LESENSE scan active."]
        #[inline(always)]
        pub const fn set_scanactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LESENSE periodic counter running."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LESENSE periodic counter running."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FIFO Read Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn readbusy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Read Busy."]
        #[inline(always)]
        pub const fn set_readbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO Flushing."]
        #[must_use]
        #[inline(always)]
        pub const fn flushing(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Flushing."]
        #[inline(always)]
        pub const fn set_flushing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("resfifov", &self.resfifov())
                .field("resfifofull", &self.resfifofull())
                .field("scanactive", &self.scanactive())
                .field("running", &self.running())
                .field("readbusy", &self.readbusy())
                .field("flushing", &self.flushing())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ resfifov: {=bool:?}, resfifofull: {=bool:?}, scanactive: {=bool:?}, running: {=bool:?}, readbusy: {=bool:?}, flushing: {=bool:?} }}",
                self.resfifov(),
                self.resfifofull(),
                self.scanactive(),
                self.running(),
                self.readbusy(),
                self.flushing()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swrst(pub u32);
    impl Swrst {
        #[doc = "Software reset command."]
        #[must_use]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset command."]
        #[inline(always)]
        pub const fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software reset busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn resetting(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset busy status."]
        #[inline(always)]
        pub const fn set_resetting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Swrst {
        #[inline(always)]
        fn default() -> Swrst {
            Swrst(0)
        }
    }
    impl core::fmt::Debug for Swrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swrst")
                .field("swrst", &self.swrst())
                .field("resetting", &self.resetting())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swrst {{ swrst: {=bool:?}, resetting: {=bool:?} }}",
                self.swrst(),
                self.resetting()
            )
        }
    }
    #[doc = "Synchronization Busy Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "Command."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Syncbusy {
        #[inline(always)]
        fn default() -> Syncbusy {
            Syncbusy(0)
        }
    }
    impl core::fmt::Debug for Syncbusy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Syncbusy").field("cmd", &self.cmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Syncbusy {{ cmd: {=bool:?} }}", self.cmd())
        }
    }
    #[doc = "Timing Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timctrl(pub u32);
    impl Timctrl {
        #[doc = "Prescaling factor for high frequency tim."]
        #[must_use]
        #[inline(always)]
        pub const fn auxpresc(&self) -> super::vals::Auxpresc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Auxpresc::from_bits(val as u8)
        }
        #[doc = "Prescaling factor for high frequency tim."]
        #[inline(always)]
        pub const fn set_auxpresc(&mut self, val: super::vals::Auxpresc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Prescaling factor for low frequency time."]
        #[must_use]
        #[inline(always)]
        pub const fn lfpresc(&self) -> super::vals::Lfpresc {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Lfpresc::from_bits(val as u8)
        }
        #[doc = "Prescaling factor for low frequency time."]
        #[inline(always)]
        pub const fn set_lfpresc(&mut self, val: super::vals::Lfpresc) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Period counter prescaling."]
        #[must_use]
        #[inline(always)]
        pub const fn pcpresc(&self) -> super::vals::Pcpresc {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Pcpresc::from_bits(val as u8)
        }
        #[doc = "Period counter prescaling."]
        #[inline(always)]
        pub const fn set_pcpresc(&mut self, val: super::vals::Pcpresc) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Period counter top value."]
        #[must_use]
        #[inline(always)]
        pub const fn pctop(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Period counter top value."]
        #[inline(always)]
        pub const fn set_pctop(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "Start delay configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn startdly(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Start delay configuration."]
        #[inline(always)]
        pub const fn set_startdly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "AUX startup config."]
        #[must_use]
        #[inline(always)]
        pub const fn auxstartup(&self) -> super::vals::Auxstartup {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Auxstartup::from_bits(val as u8)
        }
        #[doc = "AUX startup config."]
        #[inline(always)]
        pub const fn set_auxstartup(&mut self, val: super::vals::Auxstartup) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Timctrl {
        #[inline(always)]
        fn default() -> Timctrl {
            Timctrl(0)
        }
    }
    impl core::fmt::Debug for Timctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timctrl")
                .field("auxpresc", &self.auxpresc())
                .field("lfpresc", &self.lfpresc())
                .field("pcpresc", &self.pcpresc())
                .field("pctop", &self.pctop())
                .field("startdly", &self.startdly())
                .field("auxstartup", &self.auxstartup())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timctrl {{ auxpresc: {:?}, lfpresc: {:?}, pcpresc: {:?}, pctop: {=u8:?}, startdly: {=u8:?}, auxstartup: {:?} }}",
                self.auxpresc(),
                self.lfpresc(),
                self.pcpresc(),
                self.pctop(),
                self.startdly(),
                self.auxstartup()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Acmp0mode {
        #[doc = "LESENSE controls POSSEL of ACMP0."]
        Mux = 0x0,
        #[doc = "LESENSE controls POSSEL and reference divider of ACMP0."]
        Muxthres = 0x01,
    }
    impl Acmp0mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Acmp0mode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Acmp0mode {
        #[inline(always)]
        fn from(val: u8) -> Acmp0mode {
            Acmp0mode::from_bits(val)
        }
    }
    impl From<Acmp0mode> for u8 {
        #[inline(always)]
        fn from(val: Acmp0mode) -> u8 {
            Acmp0mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Acmp1mode {
        #[doc = "LESENSE controls the POSSEL of ACMP1."]
        Mux = 0x0,
        #[doc = "LESENSE POSSEL and reference divider of ACMP1."]
        Muxthres = 0x01,
    }
    impl Acmp1mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Acmp1mode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Acmp1mode {
        #[inline(always)]
        fn from(val: u8) -> Acmp1mode {
            Acmp1mode::from_bits(val)
        }
    }
    impl From<Acmp1mode> for u8 {
        #[inline(always)]
        fn from(val: Acmp1mode) -> u8 {
            Acmp1mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Auxpresc {
        #[doc = "High frequency timer is clocked at LESENSEHFCLK/1."]
        Div1 = 0x0,
        #[doc = "High frequency timer is clocked at LESENSEHFCLK/2."]
        Div2 = 0x01,
        #[doc = "High frequency timer is clocked at LESENSEHFCLK/4."]
        Div4 = 0x02,
        #[doc = "High frequency timer is clocked at LESENSEHFCLK/8."]
        Div8 = 0x03,
    }
    impl Auxpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Auxpresc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Auxpresc {
        #[inline(always)]
        fn from(val: u8) -> Auxpresc {
            Auxpresc::from_bits(val)
        }
    }
    impl From<Auxpresc> for u8 {
        #[inline(always)]
        fn from(val: Auxpresc) -> u8 {
            Auxpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Auxstartup {
        #[doc = "Request oscillator .5 LESENSECLK cycle before sensing starts."]
        Predemand = 0x0,
        #[doc = "Request oscillator at sensing time."]
        Ondemand = 0x01,
    }
    impl Auxstartup {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Auxstartup {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Auxstartup {
        #[inline(always)]
        fn from(val: u8) -> Auxstartup {
            Auxstartup::from_bits(val)
        }
    }
    impl From<Auxstartup> for u8 {
        #[inline(always)]
        fn from(val: Auxstartup) -> u8 {
            Auxstartup::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch0EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch0EvalcfgComp {
            Ch0EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch0EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch0EvalcfgComp) -> u8 {
            Ch0EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch0EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch0EvalcfgMode {
            Ch0EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch0EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch0EvalcfgMode) -> u8 {
            Ch0EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch0EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch0EvalcfgStrsample {
            Ch0EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch0EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch0EvalcfgStrsample) -> u8 {
            Ch0EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch0InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch0InteractExclk {
            Ch0InteractExclk::from_bits(val)
        }
    }
    impl From<Ch0InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch0InteractExclk) -> u8 {
            Ch0InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch0InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch0InteractExmode {
            Ch0InteractExmode::from_bits(val)
        }
    }
    impl From<Ch0InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch0InteractExmode) -> u8 {
            Ch0InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch0InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch0InteractSample {
            Ch0InteractSample::from_bits(val)
        }
    }
    impl From<Ch0InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch0InteractSample) -> u8 {
            Ch0InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch0InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch0InteractSampleclk {
            Ch0InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch0InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch0InteractSampleclk) -> u8 {
            Ch0InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch0InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch0InteractSetif {
            Ch0InteractSetif::from_bits(val)
        }
    }
    impl From<Ch0InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch0InteractSetif) -> u8 {
            Ch0InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch10EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch10EvalcfgComp {
            Ch10EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch10EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch10EvalcfgComp) -> u8 {
            Ch10EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch10EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch10EvalcfgMode {
            Ch10EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch10EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch10EvalcfgMode) -> u8 {
            Ch10EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch10EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch10EvalcfgStrsample {
            Ch10EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch10EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch10EvalcfgStrsample) -> u8 {
            Ch10EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch10InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch10InteractExclk {
            Ch10InteractExclk::from_bits(val)
        }
    }
    impl From<Ch10InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch10InteractExclk) -> u8 {
            Ch10InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch10InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch10InteractExmode {
            Ch10InteractExmode::from_bits(val)
        }
    }
    impl From<Ch10InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch10InteractExmode) -> u8 {
            Ch10InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch10InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch10InteractSample {
            Ch10InteractSample::from_bits(val)
        }
    }
    impl From<Ch10InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch10InteractSample) -> u8 {
            Ch10InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch10InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch10InteractSampleclk {
            Ch10InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch10InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch10InteractSampleclk) -> u8 {
            Ch10InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch10InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch10InteractSetif {
            Ch10InteractSetif::from_bits(val)
        }
    }
    impl From<Ch10InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch10InteractSetif) -> u8 {
            Ch10InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch11EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch11EvalcfgComp {
            Ch11EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch11EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch11EvalcfgComp) -> u8 {
            Ch11EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch11EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch11EvalcfgMode {
            Ch11EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch11EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch11EvalcfgMode) -> u8 {
            Ch11EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch11EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch11EvalcfgStrsample {
            Ch11EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch11EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch11EvalcfgStrsample) -> u8 {
            Ch11EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch11InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch11InteractExclk {
            Ch11InteractExclk::from_bits(val)
        }
    }
    impl From<Ch11InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch11InteractExclk) -> u8 {
            Ch11InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch11InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch11InteractExmode {
            Ch11InteractExmode::from_bits(val)
        }
    }
    impl From<Ch11InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch11InteractExmode) -> u8 {
            Ch11InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch11InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch11InteractSample {
            Ch11InteractSample::from_bits(val)
        }
    }
    impl From<Ch11InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch11InteractSample) -> u8 {
            Ch11InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch11InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch11InteractSampleclk {
            Ch11InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch11InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch11InteractSampleclk) -> u8 {
            Ch11InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch11InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch11InteractSetif {
            Ch11InteractSetif::from_bits(val)
        }
    }
    impl From<Ch11InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch11InteractSetif) -> u8 {
            Ch11InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch12EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch12EvalcfgComp {
            Ch12EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch12EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch12EvalcfgComp) -> u8 {
            Ch12EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch12EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch12EvalcfgMode {
            Ch12EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch12EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch12EvalcfgMode) -> u8 {
            Ch12EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch12EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch12EvalcfgStrsample {
            Ch12EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch12EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch12EvalcfgStrsample) -> u8 {
            Ch12EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch12InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch12InteractExclk {
            Ch12InteractExclk::from_bits(val)
        }
    }
    impl From<Ch12InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch12InteractExclk) -> u8 {
            Ch12InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch12InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch12InteractExmode {
            Ch12InteractExmode::from_bits(val)
        }
    }
    impl From<Ch12InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch12InteractExmode) -> u8 {
            Ch12InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch12InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch12InteractSample {
            Ch12InteractSample::from_bits(val)
        }
    }
    impl From<Ch12InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch12InteractSample) -> u8 {
            Ch12InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch12InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch12InteractSampleclk {
            Ch12InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch12InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch12InteractSampleclk) -> u8 {
            Ch12InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch12InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch12InteractSetif {
            Ch12InteractSetif::from_bits(val)
        }
    }
    impl From<Ch12InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch12InteractSetif) -> u8 {
            Ch12InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch13EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch13EvalcfgComp {
            Ch13EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch13EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch13EvalcfgComp) -> u8 {
            Ch13EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch13EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch13EvalcfgMode {
            Ch13EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch13EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch13EvalcfgMode) -> u8 {
            Ch13EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch13EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch13EvalcfgStrsample {
            Ch13EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch13EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch13EvalcfgStrsample) -> u8 {
            Ch13EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch13InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch13InteractExclk {
            Ch13InteractExclk::from_bits(val)
        }
    }
    impl From<Ch13InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch13InteractExclk) -> u8 {
            Ch13InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch13InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch13InteractExmode {
            Ch13InteractExmode::from_bits(val)
        }
    }
    impl From<Ch13InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch13InteractExmode) -> u8 {
            Ch13InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch13InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch13InteractSample {
            Ch13InteractSample::from_bits(val)
        }
    }
    impl From<Ch13InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch13InteractSample) -> u8 {
            Ch13InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch13InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch13InteractSampleclk {
            Ch13InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch13InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch13InteractSampleclk) -> u8 {
            Ch13InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch13InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch13InteractSetif {
            Ch13InteractSetif::from_bits(val)
        }
    }
    impl From<Ch13InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch13InteractSetif) -> u8 {
            Ch13InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch14EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch14EvalcfgComp {
            Ch14EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch14EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch14EvalcfgComp) -> u8 {
            Ch14EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch14EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch14EvalcfgMode {
            Ch14EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch14EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch14EvalcfgMode) -> u8 {
            Ch14EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch14EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch14EvalcfgStrsample {
            Ch14EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch14EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch14EvalcfgStrsample) -> u8 {
            Ch14EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch14InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch14InteractExclk {
            Ch14InteractExclk::from_bits(val)
        }
    }
    impl From<Ch14InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch14InteractExclk) -> u8 {
            Ch14InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch14InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch14InteractExmode {
            Ch14InteractExmode::from_bits(val)
        }
    }
    impl From<Ch14InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch14InteractExmode) -> u8 {
            Ch14InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch14InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch14InteractSample {
            Ch14InteractSample::from_bits(val)
        }
    }
    impl From<Ch14InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch14InteractSample) -> u8 {
            Ch14InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch14InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch14InteractSampleclk {
            Ch14InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch14InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch14InteractSampleclk) -> u8 {
            Ch14InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch14InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch14InteractSetif {
            Ch14InteractSetif::from_bits(val)
        }
    }
    impl From<Ch14InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch14InteractSetif) -> u8 {
            Ch14InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch15EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch15EvalcfgComp {
            Ch15EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch15EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch15EvalcfgComp) -> u8 {
            Ch15EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch15EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch15EvalcfgMode {
            Ch15EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch15EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch15EvalcfgMode) -> u8 {
            Ch15EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch15EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch15EvalcfgStrsample {
            Ch15EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch15EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch15EvalcfgStrsample) -> u8 {
            Ch15EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch15InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch15InteractExclk {
            Ch15InteractExclk::from_bits(val)
        }
    }
    impl From<Ch15InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch15InteractExclk) -> u8 {
            Ch15InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch15InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch15InteractExmode {
            Ch15InteractExmode::from_bits(val)
        }
    }
    impl From<Ch15InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch15InteractExmode) -> u8 {
            Ch15InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch15InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch15InteractSample {
            Ch15InteractSample::from_bits(val)
        }
    }
    impl From<Ch15InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch15InteractSample) -> u8 {
            Ch15InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch15InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch15InteractSampleclk {
            Ch15InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch15InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch15InteractSampleclk) -> u8 {
            Ch15InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch15InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch15InteractSetif {
            Ch15InteractSetif::from_bits(val)
        }
    }
    impl From<Ch15InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch15InteractSetif) -> u8 {
            Ch15InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch1EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch1EvalcfgComp {
            Ch1EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch1EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch1EvalcfgComp) -> u8 {
            Ch1EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch1EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch1EvalcfgMode {
            Ch1EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch1EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch1EvalcfgMode) -> u8 {
            Ch1EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch1EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch1EvalcfgStrsample {
            Ch1EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch1EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch1EvalcfgStrsample) -> u8 {
            Ch1EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch1InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch1InteractExclk {
            Ch1InteractExclk::from_bits(val)
        }
    }
    impl From<Ch1InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch1InteractExclk) -> u8 {
            Ch1InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch1InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch1InteractExmode {
            Ch1InteractExmode::from_bits(val)
        }
    }
    impl From<Ch1InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch1InteractExmode) -> u8 {
            Ch1InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch1InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch1InteractSample {
            Ch1InteractSample::from_bits(val)
        }
    }
    impl From<Ch1InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch1InteractSample) -> u8 {
            Ch1InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch1InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch1InteractSampleclk {
            Ch1InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch1InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch1InteractSampleclk) -> u8 {
            Ch1InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch1InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch1InteractSetif {
            Ch1InteractSetif::from_bits(val)
        }
    }
    impl From<Ch1InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch1InteractSetif) -> u8 {
            Ch1InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch2EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch2EvalcfgComp {
            Ch2EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch2EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch2EvalcfgComp) -> u8 {
            Ch2EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch2EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch2EvalcfgMode {
            Ch2EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch2EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch2EvalcfgMode) -> u8 {
            Ch2EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch2EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch2EvalcfgStrsample {
            Ch2EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch2EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch2EvalcfgStrsample) -> u8 {
            Ch2EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch2InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch2InteractExclk {
            Ch2InteractExclk::from_bits(val)
        }
    }
    impl From<Ch2InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch2InteractExclk) -> u8 {
            Ch2InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch2InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch2InteractExmode {
            Ch2InteractExmode::from_bits(val)
        }
    }
    impl From<Ch2InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch2InteractExmode) -> u8 {
            Ch2InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch2InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch2InteractSample {
            Ch2InteractSample::from_bits(val)
        }
    }
    impl From<Ch2InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch2InteractSample) -> u8 {
            Ch2InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch2InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch2InteractSampleclk {
            Ch2InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch2InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch2InteractSampleclk) -> u8 {
            Ch2InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch2InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch2InteractSetif {
            Ch2InteractSetif::from_bits(val)
        }
    }
    impl From<Ch2InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch2InteractSetif) -> u8 {
            Ch2InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch3EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch3EvalcfgComp {
            Ch3EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch3EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch3EvalcfgComp) -> u8 {
            Ch3EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch3EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch3EvalcfgMode {
            Ch3EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch3EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch3EvalcfgMode) -> u8 {
            Ch3EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch3EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch3EvalcfgStrsample {
            Ch3EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch3EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch3EvalcfgStrsample) -> u8 {
            Ch3EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch3InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch3InteractExclk {
            Ch3InteractExclk::from_bits(val)
        }
    }
    impl From<Ch3InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch3InteractExclk) -> u8 {
            Ch3InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch3InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch3InteractExmode {
            Ch3InteractExmode::from_bits(val)
        }
    }
    impl From<Ch3InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch3InteractExmode) -> u8 {
            Ch3InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch3InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch3InteractSample {
            Ch3InteractSample::from_bits(val)
        }
    }
    impl From<Ch3InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch3InteractSample) -> u8 {
            Ch3InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch3InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch3InteractSampleclk {
            Ch3InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch3InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch3InteractSampleclk) -> u8 {
            Ch3InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch3InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch3InteractSetif {
            Ch3InteractSetif::from_bits(val)
        }
    }
    impl From<Ch3InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch3InteractSetif) -> u8 {
            Ch3InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch4EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch4EvalcfgComp {
            Ch4EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch4EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch4EvalcfgComp) -> u8 {
            Ch4EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch4EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch4EvalcfgMode {
            Ch4EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch4EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch4EvalcfgMode) -> u8 {
            Ch4EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch4EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch4EvalcfgStrsample {
            Ch4EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch4EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch4EvalcfgStrsample) -> u8 {
            Ch4EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch4InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch4InteractExclk {
            Ch4InteractExclk::from_bits(val)
        }
    }
    impl From<Ch4InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch4InteractExclk) -> u8 {
            Ch4InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch4InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch4InteractExmode {
            Ch4InteractExmode::from_bits(val)
        }
    }
    impl From<Ch4InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch4InteractExmode) -> u8 {
            Ch4InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch4InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch4InteractSample {
            Ch4InteractSample::from_bits(val)
        }
    }
    impl From<Ch4InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch4InteractSample) -> u8 {
            Ch4InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch4InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch4InteractSampleclk {
            Ch4InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch4InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch4InteractSampleclk) -> u8 {
            Ch4InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch4InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch4InteractSetif {
            Ch4InteractSetif::from_bits(val)
        }
    }
    impl From<Ch4InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch4InteractSetif) -> u8 {
            Ch4InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch5EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch5EvalcfgComp {
            Ch5EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch5EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch5EvalcfgComp) -> u8 {
            Ch5EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch5EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch5EvalcfgMode {
            Ch5EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch5EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch5EvalcfgMode) -> u8 {
            Ch5EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch5EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch5EvalcfgStrsample {
            Ch5EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch5EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch5EvalcfgStrsample) -> u8 {
            Ch5EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch5InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch5InteractExclk {
            Ch5InteractExclk::from_bits(val)
        }
    }
    impl From<Ch5InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch5InteractExclk) -> u8 {
            Ch5InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch5InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch5InteractExmode {
            Ch5InteractExmode::from_bits(val)
        }
    }
    impl From<Ch5InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch5InteractExmode) -> u8 {
            Ch5InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch5InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch5InteractSample {
            Ch5InteractSample::from_bits(val)
        }
    }
    impl From<Ch5InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch5InteractSample) -> u8 {
            Ch5InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch5InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch5InteractSampleclk {
            Ch5InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch5InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch5InteractSampleclk) -> u8 {
            Ch5InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch5InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch5InteractSetif {
            Ch5InteractSetif::from_bits(val)
        }
    }
    impl From<Ch5InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch5InteractSetif) -> u8 {
            Ch5InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch6EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch6EvalcfgComp {
            Ch6EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch6EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch6EvalcfgComp) -> u8 {
            Ch6EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch6EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch6EvalcfgMode {
            Ch6EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch6EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch6EvalcfgMode) -> u8 {
            Ch6EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch6EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch6EvalcfgStrsample {
            Ch6EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch6EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch6EvalcfgStrsample) -> u8 {
            Ch6EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch6InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch6InteractExclk {
            Ch6InteractExclk::from_bits(val)
        }
    }
    impl From<Ch6InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch6InteractExclk) -> u8 {
            Ch6InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch6InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch6InteractExmode {
            Ch6InteractExmode::from_bits(val)
        }
    }
    impl From<Ch6InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch6InteractExmode) -> u8 {
            Ch6InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch6InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch6InteractSample {
            Ch6InteractSample::from_bits(val)
        }
    }
    impl From<Ch6InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch6InteractSample) -> u8 {
            Ch6InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch6InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch6InteractSampleclk {
            Ch6InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch6InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch6InteractSampleclk) -> u8 {
            Ch6InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch6InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch6InteractSetif {
            Ch6InteractSetif::from_bits(val)
        }
    }
    impl From<Ch6InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch6InteractSetif) -> u8 {
            Ch6InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch7EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch7EvalcfgComp {
            Ch7EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch7EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch7EvalcfgComp) -> u8 {
            Ch7EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch7EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch7EvalcfgMode {
            Ch7EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch7EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch7EvalcfgMode) -> u8 {
            Ch7EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch7EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch7EvalcfgStrsample {
            Ch7EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch7EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch7EvalcfgStrsample) -> u8 {
            Ch7EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch7InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch7InteractExclk {
            Ch7InteractExclk::from_bits(val)
        }
    }
    impl From<Ch7InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch7InteractExclk) -> u8 {
            Ch7InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch7InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch7InteractExmode {
            Ch7InteractExmode::from_bits(val)
        }
    }
    impl From<Ch7InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch7InteractExmode) -> u8 {
            Ch7InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch7InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch7InteractSample {
            Ch7InteractSample::from_bits(val)
        }
    }
    impl From<Ch7InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch7InteractSample) -> u8 {
            Ch7InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch7InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch7InteractSampleclk {
            Ch7InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch7InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch7InteractSampleclk) -> u8 {
            Ch7InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch7InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch7InteractSetif {
            Ch7InteractSetif::from_bits(val)
        }
    }
    impl From<Ch7InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch7InteractSetif) -> u8 {
            Ch7InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch8EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch8EvalcfgComp {
            Ch8EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch8EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch8EvalcfgComp) -> u8 {
            Ch8EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch8EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch8EvalcfgMode {
            Ch8EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch8EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch8EvalcfgMode) -> u8 {
            Ch8EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch8EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch8EvalcfgStrsample {
            Ch8EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch8EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch8EvalcfgStrsample) -> u8 {
            Ch8EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch8InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch8InteractExclk {
            Ch8InteractExclk::from_bits(val)
        }
    }
    impl From<Ch8InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch8InteractExclk) -> u8 {
            Ch8InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch8InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch8InteractExmode {
            Ch8InteractExmode::from_bits(val)
        }
    }
    impl From<Ch8InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch8InteractExmode) -> u8 {
            Ch8InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch8InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch8InteractSample {
            Ch8InteractSample::from_bits(val)
        }
    }
    impl From<Ch8InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch8InteractSample) -> u8 {
            Ch8InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch8InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch8InteractSampleclk {
            Ch8InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch8InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch8InteractSampleclk) -> u8 {
            Ch8InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch8InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch8InteractSetif {
            Ch8InteractSetif::from_bits(val)
        }
    }
    impl From<Ch8InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch8InteractSetif) -> u8 {
            Ch8InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9EvalcfgComp {
        #[doc = "Comparison evaluates to 1 if sensor data is less than CTRTHRESHOLD, or if the ACMP output is 0."]
        Less = 0x0,
        #[doc = "Comparison evaluates to 1 if sensor data is greater than, or equal to CTRTHRESHOLD, or if the ACMP output is 1."]
        Ge = 0x01,
    }
    impl Ch9EvalcfgComp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9EvalcfgComp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9EvalcfgComp {
        #[inline(always)]
        fn from(val: u8) -> Ch9EvalcfgComp {
            Ch9EvalcfgComp::from_bits(val)
        }
    }
    impl From<Ch9EvalcfgComp> for u8 {
        #[inline(always)]
        fn from(val: Ch9EvalcfgComp) -> u8 {
            Ch9EvalcfgComp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9EvalcfgMode {
        #[doc = "Threshold comparison is used to evaluate sensor result."]
        Thres = 0x0,
        #[doc = "Sliding window is used to evaluate sensor result."]
        Slidingwin = 0x01,
        #[doc = "Step detection is used to evaluate sensor result."]
        Stepdet = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch9EvalcfgMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9EvalcfgMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9EvalcfgMode {
        #[inline(always)]
        fn from(val: u8) -> Ch9EvalcfgMode {
            Ch9EvalcfgMode::from_bits(val)
        }
    }
    impl From<Ch9EvalcfgMode> for u8 {
        #[inline(always)]
        fn from(val: Ch9EvalcfgMode) -> u8 {
            Ch9EvalcfgMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9EvalcfgStrsample {
        #[doc = "Nothing will be stored in the result buffer."]
        Disable = 0x0,
        #[doc = "The sensor sample data will be stored in the result buffer."]
        Data = 0x01,
        #[doc = "The data source, i.e. the channel, will be stored alongside the sensor sample data."]
        Datasrc = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch9EvalcfgStrsample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9EvalcfgStrsample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9EvalcfgStrsample {
        #[inline(always)]
        fn from(val: u8) -> Ch9EvalcfgStrsample {
            Ch9EvalcfgStrsample::from_bits(val)
        }
    }
    impl From<Ch9EvalcfgStrsample> for u8 {
        #[inline(always)]
        fn from(val: Ch9EvalcfgStrsample) -> u8 {
            Ch9EvalcfgStrsample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9InteractExclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch9InteractExclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9InteractExclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9InteractExclk {
        #[inline(always)]
        fn from(val: u8) -> Ch9InteractExclk {
            Ch9InteractExclk::from_bits(val)
        }
    }
    impl From<Ch9InteractExclk> for u8 {
        #[inline(always)]
        fn from(val: Ch9InteractExclk) -> u8 {
            Ch9InteractExclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9InteractExmode {
        #[doc = "Disabled."]
        Disable = 0x0,
        #[doc = "Push Pull, GPIO is driven high."]
        High = 0x01,
        #[doc = "Push Pull, GPIO is driven low."]
        Low = 0x02,
        #[doc = "DAC output."]
        Dacout = 0x03,
    }
    impl Ch9InteractExmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9InteractExmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9InteractExmode {
        #[inline(always)]
        fn from(val: u8) -> Ch9InteractExmode {
            Ch9InteractExmode::from_bits(val)
        }
    }
    impl From<Ch9InteractExmode> for u8 {
        #[inline(always)]
        fn from(val: Ch9InteractExmode) -> u8 {
            Ch9InteractExmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9InteractSample {
        #[doc = "ACMPCOUNT."]
        Acmpcount = 0x0,
        #[doc = "ACMP."]
        Acmp = 0x01,
        #[doc = "ADC."]
        Adc = 0x02,
        #[doc = "ADCDIFF."]
        Adcdiff = 0x03,
    }
    impl Ch9InteractSample {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9InteractSample {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9InteractSample {
        #[inline(always)]
        fn from(val: u8) -> Ch9InteractSample {
            Ch9InteractSample::from_bits(val)
        }
    }
    impl From<Ch9InteractSample> for u8 {
        #[inline(always)]
        fn from(val: Ch9InteractSample) -> u8 {
            Ch9InteractSample::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9InteractSampleclk {
        #[doc = "Prescaled low-frequency LESENSECLK will be used for timing."]
        Lfaclk = 0x0,
        #[doc = "Prescaled high-frequency LESENSEHFCLK will be used for timing."]
        Auxhfrco = 0x01,
    }
    impl Ch9InteractSampleclk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9InteractSampleclk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9InteractSampleclk {
        #[inline(always)]
        fn from(val: u8) -> Ch9InteractSampleclk {
            Ch9InteractSampleclk::from_bits(val)
        }
    }
    impl From<Ch9InteractSampleclk> for u8 {
        #[inline(always)]
        fn from(val: Ch9InteractSampleclk) -> u8 {
            Ch9InteractSampleclk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9InteractSetif {
        #[doc = "No interrupt is generated."]
        None = 0x0,
        #[doc = "Set interrupt flag if the sensor triggers."]
        Level = 0x01,
        #[doc = "Set interrupt flag on positive edge of the sensor state."]
        Posedge = 0x02,
        #[doc = "Set interrupt flag on negative edge of the sensor state."]
        Negedge = 0x03,
        #[doc = "Set interrupt flag on both edges of the sensor state."]
        Bothedges = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ch9InteractSetif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9InteractSetif {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9InteractSetif {
        #[inline(always)]
        fn from(val: u8) -> Ch9InteractSetif {
            Ch9InteractSetif::from_bits(val)
        }
    }
    impl From<Ch9InteractSetif> for u8 {
        #[inline(always)]
        fn from(val: Ch9InteractSetif) -> u8 {
            Ch9InteractSetif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle0 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle0 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle0 {
        #[inline(always)]
        fn from(val: u8) -> Chidle0 {
            Chidle0::from_bits(val)
        }
    }
    impl From<Chidle0> for u8 {
        #[inline(always)]
        fn from(val: Chidle0) -> u8 {
            Chidle0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle1 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle1 {
        #[inline(always)]
        fn from(val: u8) -> Chidle1 {
            Chidle1::from_bits(val)
        }
    }
    impl From<Chidle1> for u8 {
        #[inline(always)]
        fn from(val: Chidle1) -> u8 {
            Chidle1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle10 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle10 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle10 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle10 {
        #[inline(always)]
        fn from(val: u8) -> Chidle10 {
            Chidle10::from_bits(val)
        }
    }
    impl From<Chidle10> for u8 {
        #[inline(always)]
        fn from(val: Chidle10) -> u8 {
            Chidle10::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle11 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle11 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle11 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle11 {
        #[inline(always)]
        fn from(val: u8) -> Chidle11 {
            Chidle11::from_bits(val)
        }
    }
    impl From<Chidle11> for u8 {
        #[inline(always)]
        fn from(val: Chidle11) -> u8 {
            Chidle11::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle12 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle12 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle12 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle12 {
        #[inline(always)]
        fn from(val: u8) -> Chidle12 {
            Chidle12::from_bits(val)
        }
    }
    impl From<Chidle12> for u8 {
        #[inline(always)]
        fn from(val: Chidle12) -> u8 {
            Chidle12::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle13 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle13 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle13 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle13 {
        #[inline(always)]
        fn from(val: u8) -> Chidle13 {
            Chidle13::from_bits(val)
        }
    }
    impl From<Chidle13> for u8 {
        #[inline(always)]
        fn from(val: Chidle13) -> u8 {
            Chidle13::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle14 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle14 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle14 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle14 {
        #[inline(always)]
        fn from(val: u8) -> Chidle14 {
            Chidle14::from_bits(val)
        }
    }
    impl From<Chidle14> for u8 {
        #[inline(always)]
        fn from(val: Chidle14) -> u8 {
            Chidle14::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle15 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle15 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle15 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle15 {
        #[inline(always)]
        fn from(val: u8) -> Chidle15 {
            Chidle15::from_bits(val)
        }
    }
    impl From<Chidle15> for u8 {
        #[inline(always)]
        fn from(val: Chidle15) -> u8 {
            Chidle15::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle2 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle2 {
        #[inline(always)]
        fn from(val: u8) -> Chidle2 {
            Chidle2::from_bits(val)
        }
    }
    impl From<Chidle2> for u8 {
        #[inline(always)]
        fn from(val: Chidle2) -> u8 {
            Chidle2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle3 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle3 {
        #[inline(always)]
        fn from(val: u8) -> Chidle3 {
            Chidle3::from_bits(val)
        }
    }
    impl From<Chidle3> for u8 {
        #[inline(always)]
        fn from(val: Chidle3) -> u8 {
            Chidle3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle4 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle4 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle4 {
        #[inline(always)]
        fn from(val: u8) -> Chidle4 {
            Chidle4::from_bits(val)
        }
    }
    impl From<Chidle4> for u8 {
        #[inline(always)]
        fn from(val: Chidle4) -> u8 {
            Chidle4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle5 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle5 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle5 {
        #[inline(always)]
        fn from(val: u8) -> Chidle5 {
            Chidle5::from_bits(val)
        }
    }
    impl From<Chidle5> for u8 {
        #[inline(always)]
        fn from(val: Chidle5) -> u8 {
            Chidle5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle6 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle6 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle6 {
        #[inline(always)]
        fn from(val: u8) -> Chidle6 {
            Chidle6::from_bits(val)
        }
    }
    impl From<Chidle6> for u8 {
        #[inline(always)]
        fn from(val: Chidle6) -> u8 {
            Chidle6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle7 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle7 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle7 {
        #[inline(always)]
        fn from(val: u8) -> Chidle7 {
            Chidle7::from_bits(val)
        }
    }
    impl From<Chidle7> for u8 {
        #[inline(always)]
        fn from(val: Chidle7) -> u8 {
            Chidle7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle8 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle8 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle8 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle8 {
        #[inline(always)]
        fn from(val: u8) -> Chidle8 {
            Chidle8::from_bits(val)
        }
    }
    impl From<Chidle8> for u8 {
        #[inline(always)]
        fn from(val: Chidle8) -> u8 {
            Chidle8::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chidle9 {
        #[doc = "Channel output is disabled in idle phase."]
        Disable = 0x0,
        #[doc = "Channel output is high in idle phase."]
        High = 0x01,
        #[doc = "Channel output is low in idle phase."]
        Low = 0x02,
        #[doc = "Channel output is connected to DAC output in idle phase (CH 0,1,2 only)."]
        Dac = 0x03,
    }
    impl Chidle9 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chidle9 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chidle9 {
        #[inline(always)]
        fn from(val: u8) -> Chidle9 {
            Chidle9::from_bits(val)
        }
    }
    impl From<Chidle9> for u8 {
        #[inline(always)]
        fn from(val: Chidle9) -> u8 {
            Chidle9::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacch0data {
        #[doc = "DAC data is defined by CH0DATA in the DAC interface."]
        Dacdata = 0x0,
        #[doc = "DAC data is defined by THRES in CHx_INTERACT."]
        Thres = 0x01,
    }
    impl Dacch0data {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacch0data {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacch0data {
        #[inline(always)]
        fn from(val: u8) -> Dacch0data {
            Dacch0data::from_bits(val)
        }
    }
    impl From<Dacch0data> for u8 {
        #[inline(always)]
        fn from(val: Dacch0data) -> u8 {
            Dacch0data::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacconvtrig {
        #[doc = "DAC is enabled before every LESENSE channle measurement."]
        Channelstart = 0x0,
        #[doc = "DAC is only enabled once per scan."]
        Scanstart = 0x01,
    }
    impl Dacconvtrig {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacconvtrig {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacconvtrig {
        #[inline(always)]
        fn from(val: u8) -> Dacconvtrig {
            Dacconvtrig::from_bits(val)
        }
    }
    impl From<Dacconvtrig> for u8 {
        #[inline(always)]
        fn from(val: Dacconvtrig) -> u8 {
            Dacconvtrig::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacstartup {
        #[doc = "DAC is started a full LESENSECLK before sensor interaction starts."]
        Fullcycle = 0x0,
        #[doc = "DAC is started half a LESENSECLK cycle before sensor interaction starts."]
        Halfcycle = 0x01,
    }
    impl Dacstartup {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacstartup {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacstartup {
        #[inline(always)]
        fn from(val: u8) -> Dacstartup {
            Dacstartup::from_bits(val)
        }
    }
    impl From<Dacstartup> for u8 {
        #[inline(always)]
        fn from(val: Dacstartup) -> u8 {
            Dacstartup::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Debugrun {
        #[doc = "LESENSE can not start new scans in debug mode."]
        X0 = 0x0,
        #[doc = "LESENSE can start new scans in debug mode."]
        X1 = 0x01,
    }
    impl Debugrun {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Debugrun {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Debugrun {
        #[inline(always)]
        fn from(val: u8) -> Debugrun {
            Debugrun::from_bits(val)
        }
    }
    impl From<Debugrun> for u8 {
        #[inline(always)]
        fn from(val: Debugrun) -> u8 {
            Debugrun::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lfpresc {
        #[doc = "Low frequency timer is clocked with LESENSECLK/1."]
        Div1 = 0x0,
        #[doc = "Low frequency timer is clocked with LESENSECLK/2."]
        Div2 = 0x01,
        #[doc = "Low frequency timer is clocked with LESENSECLK/4."]
        Div4 = 0x02,
        #[doc = "Low frequency timer is clocked with LESENSECLK/8."]
        Div8 = 0x03,
        #[doc = "Low frequency timer is clocked with LESENSECLK/16."]
        Div16 = 0x04,
        #[doc = "Low frequency timer is clocked with LESENSECLK/32."]
        Div32 = 0x05,
        #[doc = "Low frequency timer is clocked with LESENSECLK/64."]
        Div64 = 0x06,
        #[doc = "Low frequency timer is clocked with LESENSECLK/128."]
        Div128 = 0x07,
    }
    impl Lfpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lfpresc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lfpresc {
        #[inline(always)]
        fn from(val: u8) -> Lfpresc {
            Lfpresc::from_bits(val)
        }
    }
    impl From<Lfpresc> for u8 {
        #[inline(always)]
        fn from(val: Lfpresc) -> u8 {
            Lfpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pcpresc {
        #[doc = "The period counter clock frequency is LESENSECLK/1."]
        Div1 = 0x0,
        #[doc = "The period counter clock frequency is LESENSECLK/2."]
        Div2 = 0x01,
        #[doc = "The period counter clock frequency is LESENSECLK/4."]
        Div4 = 0x02,
        #[doc = "The period counter clock frequency is LESENSECLK/8."]
        Div8 = 0x03,
        #[doc = "The period counter clock frequency is LESENSECLK/16."]
        Div16 = 0x04,
        #[doc = "The period counter clock frequency is LESENSECLK/32."]
        Div32 = 0x05,
        #[doc = "The period counter clock frequency is LESENSECLK/64."]
        Div64 = 0x06,
        #[doc = "The period counter clock frequency is LESENSECLK/128."]
        Div128 = 0x07,
    }
    impl Pcpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcpresc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcpresc {
        #[inline(always)]
        fn from(val: u8) -> Pcpresc {
            Pcpresc::from_bits(val)
        }
    }
    impl From<Pcpresc> for u8 {
        #[inline(always)]
        fn from(val: Pcpresc) -> u8 {
            Pcpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scanconf {
        #[doc = "The channel configuration register registers used are directly mapped to the channel number."]
        Dirmap = 0x0,
        #[doc = "The channel configuration registers used are CH\\[X+8\\]_CONF for channels 0-7 and CH\\[X-8\\]_CONF for channels 8-15."]
        Invmap = 0x01,
        #[doc = "The channel configuration registers used toggle between CH\\[X\\]_CONF and CH\\[X+8\\]_CONF when channel x triggers."]
        Toggle = 0x02,
        #[doc = "The decoder state defines the CONF registers to be used."]
        Decdef = 0x03,
    }
    impl Scanconf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scanconf {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scanconf {
        #[inline(always)]
        fn from(val: u8) -> Scanconf {
            Scanconf::from_bits(val)
        }
    }
    impl From<Scanconf> for u8 {
        #[inline(always)]
        fn from(val: Scanconf) -> u8 {
            Scanconf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scanmode {
        #[doc = "A new scan is started each time the period counter overflows."]
        Periodic = 0x0,
        #[doc = "A single scan is performed when START in CMD is set."]
        Oneshot = 0x01,
        #[doc = "Pulse on PRS channel."]
        Prs = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Scanmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scanmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scanmode {
        #[inline(always)]
        fn from(val: u8) -> Scanmode {
            Scanmode::from_bits(val)
        }
    }
    impl From<Scanmode> for u8 {
        #[inline(always)]
        fn from(val: Scanmode) -> u8 {
            Scanmode::to_bits(val)
        }
    }
}
