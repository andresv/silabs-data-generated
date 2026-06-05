#[doc = "SYSCFG peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Read to get system status."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Write to enable interrupts."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Read to get the hard-wired chip revision."]
    #[inline(always)]
    pub const fn chiprevhw(self) -> crate::common::Reg<regs::Chiprevhw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Read to get the chip revision programmed by feature configuration."]
    #[inline(always)]
    pub const fn chiprev(self) -> crate::common::Reg<regs::Chiprev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configure the source of the system tick for the M33."]
    #[inline(always)]
    pub const fn cfgsystic(self) -> crate::common::Reg<regs::Cfgsystic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Configure to provide general RAM configuration."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Configure to provide general RAM retention configuration."]
    #[inline(always)]
    pub const fn dmem0retnctrl(self) -> crate::common::Reg<regs::Dmem0retnctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Read to get status of the DMEM0 ECC error address."]
    #[inline(always)]
    pub const fn dmem0eccaddr(self) -> crate::common::Reg<regs::Dmem0eccaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control."]
    #[inline(always)]
    pub const fn dmem0eccctrl(self) -> crate::common::Reg<regs::Dmem0eccctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Configure SEQRAM Retention controls."]
    #[inline(always)]
    pub const fn radioramretnctrl(self) -> crate::common::Reg<regs::Radioramretnctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control."]
    #[inline(always)]
    pub const fn radioeccctrl(self) -> crate::common::Reg<regs::Radioeccctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Read to get status of the SEQRAM ECC error address."]
    #[inline(always)]
    pub const fn seqrameccaddr(self) -> crate::common::Reg<regs::Seqrameccaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Read to get status of the FRCRAM ECC error address."]
    #[inline(always)]
    pub const fn frcrameccaddr(self) -> crate::common::Reg<regs::Frcrameccaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset."]
    #[inline(always)]
    pub const fn rootdata0(self) -> crate::common::Reg<regs::Rootdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset."]
    #[inline(always)]
    pub const fn rootdata1(self) -> crate::common::Reg<regs::Rootdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "This register returns the status of the SE managed locks."]
    #[inline(always)]
    pub const fn rootlockstatus(self) -> crate::common::Reg<regs::Rootlockstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "Read to get system status. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Write to enable interrupts. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Read to get the hard-wired chip revision. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chiprevhw_set(self) -> crate::common::Reg<regs::Chiprevhw, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Read to get the chip revision programmed by feature configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chiprev_set(self) -> crate::common::Reg<regs::Chiprev, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Configure the source of the system tick for the M33. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfgsystic_set(self) -> crate::common::Reg<regs::Cfgsystic, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Configure to provide general RAM configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "Configure to provide general RAM retention configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dmem0retnctrl_set(self) -> crate::common::Reg<regs::Dmem0retnctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1208usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dmem0eccctrl_set(self) -> crate::common::Reg<regs::Dmem0eccctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1214usize) as _) }
    }
    #[doc = "Configure SEQRAM Retention controls. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn radioramretnctrl_set(self) -> crate::common::Reg<regs::Radioramretnctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1400usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn radioeccctrl_set(self) -> crate::common::Reg<regs::Radioeccctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1408usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rootdata0_set(self) -> crate::common::Reg<regs::Rootdata0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1600usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rootdata1_set(self) -> crate::common::Reg<regs::Rootdata1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1604usize) as _) }
    }
    #[doc = "Read to get system status. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "Write to enable interrupts. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Read to get the hard-wired chip revision. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chiprevhw_clr(self) -> crate::common::Reg<regs::Chiprevhw, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "Read to get the chip revision programmed by feature configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chiprev_clr(self) -> crate::common::Reg<regs::Chiprev, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "Configure the source of the system tick for the M33. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfgsystic_clr(self) -> crate::common::Reg<regs::Cfgsystic, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Configure to provide general RAM configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "Configure to provide general RAM retention configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dmem0retnctrl_clr(self) -> crate::common::Reg<regs::Dmem0retnctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2208usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dmem0eccctrl_clr(self) -> crate::common::Reg<regs::Dmem0eccctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2214usize) as _) }
    }
    #[doc = "Configure SEQRAM Retention controls. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn radioramretnctrl_clr(self) -> crate::common::Reg<regs::Radioramretnctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn radioeccctrl_clr(self) -> crate::common::Reg<regs::Radioeccctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2408usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rootdata0_clr(self) -> crate::common::Reg<regs::Rootdata0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2600usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rootdata1_clr(self) -> crate::common::Reg<regs::Rootdata1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2604usize) as _) }
    }
    #[doc = "Read to get system status. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "Write to enable interrupts. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "Read to get the hard-wired chip revision. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chiprevhw_tgl(self) -> crate::common::Reg<regs::Chiprevhw, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "Read to get the chip revision programmed by feature configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chiprev_tgl(self) -> crate::common::Reg<regs::Chiprev, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "Configure the source of the system tick for the M33. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfgsystic_tgl(self) -> crate::common::Reg<regs::Cfgsystic, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Configure to provide general RAM configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize) as _) }
    }
    #[doc = "Configure to provide general RAM retention configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dmem0retnctrl_tgl(self) -> crate::common::Reg<regs::Dmem0retnctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3208usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dmem0eccctrl_tgl(self) -> crate::common::Reg<regs::Dmem0eccctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3214usize) as _) }
    }
    #[doc = "Configure SEQRAM Retention controls. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn radioramretnctrl_tgl(self) -> crate::common::Reg<regs::Radioramretnctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3400usize) as _) }
    }
    #[doc = "Configure to set RAM ECC control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn radioeccctrl_tgl(self) -> crate::common::Reg<regs::Radioeccctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3408usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rootdata0_tgl(self) -> crate::common::Reg<regs::Rootdata0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3600usize) as _) }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rootdata1_tgl(self) -> crate::common::Reg<regs::Rootdata1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3604usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configure the source of the system tick for the M33."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgsystic(pub u32);
    impl Cfgsystic {
        #[doc = "SysTick External Clock Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn systicextclken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SysTick External Clock Enable."]
        #[inline(always)]
        pub const fn set_systicextclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Cfgsystic {
        #[inline(always)]
        fn default() -> Cfgsystic {
            Cfgsystic(0)
        }
    }
    impl core::fmt::Debug for Cfgsystic {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgsystic")
                .field("systicextclken", &self.systicextclken())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgsystic {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgsystic {{ systicextclken: {=bool:?} }}", self.systicextclken())
        }
    }
    #[doc = "Read to get the chip revision programmed by feature configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chiprev(pub u32);
    impl Chiprev {
        #[doc = "Chip Revision Major value."]
        #[must_use]
        #[inline(always)]
        pub const fn major(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Chip Revision Major value."]
        #[inline(always)]
        pub const fn set_major(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Chip Family value."]
        #[must_use]
        #[inline(always)]
        pub const fn family(&self) -> super::vals::Family {
            let val = (self.0 >> 6usize) & 0x3f;
            super::vals::Family::from_bits(val as u8)
        }
        #[doc = "Chip Family value."]
        #[inline(always)]
        pub const fn set_family(&mut self, val: super::vals::Family) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val.to_bits() as u32) & 0x3f) << 6usize);
        }
        #[doc = "Chip Revision Minor value."]
        #[must_use]
        #[inline(always)]
        pub const fn minor(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Chip Revision Minor value."]
        #[inline(always)]
        pub const fn set_minor(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for Chiprev {
        #[inline(always)]
        fn default() -> Chiprev {
            Chiprev(0)
        }
    }
    impl core::fmt::Debug for Chiprev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chiprev")
                .field("major", &self.major())
                .field("family", &self.family())
                .field("minor", &self.minor())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chiprev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Chiprev {{ major: {=u8:?}, family: {:?}, minor: {=u8:?} }}",
                self.major(),
                self.family(),
                self.minor()
            )
        }
    }
    #[doc = "Read to get the hard-wired chip revision."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chiprevhw(pub u32);
    impl Chiprevhw {
        #[doc = "Hardwired Chip Major value."]
        #[must_use]
        #[inline(always)]
        pub const fn major(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Hardwired Chip Major value."]
        #[inline(always)]
        pub const fn set_major(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Hardwired Chip Family value."]
        #[must_use]
        #[inline(always)]
        pub const fn family(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "Hardwired Chip Family value."]
        #[inline(always)]
        pub const fn set_family(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "Hardwired Chip Minor value."]
        #[must_use]
        #[inline(always)]
        pub const fn minor(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Hardwired Chip Minor value."]
        #[inline(always)]
        pub const fn set_minor(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
    }
    impl Default for Chiprevhw {
        #[inline(always)]
        fn default() -> Chiprevhw {
            Chiprevhw(0)
        }
    }
    impl core::fmt::Debug for Chiprevhw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chiprevhw")
                .field("major", &self.major())
                .field("family", &self.family())
                .field("minor", &self.minor())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chiprevhw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Chiprevhw {{ major: {=u8:?}, family: {=u8:?}, minor: {=u8:?} }}",
                self.major(),
                self.family(),
                self.minor()
            )
        }
    }
    #[doc = "Configure to provide general RAM configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Invalid Address Bus Fault Response Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn addrfaulten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Invalid Address Bus Fault Response Enable."]
        #[inline(always)]
        pub const fn set_addrfaulten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Two bit ECC Error Bus Fault Response Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rameccerrfaulten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Two bit ECC Error Bus Fault Response Enable."]
        #[inline(always)]
        pub const fn set_rameccerrfaulten(&mut self, val: bool) {
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
                .field("addrfaulten", &self.addrfaulten())
                .field("rameccerrfaulten", &self.rameccerrfaulten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ addrfaulten: {=bool:?}, rameccerrfaulten: {=bool:?} }}",
                self.addrfaulten(),
                self.rameccerrfaulten()
            )
        }
    }
    #[doc = "Read to get status of the DMEM0 ECC error address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmem0eccaddr(pub u32);
    impl Dmem0eccaddr {
        #[doc = "DMEM0 RAM ECC Error Address."]
        #[must_use]
        #[inline(always)]
        pub const fn dmem0eccaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DMEM0 RAM ECC Error Address."]
        #[inline(always)]
        pub const fn set_dmem0eccaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmem0eccaddr {
        #[inline(always)]
        fn default() -> Dmem0eccaddr {
            Dmem0eccaddr(0)
        }
    }
    impl core::fmt::Debug for Dmem0eccaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmem0eccaddr")
                .field("dmem0eccaddr", &self.dmem0eccaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmem0eccaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmem0eccaddr {{ dmem0eccaddr: {=u32:?} }}", self.dmem0eccaddr())
        }
    }
    #[doc = "Configure to set RAM ECC control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmem0eccctrl(pub u32);
    impl Dmem0eccctrl {
        #[doc = "RAM ECC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rameccen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RAM ECC Enable."]
        #[inline(always)]
        pub const fn set_rameccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RAM ECC Error Writeback Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rameccewen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RAM ECC Error Writeback Enable."]
        #[inline(always)]
        pub const fn set_rameccewen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Dmem0eccctrl {
        #[inline(always)]
        fn default() -> Dmem0eccctrl {
            Dmem0eccctrl(0)
        }
    }
    impl core::fmt::Debug for Dmem0eccctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmem0eccctrl")
                .field("rameccen", &self.rameccen())
                .field("rameccewen", &self.rameccewen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmem0eccctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmem0eccctrl {{ rameccen: {=bool:?}, rameccewen: {=bool:?} }}",
                self.rameccen(),
                self.rameccewen()
            )
        }
    }
    #[doc = "Configure to provide general RAM retention configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmem0retnctrl(pub u32);
    impl Dmem0retnctrl {
        #[doc = "DMEM0 blockset retention control."]
        #[must_use]
        #[inline(always)]
        pub const fn ramretnctrl(&self) -> super::vals::Ramretnctrl {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ramretnctrl::from_bits(val as u8)
        }
        #[doc = "DMEM0 blockset retention control."]
        #[inline(always)]
        pub const fn set_ramretnctrl(&mut self, val: super::vals::Ramretnctrl) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Dmem0retnctrl {
        #[inline(always)]
        fn default() -> Dmem0retnctrl {
            Dmem0retnctrl(0)
        }
    }
    impl core::fmt::Debug for Dmem0retnctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmem0retnctrl")
                .field("ramretnctrl", &self.ramretnctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmem0retnctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmem0retnctrl {{ ramretnctrl: {:?} }}", self.ramretnctrl())
        }
    }
    #[doc = "Read to get status of the FRCRAM ECC error address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frcrameccaddr(pub u32);
    impl Frcrameccaddr {
        #[doc = "FRCRAM ECC Error Address."]
        #[must_use]
        #[inline(always)]
        pub const fn frcrameccaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FRCRAM ECC Error Address."]
        #[inline(always)]
        pub const fn set_frcrameccaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Frcrameccaddr {
        #[inline(always)]
        fn default() -> Frcrameccaddr {
            Frcrameccaddr(0)
        }
    }
    impl core::fmt::Debug for Frcrameccaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frcrameccaddr")
                .field("frcrameccaddr", &self.frcrameccaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Frcrameccaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Frcrameccaddr {{ frcrameccaddr: {=u32:?} }}", self.frcrameccaddr())
        }
    }
    #[doc = "Write to enable interrupts."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Software interrupt 0."]
        #[must_use]
        #[inline(always)]
        pub const fn sw0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software interrupt 0."]
        #[inline(always)]
        pub const fn set_sw0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software interrupt 1."]
        #[must_use]
        #[inline(always)]
        pub const fn sw1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software interrupt 1."]
        #[inline(always)]
        pub const fn set_sw1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Software interrupt 2."]
        #[must_use]
        #[inline(always)]
        pub const fn sw2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Software interrupt 2."]
        #[inline(always)]
        pub const fn set_sw2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Software interrupt 3."]
        #[must_use]
        #[inline(always)]
        pub const fn sw3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Software interrupt 3."]
        #[inline(always)]
        pub const fn set_sw3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RAM 1-bit Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerr1b(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RAM 1-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ramerr1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAM 2-bit Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerr2b(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAM 2-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ramerr2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SEQRAM 1-bit Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seqramerr1b(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SEQRAM 1-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_seqramerr1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SEQRAM 2-bit Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seqramerr2b(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SEQRAM 2-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_seqramerr2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "FRCRAM 1-bit Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frcramerr1b(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM 1-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_frcramerr1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "FRCRAM 2-bit Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frcramerr2b(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM 2-bit Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_frcramerr2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("sw0", &self.sw0())
                .field("sw1", &self.sw1())
                .field("sw2", &self.sw2())
                .field("sw3", &self.sw3())
                .field("ramerr1b", &self.ramerr1b())
                .field("ramerr2b", &self.ramerr2b())
                .field("seqramerr1b", &self.seqramerr1b())
                .field("seqramerr2b", &self.seqramerr2b())
                .field("frcramerr1b", &self.frcramerr1b())
                .field("frcramerr2b", &self.frcramerr2b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ sw0: {=bool:?}, sw1: {=bool:?}, sw2: {=bool:?}, sw3: {=bool:?}, ramerr1b: {=bool:?}, ramerr2b: {=bool:?}, seqramerr1b: {=bool:?}, seqramerr2b: {=bool:?}, frcramerr1b: {=bool:?}, frcramerr2b: {=bool:?} }}",
                self.sw0(),
                self.sw1(),
                self.sw2(),
                self.sw3(),
                self.ramerr1b(),
                self.ramerr2b(),
                self.seqramerr1b(),
                self.seqramerr2b(),
                self.frcramerr1b(),
                self.frcramerr2b()
            )
        }
    }
    #[doc = "Read to get system status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Software Interrupt 0."]
        #[must_use]
        #[inline(always)]
        pub const fn sw0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Interrupt 0."]
        #[inline(always)]
        pub const fn set_sw0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software Interrupt 1."]
        #[must_use]
        #[inline(always)]
        pub const fn sw1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software Interrupt 1."]
        #[inline(always)]
        pub const fn set_sw1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Software Interrupt 2."]
        #[must_use]
        #[inline(always)]
        pub const fn sw2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Software Interrupt 2."]
        #[inline(always)]
        pub const fn set_sw2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Software Interrupt 3."]
        #[must_use]
        #[inline(always)]
        pub const fn sw3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Software Interrupt 3."]
        #[inline(always)]
        pub const fn set_sw3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RAM 1-Bit Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerr1b(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RAM 1-Bit Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ramerr1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAM 2-Bit Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerr2b(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAM 2-Bit Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ramerr2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SEQRAM 1-Bit Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn seqramerr1b(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SEQRAM 1-Bit Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_seqramerr1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SEQRAM 2-Bit Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn seqramerr2b(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SEQRAM 2-Bit Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_seqramerr2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "FRCRAM 1-Bit Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn frcramerr1b(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM 1-Bit Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_frcramerr1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "FRCRAM 2-Bit Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn frcramerr2b(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM 2-Bit Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_frcramerr2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("sw0", &self.sw0())
                .field("sw1", &self.sw1())
                .field("sw2", &self.sw2())
                .field("sw3", &self.sw3())
                .field("ramerr1b", &self.ramerr1b())
                .field("ramerr2b", &self.ramerr2b())
                .field("seqramerr1b", &self.seqramerr1b())
                .field("seqramerr2b", &self.seqramerr2b())
                .field("frcramerr1b", &self.frcramerr1b())
                .field("frcramerr2b", &self.frcramerr2b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ sw0: {=bool:?}, sw1: {=bool:?}, sw2: {=bool:?}, sw3: {=bool:?}, ramerr1b: {=bool:?}, ramerr2b: {=bool:?}, seqramerr1b: {=bool:?}, seqramerr2b: {=bool:?}, frcramerr1b: {=bool:?}, frcramerr2b: {=bool:?} }}",
                self.sw0(),
                self.sw1(),
                self.sw2(),
                self.sw3(),
                self.ramerr1b(),
                self.ramerr2b(),
                self.seqramerr1b(),
                self.seqramerr2b(),
                self.frcramerr1b(),
                self.frcramerr2b()
            )
        }
    }
    #[doc = "Configure to set RAM ECC control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radioeccctrl(pub u32);
    impl Radioeccctrl {
        #[doc = "SEQRAM ECC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seqrameccen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SEQRAM ECC Enable."]
        #[inline(always)]
        pub const fn set_seqrameccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SEQRAM ECC Error Writeback Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seqrameccewen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SEQRAM ECC Error Writeback Enable."]
        #[inline(always)]
        pub const fn set_seqrameccewen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FRCRAM ECC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frcrameccen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM ECC Enable."]
        #[inline(always)]
        pub const fn set_frcrameccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FRCRAM ECC Error Writeback Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn frcrameccewen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM ECC Error Writeback Enable."]
        #[inline(always)]
        pub const fn set_frcrameccewen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Radioeccctrl {
        #[inline(always)]
        fn default() -> Radioeccctrl {
            Radioeccctrl(0)
        }
    }
    impl core::fmt::Debug for Radioeccctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Radioeccctrl")
                .field("seqrameccen", &self.seqrameccen())
                .field("seqrameccewen", &self.seqrameccewen())
                .field("frcrameccen", &self.frcrameccen())
                .field("frcrameccewen", &self.frcrameccewen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Radioeccctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Radioeccctrl {{ seqrameccen: {=bool:?}, seqrameccewen: {=bool:?}, frcrameccen: {=bool:?}, frcrameccewen: {=bool:?} }}",
                self.seqrameccen(),
                self.seqrameccewen(),
                self.frcrameccen(),
                self.frcrameccewen()
            )
        }
    }
    #[doc = "Configure SEQRAM Retention controls."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radioramretnctrl(pub u32);
    impl Radioramretnctrl {
        #[doc = "SEQRAM Retention Control."]
        #[must_use]
        #[inline(always)]
        pub const fn seqramretnctrl(&self) -> super::vals::Seqramretnctrl {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Seqramretnctrl::from_bits(val as u8)
        }
        #[doc = "SEQRAM Retention Control."]
        #[inline(always)]
        pub const fn set_seqramretnctrl(&mut self, val: super::vals::Seqramretnctrl) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "FRCRAM Retention Control."]
        #[must_use]
        #[inline(always)]
        pub const fn frcramretnctrl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FRCRAM Retention Control."]
        #[inline(always)]
        pub const fn set_frcramretnctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Radioramretnctrl {
        #[inline(always)]
        fn default() -> Radioramretnctrl {
            Radioramretnctrl(0)
        }
    }
    impl core::fmt::Debug for Radioramretnctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Radioramretnctrl")
                .field("seqramretnctrl", &self.seqramretnctrl())
                .field("frcramretnctrl", &self.frcramretnctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Radioramretnctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Radioramretnctrl {{ seqramretnctrl: {:?}, frcramretnctrl: {=bool:?} }}",
                self.seqramretnctrl(),
                self.frcramretnctrl()
            )
        }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rootdata0(pub u32);
    impl Rootdata0 {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rootdata0 {
        #[inline(always)]
        fn default() -> Rootdata0 {
            Rootdata0(0)
        }
    }
    impl core::fmt::Debug for Rootdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rootdata0").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rootdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rootdata0 {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Data in this register is passed to the trusted root firmware upon reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rootdata1(pub u32);
    impl Rootdata1 {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rootdata1 {
        #[inline(always)]
        fn default() -> Rootdata1 {
            Rootdata1(0)
        }
    }
    impl core::fmt::Debug for Rootdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rootdata1").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rootdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rootdata1 {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "This register returns the status of the SE managed locks."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rootlockstatus(pub u32);
    impl Rootlockstatus {
        #[doc = "Bus Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn buslock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Lock."]
        #[inline(always)]
        pub const fn set_buslock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Register Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn reglock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Register Lock."]
        #[inline(always)]
        pub const fn set_reglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Manufacture Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn mfrlock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Manufacture Lock."]
        #[inline(always)]
        pub const fn set_mfrlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Root Mode Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn rootmodelock(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Root Mode Lock."]
        #[inline(always)]
        pub const fn set_rootmodelock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Root Debug Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn rootdbglock(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Root Debug Lock."]
        #[inline(always)]
        pub const fn set_rootdbglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "User Invasive Debug Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn userdbglock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "User Invasive Debug Lock."]
        #[inline(always)]
        pub const fn set_userdbglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "User Non-invasive Debug Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn usernidlock(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "User Non-invasive Debug Lock."]
        #[inline(always)]
        pub const fn set_usernidlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "User Secure Invasive Debug Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn userspidlock(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "User Secure Invasive Debug Lock."]
        #[inline(always)]
        pub const fn set_userspidlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "User Secure Non-invasive Debug Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn userspnidlock(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "User Secure Non-invasive Debug Lock."]
        #[inline(always)]
        pub const fn set_userspnidlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "User Debug Access Port Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn userdbgaplock(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "User Debug Access Port Lock."]
        #[inline(always)]
        pub const fn set_userdbgaplock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Radio Debug Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn radiodbglock(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Radio Debug Lock."]
        #[inline(always)]
        pub const fn set_radiodbglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Rootlockstatus {
        #[inline(always)]
        fn default() -> Rootlockstatus {
            Rootlockstatus(0)
        }
    }
    impl core::fmt::Debug for Rootlockstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rootlockstatus")
                .field("buslock", &self.buslock())
                .field("reglock", &self.reglock())
                .field("mfrlock", &self.mfrlock())
                .field("rootmodelock", &self.rootmodelock())
                .field("rootdbglock", &self.rootdbglock())
                .field("userdbglock", &self.userdbglock())
                .field("usernidlock", &self.usernidlock())
                .field("userspidlock", &self.userspidlock())
                .field("userspnidlock", &self.userspnidlock())
                .field("userdbgaplock", &self.userdbgaplock())
                .field("radiodbglock", &self.radiodbglock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rootlockstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rootlockstatus {{ buslock: {=bool:?}, reglock: {=bool:?}, mfrlock: {=bool:?}, rootmodelock: {=bool:?}, rootdbglock: {=bool:?}, userdbglock: {=bool:?}, usernidlock: {=bool:?}, userspidlock: {=bool:?}, userspnidlock: {=bool:?}, userdbgaplock: {=bool:?}, radiodbglock: {=bool:?} }}",
                self.buslock(),
                self.reglock(),
                self.mfrlock(),
                self.rootmodelock(),
                self.rootdbglock(),
                self.userdbglock(),
                self.usernidlock(),
                self.userspidlock(),
                self.userspnidlock(),
                self.userdbgaplock(),
                self.radiodbglock()
            )
        }
    }
    #[doc = "Read to get status of the SEQRAM ECC error address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seqrameccaddr(pub u32);
    impl Seqrameccaddr {
        #[doc = "SEQRAM ECC Address."]
        #[must_use]
        #[inline(always)]
        pub const fn seqrameccaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "SEQRAM ECC Address."]
        #[inline(always)]
        pub const fn set_seqrameccaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Seqrameccaddr {
        #[inline(always)]
        fn default() -> Seqrameccaddr {
            Seqrameccaddr(0)
        }
    }
    impl core::fmt::Debug for Seqrameccaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seqrameccaddr")
                .field("seqrameccaddr", &self.seqrameccaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seqrameccaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Seqrameccaddr {{ seqrameccaddr: {=u32:?} }}", self.seqrameccaddr())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Family {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        #[doc = "Product is in PG22 family."]
        Pg22 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        #[doc = "Product is in MG22 family."]
        Mg22 = 0x34,
        #[doc = "Product is in BG22 family."]
        Bg22 = 0x35,
        _RESERVED_36 = 0x36,
        #[doc = "Product is in FG22 family."]
        Fg22 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Family {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Family {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Family {
        #[inline(always)]
        fn from(val: u8) -> Family {
            Family::from_bits(val)
        }
    }
    impl From<Family> for u8 {
        #[inline(always)]
        fn from(val: Family) -> u8 {
            Family::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ramretnctrl {
        #[doc = "None of the RAM blocks powered down."]
        Allon = 0x0,
        #[doc = "Power down RAM block 0."]
        Blk0 = 0x01,
        #[doc = "Power down RAM block 1."]
        Blk1 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ramretnctrl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ramretnctrl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ramretnctrl {
        #[inline(always)]
        fn from(val: u8) -> Ramretnctrl {
            Ramretnctrl::from_bits(val)
        }
    }
    impl From<Ramretnctrl> for u8 {
        #[inline(always)]
        fn from(val: Ramretnctrl) -> u8 {
            Ramretnctrl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Seqramretnctrl {
        #[doc = "SEQRAM not powered down."]
        Allon = 0x0,
        #[doc = "Power down SEQRAM block 0."]
        Blk0 = 0x01,
        #[doc = "Power down SEQRAM block 1."]
        Blk1 = 0x02,
        #[doc = "Power down all SEQRAM blocks."]
        Alloff = 0x03,
    }
    impl Seqramretnctrl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Seqramretnctrl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Seqramretnctrl {
        #[inline(always)]
        fn from(val: u8) -> Seqramretnctrl {
            Seqramretnctrl::from_bits(val)
        }
    }
    impl From<Seqramretnctrl> for u8 {
        #[inline(always)]
        fn from(val: Seqramretnctrl) -> u8 {
            Seqramretnctrl::to_bits(val)
        }
    }
}
