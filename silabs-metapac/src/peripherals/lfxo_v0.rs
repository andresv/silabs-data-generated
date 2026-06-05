#[doc = "LFXO peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfxo {
    ptr: *mut u8,
}
unsafe impl Send for Lfxo {}
unsafe impl Sync for Lfxo {}
impl Lfxo {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low."]
    #[inline(always)]
    pub const fn cal(self) -> crate::common::Reg<regs::Cal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cal_set(self) -> crate::common::Reg<regs::Cal, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cal_clr(self) -> crate::common::Reg<regs::Cal, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cal_tgl(self) -> crate::common::Reg<regs::Cal, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
}
pub mod regs {
    #[doc = "Do not write to this register unless CALBSY in SYNCBUSY register is low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cal(pub u32);
    impl Cal {
        #[doc = "Internal Capacitance Tuning."]
        #[must_use]
        #[inline(always)]
        pub const fn captune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Internal Capacitance Tuning."]
        #[inline(always)]
        pub const fn set_captune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "LFXO Startup Gain."]
        #[must_use]
        #[inline(always)]
        pub const fn gain(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "LFXO Startup Gain."]
        #[inline(always)]
        pub const fn set_gain(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Cal {
        #[inline(always)]
        fn default() -> Cal {
            Cal(0)
        }
    }
    impl core::fmt::Debug for Cal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cal")
                .field("captune", &self.captune())
                .field("gain", &self.gain())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cal {{ captune: {=u8:?}, gain: {=u8:?} }}",
                self.captune(),
                self.gain()
            )
        }
    }
    #[doc = "Do not write to this register unless the oscillator is forced off. The oscillator is forced off if DISONDEMAND is set and FORCEEN is cleared."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "LFXO AGC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn agc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO AGC Enable."]
        #[inline(always)]
        pub const fn set_agc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LFXO High Amplitude Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn highampl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO High Amplitude Enable."]
        #[inline(always)]
        pub const fn set_highampl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LFXO Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "LFXO Mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "LFXO Start-up Delay."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> super::vals::Timeout {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Timeout::from_bits(val as u8)
        }
        #[doc = "LFXO Start-up Delay."]
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: super::vals::Timeout) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
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
                .field("agc", &self.agc())
                .field("highampl", &self.highampl())
                .field("mode", &self.mode())
                .field("timeout", &self.timeout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ agc: {=bool:?}, highampl: {=bool:?}, mode: {:?}, timeout: {:?} }}",
                self.agc(),
                self.highampl(),
                self.mode(),
                self.timeout()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "LFXO Force Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn forceen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Force Enable."]
        #[inline(always)]
        pub const fn set_forceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LFXO Disable On-demand requests."]
        #[must_use]
        #[inline(always)]
        pub const fn disondemand(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Disable On-demand requests."]
        #[inline(always)]
        pub const fn set_disondemand(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LFXO Failure Detection Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn faildeten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Failure Detection Enable."]
        #[inline(always)]
        pub const fn set_faildeten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LFXO Failure Detection EM4WU Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn faildetem4wuen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Failure Detection EM4WU Enable."]
        #[inline(always)]
        pub const fn set_faildetem4wuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("forceen", &self.forceen())
                .field("disondemand", &self.disondemand())
                .field("faildeten", &self.faildeten())
                .field("faildetem4wuen", &self.faildetem4wuen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ forceen: {=bool:?}, disondemand: {=bool:?}, faildeten: {=bool:?}, faildetem4wuen: {=bool:?} }}",
                self.forceen(),
                self.disondemand(),
                self.faildeten(),
                self.faildetem4wuen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "LFXO Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rising Edge Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn posedge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rising Edge Interrupt Enable."]
        #[inline(always)]
        pub const fn set_posedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Falling Edge Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Falling Edge Interrupt Enable."]
        #[inline(always)]
        pub const fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LFXO Failure Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fail(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Failure Interrupt Enable."]
        #[inline(always)]
        pub const fn set_fail(&mut self, val: bool) {
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
                .field("rdy", &self.rdy())
                .field("posedge", &self.posedge())
                .field("negedge", &self.negedge())
                .field("fail", &self.fail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ rdy: {=bool:?}, posedge: {=bool:?}, negedge: {=bool:?}, fail: {=bool:?} }}",
                self.rdy(),
                self.posedge(),
                self.negedge(),
                self.fail()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "LFXO Ready Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Ready Interrupt Flag."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rising Edge Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn posedge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rising Edge Interrupt Flag."]
        #[inline(always)]
        pub const fn set_posedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Falling Edge Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Falling Edge Interrupt Flag."]
        #[inline(always)]
        pub const fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LFXO Failure Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn fail(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Failure Interrupt Flag."]
        #[inline(always)]
        pub const fn set_fail(&mut self, val: bool) {
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
                .field("rdy", &self.rdy())
                .field("posedge", &self.posedge())
                .field("negedge", &self.negedge())
                .field("fail", &self.fail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ rdy: {=bool:?}, posedge: {=bool:?}, negedge: {=bool:?}, fail: {=bool:?} }}",
                self.rdy(),
                self.posedge(),
                self.negedge(),
                self.fail()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP Version ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP Version ID."]
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
        #[doc = "Lock Key."]
        #[must_use]
        #[inline(always)]
        pub const fn lockkey(&self) -> super::vals::Lockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Lockkey::from_bits(val as u16)
        }
        #[doc = "Lock Key."]
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
    pub struct Status(pub u32);
    impl Status {
        #[doc = "LFXO Ready Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Ready Status."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LFXO Enable Status."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Enable Status."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LFXO Locked Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Locked Status."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("rdy", &self.rdy())
                .field("ens", &self.ens())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ rdy: {=bool:?}, ens: {=bool:?}, lock: {=bool:?} }}",
                self.rdy(),
                self.ens(),
                self.lock()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "LFXO Synchronization status."]
        #[must_use]
        #[inline(always)]
        pub const fn cal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Synchronization status."]
        #[inline(always)]
        pub const fn set_cal(&mut self, val: bool) {
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
            f.debug_struct("Syncbusy").field("cal", &self.cal()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Syncbusy {{ cal: {=bool:?} }}", self.cal())
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Unlock LFXO lockable registers."]
        pub const Unlock: Self = Self(0x1a20);
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
                0x1a20 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x1a20 => defmt::write!(f, "Unlock"),
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
        #[doc = "A 32768Hz crystal should be connected to the LF crystal pads. Voltage must not exceed VDDIO."]
        Xtal = 0x0,
        #[doc = "An external sine source with minimum amplitude 100mv (zero-to-peak) and maximum amplitude 500mV (zero-to-peak) should be connected in series with LFXTAL_I pin. Minimum voltage should be larger than ground and maximum voltage smaller than VDDIO. The sine source does not need to be ac coupled externally as it is ac couples inside LFXO. LFXTAL_O is free to be used as a general purpose GPIO."]
        Bufextclk = 0x01,
        #[doc = "An external 32KHz CMOS clock should be provided on LFXTAL_I. LFXTAL_O is free to be used as a general purpose GPIO."]
        Digextclk = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Timeout {
        #[doc = "Timeout period of 2 cycles."]
        Cycles2 = 0x0,
        #[doc = "Timeout period of 256 cycles."]
        Cycles256 = 0x01,
        #[doc = "Timeout period of 1024 cycles."]
        Cycles1k = 0x02,
        #[doc = "Timeout period of 2048 cycles."]
        Cycles2k = 0x03,
        #[doc = "Timeout period of 4096 cycles."]
        Cycles4k = 0x04,
        #[doc = "Timeout period of 8192 cycles."]
        Cycles8k = 0x05,
        #[doc = "Timeout period of 16384 cycles."]
        Cycles16k = 0x06,
        #[doc = "Timeout period of 32768 cycles."]
        Cycles32k = 0x07,
    }
    impl Timeout {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timeout {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timeout {
        #[inline(always)]
        fn from(val: u8) -> Timeout {
            Timeout::from_bits(val)
        }
    }
    impl From<Timeout> for u8 {
        #[inline(always)]
        fn from(val: Timeout) -> u8 {
            Timeout::to_bits(val)
        }
    }
}
