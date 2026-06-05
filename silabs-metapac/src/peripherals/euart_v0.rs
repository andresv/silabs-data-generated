#[doc = "EUART peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Euart {
    ptr: *mut u8,
}
unsafe impl Send for Euart {}
unsafe impl Sync for Euart {}
impl Euart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ipversion(self) -> crate::common::Reg<regs::Ipversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn framecfg(self) -> crate::common::Reg<regs::Framecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn irhfcfg(self) -> crate::common::Reg<regs::Irhfcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn irlfcfg(self) -> crate::common::Reg<regs::Irlfcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn timingcfg(self) -> crate::common::Reg<regs::Timingcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn startframecfg(self) -> crate::common::Reg<regs::Startframecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sigframecfg(self) -> crate::common::Reg<regs::Sigframecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn clkdiv(self) -> crate::common::Reg<regs::Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn trigctrl(self) -> crate::common::Reg<regs::Trigctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rxdata(self) -> crate::common::Reg<regs::Rxdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rxdatap(self) -> crate::common::Reg<regs::Rxdatap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn txdata(self) -> crate::common::Reg<regs::Txdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "Loopback Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loopbk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback Enable."]
        #[inline(always)]
        pub const fn set_loopbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Collision Check Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Collision Check Enable."]
        #[inline(always)]
        pub const fn set_ccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Multi-Processor Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mpm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-Processor Mode."]
        #[inline(always)]
        pub const fn set_mpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Multi-Processor Address-Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn mpab(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-Processor Address-Bit."]
        #[inline(always)]
        pub const fn set_mpab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Oversampling."]
        #[must_use]
        #[inline(always)]
        pub const fn ovs(&self) -> super::vals::Ovs {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Ovs::from_bits(val as u8)
        }
        #[doc = "Oversampling."]
        #[inline(always)]
        pub const fn set_ovs(&mut self, val: super::vals::Ovs) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "Most Significant Bit First."]
        #[must_use]
        #[inline(always)]
        pub const fn msbf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Most Significant Bit First."]
        #[inline(always)]
        pub const fn set_msbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver Input Invert."]
        #[must_use]
        #[inline(always)]
        pub const fn rxinv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Input Invert."]
        #[inline(always)]
        pub const fn set_rxinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmitter output Invert."]
        #[must_use]
        #[inline(always)]
        pub const fn txinv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter output Invert."]
        #[inline(always)]
        pub const fn set_txinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Automatic TX Tristate."]
        #[must_use]
        #[inline(always)]
        pub const fn autotri(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic TX Tristate."]
        #[inline(always)]
        pub const fn set_autotri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Skip Parity Error Frames."]
        #[must_use]
        #[inline(always)]
        pub const fn skipperrf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Skip Parity Error Frames."]
        #[inline(always)]
        pub const fn set_skipperrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Halt DMA Read On Error."]
        #[must_use]
        #[inline(always)]
        pub const fn errsdma(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Halt DMA Read On Error."]
        #[inline(always)]
        pub const fn set_errsdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Disable RX On Error."]
        #[must_use]
        #[inline(always)]
        pub const fn errsrx(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Disable RX On Error."]
        #[inline(always)]
        pub const fn set_errsrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Disable TX On Error."]
        #[must_use]
        #[inline(always)]
        pub const fn errstx(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable TX On Error."]
        #[inline(always)]
        pub const fn set_errstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Majority Vote Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn mvdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Majority Vote Disable."]
        #[inline(always)]
        pub const fn set_mvdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AUTOBAUD detection enable."]
        #[must_use]
        #[inline(always)]
        pub const fn autobauden(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AUTOBAUD detection enable."]
        #[inline(always)]
        pub const fn set_autobauden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    impl core::fmt::Debug for Cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg0")
                .field("loopbk", &self.loopbk())
                .field("ccen", &self.ccen())
                .field("mpm", &self.mpm())
                .field("mpab", &self.mpab())
                .field("ovs", &self.ovs())
                .field("msbf", &self.msbf())
                .field("rxinv", &self.rxinv())
                .field("txinv", &self.txinv())
                .field("autotri", &self.autotri())
                .field("skipperrf", &self.skipperrf())
                .field("errsdma", &self.errsdma())
                .field("errsrx", &self.errsrx())
                .field("errstx", &self.errstx())
                .field("mvdis", &self.mvdis())
                .field("autobauden", &self.autobauden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg0 {{ loopbk: {=bool:?}, ccen: {=bool:?}, mpm: {=bool:?}, mpab: {=bool:?}, ovs: {:?}, msbf: {=bool:?}, rxinv: {=bool:?}, txinv: {=bool:?}, autotri: {=bool:?}, skipperrf: {=bool:?}, errsdma: {=bool:?}, errsrx: {=bool:?}, errstx: {=bool:?}, mvdis: {=bool:?}, autobauden: {=bool:?} }}",
                self.loopbk(),
                self.ccen(),
                self.mpm(),
                self.mpab(),
                self.ovs(),
                self.msbf(),
                self.rxinv(),
                self.txinv(),
                self.autotri(),
                self.skipperrf(),
                self.errsdma(),
                self.errsrx(),
                self.errstx(),
                self.mvdis(),
                self.autobauden()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "Debug halt."]
        #[must_use]
        #[inline(always)]
        pub const fn dbghalt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug halt."]
        #[inline(always)]
        pub const fn set_dbghalt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear-to-send Invert Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ctsinv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear-to-send Invert Enable."]
        #[inline(always)]
        pub const fn set_ctsinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear-to-send Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ctsen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear-to-send Enable."]
        #[inline(always)]
        pub const fn set_ctsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Request-to-send Invert Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtsinv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Request-to-send Invert Enable."]
        #[inline(always)]
        pub const fn set_rtsinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmitter DMA Wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn txdmawu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter DMA Wakeup."]
        #[inline(always)]
        pub const fn set_txdmawu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Receiver DMA Wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdmawu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver DMA Wakeup."]
        #[inline(always)]
        pub const fn set_rxdmawu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Start Frame Unblock Receiver."]
        #[must_use]
        #[inline(always)]
        pub const fn sfubrx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Start Frame Unblock Receiver."]
        #[inline(always)]
        pub const fn set_sfubrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PRS RX Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxprsen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PRS RX Enable."]
        #[inline(always)]
        pub const fn set_rxprsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TX FIFO Interrupt Watermark."]
        #[must_use]
        #[inline(always)]
        pub const fn txfiw(&self) -> super::vals::Txfiw {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Txfiw::from_bits(val as u8)
        }
        #[doc = "TX FIFO Interrupt Watermark."]
        #[inline(always)]
        pub const fn set_txfiw(&mut self, val: super::vals::Txfiw) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "RX FIFO Interrupt Watermark."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfiw(&self) -> super::vals::Rxfiw {
            let val = (self.0 >> 19usize) & 0x03;
            super::vals::Rxfiw::from_bits(val as u8)
        }
        #[doc = "RX FIFO Interrupt Watermark."]
        #[inline(always)]
        pub const fn set_rxfiw(&mut self, val: super::vals::Rxfiw) {
            self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
        }
        #[doc = "Request-to-send RX FIFO Watermark."]
        #[must_use]
        #[inline(always)]
        pub const fn rtsrxfw(&self) -> super::vals::Rtsrxfw {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Rtsrxfw::from_bits(val as u8)
        }
        #[doc = "Request-to-send RX FIFO Watermark."]
        #[inline(always)]
        pub const fn set_rtsrxfw(&mut self, val: super::vals::Rtsrxfw) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    impl core::fmt::Debug for Cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg1")
                .field("dbghalt", &self.dbghalt())
                .field("ctsinv", &self.ctsinv())
                .field("ctsen", &self.ctsen())
                .field("rtsinv", &self.rtsinv())
                .field("txdmawu", &self.txdmawu())
                .field("rxdmawu", &self.rxdmawu())
                .field("sfubrx", &self.sfubrx())
                .field("rxprsen", &self.rxprsen())
                .field("txfiw", &self.txfiw())
                .field("rxfiw", &self.rxfiw())
                .field("rtsrxfw", &self.rtsrxfw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg1 {{ dbghalt: {=bool:?}, ctsinv: {=bool:?}, ctsen: {=bool:?}, rtsinv: {=bool:?}, txdmawu: {=bool:?}, rxdmawu: {=bool:?}, sfubrx: {=bool:?}, rxprsen: {=bool:?}, txfiw: {:?}, rxfiw: {:?}, rtsrxfw: {:?} }}",
                self.dbghalt(),
                self.ctsinv(),
                self.ctsen(),
                self.rtsinv(),
                self.txdmawu(),
                self.rxdmawu(),
                self.sfubrx(),
                self.rxprsen(),
                self.txfiw(),
                self.rxfiw(),
                self.rtsrxfw()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkdiv(pub u32);
    impl Clkdiv {
        #[doc = "Fractional Clock Divider."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Fractional Clock Divider."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 3usize)) | (((val as u32) & 0x000f_ffff) << 3usize);
        }
    }
    impl Default for Clkdiv {
        #[inline(always)]
        fn default() -> Clkdiv {
            Clkdiv(0)
        }
    }
    impl core::fmt::Debug for Clkdiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkdiv").field("div", &self.div()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clkdiv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Clkdiv {{ div: {=u32:?} }}", self.div())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Receiver Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Enable."]
        #[inline(always)]
        pub const fn set_rxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Disable."]
        #[inline(always)]
        pub const fn set_rxdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmitter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Enable."]
        #[inline(always)]
        pub const fn set_txen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn txdis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Disable."]
        #[inline(always)]
        pub const fn set_txdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receiver Block Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxblocken(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Block Enable."]
        #[inline(always)]
        pub const fn set_rxblocken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Receiver Block Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxblockdis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Block Disable."]
        #[inline(always)]
        pub const fn set_rxblockdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmitter Tristate Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txtrien(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Tristate Enable."]
        #[inline(always)]
        pub const fn set_txtrien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmitter Tristate Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn txtridis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Tristate Disable."]
        #[inline(always)]
        pub const fn set_txtridis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clear TX FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn cleartx(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear TX FIFO."]
        #[inline(always)]
        pub const fn set_cleartx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("rxen", &self.rxen())
                .field("rxdis", &self.rxdis())
                .field("txen", &self.txen())
                .field("txdis", &self.txdis())
                .field("rxblocken", &self.rxblocken())
                .field("rxblockdis", &self.rxblockdis())
                .field("txtrien", &self.txtrien())
                .field("txtridis", &self.txtridis())
                .field("cleartx", &self.cleartx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ rxen: {=bool:?}, rxdis: {=bool:?}, txen: {=bool:?}, txdis: {=bool:?}, rxblocken: {=bool:?}, rxblockdis: {=bool:?}, txtrien: {=bool:?}, txtridis: {=bool:?}, cleartx: {=bool:?} }}",
                self.rxen(),
                self.rxdis(),
                self.txen(),
                self.txdis(),
                self.rxblocken(),
                self.rxblockdis(),
                self.txtrien(),
                self.txtridis(),
                self.cleartx()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Module enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Module enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("En").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for En {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "En {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Framecfg(pub u32);
    impl Framecfg {
        #[doc = "Data-Bit Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn databits(&self) -> super::vals::Databits {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Databits::from_bits(val as u8)
        }
        #[doc = "Data-Bit Mode."]
        #[inline(always)]
        pub const fn set_databits(&mut self, val: super::vals::Databits) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Parity-Bit Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn parity(&self) -> super::vals::Parity {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Parity::from_bits(val as u8)
        }
        #[doc = "Parity-Bit Mode."]
        #[inline(always)]
        pub const fn set_parity(&mut self, val: super::vals::Parity) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Stop-Bit Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn stopbits(&self) -> super::vals::Stopbits {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stopbits::from_bits(val as u8)
        }
        #[doc = "Stop-Bit Mode."]
        #[inline(always)]
        pub const fn set_stopbits(&mut self, val: super::vals::Stopbits) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Framecfg {
        #[inline(always)]
        fn default() -> Framecfg {
            Framecfg(0)
        }
    }
    impl core::fmt::Debug for Framecfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Framecfg")
                .field("databits", &self.databits())
                .field("parity", &self.parity())
                .field("stopbits", &self.stopbits())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Framecfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Framecfg {{ databits: {:?}, parity: {:?}, stopbits: {:?} }}",
                self.databits(),
                self.parity(),
                self.stopbits()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "TX Complete IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX Complete IEN."]
        #[inline(always)]
        pub const fn set_txc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX FIFO Level IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txfl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Level IEN."]
        #[inline(always)]
        pub const fn set_txfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX FIFO Level IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Level IEN."]
        #[inline(always)]
        pub const fn set_rxfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RX FIFO Full IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfull(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Full IEN."]
        #[inline(always)]
        pub const fn set_rxfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RX FIFO Overflow IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Overflow IEN."]
        #[inline(always)]
        pub const fn set_rxof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX FIFO Underflow IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxuf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Underflow IEN."]
        #[inline(always)]
        pub const fn set_rxuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TX FIFO Overflow IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txof(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Overflow IEN."]
        #[inline(always)]
        pub const fn set_txof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Parity Error IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error IEN."]
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Framing Error IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ferr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Framing Error IEN."]
        #[inline(always)]
        pub const fn set_ferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Multi-Processor Addr Frame IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn mpaf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-Processor Addr Frame IEN."]
        #[inline(always)]
        pub const fn set_mpaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Collision Check Fail IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Collision Check Fail IEN."]
        #[inline(always)]
        pub const fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TX IDLE IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txidle(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TX IDLE IEN."]
        #[inline(always)]
        pub const fn set_txidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Start Frame IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn startf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Start Frame IEN."]
        #[inline(always)]
        pub const fn set_startf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Signal Frame IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn sigf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Signal Frame IEN."]
        #[inline(always)]
        pub const fn set_sigf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Auto Baud Complete IEN."]
        #[must_use]
        #[inline(always)]
        pub const fn autobauddone(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auto Baud Complete IEN."]
        #[inline(always)]
        pub const fn set_autobauddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("txc", &self.txc())
                .field("txfl", &self.txfl())
                .field("rxfl", &self.rxfl())
                .field("rxfull", &self.rxfull())
                .field("rxof", &self.rxof())
                .field("rxuf", &self.rxuf())
                .field("txof", &self.txof())
                .field("perr", &self.perr())
                .field("ferr", &self.ferr())
                .field("mpaf", &self.mpaf())
                .field("ccf", &self.ccf())
                .field("txidle", &self.txidle())
                .field("startf", &self.startf())
                .field("sigf", &self.sigf())
                .field("autobauddone", &self.autobauddone())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ txc: {=bool:?}, txfl: {=bool:?}, rxfl: {=bool:?}, rxfull: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, ccf: {=bool:?}, txidle: {=bool:?}, startf: {=bool:?}, sigf: {=bool:?}, autobauddone: {=bool:?} }}",
                self.txc(),
                self.txfl(),
                self.rxfl(),
                self.rxfull(),
                self.rxof(),
                self.rxuf(),
                self.txof(),
                self.perr(),
                self.ferr(),
                self.mpaf(),
                self.ccf(),
                self.txidle(),
                self.startf(),
                self.sigf(),
                self.autobauddone()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "TX Complete Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn txc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX Complete Interrupt Flag."]
        #[inline(always)]
        pub const fn set_txc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX FIFO Level Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn txfl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Level Interrupt Flag."]
        #[inline(always)]
        pub const fn set_txfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX FIFO Level Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Level Interrupt Flag."]
        #[inline(always)]
        pub const fn set_rxfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RX FIFO Full Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfull(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Full Interrupt Flag."]
        #[inline(always)]
        pub const fn set_rxfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RX FIFO Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rxof(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_rxof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX FIFO Underflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rxuf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Underflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_rxuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TX FIFO Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn txof(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_txof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Parity Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Framing Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ferr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Framing Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Multi-Processor Address Frame Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn mpaf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Multi-Processor Address Frame Interrupt."]
        #[inline(always)]
        pub const fn set_mpaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Collision Check Fail Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Collision Check Fail Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TX Idle Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn txidle(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TX Idle Interrupt Flag."]
        #[inline(always)]
        pub const fn set_txidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Start Frame Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn startf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Start Frame Interrupt Flag."]
        #[inline(always)]
        pub const fn set_startf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Signal Frame Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sigf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Signal Frame Interrupt Flag."]
        #[inline(always)]
        pub const fn set_sigf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Auto Baud Complete Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn autobauddone(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auto Baud Complete Interrupt Flag."]
        #[inline(always)]
        pub const fn set_autobauddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("txc", &self.txc())
                .field("txfl", &self.txfl())
                .field("rxfl", &self.rxfl())
                .field("rxfull", &self.rxfull())
                .field("rxof", &self.rxof())
                .field("rxuf", &self.rxuf())
                .field("txof", &self.txof())
                .field("perr", &self.perr())
                .field("ferr", &self.ferr())
                .field("mpaf", &self.mpaf())
                .field("ccf", &self.ccf())
                .field("txidle", &self.txidle())
                .field("startf", &self.startf())
                .field("sigf", &self.sigf())
                .field("autobauddone", &self.autobauddone())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ txc: {=bool:?}, txfl: {=bool:?}, rxfl: {=bool:?}, rxfull: {=bool:?}, rxof: {=bool:?}, rxuf: {=bool:?}, txof: {=bool:?}, perr: {=bool:?}, ferr: {=bool:?}, mpaf: {=bool:?}, ccf: {=bool:?}, txidle: {=bool:?}, startf: {=bool:?}, sigf: {=bool:?}, autobauddone: {=bool:?} }}",
                self.txc(),
                self.txfl(),
                self.rxfl(),
                self.rxfull(),
                self.rxof(),
                self.rxuf(),
                self.txof(),
                self.perr(),
                self.ferr(),
                self.mpaf(),
                self.ccf(),
                self.txidle(),
                self.startf(),
                self.sigf(),
                self.autobauddone()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP version ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP version ID."]
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
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irhfcfg(pub u32);
    impl Irhfcfg {
        #[doc = "Enable IrDA Module."]
        #[must_use]
        #[inline(always)]
        pub const fn irhfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable IrDA Module."]
        #[inline(always)]
        pub const fn set_irhfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA TX Pulse Width."]
        #[must_use]
        #[inline(always)]
        pub const fn irhfpw(&self) -> super::vals::Irhfpw {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Irhfpw::from_bits(val as u8)
        }
        #[doc = "IrDA TX Pulse Width."]
        #[inline(always)]
        pub const fn set_irhfpw(&mut self, val: super::vals::Irhfpw) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "IrDA RX Filter."]
        #[must_use]
        #[inline(always)]
        pub const fn irhffilt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA RX Filter."]
        #[inline(always)]
        pub const fn set_irhffilt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Irhfcfg {
        #[inline(always)]
        fn default() -> Irhfcfg {
            Irhfcfg(0)
        }
    }
    impl core::fmt::Debug for Irhfcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irhfcfg")
                .field("irhfen", &self.irhfen())
                .field("irhfpw", &self.irhfpw())
                .field("irhffilt", &self.irhffilt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irhfcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Irhfcfg {{ irhfen: {=bool:?}, irhfpw: {:?}, irhffilt: {=bool:?} }}",
                self.irhfen(),
                self.irhfpw(),
                self.irhffilt()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irlfcfg(pub u32);
    impl Irlfcfg {
        #[doc = "Pulse Generator/Extender Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn irlfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Pulse Generator/Extender Enable."]
        #[inline(always)]
        pub const fn set_irlfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Irlfcfg {
        #[inline(always)]
        fn default() -> Irlfcfg {
            Irlfcfg(0)
        }
    }
    impl core::fmt::Debug for Irlfcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irlfcfg").field("irlfen", &self.irlfen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irlfcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Irlfcfg {{ irlfen: {=bool:?} }}", self.irlfen())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdata(pub u32);
    impl Rxdata {
        #[doc = "RX Data."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "RX Data."]
        #[inline(always)]
        pub const fn set_rxdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Parity Error."]
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error."]
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Framing Error."]
        #[must_use]
        #[inline(always)]
        pub const fn ferr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Framing Error."]
        #[inline(always)]
        pub const fn set_ferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Rxdata {
        #[inline(always)]
        fn default() -> Rxdata {
            Rxdata(0)
        }
    }
    impl core::fmt::Debug for Rxdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdata")
                .field("rxdata", &self.rxdata())
                .field("perr", &self.perr())
                .field("ferr", &self.ferr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxdata {{ rxdata: {=u16:?}, perr: {=bool:?}, ferr: {=bool:?} }}",
                self.rxdata(),
                self.perr(),
                self.ferr()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdatap(pub u32);
    impl Rxdatap {
        #[doc = "RX Data Peek."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdatap(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "RX Data Peek."]
        #[inline(always)]
        pub const fn set_rxdatap(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Parity Error Peek."]
        #[must_use]
        #[inline(always)]
        pub const fn perrp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Parity Error Peek."]
        #[inline(always)]
        pub const fn set_perrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Framing Error Peek."]
        #[must_use]
        #[inline(always)]
        pub const fn ferrp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Framing Error Peek."]
        #[inline(always)]
        pub const fn set_ferrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Rxdatap {
        #[inline(always)]
        fn default() -> Rxdatap {
            Rxdatap(0)
        }
    }
    impl core::fmt::Debug for Rxdatap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdatap")
                .field("rxdatap", &self.rxdatap())
                .field("perrp", &self.perrp())
                .field("ferrp", &self.ferrp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdatap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxdatap {{ rxdatap: {=u16:?}, perrp: {=bool:?}, ferrp: {=bool:?} }}",
                self.rxdatap(),
                self.perrp(),
                self.ferrp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sigframecfg(pub u32);
    impl Sigframecfg {
        #[doc = "Signal Frame Value."]
        #[must_use]
        #[inline(always)]
        pub const fn sigframe(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Signal Frame Value."]
        #[inline(always)]
        pub const fn set_sigframe(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Sigframecfg {
        #[inline(always)]
        fn default() -> Sigframecfg {
            Sigframecfg(0)
        }
    }
    impl core::fmt::Debug for Sigframecfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sigframecfg")
                .field("sigframe", &self.sigframe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sigframecfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sigframecfg {{ sigframe: {=u16:?} }}", self.sigframe())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Startframecfg(pub u32);
    impl Startframecfg {
        #[doc = "Start Frame."]
        #[must_use]
        #[inline(always)]
        pub const fn startframe(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Start Frame."]
        #[inline(always)]
        pub const fn set_startframe(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Startframecfg {
        #[inline(always)]
        fn default() -> Startframecfg {
            Startframecfg(0)
        }
    }
    impl core::fmt::Debug for Startframecfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Startframecfg")
                .field("startframe", &self.startframe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Startframecfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Startframecfg {{ startframe: {=u16:?} }}", self.startframe())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Receiver Enable Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxens(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver Enable Status."]
        #[inline(always)]
        pub const fn set_rxens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmitter Enable Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txens(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Enable Status."]
        #[inline(always)]
        pub const fn set_txens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Block Incoming Data."]
        #[must_use]
        #[inline(always)]
        pub const fn rxblock(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Block Incoming Data."]
        #[inline(always)]
        pub const fn set_rxblock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmitter Tristated."]
        #[must_use]
        #[inline(always)]
        pub const fn txtri(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter Tristated."]
        #[inline(always)]
        pub const fn set_txtri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TX Complete."]
        #[must_use]
        #[inline(always)]
        pub const fn txc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TX Complete."]
        #[inline(always)]
        pub const fn set_txc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TX FIFO Level."]
        #[must_use]
        #[inline(always)]
        pub const fn txfl(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Level."]
        #[inline(always)]
        pub const fn set_txfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RX FIFO Level."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Level."]
        #[inline(always)]
        pub const fn set_rxfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RX FIFO Full."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfull(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RX FIFO Full."]
        #[inline(always)]
        pub const fn set_rxfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RX Idle."]
        #[must_use]
        #[inline(always)]
        pub const fn rxidle(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RX Idle."]
        #[inline(always)]
        pub const fn set_rxidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TX Idle."]
        #[must_use]
        #[inline(always)]
        pub const fn txidle(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TX Idle."]
        #[inline(always)]
        pub const fn set_txidle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Valid entries in TX FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn txfcnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Valid entries in TX FIFO."]
        #[inline(always)]
        pub const fn set_txfcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "TX FIFO Clear Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn cleartxbusy(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TX FIFO Clear Busy."]
        #[inline(always)]
        pub const fn set_cleartxbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Auto Baud Rate Detection Completed."]
        #[must_use]
        #[inline(always)]
        pub const fn autobauddone(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Auto Baud Rate Detection Completed."]
        #[inline(always)]
        pub const fn set_autobauddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("rxens", &self.rxens())
                .field("txens", &self.txens())
                .field("rxblock", &self.rxblock())
                .field("txtri", &self.txtri())
                .field("txc", &self.txc())
                .field("txfl", &self.txfl())
                .field("rxfl", &self.rxfl())
                .field("rxfull", &self.rxfull())
                .field("rxidle", &self.rxidle())
                .field("txidle", &self.txidle())
                .field("txfcnt", &self.txfcnt())
                .field("cleartxbusy", &self.cleartxbusy())
                .field("autobauddone", &self.autobauddone())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ rxens: {=bool:?}, txens: {=bool:?}, rxblock: {=bool:?}, txtri: {=bool:?}, txc: {=bool:?}, txfl: {=bool:?}, rxfl: {=bool:?}, rxfull: {=bool:?}, rxidle: {=bool:?}, txidle: {=bool:?}, txfcnt: {=u8:?}, cleartxbusy: {=bool:?}, autobauddone: {=bool:?} }}",
                self.rxens(),
                self.txens(),
                self.rxblock(),
                self.txtri(),
                self.txc(),
                self.txfl(),
                self.rxfl(),
                self.rxfull(),
                self.rxidle(),
                self.txidle(),
                self.txfcnt(),
                self.cleartxbusy(),
                self.autobauddone()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "SYNCBUSY for DIV in CLKDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for DIV in CLKDIV."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SYNCBUSY for RXTEN in TRIGCTRL."]
        #[must_use]
        #[inline(always)]
        pub const fn rxten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for RXTEN in TRIGCTRL."]
        #[inline(always)]
        pub const fn set_rxten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SYNCBUSY for TXTEN in TRIGCTRL."]
        #[must_use]
        #[inline(always)]
        pub const fn txten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for TXTEN in TRIGCTRL."]
        #[inline(always)]
        pub const fn set_txten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SYNCBUSY for RXEN in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for RXEN in CMD."]
        #[inline(always)]
        pub const fn set_rxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SYNCBUSY for RXDIS in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for RXDIS in CMD."]
        #[inline(always)]
        pub const fn set_rxdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SYNCBUSY for TXEN in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn txen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for TXEN in CMD."]
        #[inline(always)]
        pub const fn set_txen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SYNCBUSY for TXDIS in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn txdis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for TXDIS in CMD."]
        #[inline(always)]
        pub const fn set_txdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SYNCBUSY for RXBLOCKEN in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxblocken(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for RXBLOCKEN in CMD."]
        #[inline(always)]
        pub const fn set_rxblocken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SYNCBUSY for RXBLOCKDIS in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxblockdis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for RXBLOCKDIS in CMD."]
        #[inline(always)]
        pub const fn set_rxblockdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SYNCBUSY for TXTRIEN in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn txtrien(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY for TXTRIEN in CMD."]
        #[inline(always)]
        pub const fn set_txtrien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SYNCBUSY in TXTRIDIS in CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn txtridis(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY in TXTRIDIS in CMD."]
        #[inline(always)]
        pub const fn set_txtridis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            f.debug_struct("Syncbusy")
                .field("div", &self.div())
                .field("rxten", &self.rxten())
                .field("txten", &self.txten())
                .field("rxen", &self.rxen())
                .field("rxdis", &self.rxdis())
                .field("txen", &self.txen())
                .field("txdis", &self.txdis())
                .field("rxblocken", &self.rxblocken())
                .field("rxblockdis", &self.rxblockdis())
                .field("txtrien", &self.txtrien())
                .field("txtridis", &self.txtridis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ div: {=bool:?}, rxten: {=bool:?}, txten: {=bool:?}, rxen: {=bool:?}, rxdis: {=bool:?}, txen: {=bool:?}, txdis: {=bool:?}, rxblocken: {=bool:?}, rxblockdis: {=bool:?}, txtrien: {=bool:?}, txtridis: {=bool:?} }}",
                self.div(),
                self.rxten(),
                self.txten(),
                self.rxen(),
                self.rxdis(),
                self.txen(),
                self.txdis(),
                self.rxblocken(),
                self.rxblockdis(),
                self.txtrien(),
                self.txtridis()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingcfg(pub u32);
    impl Timingcfg {
        #[doc = "TX Delay Transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn txdelay(&self) -> super::vals::Txdelay {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Txdelay::from_bits(val as u8)
        }
        #[doc = "TX Delay Transmission."]
        #[inline(always)]
        pub const fn set_txdelay(&mut self, val: super::vals::Txdelay) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Timingcfg {
        #[inline(always)]
        fn default() -> Timingcfg {
            Timingcfg(0)
        }
    }
    impl core::fmt::Debug for Timingcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingcfg").field("txdelay", &self.txdelay()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timingcfg {{ txdelay: {:?} }}", self.txdelay())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigctrl(pub u32);
    impl Trigctrl {
        #[doc = "Receive Trigger Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Trigger Enable."]
        #[inline(always)]
        pub const fn set_rxten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit Trigger Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit Trigger Enable."]
        #[inline(always)]
        pub const fn set_txten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Trigctrl {
        #[inline(always)]
        fn default() -> Trigctrl {
            Trigctrl(0)
        }
    }
    impl core::fmt::Debug for Trigctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trigctrl")
                .field("rxten", &self.rxten())
                .field("txten", &self.txten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trigctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trigctrl {{ rxten: {=bool:?}, txten: {=bool:?} }}",
                self.rxten(),
                self.txten()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdata(pub u32);
    impl Txdata {
        #[doc = "TX Data."]
        #[must_use]
        #[inline(always)]
        pub const fn txdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "TX Data."]
        #[inline(always)]
        pub const fn set_txdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Unblock RX After Transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn ubrxat(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Unblock RX After Transmission."]
        #[inline(always)]
        pub const fn set_ubrxat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Set TXTRI After Transmisssion."]
        #[must_use]
        #[inline(always)]
        pub const fn txtriat(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Set TXTRI After Transmisssion."]
        #[inline(always)]
        pub const fn set_txtriat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Transit Data as Break."]
        #[must_use]
        #[inline(always)]
        pub const fn txbreak(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Transit Data as Break."]
        #[inline(always)]
        pub const fn set_txbreak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Clear TXEN After Transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn txdisat(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Clear TXEN After Transmission."]
        #[inline(always)]
        pub const fn set_txdisat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable RXEN After Transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn rxenat(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable RXEN After Transmission."]
        #[inline(always)]
        pub const fn set_rxenat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Txdata {
        #[inline(always)]
        fn default() -> Txdata {
            Txdata(0)
        }
    }
    impl core::fmt::Debug for Txdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdata")
                .field("txdata", &self.txdata())
                .field("ubrxat", &self.ubrxat())
                .field("txtriat", &self.txtriat())
                .field("txbreak", &self.txbreak())
                .field("txdisat", &self.txdisat())
                .field("rxenat", &self.rxenat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txdata {{ txdata: {=u16:?}, ubrxat: {=bool:?}, txtriat: {=bool:?}, txbreak: {=bool:?}, txdisat: {=bool:?}, rxenat: {=bool:?} }}",
                self.txdata(),
                self.ubrxat(),
                self.txtriat(),
                self.txbreak(),
                self.txdisat(),
                self.rxenat()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Databits {
        _RESERVED_0 = 0x0,
        #[doc = "Each frame contains 7 data bits."]
        Seven = 0x01,
        #[doc = "Each frame contains 8 data bits."]
        Eight = 0x02,
        #[doc = "Each frame contains 9 data bits."]
        Nine = 0x03,
    }
    impl Databits {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Databits {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Databits {
        #[inline(always)]
        fn from(val: u8) -> Databits {
            Databits::from_bits(val)
        }
    }
    impl From<Databits> for u8 {
        #[inline(always)]
        fn from(val: Databits) -> u8 {
            Databits::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Irhfpw {
        #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1."]
        One = 0x0,
        #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1."]
        Two = 0x01,
        #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1."]
        Three = 0x02,
        #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1."]
        Four = 0x03,
    }
    impl Irhfpw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Irhfpw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Irhfpw {
        #[inline(always)]
        fn from(val: u8) -> Irhfpw {
            Irhfpw::from_bits(val)
        }
    }
    impl From<Irhfpw> for u8 {
        #[inline(always)]
        fn from(val: Irhfpw) -> u8 {
            Irhfpw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ovs {
        #[doc = "16X oversampling."]
        X16 = 0x0,
        #[doc = "8X oversampling."]
        X8 = 0x01,
        #[doc = "6X oversampling."]
        X6 = 0x02,
        #[doc = "4X oversampling."]
        X4 = 0x03,
        #[doc = "Disable oversampling (for LF operation)."]
        Disable = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ovs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovs {
        #[inline(always)]
        fn from(val: u8) -> Ovs {
            Ovs::from_bits(val)
        }
    }
    impl From<Ovs> for u8 {
        #[inline(always)]
        fn from(val: Ovs) -> u8 {
            Ovs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Parity {
        #[doc = "Parity bits are not used."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
        Even = 0x02,
        #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
        Odd = 0x03,
    }
    impl Parity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Parity {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Parity {
        #[inline(always)]
        fn from(val: u8) -> Parity {
            Parity::from_bits(val)
        }
    }
    impl From<Parity> for u8 {
        #[inline(always)]
        fn from(val: Parity) -> u8 {
            Parity::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtsrxfw {
        #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
        Oneframe = 0x0,
        #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
        Twoframes = 0x01,
        #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
        Threeframes = 0x02,
        #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
        Fourframes = 0x03,
    }
    impl Rtsrxfw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtsrxfw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtsrxfw {
        #[inline(always)]
        fn from(val: u8) -> Rtsrxfw {
            Rtsrxfw::from_bits(val)
        }
    }
    impl From<Rtsrxfw> for u8 {
        #[inline(always)]
        fn from(val: Rtsrxfw) -> u8 {
            Rtsrxfw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rxfiw {
        #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
        Oneframe = 0x0,
        #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
        Twoframes = 0x01,
        #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
        Threeframes = 0x02,
        #[doc = "RXFL status flag and IF are set when the RX FIFO has four frames in it."]
        Fourframes = 0x03,
    }
    impl Rxfiw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxfiw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxfiw {
        #[inline(always)]
        fn from(val: u8) -> Rxfiw {
            Rxfiw::from_bits(val)
        }
    }
    impl From<Rxfiw> for u8 {
        #[inline(always)]
        fn from(val: Rxfiw) -> u8 {
            Rxfiw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopbits {
        #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver."]
        Half = 0x0,
        #[doc = "One stop bit is generated and verified."]
        One = 0x01,
        #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit."]
        Oneandahalf = 0x02,
        #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only."]
        Two = 0x03,
    }
    impl Stopbits {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopbits {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopbits {
        #[inline(always)]
        fn from(val: u8) -> Stopbits {
            Stopbits::from_bits(val)
        }
    }
    impl From<Stopbits> for u8 {
        #[inline(always)]
        fn from(val: Stopbits) -> u8 {
            Stopbits::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Txdelay {
        #[doc = "Frames are transmitted immediately."]
        None = 0x0,
        #[doc = "Transmission of new frames is delayed by a single bit period."]
        Single = 0x01,
        #[doc = "Transmission of new frames is delayed by a two bit periods."]
        Double = 0x02,
        #[doc = "Transmission of new frames is delayed by a three bit periods."]
        Tripple = 0x03,
    }
    impl Txdelay {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Txdelay {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Txdelay {
        #[inline(always)]
        fn from(val: u8) -> Txdelay {
            Txdelay::from_bits(val)
        }
    }
    impl From<Txdelay> for u8 {
        #[inline(always)]
        fn from(val: Txdelay) -> u8 {
            Txdelay::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Txfiw {
        #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
        Oneframe = 0x0,
        #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
        Twoframes = 0x01,
        #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
        Threeframes = 0x02,
        #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
        Fourframes = 0x03,
    }
    impl Txfiw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Txfiw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Txfiw {
        #[inline(always)]
        fn from(val: u8) -> Txfiw {
            Txfiw::from_bits(val)
        }
    }
    impl From<Txfiw> for u8 {
        #[inline(always)]
        fn from(val: Txfiw) -> u8 {
            Txfiw::to_bits(val)
        }
    }
}
