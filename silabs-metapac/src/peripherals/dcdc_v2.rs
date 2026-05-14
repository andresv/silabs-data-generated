#[doc = "DCDC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdc {
    ptr: *mut u8,
}
unsafe impl Send for Dcdc {}
unsafe impl Sync for Dcdc {}
impl Dcdc {
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
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "EM01 Configurations."]
    #[inline(always)]
    pub const fn em01ctrl0(self) -> crate::common::Reg<regs::Em01ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "EM23 Configurations."]
    #[inline(always)]
    pub const fn em23ctrl0(self) -> crate::common::Reg<regs::Em23ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PFMX Control Register."]
    #[inline(always)]
    pub const fn pfmxctrl(self) -> crate::common::Reg<regs::Pfmxctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Flags."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "DCDC Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Syncbusy Status Register."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lockstatus(self) -> crate::common::Reg<regs::Lockstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "EM01 Configurations. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em01ctrl0_set(self) -> crate::common::Reg<regs::Em01ctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "EM23 Configurations. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em23ctrl0_set(self) -> crate::common::Reg<regs::Em23ctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "PFMX Control Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pfmxctrl_set(self) -> crate::common::Reg<regs::Pfmxctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "Control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "EM01 Configurations. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em01ctrl0_clr(self) -> crate::common::Reg<regs::Em01ctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "EM23 Configurations. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em23ctrl0_clr(self) -> crate::common::Reg<regs::Em23ctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "PFMX Control Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pfmxctrl_clr(self) -> crate::common::Reg<regs::Pfmxctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "Control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "EM01 Configurations. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em01ctrl0_tgl(self) -> crate::common::Reg<regs::Em01ctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "EM23 Configurations. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em23ctrl0_tgl(self) -> crate::common::Reg<regs::Em23ctrl0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "PFMX Control Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pfmxctrl_tgl(self) -> crate::common::Reg<regs::Pfmxctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "DCDC/Bypass Mode Control."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "DCDC/Bypass Mode Control."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Ton_max timeout control."]
        #[must_use]
        #[inline(always)]
        pub const fn ipktmaxctrl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[doc = "Ton_max timeout control."]
        #[inline(always)]
        pub const fn set_ipktmaxctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[doc = "PFMX Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pfmxextreq(&self) -> super::vals::Pfmxextreq {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Pfmxextreq::from_bits(val as u8)
        }
        #[doc = "PFMX Enable."]
        #[inline(always)]
        pub const fn set_pfmxextreq(&mut self, val: super::vals::Pfmxextreq) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("mode", &self.mode())
                .field("ipktmaxctrl", &self.ipktmaxctrl())
                .field("pfmxextreq", &self.pfmxextreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ mode: {:?}, ipktmaxctrl: {=u8:?}, pfmxextreq: {:?} }}",
                self.mode(),
                self.ipktmaxctrl(),
                self.pfmxextreq()
            )
        }
    }
    #[doc = "EM01 Configurations."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em01ctrl0(pub u32);
    impl Em01ctrl0 {
        #[doc = "EM01 Peak Current Setting."]
        #[must_use]
        #[inline(always)]
        pub const fn ipkval(&self) -> super::vals::Em01ctrl0Ipkval {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Em01ctrl0Ipkval::from_bits(val as u8)
        }
        #[doc = "EM01 Peak Current Setting."]
        #[inline(always)]
        pub const fn set_ipkval(&mut self, val: super::vals::Em01ctrl0Ipkval) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "EM01 Drive Speed Setting."]
        #[must_use]
        #[inline(always)]
        pub const fn drvspeed(&self) -> super::vals::Em01ctrl0Drvspeed {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Em01ctrl0Drvspeed::from_bits(val as u8)
        }
        #[doc = "EM01 Drive Speed Setting."]
        #[inline(always)]
        pub const fn set_drvspeed(&mut self, val: super::vals::Em01ctrl0Drvspeed) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Em01ctrl0 {
        #[inline(always)]
        fn default() -> Em01ctrl0 {
            Em01ctrl0(0)
        }
    }
    impl core::fmt::Debug for Em01ctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em01ctrl0")
                .field("ipkval", &self.ipkval())
                .field("drvspeed", &self.drvspeed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em01ctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Em01ctrl0 {{ ipkval: {:?}, drvspeed: {:?} }}",
                self.ipkval(),
                self.drvspeed()
            )
        }
    }
    #[doc = "EM23 Configurations."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em23ctrl0(pub u32);
    impl Em23ctrl0 {
        #[doc = "EM23 Peak Current Setting."]
        #[must_use]
        #[inline(always)]
        pub const fn ipkval(&self) -> super::vals::Em23ctrl0Ipkval {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Em23ctrl0Ipkval::from_bits(val as u8)
        }
        #[doc = "EM23 Peak Current Setting."]
        #[inline(always)]
        pub const fn set_ipkval(&mut self, val: super::vals::Em23ctrl0Ipkval) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "EM23 Drive Speed Setting."]
        #[must_use]
        #[inline(always)]
        pub const fn drvspeed(&self) -> super::vals::Em23ctrl0Drvspeed {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Em23ctrl0Drvspeed::from_bits(val as u8)
        }
        #[doc = "EM23 Drive Speed Setting."]
        #[inline(always)]
        pub const fn set_drvspeed(&mut self, val: super::vals::Em23ctrl0Drvspeed) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Em23ctrl0 {
        #[inline(always)]
        fn default() -> Em23ctrl0 {
            Em23ctrl0(0)
        }
    }
    impl core::fmt::Debug for Em23ctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em23ctrl0")
                .field("ipkval", &self.ipkval())
                .field("drvspeed", &self.drvspeed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em23ctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Em23ctrl0 {{ ipkval: {:?}, drvspeed: {:?} }}",
                self.ipkval(),
                self.drvspeed()
            )
        }
    }
    #[doc = "Interrupt Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Bypass Switch Enabled Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bypsw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Switch Enabled Interrupt Enable."]
        #[inline(always)]
        pub const fn set_bypsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCDC Warmup Time Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn warm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Warmup Time Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_warm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCDC Running Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Running Interrupt Enable."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VREGIN below threshold Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vreginlow(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VREGIN below threshold Interrupt Enable."]
        #[inline(always)]
        pub const fn set_vreginlow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "VREGIN above threshold Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vreginhigh(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "VREGIN above threshold Interrupt Enable."]
        #[inline(always)]
        pub const fn set_vreginhigh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DCDC in Regulation Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn regulation(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC in Regulation Interrupt Enable."]
        #[inline(always)]
        pub const fn set_regulation(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ton_max Timeout Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmax(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ton_max Timeout Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tmax(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EM4 Entry Req Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn em4err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Entry Req Interrupt Enable."]
        #[inline(always)]
        pub const fn set_em4err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PFMX Mode Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pfmxmode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PFMX Mode Interrupt Enable."]
        #[inline(always)]
        pub const fn set_pfmxmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("bypsw", &self.bypsw())
                .field("warm", &self.warm())
                .field("running", &self.running())
                .field("vreginlow", &self.vreginlow())
                .field("vreginhigh", &self.vreginhigh())
                .field("regulation", &self.regulation())
                .field("tmax", &self.tmax())
                .field("em4err", &self.em4err())
                .field("pfmxmode", &self.pfmxmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ bypsw: {=bool:?}, warm: {=bool:?}, running: {=bool:?}, vreginlow: {=bool:?}, vreginhigh: {=bool:?}, regulation: {=bool:?}, tmax: {=bool:?}, em4err: {=bool:?}, pfmxmode: {=bool:?} }}",
                self.bypsw(),
                self.warm(),
                self.running(),
                self.vreginlow(),
                self.vreginhigh(),
                self.regulation(),
                self.tmax(),
                self.em4err(),
                self.pfmxmode()
            )
        }
    }
    #[doc = "Interrupt Flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Bypass Switch Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bypsw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Switch Enabled."]
        #[inline(always)]
        pub const fn set_bypsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCDC Warmup Time Done."]
        #[must_use]
        #[inline(always)]
        pub const fn warm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Warmup Time Done."]
        #[inline(always)]
        pub const fn set_warm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCDC Running."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Running."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VREGIN below threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn vreginlow(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VREGIN below threshold."]
        #[inline(always)]
        pub const fn set_vreginlow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "VREGIN above threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn vreginhigh(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "VREGIN above threshold."]
        #[inline(always)]
        pub const fn set_vreginhigh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DCDC in regulation."]
        #[must_use]
        #[inline(always)]
        pub const fn regulation(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC in regulation."]
        #[inline(always)]
        pub const fn set_regulation(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ton_max Timeout Reached."]
        #[must_use]
        #[inline(always)]
        pub const fn tmax(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ton_max Timeout Reached."]
        #[inline(always)]
        pub const fn set_tmax(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EM4 Entry Request Error."]
        #[must_use]
        #[inline(always)]
        pub const fn em4err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Entry Request Error."]
        #[inline(always)]
        pub const fn set_em4err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Entered PFMX mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pfmxmode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Entered PFMX mode."]
        #[inline(always)]
        pub const fn set_pfmxmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("bypsw", &self.bypsw())
                .field("warm", &self.warm())
                .field("running", &self.running())
                .field("vreginlow", &self.vreginlow())
                .field("vreginhigh", &self.vreginhigh())
                .field("regulation", &self.regulation())
                .field("tmax", &self.tmax())
                .field("em4err", &self.em4err())
                .field("pfmxmode", &self.pfmxmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ bypsw: {=bool:?}, warm: {=bool:?}, running: {=bool:?}, vreginlow: {=bool:?}, vreginhigh: {=bool:?}, regulation: {=bool:?}, tmax: {=bool:?}, em4err: {=bool:?}, pfmxmode: {=bool:?} }}",
                self.bypsw(),
                self.warm(),
                self.running(),
                self.vreginlow(),
                self.vreginhigh(),
                self.regulation(),
                self.tmax(),
                self.em4err(),
                self.pfmxmode()
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
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "Configuration Lock Key."]
        #[must_use]
        #[inline(always)]
        pub const fn lockkey(&self) -> super::vals::Lockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Lockkey::from_bits(val as u16)
        }
        #[doc = "Configuration Lock Key."]
        #[inline(always)]
        pub const fn set_lockkey(&mut self, val: super::vals::Lockkey) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
    impl core::fmt::Debug for Lock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lock").field("lockkey", &self.lockkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lock {{ lockkey: {:?} }}", self.lockkey())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lockstatus(pub u32);
    impl Lockstatus {
        #[doc = "Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Lock Status."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Lockstatus {
        #[inline(always)]
        fn default() -> Lockstatus {
            Lockstatus(0)
        }
    }
    impl core::fmt::Debug for Lockstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lockstatus").field("lock", &self.lock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lockstatus {{ lock: {=bool:?} }}", self.lock())
        }
    }
    #[doc = "PFMX Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfmxctrl(pub u32);
    impl Pfmxctrl {
        #[doc = "PFMX mode Peak Current Setting."]
        #[must_use]
        #[inline(always)]
        pub const fn ipkval(&self) -> super::vals::PfmxctrlIpkval {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::PfmxctrlIpkval::from_bits(val as u8)
        }
        #[doc = "PFMX mode Peak Current Setting."]
        #[inline(always)]
        pub const fn set_ipkval(&mut self, val: super::vals::PfmxctrlIpkval) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Ton_max timeout control."]
        #[must_use]
        #[inline(always)]
        pub const fn ipktmaxctrl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Ton_max timeout control."]
        #[inline(always)]
        pub const fn set_ipktmaxctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Pfmxctrl {
        #[inline(always)]
        fn default() -> Pfmxctrl {
            Pfmxctrl(0)
        }
    }
    impl core::fmt::Debug for Pfmxctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfmxctrl")
                .field("ipkval", &self.ipkval())
                .field("ipktmaxctrl", &self.ipktmaxctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pfmxctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pfmxctrl {{ ipkval: {:?}, ipktmaxctrl: {=u8:?} }}",
                self.ipkval(),
                self.ipktmaxctrl()
            )
        }
    }
    #[doc = "DCDC Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Bypass Switch is currently enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bypsw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Switch is currently enabled."]
        #[inline(always)]
        pub const fn set_bypsw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCDC Warmup Done."]
        #[must_use]
        #[inline(always)]
        pub const fn warm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Warmup Done."]
        #[inline(always)]
        pub const fn set_warm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DCDC is running."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC is running."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VREGVDD comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn vregin(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VREGVDD comparator status."]
        #[inline(always)]
        pub const fn set_vregin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Bypass Comparator Output."]
        #[must_use]
        #[inline(always)]
        pub const fn bypcmpout(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Comparator Output."]
        #[inline(always)]
        pub const fn set_bypcmpout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DCDC in PFMX mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pfmxmode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC in PFMX mode."]
        #[inline(always)]
        pub const fn set_pfmxmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("bypsw", &self.bypsw())
                .field("warm", &self.warm())
                .field("running", &self.running())
                .field("vregin", &self.vregin())
                .field("bypcmpout", &self.bypcmpout())
                .field("pfmxmode", &self.pfmxmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ bypsw: {=bool:?}, warm: {=bool:?}, running: {=bool:?}, vregin: {=bool:?}, bypcmpout: {=bool:?}, pfmxmode: {=bool:?} }}",
                self.bypsw(),
                self.warm(),
                self.running(),
                self.vregin(),
                self.bypcmpout(),
                self.pfmxmode()
            )
        }
    }
    #[doc = "Syncbusy Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "CTRL Sync Busy Status."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CTRL Sync Busy Status."]
        #[inline(always)]
        pub const fn set_ctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EM01CTRL0 Sync Busy Status."]
        #[must_use]
        #[inline(always)]
        pub const fn em01ctrl0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EM01CTRL0 Sync Busy Status."]
        #[inline(always)]
        pub const fn set_em01ctrl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EM01CTRL1 Sync Bust Status."]
        #[must_use]
        #[inline(always)]
        pub const fn em01ctrl1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EM01CTRL1 Sync Bust Status."]
        #[inline(always)]
        pub const fn set_em01ctrl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EM23CTRL0 Sync Busy Status."]
        #[must_use]
        #[inline(always)]
        pub const fn em23ctrl0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EM23CTRL0 Sync Busy Status."]
        #[inline(always)]
        pub const fn set_em23ctrl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PFMXCTRL Sync Busy Status."]
        #[must_use]
        #[inline(always)]
        pub const fn pfmxctrl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PFMXCTRL Sync Busy Status."]
        #[inline(always)]
        pub const fn set_pfmxctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("ctrl", &self.ctrl())
                .field("em01ctrl0", &self.em01ctrl0())
                .field("em01ctrl1", &self.em01ctrl1())
                .field("em23ctrl0", &self.em23ctrl0())
                .field("pfmxctrl", &self.pfmxctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ ctrl: {=bool:?}, em01ctrl0: {=bool:?}, em01ctrl1: {=bool:?}, em23ctrl0: {=bool:?}, pfmxctrl: {=bool:?} }}",
                self.ctrl(),
                self.em01ctrl0(),
                self.em01ctrl1(),
                self.em23ctrl0(),
                self.pfmxctrl()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em01ctrl0Drvspeed {
        #[doc = "Not recommended for use (no benefit to this setting)."]
        BestEmi = 0x0,
        #[doc = "Recommended for use for best efficiency and low EMI."]
        DefaultSetting = 0x01,
        #[doc = "Not recommended for use (no benefit to this setting)."]
        Intermediate = 0x02,
        #[doc = "Not recommended for use (no benefit to this setting)."]
        BestEfficiency = 0x03,
    }
    impl Em01ctrl0Drvspeed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em01ctrl0Drvspeed {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em01ctrl0Drvspeed {
        #[inline(always)]
        fn from(val: u8) -> Em01ctrl0Drvspeed {
            Em01ctrl0Drvspeed::from_bits(val)
        }
    }
    impl From<Em01ctrl0Drvspeed> for u8 {
        #[inline(always)]
        fn from(val: Em01ctrl0Drvspeed) -> u8 {
            Em01ctrl0Drvspeed::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em01ctrl0Ipkval {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Ipeak = 90mA, Iload = 36mA."]
        Load36mA = 0x03,
        #[doc = "Ipeak = 100mA, Iload = 40mA."]
        Load40mA = 0x04,
        #[doc = "Ipeak = 110mA, Iload = 44mA."]
        Load44mA = 0x05,
        #[doc = "Ipeak = 120mA, Iload = 48mA."]
        Load48mA = 0x06,
        #[doc = "Ipeak = 130mA, Iload = 52mA."]
        Load52mA = 0x07,
        #[doc = "Ipeak = 140mA, Iload = 56mA."]
        Load56mA = 0x08,
        #[doc = "Ipeak = 150mA, Iload = 60mA."]
        Load60mA = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Em01ctrl0Ipkval {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em01ctrl0Ipkval {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em01ctrl0Ipkval {
        #[inline(always)]
        fn from(val: u8) -> Em01ctrl0Ipkval {
            Em01ctrl0Ipkval::from_bits(val)
        }
    }
    impl From<Em01ctrl0Ipkval> for u8 {
        #[inline(always)]
        fn from(val: Em01ctrl0Ipkval) -> u8 {
            Em01ctrl0Ipkval::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em23ctrl0Drvspeed {
        #[doc = "Not recommended for use (no benefit to this setting)."]
        BestEmi = 0x0,
        #[doc = "Recommended for use for best efficiency and low EMI."]
        DefaultSetting = 0x01,
        #[doc = "Not recommended for use (no benefit to this setting)."]
        Intermediate = 0x02,
        #[doc = "Not recommended for use (no benefit to this setting)."]
        BestEfficiency = 0x03,
    }
    impl Em23ctrl0Drvspeed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em23ctrl0Drvspeed {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em23ctrl0Drvspeed {
        #[inline(always)]
        fn from(val: u8) -> Em23ctrl0Drvspeed {
            Em23ctrl0Drvspeed::from_bits(val)
        }
    }
    impl From<Em23ctrl0Drvspeed> for u8 {
        #[inline(always)]
        fn from(val: Em23ctrl0Drvspeed) -> u8 {
            Em23ctrl0Drvspeed::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em23ctrl0Ipkval {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Ipeak = 90mA, Iload = 5 mA."]
        Load5mA = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        #[doc = "Ipeak = 150mA, Iload = 10 mA."]
        Load10mA = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Em23ctrl0Ipkval {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em23ctrl0Ipkval {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em23ctrl0Ipkval {
        #[inline(always)]
        fn from(val: u8) -> Em23ctrl0Ipkval {
            Em23ctrl0Ipkval::from_bits(val)
        }
    }
    impl From<Em23ctrl0Ipkval> for u8 {
        #[inline(always)]
        fn from(val: Em23ctrl0Ipkval) -> u8 {
            Em23ctrl0Ipkval::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "UNLOCKKEY."]
        pub const Unlockkey: Self = Self(0xabcd);
    }
    impl Lockkey {
        pub const fn from_bits(val: u16) -> Lockkey {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Lockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xabcd => f.write_str("Unlockkey"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xabcd => defmt::write!(f, "Unlockkey"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Lockkey {
        #[inline(always)]
        fn from(val: u16) -> Lockkey {
            Lockkey::from_bits(val)
        }
    }
    impl From<Lockkey> for u16 {
        #[inline(always)]
        fn from(val: Lockkey) -> u16 {
            Lockkey::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "DCDC is OFF, bypass switch is enabled."]
        Bypass = 0x0,
        #[doc = "Request DCDC regulation, bypass switch disabled."]
        Dcdcregulation = 0x01,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PfmxctrlIpkval {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Ipeak = 90 mA, Iload = 50 mA."]
        Load50ma = 0x03,
        #[doc = "Ipeak = 100 mA, Iload = 65 mA."]
        Load65ma = 0x04,
        #[doc = "Ipeak = 110 mA, Iload = 73 mA."]
        Load73ma = 0x05,
        #[doc = "Ipeak = 120 mA, Iload = 80 mA."]
        Load80ma = 0x06,
        #[doc = "Ipeak = 130 mA, Iload = 86 mA."]
        Load86ma = 0x07,
        #[doc = "Ipeak = 140 mA, Iload = 93 mA."]
        Load93ma = 0x08,
        #[doc = "Ipeak = 150 mA, Iload = 100 mA."]
        Load100ma = 0x09,
        #[doc = "Ipeak = 160 mA, Iload = 106 mA."]
        Load106ma = 0x0a,
        #[doc = "Ipeak = 170 mA, Iload = 113 mA."]
        Load113ma = 0x0b,
        #[doc = "Ipeak = 180 mA, Iload = 120 mA."]
        Load120ma = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl PfmxctrlIpkval {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PfmxctrlIpkval {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PfmxctrlIpkval {
        #[inline(always)]
        fn from(val: u8) -> PfmxctrlIpkval {
            PfmxctrlIpkval::from_bits(val)
        }
    }
    impl From<PfmxctrlIpkval> for u8 {
        #[inline(always)]
        fn from(val: PfmxctrlIpkval) -> u8 {
            PfmxctrlIpkval::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pfmxextreq {
        #[doc = "Disable PFMX mode."]
        Extlowload = 0x0,
        #[doc = "Enable PFMX mode to support high load current."]
        Exthighload = 0x01,
    }
    impl Pfmxextreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pfmxextreq {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pfmxextreq {
        #[inline(always)]
        fn from(val: u8) -> Pfmxextreq {
            Pfmxextreq::from_bits(val)
        }
    }
    impl From<Pfmxextreq> for u8 {
        #[inline(always)]
        fn from(val: Pfmxextreq) -> u8 {
            Pfmxextreq::to_bits(val)
        }
    }
}
