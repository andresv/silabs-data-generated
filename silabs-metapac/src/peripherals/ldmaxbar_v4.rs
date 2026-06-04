#[doc = "LDMAXBAR peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldmaxbar {
    ptr: *mut u8,
}
unsafe impl Send for Ldmaxbar {}
unsafe impl Sync for Ldmaxbar {}
impl Ldmaxbar {
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
    pub const fn ch0_reqsel(self) -> crate::common::Reg<regs::Ch0Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch1_reqsel(self) -> crate::common::Reg<regs::Ch1Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch2_reqsel(self) -> crate::common::Reg<regs::Ch2Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch3_reqsel(self) -> crate::common::Reg<regs::Ch3Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch4_reqsel(self) -> crate::common::Reg<regs::Ch4Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch5_reqsel(self) -> crate::common::Reg<regs::Ch5Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch6_reqsel(self) -> crate::common::Reg<regs::Ch6Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch7_reqsel(self) -> crate::common::Reg<regs::Ch7Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch8_reqsel(self) -> crate::common::Reg<regs::Ch8Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch9_reqsel(self) -> crate::common::Reg<regs::Ch9Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch10_reqsel(self) -> crate::common::Reg<regs::Ch10Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch11_reqsel(self) -> crate::common::Reg<regs::Ch11Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch12_reqsel(self) -> crate::common::Reg<regs::Ch12Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch13_reqsel(self) -> crate::common::Reg<regs::Ch13Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch14_reqsel(self) -> crate::common::Reg<regs::Ch14Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ch15_reqsel(self) -> crate::common::Reg<regs::Ch15Reqsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_reqsel_set(self) -> crate::common::Reg<regs::Ch0Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_reqsel_set(self) -> crate::common::Reg<regs::Ch1Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_reqsel_set(self) -> crate::common::Reg<regs::Ch2Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_reqsel_set(self) -> crate::common::Reg<regs::Ch3Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_reqsel_set(self) -> crate::common::Reg<regs::Ch4Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_reqsel_set(self) -> crate::common::Reg<regs::Ch5Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_reqsel_set(self) -> crate::common::Reg<regs::Ch6Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_reqsel_set(self) -> crate::common::Reg<regs::Ch7Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_reqsel_set(self) -> crate::common::Reg<regs::Ch8Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_reqsel_set(self) -> crate::common::Reg<regs::Ch9Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_reqsel_set(self) -> crate::common::Reg<regs::Ch10Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_reqsel_set(self) -> crate::common::Reg<regs::Ch11Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_reqsel_set(self) -> crate::common::Reg<regs::Ch12Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_reqsel_set(self) -> crate::common::Reg<regs::Ch13Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_reqsel_set(self) -> crate::common::Reg<regs::Ch14Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_reqsel_set(self) -> crate::common::Reg<regs::Ch15Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_reqsel_clr(self) -> crate::common::Reg<regs::Ch0Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_reqsel_clr(self) -> crate::common::Reg<regs::Ch1Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_reqsel_clr(self) -> crate::common::Reg<regs::Ch2Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_reqsel_clr(self) -> crate::common::Reg<regs::Ch3Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_reqsel_clr(self) -> crate::common::Reg<regs::Ch4Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_reqsel_clr(self) -> crate::common::Reg<regs::Ch5Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_reqsel_clr(self) -> crate::common::Reg<regs::Ch6Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_reqsel_clr(self) -> crate::common::Reg<regs::Ch7Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_reqsel_clr(self) -> crate::common::Reg<regs::Ch8Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_reqsel_clr(self) -> crate::common::Reg<regs::Ch9Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_reqsel_clr(self) -> crate::common::Reg<regs::Ch10Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_reqsel_clr(self) -> crate::common::Reg<regs::Ch11Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_reqsel_clr(self) -> crate::common::Reg<regs::Ch12Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_reqsel_clr(self) -> crate::common::Reg<regs::Ch13Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_reqsel_clr(self) -> crate::common::Reg<regs::Ch14Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_reqsel_clr(self) -> crate::common::Reg<regs::Ch15Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_reqsel_tgl(self) -> crate::common::Reg<regs::Ch0Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_reqsel_tgl(self) -> crate::common::Reg<regs::Ch1Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_reqsel_tgl(self) -> crate::common::Reg<regs::Ch2Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_reqsel_tgl(self) -> crate::common::Reg<regs::Ch3Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_reqsel_tgl(self) -> crate::common::Reg<regs::Ch4Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_reqsel_tgl(self) -> crate::common::Reg<regs::Ch5Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_reqsel_tgl(self) -> crate::common::Reg<regs::Ch6Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_reqsel_tgl(self) -> crate::common::Reg<regs::Ch7Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_reqsel_tgl(self) -> crate::common::Reg<regs::Ch8Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_reqsel_tgl(self) -> crate::common::Reg<regs::Ch9Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_reqsel_tgl(self) -> crate::common::Reg<regs::Ch10Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_reqsel_tgl(self) -> crate::common::Reg<regs::Ch11Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_reqsel_tgl(self) -> crate::common::Reg<regs::Ch12Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_reqsel_tgl(self) -> crate::common::Reg<regs::Ch13Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_reqsel_tgl(self) -> crate::common::Reg<regs::Ch14Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_reqsel_tgl(self) -> crate::common::Reg<regs::Ch15Reqsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Reqsel(pub u32);
    impl Ch0Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch0Reqsel {
        #[inline(always)]
        fn default() -> Ch0Reqsel {
            Ch0Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch0Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Reqsel(pub u32);
    impl Ch10Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch10Reqsel {
        #[inline(always)]
        fn default() -> Ch10Reqsel {
            Ch10Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch10Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Reqsel(pub u32);
    impl Ch11Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch11Reqsel {
        #[inline(always)]
        fn default() -> Ch11Reqsel {
            Ch11Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch11Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Reqsel(pub u32);
    impl Ch12Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch12Reqsel {
        #[inline(always)]
        fn default() -> Ch12Reqsel {
            Ch12Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch12Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Reqsel(pub u32);
    impl Ch13Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch13Reqsel {
        #[inline(always)]
        fn default() -> Ch13Reqsel {
            Ch13Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch13Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Reqsel(pub u32);
    impl Ch14Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch14Reqsel {
        #[inline(always)]
        fn default() -> Ch14Reqsel {
            Ch14Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch14Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Reqsel(pub u32);
    impl Ch15Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch15Reqsel {
        #[inline(always)]
        fn default() -> Ch15Reqsel {
            Ch15Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch15Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Reqsel(pub u32);
    impl Ch1Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch1Reqsel {
        #[inline(always)]
        fn default() -> Ch1Reqsel {
            Ch1Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch1Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Reqsel(pub u32);
    impl Ch2Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch2Reqsel {
        #[inline(always)]
        fn default() -> Ch2Reqsel {
            Ch2Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch2Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Reqsel(pub u32);
    impl Ch3Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch3Reqsel {
        #[inline(always)]
        fn default() -> Ch3Reqsel {
            Ch3Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch3Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Reqsel(pub u32);
    impl Ch4Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch4Reqsel {
        #[inline(always)]
        fn default() -> Ch4Reqsel {
            Ch4Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch4Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Reqsel(pub u32);
    impl Ch5Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch5Reqsel {
        #[inline(always)]
        fn default() -> Ch5Reqsel {
            Ch5Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch5Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Reqsel(pub u32);
    impl Ch6Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch6Reqsel {
        #[inline(always)]
        fn default() -> Ch6Reqsel {
            Ch6Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch6Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Reqsel(pub u32);
    impl Ch7Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch7Reqsel {
        #[inline(always)]
        fn default() -> Ch7Reqsel {
            Ch7Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch7Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Reqsel(pub u32);
    impl Ch8Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch8Reqsel {
        #[inline(always)]
        fn default() -> Ch8Reqsel {
            Ch8Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch8Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Reqsel(pub u32);
    impl Ch9Reqsel {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ch9Reqsel {
        #[inline(always)]
        fn default() -> Ch9Reqsel {
            Ch9Reqsel(0)
        }
    }
    impl core::fmt::Debug for Ch9Reqsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Reqsel")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Reqsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Reqsel {{ sigsel: {=u8:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
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
}
