#[doc = "LFRCO peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfrco {
    ptr: *mut u8,
}
unsafe impl Send for Lfrco {}
unsafe impl Sync for Lfrco {}
impl Lfrco {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Contains the LFRCO ip version."]
    #[inline(always)]
    pub const fn ipversion(self) -> crate::common::Reg<regs::Ipversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Calibration register."]
    #[inline(always)]
    pub const fn cal(self) -> crate::common::Reg<regs::Cal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt flag register."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt enable register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Synchronization busy register."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Calibration register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cal_set(self) -> crate::common::Reg<regs::Cal, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Interrupt flag register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Interrupt enable register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Calibration register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cal_clr(self) -> crate::common::Reg<regs::Cal, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "Interrupt flag register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "Interrupt enable register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Calibration register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cal_tgl(self) -> crate::common::Reg<regs::Cal, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "Interrupt flag register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "Interrupt enable register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
}
pub mod regs {
    #[doc = "Calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cal(pub u32);
    impl Cal {
        #[doc = "Frequency Trim."]
        #[must_use]
        #[inline(always)]
        pub const fn freqtrim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frequency Trim."]
        #[inline(always)]
        pub const fn set_freqtrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            f.debug_struct("Cal").field("freqtrim", &self.freqtrim()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cal {{ freqtrim: {=u8:?} }}", self.freqtrim())
        }
    }
    #[doc = "Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ready Interrupt Enable."]
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ rdy: {=bool:?}, posedge: {=bool:?}, negedge: {=bool:?} }}",
                self.rdy(),
                self.posedge(),
                self.negedge()
            )
        }
    }
    #[doc = "Interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Ready Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ready Interrupt Flag."]
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ rdy: {=bool:?}, posedge: {=bool:?}, negedge: {=bool:?} }}",
                self.rdy(),
                self.posedge(),
                self.negedge()
            )
        }
    }
    #[doc = "Contains the LFRCO ip version."]
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
    #[doc = "Configuration lock register. Locks/unlocks access to cofiguration registers."]
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
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Ready Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ready Status."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enabled Status."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enabled Status."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock Status."]
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
    #[doc = "Synchronization busy register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "CAL Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn cal(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CAL Busy."]
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
        #[doc = "Lock Configuration Registers."]
        pub const Lock: Self = Self(0x0);
        #[doc = "Unlock Configuaration Registers."]
        pub const Unlock: Self = Self(0x2603);
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
                0x0 => f.write_str("Lock"),
                0x2603 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Lock"),
                0x2603 => defmt::write!(f, "Unlock"),
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
}
