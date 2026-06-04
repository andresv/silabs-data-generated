#[doc = "SYSCFG_NS_CFGNS peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscfgNsCfgns {
    ptr: *mut u8,
}
unsafe impl Send for SyscfgNsCfgns {}
unsafe impl Sync for SyscfgNsCfgns {}
impl SyscfgNsCfgns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configure to define the system tick for the M33."]
    #[inline(always)]
    pub const fn cfgnstcalib(self) -> crate::common::Reg<regs::Cfgnstcalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Generic data space for user to pass to root, e.g., address of struct in mem."]
    #[inline(always)]
    pub const fn rootnsdata0(self) -> crate::common::Reg<regs::Rootnsdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Generic data space for user to pass to root, e.g., address of struct in mem."]
    #[inline(always)]
    pub const fn rootnsdata1(self) -> crate::common::Reg<regs::Rootnsdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[inline(always)]
    pub const fn cfgnsrpuratd0(self) -> crate::common::Reg<regs::Cfgnsrpuratd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[inline(always)]
    pub const fn cfgnsrpuratd12(self) -> crate::common::Reg<regs::Cfgnsrpuratd12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0638usize) as _) }
    }
}
pub mod regs {
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgnsrpuratd0(pub u32);
    impl Cfgnsrpuratd0 {
        #[doc = "CFGNSTCALIB Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdcfgnstcalib(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CFGNSTCALIB Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdcfgnstcalib(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CFGNSSYSTIC Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdcfgnssystic(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CFGNSSYSTIC Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdcfgnssystic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfgnsrpuratd0 {
        #[inline(always)]
        fn default() -> Cfgnsrpuratd0 {
            Cfgnsrpuratd0(0)
        }
    }
    impl core::fmt::Debug for Cfgnsrpuratd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgnsrpuratd0")
                .field("ratdcfgnstcalib", &self.ratdcfgnstcalib())
                .field("ratdcfgnssystic", &self.ratdcfgnssystic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgnsrpuratd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgnsrpuratd0 {{ ratdcfgnstcalib: {=bool:?}, ratdcfgnssystic: {=bool:?} }}",
                self.ratdcfgnstcalib(),
                self.ratdcfgnssystic()
            )
        }
    }
    #[doc = "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgnsrpuratd12(pub u32);
    impl Cfgnsrpuratd12 {
        #[doc = "DATA0 Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdrootnsdata0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DATA0 Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdrootnsdata0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DATA1 Protection Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ratdrootnsdata1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DATA1 Protection Bit."]
        #[inline(always)]
        pub const fn set_ratdrootnsdata1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cfgnsrpuratd12 {
        #[inline(always)]
        fn default() -> Cfgnsrpuratd12 {
            Cfgnsrpuratd12(0)
        }
    }
    impl core::fmt::Debug for Cfgnsrpuratd12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgnsrpuratd12")
                .field("ratdrootnsdata0", &self.ratdrootnsdata0())
                .field("ratdrootnsdata1", &self.ratdrootnsdata1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgnsrpuratd12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgnsrpuratd12 {{ ratdrootnsdata0: {=bool:?}, ratdrootnsdata1: {=bool:?} }}",
                self.ratdrootnsdata0(),
                self.ratdrootnsdata1()
            )
        }
    }
    #[doc = "Configure to define the system tick for the M33."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgnstcalib(pub u32);
    impl Cfgnstcalib {
        #[doc = "Ten Milliseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn tenms(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Ten Milliseconds."]
        #[inline(always)]
        pub const fn set_tenms(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Skew."]
        #[must_use]
        #[inline(always)]
        pub const fn skew(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Skew."]
        #[inline(always)]
        pub const fn set_skew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No Reference."]
        #[must_use]
        #[inline(always)]
        pub const fn noref(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No Reference."]
        #[inline(always)]
        pub const fn set_noref(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Cfgnstcalib {
        #[inline(always)]
        fn default() -> Cfgnstcalib {
            Cfgnstcalib(0)
        }
    }
    impl core::fmt::Debug for Cfgnstcalib {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgnstcalib")
                .field("tenms", &self.tenms())
                .field("skew", &self.skew())
                .field("noref", &self.noref())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgnstcalib {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgnstcalib {{ tenms: {=u32:?}, skew: {=bool:?}, noref: {=bool:?} }}",
                self.tenms(),
                self.skew(),
                self.noref()
            )
        }
    }
    #[doc = "Generic data space for user to pass to root, e.g., address of struct in mem."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rootnsdata0(pub u32);
    impl Rootnsdata0 {
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
    impl Default for Rootnsdata0 {
        #[inline(always)]
        fn default() -> Rootnsdata0 {
            Rootnsdata0(0)
        }
    }
    impl core::fmt::Debug for Rootnsdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rootnsdata0").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rootnsdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rootnsdata0 {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Generic data space for user to pass to root, e.g., address of struct in mem."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rootnsdata1(pub u32);
    impl Rootnsdata1 {
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
    impl Default for Rootnsdata1 {
        #[inline(always)]
        fn default() -> Rootnsdata1 {
            Rootnsdata1(0)
        }
    }
    impl core::fmt::Debug for Rootnsdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rootnsdata1").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rootnsdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rootnsdata1 {{ data: {=u32:?} }}", self.data())
        }
    }
}
