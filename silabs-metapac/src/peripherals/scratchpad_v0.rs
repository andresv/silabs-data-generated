#[doc = "SCRATCHPAD peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scratchpad {
    ptr: *mut u8,
}
unsafe impl Send for Scratchpad {}
unsafe impl Sync for Scratchpad {}
impl Scratchpad {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Used for SIMCTRL Pointer in Verification Environment."]
    #[inline(always)]
    pub const fn sreg0(self) -> crate::common::Reg<regs::Sreg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Used for SIMCTRL Data Access in Verification Environment."]
    #[inline(always)]
    pub const fn sreg1(self) -> crate::common::Reg<regs::Sreg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Used for SIMCTRL Pointer in Verification Environment. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sreg0_set(self) -> crate::common::Reg<regs::Sreg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Used for SIMCTRL Data Access in Verification Environment. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sreg1_set(self) -> crate::common::Reg<regs::Sreg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Used for SIMCTRL Pointer in Verification Environment. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sreg0_clr(self) -> crate::common::Reg<regs::Sreg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "Used for SIMCTRL Data Access in Verification Environment. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sreg1_clr(self) -> crate::common::Reg<regs::Sreg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Used for SIMCTRL Pointer in Verification Environment. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sreg0_tgl(self) -> crate::common::Reg<regs::Sreg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "Used for SIMCTRL Data Access in Verification Environment. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sreg1_tgl(self) -> crate::common::Reg<regs::Sreg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
}
pub mod regs {
    #[doc = "Used for SIMCTRL Pointer in Verification Environment."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sreg0(pub u32);
    impl Sreg0 {
        #[doc = "Scratch Pad Register."]
        #[must_use]
        #[inline(always)]
        pub const fn scratch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Scratch Pad Register."]
        #[inline(always)]
        pub const fn set_scratch(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sreg0 {
        #[inline(always)]
        fn default() -> Sreg0 {
            Sreg0(0)
        }
    }
    impl core::fmt::Debug for Sreg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sreg0").field("scratch", &self.scratch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sreg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sreg0 {{ scratch: {=u32:?} }}", self.scratch())
        }
    }
    #[doc = "Used for SIMCTRL Data Access in Verification Environment."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sreg1(pub u32);
    impl Sreg1 {
        #[doc = "Scratch Register."]
        #[must_use]
        #[inline(always)]
        pub const fn scratch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Scratch Register."]
        #[inline(always)]
        pub const fn set_scratch(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sreg1 {
        #[inline(always)]
        fn default() -> Sreg1 {
            Sreg1(0)
        }
    }
    impl core::fmt::Debug for Sreg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sreg1").field("scratch", &self.scratch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sreg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sreg1 {{ scratch: {=u32:?} }}", self.scratch())
        }
    }
}
