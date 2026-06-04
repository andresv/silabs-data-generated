#[doc = "PFMXPPRF peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfmxpprf {
    ptr: *mut u8,
}
unsafe impl Send for Pfmxpprf {}
unsafe impl Sync for Pfmxpprf {}
impl Pfmxpprf {
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
    pub const fn rfimdcdcctrl0(self) -> crate::common::Reg<regs::Rfimdcdcctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rfimdcdcctrl1(self) -> crate::common::Reg<regs::Rfimdcdcctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rfimdcdcctrl2(self) -> crate::common::Reg<regs::Rfimdcdcctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rfimdcdcstatus(self) -> crate::common::Reg<regs::Rfimdcdcstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[inline(always)]
    pub const fn rpuratd0(self) -> crate::common::Reg<regs::Rpuratd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl0_set(self) -> crate::common::Reg<regs::Rfimdcdcctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl1_set(self) -> crate::common::Reg<regs::Rfimdcdcctrl1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl2_set(self) -> crate::common::Reg<regs::Rfimdcdcctrl2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rpuratd0_set(self) -> crate::common::Reg<regs::Rpuratd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl0_clr(self) -> crate::common::Reg<regs::Rfimdcdcctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl1_clr(self) -> crate::common::Reg<regs::Rfimdcdcctrl1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl2_clr(self) -> crate::common::Reg<regs::Rfimdcdcctrl2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rpuratd0_clr(self) -> crate::common::Reg<regs::Rpuratd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl0_tgl(self) -> crate::common::Reg<regs::Rfimdcdcctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl1_tgl(self) -> crate::common::Reg<regs::Rfimdcdcctrl1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rfimdcdcctrl2_tgl(self) -> crate::common::Reg<regs::Rfimdcdcctrl2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rpuratd0_tgl(self) -> crate::common::Reg<regs::Rpuratd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfimdcdcctrl0(pub u32);
    impl Rfimdcdcctrl0 {
        #[doc = "TX Max Req."]
        #[must_use]
        #[inline(always)]
        pub const fn txmaxreq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX Max Req."]
        #[inline(always)]
        pub const fn set_txmaxreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RX PP Req."]
        #[must_use]
        #[inline(always)]
        pub const fn rxppreq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX PP Req."]
        #[inline(always)]
        pub const fn set_rxppreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Rfimdcdcctrl0 {
        #[inline(always)]
        fn default() -> Rfimdcdcctrl0 {
            Rfimdcdcctrl0(0)
        }
    }
    impl core::fmt::Debug for Rfimdcdcctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfimdcdcctrl0")
                .field("txmaxreq", &self.txmaxreq())
                .field("rxppreq", &self.rxppreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfimdcdcctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfimdcdcctrl0 {{ txmaxreq: {=bool:?}, rxppreq: {=bool:?} }}",
                self.txmaxreq(),
                self.rxppreq()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfimdcdcctrl1(pub u32);
    impl Rfimdcdcctrl1 {
        #[doc = "DCDC DIV Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdcdiven(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC DIV Enable."]
        #[inline(always)]
        pub const fn set_dcdcdiven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCDC DIV Inverter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdcdivinven(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC DIV Inverter Enable."]
        #[inline(always)]
        pub const fn set_dcdcdivinven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCDC DIV Ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdcdivratio(&self) -> super::vals::Dcdcdivratio {
            let val = (self.0 >> 2usize) & 0x0f;
            super::vals::Dcdcdivratio::from_bits(val as u8)
        }
        #[doc = "DCDC DIV Ratio."]
        #[inline(always)]
        pub const fn set_dcdcdivratio(&mut self, val: super::vals::Dcdcdivratio) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
        }
    }
    impl Default for Rfimdcdcctrl1 {
        #[inline(always)]
        fn default() -> Rfimdcdcctrl1 {
            Rfimdcdcctrl1(0)
        }
    }
    impl core::fmt::Debug for Rfimdcdcctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfimdcdcctrl1")
                .field("dcdcdiven", &self.dcdcdiven())
                .field("dcdcdivinven", &self.dcdcdivinven())
                .field("dcdcdivratio", &self.dcdcdivratio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfimdcdcctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfimdcdcctrl1 {{ dcdcdiven: {=bool:?}, dcdcdivinven: {=bool:?}, dcdcdivratio: {:?} }}",
                self.dcdcdiven(),
                self.dcdcdivinven(),
                self.dcdcdivratio()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfimdcdcctrl2(pub u32);
    impl Rfimdcdcctrl2 {
        #[doc = "Pulse Pairing Time Max."]
        #[must_use]
        #[inline(always)]
        pub const fn pptmax(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Pulse Pairing Time Max."]
        #[inline(always)]
        pub const fn set_pptmax(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Pulse Pairing Time Min."]
        #[must_use]
        #[inline(always)]
        pub const fn pptmin(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x01ff;
            val as u16
        }
        #[doc = "Pulse Pairing Time Min."]
        #[inline(always)]
        pub const fn set_pptmin(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
        }
        #[doc = "Pulse Pairing Period."]
        #[must_use]
        #[inline(always)]
        pub const fn ppnd(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x01ff;
            val as u16
        }
        #[doc = "Pulse Pairing Period."]
        #[inline(always)]
        pub const fn set_ppnd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
        }
        #[doc = "Pulse Pairing Calibration Loop Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppcalen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Pulse Pairing Calibration Loop Enable."]
        #[inline(always)]
        pub const fn set_ppcalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Pulse Pairing Sync Only."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsynconly(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Pulse Pairing Sync Only."]
        #[inline(always)]
        pub const fn set_ppsynconly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Rfimdcdcctrl2 {
        #[inline(always)]
        fn default() -> Rfimdcdcctrl2 {
            Rfimdcdcctrl2(0)
        }
    }
    impl core::fmt::Debug for Rfimdcdcctrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfimdcdcctrl2")
                .field("pptmax", &self.pptmax())
                .field("pptmin", &self.pptmin())
                .field("ppnd", &self.ppnd())
                .field("ppcalen", &self.ppcalen())
                .field("ppsynconly", &self.ppsynconly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfimdcdcctrl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfimdcdcctrl2 {{ pptmax: {=u16:?}, pptmin: {=u16:?}, ppnd: {=u16:?}, ppcalen: {=bool:?}, ppsynconly: {=bool:?} }}",
                self.pptmax(),
                self.pptmin(),
                self.ppnd(),
                self.ppcalen(),
                self.ppsynconly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfimdcdcstatus(pub u32);
    impl Rfimdcdcstatus {
        #[doc = "DCDC Enable Status."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Enable Status."]
        #[inline(always)]
        pub const fn set_dcdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX MAX Status."]
        #[must_use]
        #[inline(always)]
        pub const fn txmaxstatus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX MAX Status."]
        #[inline(always)]
        pub const fn set_txmaxstatus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX PP Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rxppstatus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX PP Status."]
        #[inline(always)]
        pub const fn set_rxppstatus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Cal Loop WNO1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn wno1(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x01ff;
            val as u16
        }
        #[doc = "Cal Loop WNO1 value."]
        #[inline(always)]
        pub const fn set_wno1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
        }
    }
    impl Default for Rfimdcdcstatus {
        #[inline(always)]
        fn default() -> Rfimdcdcstatus {
            Rfimdcdcstatus(0)
        }
    }
    impl core::fmt::Debug for Rfimdcdcstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfimdcdcstatus")
                .field("dcdcen", &self.dcdcen())
                .field("txmaxstatus", &self.txmaxstatus())
                .field("rxppstatus", &self.rxppstatus())
                .field("wno1", &self.wno1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfimdcdcstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfimdcdcstatus {{ dcdcen: {=bool:?}, txmaxstatus: {=bool:?}, rxppstatus: {=bool:?}, wno1: {=u16:?} }}",
                self.dcdcen(),
                self.txmaxstatus(),
                self.rxppstatus(),
                self.wno1()
            )
        }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpuratd0(pub u32);
    impl Rpuratd0 {
        #[doc = "RFIMDCDCCTRL0 Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdrfimdcdcctrl0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RFIMDCDCCTRL0 Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdrfimdcdcctrl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RFIMDCDCCTRL1 Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdrfimdcdcctrl1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RFIMDCDCCTRL1 Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdrfimdcdcctrl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RFIMDCDCCTRL2 Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdrfimdcdcctrl2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RFIMDCDCCTRL2 Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdrfimdcdcctrl2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Rpuratd0 {
        #[inline(always)]
        fn default() -> Rpuratd0 {
            Rpuratd0(0)
        }
    }
    impl core::fmt::Debug for Rpuratd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpuratd0")
                .field("ratdrfimdcdcctrl0", &self.ratdrfimdcdcctrl0())
                .field("ratdrfimdcdcctrl1", &self.ratdrfimdcdcctrl1())
                .field("ratdrfimdcdcctrl2", &self.ratdrfimdcdcctrl2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rpuratd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rpuratd0 {{ ratdrfimdcdcctrl0: {=bool:?}, ratdrfimdcdcctrl1: {=bool:?}, ratdrfimdcdcctrl2: {=bool:?} }}",
                self.ratdrfimdcdcctrl0(),
                self.ratdrfimdcdcctrl1(),
                self.ratdrfimdcdcctrl2()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dcdcdivratio {
        #[doc = "Dividing master_rf clk by 8, D=50%."]
        Divratio8 = 0x0,
        #[doc = "Dividing master_rf clk by 9, D=44.4%."]
        Divratio9 = 0x01,
        #[doc = "Dividing master_rf clk by 10, D=40%."]
        Divratio10 = 0x02,
        #[doc = "Dividing master_rf clk by 11, D=36.4%."]
        Divratio11 = 0x03,
        #[doc = "Dividing master_rf clk by 12, D=50%."]
        Divratio12 = 0x04,
        #[doc = "Dividing master_rf clk by 13, D=46.2%."]
        Divratio13 = 0x05,
        #[doc = "Dividing master_rf clk by 14, D=42.9%."]
        Divratio14 = 0x06,
        #[doc = "Dividing master_rf clk by 15, D=40%."]
        Divratio15 = 0x07,
        #[doc = "Dividing master_rf clk by 16, D=50%."]
        Divratio16 = 0x08,
        #[doc = "Dividing master_rf clk by 17, D=47.1%."]
        Divratio17 = 0x09,
        #[doc = "Dividing master_rf clk by 18, D=44.4%."]
        Divratio18 = 0x0a,
        #[doc = "Dividing master_rf clk by 19, D=42.1%."]
        Divratio19 = 0x0b,
        #[doc = "Dividing master_rf clk by 20, D=60%."]
        Divratio20 = 0x0c,
        #[doc = "Dividing master_rf clk by 21, D=57.1%."]
        Divratio21 = 0x0d,
        #[doc = "Dividing master_rf clk by 22, D=54.5%."]
        Divratio22 = 0x0e,
        #[doc = "Dividing master_rf clk by 23, D=52.2%."]
        Divratio23 = 0x0f,
    }
    impl Dcdcdivratio {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dcdcdivratio {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dcdcdivratio {
        #[inline(always)]
        fn from(val: u8) -> Dcdcdivratio {
            Dcdcdivratio::from_bits(val)
        }
    }
    impl From<Dcdcdivratio> for u8 {
        #[inline(always)]
        fn from(val: Dcdcdivratio) -> u8 {
            Dcdcdivratio::to_bits(val)
        }
    }
}
