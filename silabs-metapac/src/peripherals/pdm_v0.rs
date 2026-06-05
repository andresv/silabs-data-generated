#[doc = "PDM peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdm {
    ptr: *mut u8,
}
unsafe impl Send for Pdm {}
unsafe impl Sync for Pdm {}
impl Pdm {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rxdata(self) -> crate::common::Reg<regs::Rxdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg0_set(self) -> crate::common::Reg<regs::Cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg1_set(self) -> crate::common::Reg<regs::Cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg0_clr(self) -> crate::common::Reg<regs::Cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg1_clr(self) -> crate::common::Reg<regs::Cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg0_tgl(self) -> crate::common::Reg<regs::Cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg1_tgl(self) -> crate::common::Reg<regs::Cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "Filter order."]
        #[must_use]
        #[inline(always)]
        pub const fn forder(&self) -> super::vals::Forder {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Forder::from_bits(val as u8)
        }
        #[doc = "Filter order."]
        #[inline(always)]
        pub const fn set_forder(&mut self, val: super::vals::Forder) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of Channels."]
        #[must_use]
        #[inline(always)]
        pub const fn numch(&self) -> super::vals::Numch {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Numch::from_bits(val as u8)
        }
        #[doc = "Number of Channels."]
        #[inline(always)]
        pub const fn set_numch(&mut self, val: super::vals::Numch) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Filter output format."]
        #[must_use]
        #[inline(always)]
        pub const fn dataformat(&self) -> super::vals::Dataformat {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Dataformat::from_bits(val as u8)
        }
        #[doc = "Filter output format."]
        #[inline(always)]
        pub const fn set_dataformat(&mut self, val: super::vals::Dataformat) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Data Valid level in FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifodvl(&self) -> super::vals::Fifodvl {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Fifodvl::from_bits(val as u8)
        }
        #[doc = "Data Valid level in FIFO."]
        #[inline(always)]
        pub const fn set_fifodvl(&mut self, val: super::vals::Fifodvl) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Stereo mode CH01."]
        #[must_use]
        #[inline(always)]
        pub const fn stereomodech01(&self) -> super::vals::Stereomodech01 {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Stereomodech01::from_bits(val as u8)
        }
        #[doc = "Stereo mode CH01."]
        #[inline(always)]
        pub const fn set_stereomodech01(&mut self, val: super::vals::Stereomodech01) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "CH0 CLK Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0clkpol(&self) -> super::vals::Ch0clkpol {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch0clkpol::from_bits(val as u8)
        }
        #[doc = "CH0 CLK Polarity."]
        #[inline(always)]
        pub const fn set_ch0clkpol(&mut self, val: super::vals::Ch0clkpol) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "CH1 CLK Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1clkpol(&self) -> super::vals::Ch1clkpol {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Ch1clkpol::from_bits(val as u8)
        }
        #[doc = "CH1 CLK Polarity."]
        #[inline(always)]
        pub const fn set_ch1clkpol(&mut self, val: super::vals::Ch1clkpol) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
                .field("forder", &self.forder())
                .field("numch", &self.numch())
                .field("dataformat", &self.dataformat())
                .field("fifodvl", &self.fifodvl())
                .field("stereomodech01", &self.stereomodech01())
                .field("ch0clkpol", &self.ch0clkpol())
                .field("ch1clkpol", &self.ch1clkpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg0 {{ forder: {:?}, numch: {:?}, dataformat: {:?}, fifodvl: {:?}, stereomodech01: {:?}, ch0clkpol: {:?}, ch1clkpol: {:?} }}",
                self.forder(),
                self.numch(),
                self.dataformat(),
                self.fifodvl(),
                self.stereomodech01(),
                self.ch0clkpol(),
                self.ch1clkpol()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "Prescalar Setting for PDM sample."]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Prescalar Setting for PDM sample."]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Data delay buffer mux selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dlymuxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Data delay buffer mux selection."]
        #[inline(always)]
        pub const fn set_dlymuxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
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
                .field("presc", &self.presc())
                .field("dlymuxsel", &self.dlymuxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg1 {{ presc: {=u16:?}, dlymuxsel: {=u8:?} }}",
                self.presc(),
                self.dlymuxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Start DCF."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start DCF."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop DCF."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Stop DCF."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear DCF."]
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear DCF."]
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FIFO Flush."]
        #[must_use]
        #[inline(always)]
        pub const fn fifofl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Flush."]
        #[inline(always)]
        pub const fn set_fifofl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("clear", &self.clear())
                .field("fifofl", &self.fifofl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ start: {=bool:?}, stop: {=bool:?}, clear: {=bool:?}, fifofl: {=bool:?} }}",
                self.start(),
                self.stop(),
                self.clear(),
                self.fifofl()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Selects Gain factor of DCF."]
        #[must_use]
        #[inline(always)]
        pub const fn gain(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Selects Gain factor of DCF."]
        #[inline(always)]
        pub const fn set_gain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Down sampling rate of Decimation filter."]
        #[must_use]
        #[inline(always)]
        pub const fn dsr(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x0fff;
            val as u16
        }
        #[doc = "Down sampling rate of Decimation filter."]
        #[inline(always)]
        pub const fn set_dsr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("gain", &self.gain())
                .field("dsr", &self.dsr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ctrl {{ gain: {=u8:?}, dsr: {=u16:?} }}", self.gain(), self.dsr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "PDM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PDM enable."]
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
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Data Valid Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Valid Interrupt Enable."]
        #[inline(always)]
        pub const fn set_dv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data Valid Level Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dvl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data Valid Level Interrupt Enable."]
        #[inline(always)]
        pub const fn set_dvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO Undeflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Undeflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("dv", &self.dv())
                .field("dvl", &self.dvl())
                .field("of", &self.of())
                .field("uf", &self.uf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ dv: {=bool:?}, dvl: {=bool:?}, of: {=bool:?}, uf: {=bool:?} }}",
                self.dv(),
                self.dvl(),
                self.of(),
                self.uf()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Data Valid Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Valid Interrupt Flag."]
        #[inline(always)]
        pub const fn set_dv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data Valid Level Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dvl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data Valid Level Interrupt Flag."]
        #[inline(always)]
        pub const fn set_dvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FIFO Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO Undeflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Undeflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("dv", &self.dv())
                .field("dvl", &self.dvl())
                .field("of", &self.of())
                .field("uf", &self.uf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ dv: {=bool:?}, dvl: {=bool:?}, of: {=bool:?}, uf: {=bool:?} }}",
                self.dv(),
                self.dvl(),
                self.of(),
                self.uf()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP VERSION."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP VERSION."]
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
    pub struct Rxdata(pub u32);
    impl Rxdata {
        #[doc = "PDM received data."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PDM received data."]
        #[inline(always)]
        pub const fn set_rxdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Rxdata").field("rxdata", &self.rxdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rxdata {{ rxdata: {=u32:?} }}", self.rxdata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "PDM is active."]
        #[must_use]
        #[inline(always)]
        pub const fn act(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PDM is active."]
        #[inline(always)]
        pub const fn set_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO FULL Status."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO FULL Status."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FIFO EMPTY Status."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO EMPTY Status."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FIFO CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn fifocnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "FIFO CNT."]
        #[inline(always)]
        pub const fn set_fifocnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
                .field("act", &self.act())
                .field("full", &self.full())
                .field("empty", &self.empty())
                .field("fifocnt", &self.fifocnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ act: {=bool:?}, full: {=bool:?}, empty: {=bool:?}, fifocnt: {=u8:?} }}",
                self.act(),
                self.full(),
                self.empty(),
                self.fifocnt()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "sync busy."]
        #[must_use]
        #[inline(always)]
        pub const fn syncbusy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sync busy."]
        #[inline(always)]
        pub const fn set_syncbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Flush Sync busy."]
        #[must_use]
        #[inline(always)]
        pub const fn fifoflbusy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Flush Sync busy."]
        #[inline(always)]
        pub const fn set_fifoflbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("syncbusy", &self.syncbusy())
                .field("fifoflbusy", &self.fifoflbusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ syncbusy: {=bool:?}, fifoflbusy: {=bool:?} }}",
                self.syncbusy(),
                self.fifoflbusy()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0clkpol {
        #[doc = "Input data clocked on rising clock edge."]
        Normal = 0x0,
        #[doc = "Input data clocked on falling clock edge."]
        Invert = 0x01,
    }
    impl Ch0clkpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0clkpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0clkpol {
        #[inline(always)]
        fn from(val: u8) -> Ch0clkpol {
            Ch0clkpol::from_bits(val)
        }
    }
    impl From<Ch0clkpol> for u8 {
        #[inline(always)]
        fn from(val: Ch0clkpol) -> u8 {
            Ch0clkpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1clkpol {
        #[doc = "Input data clocked on rising clock edge."]
        Normal = 0x0,
        #[doc = "Input data clocked on falling clock edge."]
        Invert = 0x01,
    }
    impl Ch1clkpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1clkpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1clkpol {
        #[inline(always)]
        fn from(val: u8) -> Ch1clkpol {
            Ch1clkpol::from_bits(val)
        }
    }
    impl From<Ch1clkpol> for u8 {
        #[inline(always)]
        fn from(val: Ch1clkpol) -> u8 {
            Ch1clkpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dataformat {
        #[doc = "Right aligned 16-bit, left bits are sign extended."]
        Right16 = 0x0,
        #[doc = "Pack two 16-bit samples into one 32-bit word."]
        Double16 = 0x01,
        #[doc = "Right aligned 24bit, left bits are sign extended."]
        Right24 = 0x02,
        #[doc = "32 bit data."]
        Full32bit = 0x03,
        #[doc = "Left aligned 16-bit, right bits are zeros."]
        Left16 = 0x04,
        #[doc = "Left aligned 24-bit, right bits are zeros."]
        Left24 = 0x05,
        #[doc = "RAW 32 bit data from Integrator."]
        Raw32bit = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Dataformat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dataformat {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dataformat {
        #[inline(always)]
        fn from(val: u8) -> Dataformat {
            Dataformat::from_bits(val)
        }
    }
    impl From<Dataformat> for u8 {
        #[inline(always)]
        fn from(val: Dataformat) -> u8 {
            Dataformat::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fifodvl {
        #[doc = "Atleast one word."]
        One = 0x0,
        #[doc = "Two words."]
        Two = 0x01,
        #[doc = "Three words."]
        Three = 0x02,
        #[doc = "Four words."]
        Four = 0x03,
    }
    impl Fifodvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fifodvl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fifodvl {
        #[inline(always)]
        fn from(val: u8) -> Fifodvl {
            Fifodvl::from_bits(val)
        }
    }
    impl From<Fifodvl> for u8 {
        #[inline(always)]
        fn from(val: Fifodvl) -> u8 {
            Fifodvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Forder {
        #[doc = "Second order filter."]
        Second = 0x0,
        #[doc = "Third order filter."]
        Third = 0x01,
        #[doc = "Fourth order filter."]
        Fourth = 0x02,
        #[doc = "Fifth order filter."]
        Fifth = 0x03,
    }
    impl Forder {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Forder {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Forder {
        #[inline(always)]
        fn from(val: u8) -> Forder {
            Forder::from_bits(val)
        }
    }
    impl From<Forder> for u8 {
        #[inline(always)]
        fn from(val: Forder) -> u8 {
            Forder::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Numch {
        #[doc = "One channel."]
        One = 0x0,
        #[doc = "Two channels."]
        Two = 0x01,
    }
    impl Numch {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Numch {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Numch {
        #[inline(always)]
        fn from(val: u8) -> Numch {
            Numch::from_bits(val)
        }
    }
    impl From<Numch> for u8 {
        #[inline(always)]
        fn from(val: Numch) -> u8 {
            Numch::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stereomodech01 {
        #[doc = "No Stereo mode."]
        Disable = 0x0,
        #[doc = "CH0 and CH1 in Stereo mode."]
        Ch01enable = 0x01,
    }
    impl Stereomodech01 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stereomodech01 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stereomodech01 {
        #[inline(always)]
        fn from(val: u8) -> Stereomodech01 {
            Stereomodech01::from_bits(val)
        }
    }
    impl From<Stereomodech01> for u8 {
        #[inline(always)]
        fn from(val: Stereomodech01) -> u8 {
            Stereomodech01::to_bits(val)
        }
    }
}
