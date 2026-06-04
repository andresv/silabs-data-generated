#[doc = "ETAMPDET peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etampdet {
    ptr: *mut u8,
}
unsafe impl Send for Etampdet {}
unsafe impl Sync for Etampdet {}
impl Etampdet {
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
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cntmismatchmax(self) -> crate::common::Reg<regs::Cntmismatchmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn chnlfiltwinsize(self) -> crate::common::Reg<regs::Chnlfiltwinsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4wuen(self) -> crate::common::Reg<regs::Em4wuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn chnlseedval0(self) -> crate::common::Reg<regs::Chnlseedval0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn chnlseedval1(self) -> crate::common::Reg<regs::Chnlseedval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER."]
    #[inline(always)]
    pub const fn clkprescval(self) -> crate::common::Reg<regs::Clkprescval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cntmismatchmax_set(self) -> crate::common::Reg<regs::Cntmismatchmax, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chnlfiltwinsize_set(self) -> crate::common::Reg<regs::Chnlfiltwinsize, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4wuen_set(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chnlseedval0_set(self) -> crate::common::Reg<regs::Chnlseedval0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chnlseedval1_set(self) -> crate::common::Reg<regs::Chnlseedval1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn clkprescval_set(self) -> crate::common::Reg<regs::Clkprescval, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cntmismatchmax_clr(self) -> crate::common::Reg<regs::Cntmismatchmax, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chnlfiltwinsize_clr(self) -> crate::common::Reg<regs::Chnlfiltwinsize, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4wuen_clr(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chnlseedval0_clr(self) -> crate::common::Reg<regs::Chnlseedval0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chnlseedval1_clr(self) -> crate::common::Reg<regs::Chnlseedval1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn clkprescval_clr(self) -> crate::common::Reg<regs::Clkprescval, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cntmismatchmax_tgl(self) -> crate::common::Reg<regs::Cntmismatchmax, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chnlfiltwinsize_tgl(self) -> crate::common::Reg<regs::Chnlfiltwinsize, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4wuen_tgl(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chnlseedval0_tgl(self) -> crate::common::Reg<regs::Chnlseedval0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chnlseedval1_tgl(self) -> crate::common::Reg<regs::Chnlseedval1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn clkprescval_tgl(self) -> crate::common::Reg<regs::Clkprescval, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "enable delay for comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlcmpdlyen0(&self) -> super::vals::Chnlcmpdlyen0 {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Chnlcmpdlyen0::from_bits(val as u8)
        }
        #[doc = "enable delay for comparison."]
        #[inline(always)]
        pub const fn set_chnlcmpdlyen0(&mut self, val: super::vals::Chnlcmpdlyen0) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "enable detect filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn chnltampdetfilten0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable detect filtering."]
        #[inline(always)]
        pub const fn set_chnltampdetfilten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable driving pad."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlpaden0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable driving pad."]
        #[inline(always)]
        pub const fn set_chnlpaden0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable delay for comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlcmpdlyen1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable delay for comparison."]
        #[inline(always)]
        pub const fn set_chnlcmpdlyen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable detect filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn chnltampdetfilten1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable detect filtering."]
        #[inline(always)]
        pub const fn set_chnltampdetfilten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "enable driving pad."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlpaden1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable driving pad."]
        #[inline(always)]
        pub const fn set_chnlpaden1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("chnlcmpdlyen0", &self.chnlcmpdlyen0())
                .field("chnltampdetfilten0", &self.chnltampdetfilten0())
                .field("chnlpaden0", &self.chnlpaden0())
                .field("chnlcmpdlyen1", &self.chnlcmpdlyen1())
                .field("chnltampdetfilten1", &self.chnltampdetfilten1())
                .field("chnlpaden1", &self.chnlpaden1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ chnlcmpdlyen0: {:?}, chnltampdetfilten0: {=bool:?}, chnlpaden0: {=bool:?}, chnlcmpdlyen1: {=bool:?}, chnltampdetfilten1: {=bool:?}, chnlpaden1: {=bool:?} }}",
                self.chnlcmpdlyen0(),
                self.chnltampdetfilten0(),
                self.chnlpaden0(),
                self.chnlcmpdlyen1(),
                self.chnltampdetfilten1(),
                self.chnlpaden1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chnlfiltwinsize(pub u32);
    impl Chnlfiltwinsize {
        #[doc = "channel 0 Filter moving window size."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlfiltwinsize0(&self) -> super::vals::Chnlfiltwinsize0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Chnlfiltwinsize0::from_bits(val as u8)
        }
        #[doc = "channel 0 Filter moving window size."]
        #[inline(always)]
        pub const fn set_chnlfiltwinsize0(&mut self, val: super::vals::Chnlfiltwinsize0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "channel 1 Filter moving window size."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlfiltwinsize1(&self) -> super::vals::Chnlfiltwinsize1 {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Chnlfiltwinsize1::from_bits(val as u8)
        }
        #[doc = "channel 1 Filter moving window size."]
        #[inline(always)]
        pub const fn set_chnlfiltwinsize1(&mut self, val: super::vals::Chnlfiltwinsize1) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Chnlfiltwinsize {
        #[inline(always)]
        fn default() -> Chnlfiltwinsize {
            Chnlfiltwinsize(0)
        }
    }
    impl core::fmt::Debug for Chnlfiltwinsize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chnlfiltwinsize")
                .field("chnlfiltwinsize0", &self.chnlfiltwinsize0())
                .field("chnlfiltwinsize1", &self.chnlfiltwinsize1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chnlfiltwinsize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Chnlfiltwinsize {{ chnlfiltwinsize0: {:?}, chnlfiltwinsize1: {:?} }}",
                self.chnlfiltwinsize0(),
                self.chnlfiltwinsize1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chnlseedval0(pub u32);
    impl Chnlseedval0 {
        #[doc = "Channel 0 LFSR Seed Value."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlseedval0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Channel 0 LFSR Seed Value."]
        #[inline(always)]
        pub const fn set_chnlseedval0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Chnlseedval0 {
        #[inline(always)]
        fn default() -> Chnlseedval0 {
            Chnlseedval0(0)
        }
    }
    impl core::fmt::Debug for Chnlseedval0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chnlseedval0")
                .field("chnlseedval0", &self.chnlseedval0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chnlseedval0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chnlseedval0 {{ chnlseedval0: {=u32:?} }}", self.chnlseedval0())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chnlseedval1(pub u32);
    impl Chnlseedval1 {
        #[doc = "Channel 1 LFSR Seed Value."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlseedval1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Channel 1 LFSR Seed Value."]
        #[inline(always)]
        pub const fn set_chnlseedval1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Chnlseedval1 {
        #[inline(always)]
        fn default() -> Chnlseedval1 {
            Chnlseedval1(0)
        }
    }
    impl core::fmt::Debug for Chnlseedval1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chnlseedval1")
                .field("chnlseedval1", &self.chnlseedval1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chnlseedval1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chnlseedval1 {{ chnlseedval1: {=u32:?} }}", self.chnlseedval1())
        }
    }
    #[doc = "Finial dividing factor = RIPPLECNTUPPER * CNTLOWER."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkprescval(pub u32);
    impl Clkprescval {
        #[doc = "Lower part of divider binary counter."]
        #[must_use]
        #[inline(always)]
        pub const fn lowerpresc(&self) -> super::vals::Lowerpresc {
            let val = (self.0 >> 0usize) & 0x3f;
            super::vals::Lowerpresc::from_bits(val as u8)
        }
        #[doc = "Lower part of divider binary counter."]
        #[inline(always)]
        pub const fn set_lowerpresc(&mut self, val: super::vals::Lowerpresc) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
        }
        #[doc = "Upper part of divider ripple counter."]
        #[must_use]
        #[inline(always)]
        pub const fn upperpresc(&self) -> super::vals::Upperpresc {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Upperpresc::from_bits(val as u8)
        }
        #[doc = "Upper part of divider ripple counter."]
        #[inline(always)]
        pub const fn set_upperpresc(&mut self, val: super::vals::Upperpresc) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Clkprescval {
        #[inline(always)]
        fn default() -> Clkprescval {
            Clkprescval(0)
        }
    }
    impl core::fmt::Debug for Clkprescval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkprescval")
                .field("lowerpresc", &self.lowerpresc())
                .field("upperpresc", &self.upperpresc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clkprescval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clkprescval {{ lowerpresc: {:?}, upperpresc: {:?} }}",
                self.lowerpresc(),
                self.upperpresc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Start channel 0 tamper detection."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstart0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start channel 0 tamper detection."]
        #[inline(always)]
        pub const fn set_chnlstart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop channel 0 tamper detection."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstop0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop channel 0 tamper detection."]
        #[inline(always)]
        pub const fn set_chnlstop0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Start channel 0 tamper detection."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlload0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Start channel 0 tamper detection."]
        #[inline(always)]
        pub const fn set_chnlload0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start channel 1 tamper detection."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstart1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start channel 1 tamper detection."]
        #[inline(always)]
        pub const fn set_chnlstart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Stop channel 1 tamper detection."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstop1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Stop channel 1 tamper detection."]
        #[inline(always)]
        pub const fn set_chnlstop1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Start channel 1 tamper detection."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlload1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Start channel 1 tamper detection."]
        #[inline(always)]
        pub const fn set_chnlload1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("chnlstart0", &self.chnlstart0())
                .field("chnlstop0", &self.chnlstop0())
                .field("chnlload0", &self.chnlload0())
                .field("chnlstart1", &self.chnlstart1())
                .field("chnlstop1", &self.chnlstop1())
                .field("chnlload1", &self.chnlload1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ chnlstart0: {=bool:?}, chnlstop0: {=bool:?}, chnlload0: {=bool:?}, chnlstart1: {=bool:?}, chnlstop1: {=bool:?}, chnlload1: {=bool:?} }}",
                self.chnlstart0(),
                self.chnlstop0(),
                self.chnlload0(),
                self.chnlstart1(),
                self.chnlstop1(),
                self.chnlload1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntmismatchmax(pub u32);
    impl Cntmismatchmax {
        #[doc = "channel 0 filter counter threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlcntmismatchmax0(&self) -> super::vals::Chnlcntmismatchmax0 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Chnlcntmismatchmax0::from_bits(val as u8)
        }
        #[doc = "channel 0 filter counter threshold."]
        #[inline(always)]
        pub const fn set_chnlcntmismatchmax0(&mut self, val: super::vals::Chnlcntmismatchmax0) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "channel 1 filter counter threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlcntmismatchmax1(&self) -> super::vals::Chnlcntmismatchmax1 {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Chnlcntmismatchmax1::from_bits(val as u8)
        }
        #[doc = "channel 1 filter counter threshold."]
        #[inline(always)]
        pub const fn set_chnlcntmismatchmax1(&mut self, val: super::vals::Chnlcntmismatchmax1) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
    }
    impl Default for Cntmismatchmax {
        #[inline(always)]
        fn default() -> Cntmismatchmax {
            Cntmismatchmax(0)
        }
    }
    impl core::fmt::Debug for Cntmismatchmax {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntmismatchmax")
                .field("chnlcntmismatchmax0", &self.chnlcntmismatchmax0())
                .field("chnlcntmismatchmax1", &self.chnlcntmismatchmax1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cntmismatchmax {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cntmismatchmax {{ chnlcntmismatchmax0: {:?}, chnlcntmismatchmax1: {:?} }}",
                self.chnlcntmismatchmax0(),
                self.chnlcntmismatchmax1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em4wuen(pub u32);
    impl Em4wuen {
        #[doc = "Channel0 Tampdet EM4 Wakeup Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlem4wuen0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel0 Tampdet EM4 Wakeup Enable."]
        #[inline(always)]
        pub const fn set_chnlem4wuen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel1 Tampdet EM4 Wakeup Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlem4wuen1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel1 Tampdet EM4 Wakeup Enable."]
        #[inline(always)]
        pub const fn set_chnlem4wuen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Em4wuen {
        #[inline(always)]
        fn default() -> Em4wuen {
            Em4wuen(0)
        }
    }
    impl core::fmt::Debug for Em4wuen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em4wuen")
                .field("chnlem4wuen0", &self.chnlem4wuen0())
                .field("chnlem4wuen1", &self.chnlem4wuen1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em4wuen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Em4wuen {{ chnlem4wuen0: {=bool:?}, chnlem4wuen1: {=bool:?} }}",
                self.chnlem4wuen0(),
                self.chnlem4wuen1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "ETAMPDET Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ETAMPDET Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Disablement busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn disabling(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Disablement busy status."]
        #[inline(always)]
        pub const fn set_disabling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            f.debug_struct("En")
                .field("en", &self.en())
                .field("disabling", &self.disabling())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for En {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "En {{ en: {=bool:?}, disabling: {=bool:?} }}",
                self.en(),
                self.disabling()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "TAMPDET0 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tampdet0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPDET0 interrupt enable."]
        #[inline(always)]
        pub const fn set_tampdet0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TAMPDET1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tampdet1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPDET1 interrupt enable."]
        #[inline(always)]
        pub const fn set_tampdet1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("tampdet0", &self.tampdet0())
                .field("tampdet1", &self.tampdet1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ tampdet0: {=bool:?}, tampdet1: {=bool:?} }}",
                self.tampdet0(),
                self.tampdet1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Tamper0 Detect Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tampdet0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper0 Detect Flag."]
        #[inline(always)]
        pub const fn set_tampdet0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Tamper1 Detect Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tampdet1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper1 Detect Flag."]
        #[inline(always)]
        pub const fn set_tampdet1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("tampdet0", &self.tampdet0())
                .field("tampdet1", &self.tampdet1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ tampdet0: {=bool:?}, tampdet1: {=bool:?} }}",
                self.tampdet0(),
                self.tampdet1()
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
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Channel0 Running Status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlrunning0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel0 Running Status."]
        #[inline(always)]
        pub const fn set_chnlrunning0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel1 Running Status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlrunning1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel1 Running Status."]
        #[inline(always)]
        pub const fn set_chnlrunning1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lockstatus(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock Status."]
        #[inline(always)]
        pub const fn set_lockstatus(&mut self, val: bool) {
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
                .field("chnlrunning0", &self.chnlrunning0())
                .field("chnlrunning1", &self.chnlrunning1())
                .field("lockstatus", &self.lockstatus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ chnlrunning0: {=bool:?}, chnlrunning1: {=bool:?}, lockstatus: {=bool:?} }}",
                self.chnlrunning0(),
                self.chnlrunning1(),
                self.lockstatus()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "Synchronizer busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstart0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronizer busy status."]
        #[inline(always)]
        pub const fn set_chnlstart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Synchronizer busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstop0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronizer busy status."]
        #[inline(always)]
        pub const fn set_chnlstop0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Synchronizer busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlload0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronizer busy status."]
        #[inline(always)]
        pub const fn set_chnlload0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Synchronizer busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstart1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronizer busy status."]
        #[inline(always)]
        pub const fn set_chnlstart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Synchronizer busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlstop1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronizer busy status."]
        #[inline(always)]
        pub const fn set_chnlstop1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Synchronizer busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn chnlload1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronizer busy status."]
        #[inline(always)]
        pub const fn set_chnlload1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("chnlstart0", &self.chnlstart0())
                .field("chnlstop0", &self.chnlstop0())
                .field("chnlload0", &self.chnlload0())
                .field("chnlstart1", &self.chnlstart1())
                .field("chnlstop1", &self.chnlstop1())
                .field("chnlload1", &self.chnlload1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ chnlstart0: {=bool:?}, chnlstop0: {=bool:?}, chnlload0: {=bool:?}, chnlstart1: {=bool:?}, chnlstop1: {=bool:?}, chnlload1: {=bool:?} }}",
                self.chnlstart0(),
                self.chnlstop0(),
                self.chnlload0(),
                self.chnlstart1(),
                self.chnlstop1(),
                self.chnlload1()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chnlcmpdlyen0 {
        #[doc = "Disables 1 clock delay to the TX value used for comparison."]
        X0 = 0x0,
        #[doc = "Enables 1 clock delay to the TX value used for comparison."]
        X1 = 0x01,
    }
    impl Chnlcmpdlyen0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chnlcmpdlyen0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chnlcmpdlyen0 {
        #[inline(always)]
        fn from(val: u8) -> Chnlcmpdlyen0 {
            Chnlcmpdlyen0::from_bits(val)
        }
    }
    impl From<Chnlcmpdlyen0> for u8 {
        #[inline(always)]
        fn from(val: Chnlcmpdlyen0) -> u8 {
            Chnlcmpdlyen0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chnlcntmismatchmax0 {
        #[doc = "Detect filter raise detect flag after seeing 1 event before reset counter expire."]
        DetectFilterThreshold1 = 0x0,
        #[doc = "Detect filter raise detect flag after seeing 2 events before reset counter expire."]
        DetectFilterThreshold2 = 0x01,
        #[doc = "Detect filter raise detect flag after seeing 3 events before reset counter expire."]
        DetectFilterThreshold3 = 0x02,
        #[doc = "Detect filter raise detect flag after seeing 4 events before reset counter expire."]
        DetectFilterThreshold4 = 0x03,
        #[doc = "Detect filter raise detect flag after seeing 5 events before reset counter expire."]
        DetectFilterThreshold5 = 0x04,
        #[doc = "Detect filter raise detect flag after seeing 6 events before reset counter expire."]
        DetectFilterThreshold6 = 0x05,
        #[doc = "Detect filter raise detect flag after seeing 7 events before reset counter expire."]
        DetectFilterThreshold7 = 0x06,
        #[doc = "Detect filter raise detect flag after seeing 8 events before reset counter expire."]
        DetectFilterThreshold8 = 0x07,
    }
    impl Chnlcntmismatchmax0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chnlcntmismatchmax0 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chnlcntmismatchmax0 {
        #[inline(always)]
        fn from(val: u8) -> Chnlcntmismatchmax0 {
            Chnlcntmismatchmax0::from_bits(val)
        }
    }
    impl From<Chnlcntmismatchmax0> for u8 {
        #[inline(always)]
        fn from(val: Chnlcntmismatchmax0) -> u8 {
            Chnlcntmismatchmax0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chnlcntmismatchmax1 {
        #[doc = "Detect filter raise detect flag after seeing 1 event before reset counter expire."]
        DetectFilterThreshold1 = 0x0,
        #[doc = "Detect filter raise detect flag after seeing 2 events before reset counter expire."]
        DetectFilterThreshold2 = 0x01,
        #[doc = "Detect filter raise detect flag after seeing 3 events before reset counter expire."]
        DetectFilterThreshold3 = 0x02,
        #[doc = "Detect filter raise detect flag after seeing 4 events before reset counter expire."]
        DetectFilterThreshold4 = 0x03,
        #[doc = "Detect filter raise detect flag after seeing 5 events before reset counter expire."]
        DetectFilterThreshold5 = 0x04,
        #[doc = "Detect filter raise detect flag after seeing 6 events before reset counter expire."]
        DetectFilterThreshold6 = 0x05,
        #[doc = "Detect filter raise detect flag after seeing 7 events before reset counter expire."]
        DetectFilterThreshold7 = 0x06,
        #[doc = "Detect filter raise detect flag after seeing 8 events before reset counter expire."]
        DetectFilterThreshold8 = 0x07,
    }
    impl Chnlcntmismatchmax1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chnlcntmismatchmax1 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chnlcntmismatchmax1 {
        #[inline(always)]
        fn from(val: u8) -> Chnlcntmismatchmax1 {
            Chnlcntmismatchmax1::from_bits(val)
        }
    }
    impl From<Chnlcntmismatchmax1> for u8 {
        #[inline(always)]
        fn from(val: Chnlcntmismatchmax1) -> u8 {
            Chnlcntmismatchmax1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chnlfiltwinsize0 {
        #[doc = "Reserved."]
        Reserved = 0x0,
        #[doc = "Detect filter moving window size is 2."]
        DetectFilterMovingWinSize2 = 0x01,
        #[doc = "Detect filter moving window size is 3."]
        DetectFilterMovingWinSize3 = 0x02,
        #[doc = "Detect filter moving window size is 4."]
        DetectFilterMovingWinSize4 = 0x03,
        #[doc = "Detect filter moving window size is 5."]
        DetectFilterMovingWinSize5 = 0x04,
        #[doc = "Detect filter moving window size is 6."]
        DetectFilterMovingWinSize6 = 0x05,
        #[doc = "Detect filter moving window size is 7."]
        DetectFilterMovingWinSize7 = 0x06,
        #[doc = "Detect filter moving window size is 8."]
        DetectFilterMovingWinSize8 = 0x07,
        #[doc = "Detect filter moving window size is 9."]
        DetectFilterMovingWinSize9 = 0x08,
        #[doc = "Detect filter moving window size is 10."]
        DetectFilterMovingWinSize10 = 0x09,
        #[doc = "Detect filter moving window size is 11."]
        DetectFilterMovingWinSize11 = 0x0a,
        #[doc = "Detect filter moving window size is 12."]
        DetectFilterMovingWinSize12 = 0x0b,
        #[doc = "Detect filter moving window size is 13."]
        DetectFilterMovingWinSize13 = 0x0c,
        #[doc = "Detect filter moving window size is 14."]
        DetectFilterMovingWinSize14 = 0x0d,
        #[doc = "Detect filter moving window size is 15."]
        DetectFilterMovingWinSize15 = 0x0e,
        #[doc = "Detect filter moving window size is 16."]
        DetectFilterMovingWinSize16 = 0x0f,
    }
    impl Chnlfiltwinsize0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chnlfiltwinsize0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chnlfiltwinsize0 {
        #[inline(always)]
        fn from(val: u8) -> Chnlfiltwinsize0 {
            Chnlfiltwinsize0::from_bits(val)
        }
    }
    impl From<Chnlfiltwinsize0> for u8 {
        #[inline(always)]
        fn from(val: Chnlfiltwinsize0) -> u8 {
            Chnlfiltwinsize0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Chnlfiltwinsize1 {
        #[doc = "Reserved."]
        Reserved = 0x0,
        #[doc = "Detect filter moving window size is 2."]
        DetectFilterMovingWinSize2 = 0x01,
        #[doc = "Detect filter moving window size is 3."]
        DetectFilterMovingWinSize3 = 0x02,
        #[doc = "Detect filter moving window size is 4."]
        DetectFilterMovingWinSize4 = 0x03,
        #[doc = "Detect filter moving window size is 5."]
        DetectFilterMovingWinSize5 = 0x04,
        #[doc = "Detect filter moving window size is 6."]
        DetectFilterMovingWinSize6 = 0x05,
        #[doc = "Detect filter moving window size is 7."]
        DetectFilterMovingWinSize7 = 0x06,
        #[doc = "Detect filter moving window size is 8."]
        DetectFilterMovingWinSize8 = 0x07,
        #[doc = "Detect filter moving window size is 9."]
        DetectFilterMovingWinSize9 = 0x08,
        #[doc = "Detect filter moving window size is 10."]
        DetectFilterMovingWinSize10 = 0x09,
        #[doc = "Detect filter moving window size is 11."]
        DetectFilterMovingWinSize11 = 0x0a,
        #[doc = "Detect filter moving window size is 12."]
        DetectFilterMovingWinSize12 = 0x0b,
        #[doc = "Detect filter moving window size is 13."]
        DetectFilterMovingWinSize13 = 0x0c,
        #[doc = "Detect filter moving window size is 14."]
        DetectFilterMovingWinSize14 = 0x0d,
        #[doc = "Detect filter moving window size is 15."]
        DetectFilterMovingWinSize15 = 0x0e,
        #[doc = "Detect filter moving window size is 16."]
        DetectFilterMovingWinSize16 = 0x0f,
    }
    impl Chnlfiltwinsize1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chnlfiltwinsize1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chnlfiltwinsize1 {
        #[inline(always)]
        fn from(val: u8) -> Chnlfiltwinsize1 {
            Chnlfiltwinsize1::from_bits(val)
        }
    }
    impl From<Chnlfiltwinsize1> for u8 {
        #[inline(always)]
        fn from(val: Chnlfiltwinsize1) -> u8 {
            Chnlfiltwinsize1::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Write to unlock all BURTC lockable registers."]
        pub const Unlock: Self = Self(0xaee8);
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
                0xaee8 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xaee8 => defmt::write!(f, "Unlock"),
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
    pub enum Lowerpresc {
        #[doc = "Divider is bypassed."]
        Bypass = 0x0,
        #[doc = "Divide by 2."]
        DivideBy2 = 0x01,
        #[doc = "Divide by 3."]
        DivideBy3 = 0x02,
        #[doc = "Divide by 4."]
        DivideBy4 = 0x03,
        #[doc = "Divide by 5."]
        DivideBy5 = 0x04,
        #[doc = "Divide by 6."]
        DivideBy6 = 0x05,
        #[doc = "Divide by 7."]
        DivideBy7 = 0x06,
        #[doc = "Divide by 8."]
        DivideBy8 = 0x07,
        #[doc = "Divide by 9."]
        DivideBy9 = 0x08,
        #[doc = "Divide by 10."]
        DivideBy10 = 0x09,
        #[doc = "Divide by 11."]
        DivideBy11 = 0x0a,
        #[doc = "Divide by 12."]
        DivideBy12 = 0x0b,
        #[doc = "Divide by 13."]
        DivideBy13 = 0x0c,
        #[doc = "Divide by 14."]
        DivideBy14 = 0x0d,
        #[doc = "Divide by 15."]
        DivideBy15 = 0x0e,
        #[doc = "Divide by 16."]
        DivideBy16 = 0x0f,
        #[doc = "Divide by 17."]
        DivideBy17 = 0x10,
        #[doc = "Divide by 18."]
        DivideBy18 = 0x11,
        #[doc = "Divide by 19."]
        DivideBy19 = 0x12,
        #[doc = "Divide by 20."]
        DivideBy20 = 0x13,
        #[doc = "Divide by 21."]
        DivideBy21 = 0x14,
        #[doc = "Divide by 22."]
        DivideBy22 = 0x15,
        #[doc = "Divide by 23."]
        DivideBy23 = 0x16,
        #[doc = "Divide by 24."]
        DivideBy24 = 0x17,
        #[doc = "Divide by 25."]
        DivideBy25 = 0x18,
        #[doc = "Divide by 26."]
        DivideBy26 = 0x19,
        #[doc = "Divide by 27."]
        DivideBy27 = 0x1a,
        #[doc = "Divide by 28."]
        DivideBy28 = 0x1b,
        #[doc = "Divide by 29."]
        DivideBy29 = 0x1c,
        #[doc = "Divide by 30."]
        DivideBy30 = 0x1d,
        #[doc = "Divide by 31."]
        DivideBy31 = 0x1e,
        #[doc = "Divide by 32."]
        DivideBy32 = 0x1f,
        #[doc = "Divide by 33."]
        DivideBy33 = 0x20,
        #[doc = "Divide by 34."]
        DivideBy34 = 0x21,
        #[doc = "Divide by 35."]
        DivideBy35 = 0x22,
        #[doc = "Divide by 36."]
        DivideBy36 = 0x23,
        #[doc = "Divide by 37."]
        DivideBy37 = 0x24,
        #[doc = "Divide by 38."]
        DivideBy38 = 0x25,
        #[doc = "Divide by 39."]
        DivideBy39 = 0x26,
        #[doc = "Divide by 40."]
        DivideBy40 = 0x27,
        #[doc = "Divide by 41."]
        DivideBy41 = 0x28,
        #[doc = "Divide by 42."]
        DivideBy42 = 0x29,
        #[doc = "Divide by 43."]
        DivideBy43 = 0x2a,
        #[doc = "Divide by 44."]
        DivideBy44 = 0x2b,
        #[doc = "Divide by 45."]
        DivideBy45 = 0x2c,
        #[doc = "Divide by 46."]
        DivideBy46 = 0x2d,
        #[doc = "Divide by 47."]
        DivideBy47 = 0x2e,
        #[doc = "Divide by 48."]
        DivideBy48 = 0x2f,
        #[doc = "Divide by 49."]
        DivideBy49 = 0x30,
        #[doc = "Divide by 50."]
        DivideBy50 = 0x31,
        #[doc = "Divide by 51."]
        DivideBy51 = 0x32,
        #[doc = "Divide by 52."]
        DivideBy52 = 0x33,
        #[doc = "Divide by 53."]
        DivideBy53 = 0x34,
        #[doc = "Divide by 54."]
        DivideBy54 = 0x35,
        #[doc = "Divide by 55."]
        DivideBy55 = 0x36,
        #[doc = "Divide by 56."]
        DivideBy56 = 0x37,
        #[doc = "Divide by 57."]
        DivideBy57 = 0x38,
        #[doc = "Divide by 58."]
        DivideBy58 = 0x39,
        #[doc = "Divide by 59."]
        DivideBy59 = 0x3a,
        #[doc = "Divide by 60."]
        DivideBy60 = 0x3b,
        #[doc = "Divide by 61."]
        DivideBy61 = 0x3c,
        #[doc = "Divide by 62."]
        DivideBy62 = 0x3d,
        #[doc = "Divide by 63."]
        DivideBy63 = 0x3e,
        #[doc = "Divide by 64."]
        DivideBy64 = 0x3f,
    }
    impl Lowerpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lowerpresc {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lowerpresc {
        #[inline(always)]
        fn from(val: u8) -> Lowerpresc {
            Lowerpresc::from_bits(val)
        }
    }
    impl From<Lowerpresc> for u8 {
        #[inline(always)]
        fn from(val: Lowerpresc) -> u8 {
            Lowerpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Upperpresc {
        #[doc = "Ripple counter divider is bypassed."]
        Bypass = 0x0,
        #[doc = "Divide by 2."]
        DivideBy2 = 0x01,
        #[doc = "Divide by 4."]
        DivideBy4 = 0x02,
        #[doc = "Divide by 8."]
        DivideBy8 = 0x03,
        #[doc = "Divide by 16."]
        DivideBy16 = 0x04,
        #[doc = "Divide by 32."]
        DivideBy32 = 0x05,
        #[doc = "Divide by 64."]
        DivideBy64 = 0x06,
        #[doc = "Reserved, programing to this value is equal to programing 0x0."]
        Reserved = 0x07,
    }
    impl Upperpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Upperpresc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Upperpresc {
        #[inline(always)]
        fn from(val: u8) -> Upperpresc {
            Upperpresc::from_bits(val)
        }
    }
    impl From<Upperpresc> for u8 {
        #[inline(always)]
        fn from(val: Upperpresc) -> u8 {
            Upperpresc::to_bits(val)
        }
    }
}
