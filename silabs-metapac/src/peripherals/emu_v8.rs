#[doc = "EMU peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emu {
    ptr: *mut u8,
}
unsafe impl Send for Emu {}
unsafe impl Sync for Emu {}
impl Emu {
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
    pub const fn decbod(self) -> crate::common::Reg<regs::Decbod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn bod3sense(self) -> crate::common::Reg<regs::Bod3sense, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn vregvddcmpctrl(self) -> crate::common::Reg<regs::Vregvddcmpctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pd1paretctrl(self) -> crate::common::Reg<regs::Pd1paretctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4ctrl(self) -> crate::common::Reg<regs::Em4ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn templimits(self) -> crate::common::Reg<regs::Templimits, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn temp(self) -> crate::common::Reg<regs::Temp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rstctrl(self) -> crate::common::Reg<regs::Rstctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rstcause(self) -> crate::common::Reg<regs::Rstcause, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn dgif(self) -> crate::common::Reg<regs::Dgif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn dgien(self) -> crate::common::Reg<regs::Dgien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn efpif(self) -> crate::common::Reg<regs::Efpif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn efpien(self) -> crate::common::Reg<regs::Efpien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn decbod_set(self) -> crate::common::Reg<regs::Decbod, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bod3sense_set(self) -> crate::common::Reg<regs::Bod3sense, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn vregvddcmpctrl_set(self) -> crate::common::Reg<regs::Vregvddcmpctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pd1paretctrl_set(self) -> crate::common::Reg<regs::Pd1paretctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4ctrl_set(self) -> crate::common::Reg<regs::Em4ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn templimits_set(self) -> crate::common::Reg<regs::Templimits, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rstctrl_set(self) -> crate::common::Reg<regs::Rstctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dgif_set(self) -> crate::common::Reg<regs::Dgif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dgien_set(self) -> crate::common::Reg<regs::Dgien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn efpif_set(self) -> crate::common::Reg<regs::Efpif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn efpien_set(self) -> crate::common::Reg<regs::Efpien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn decbod_clr(self) -> crate::common::Reg<regs::Decbod, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bod3sense_clr(self) -> crate::common::Reg<regs::Bod3sense, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn vregvddcmpctrl_clr(self) -> crate::common::Reg<regs::Vregvddcmpctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pd1paretctrl_clr(self) -> crate::common::Reg<regs::Pd1paretctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4ctrl_clr(self) -> crate::common::Reg<regs::Em4ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x206cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn templimits_clr(self) -> crate::common::Reg<regs::Templimits, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2078usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rstctrl_clr(self) -> crate::common::Reg<regs::Rstctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dgif_clr(self) -> crate::common::Reg<regs::Dgif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dgien_clr(self) -> crate::common::Reg<regs::Dgien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn efpif_clr(self) -> crate::common::Reg<regs::Efpif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn efpien_clr(self) -> crate::common::Reg<regs::Efpien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn decbod_tgl(self) -> crate::common::Reg<regs::Decbod, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bod3sense_tgl(self) -> crate::common::Reg<regs::Bod3sense, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn vregvddcmpctrl_tgl(self) -> crate::common::Reg<regs::Vregvddcmpctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pd1paretctrl_tgl(self) -> crate::common::Reg<regs::Pd1paretctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4ctrl_tgl(self) -> crate::common::Reg<regs::Em4ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn templimits_tgl(self) -> crate::common::Reg<regs::Templimits, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3078usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rstctrl_tgl(self) -> crate::common::Reg<regs::Rstctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dgif_tgl(self) -> crate::common::Reg<regs::Dgif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dgien_tgl(self) -> crate::common::Reg<regs::Dgien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn efpif_tgl(self) -> crate::common::Reg<regs::Efpif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn efpien_tgl(self) -> crate::common::Reg<regs::Efpien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3104usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bod3sense(pub u32);
    impl Bod3sense {
        #[doc = "AVDD BOD enable."]
        #[must_use]
        #[inline(always)]
        pub const fn avddboden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AVDD BOD enable."]
        #[inline(always)]
        pub const fn set_avddboden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VDDIO0 BOD enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vddio0boden(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO0 BOD enable."]
        #[inline(always)]
        pub const fn set_vddio0boden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VDDIO1 BOD enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vddio1boden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO1 BOD enable."]
        #[inline(always)]
        pub const fn set_vddio1boden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Bod3sense {
        #[inline(always)]
        fn default() -> Bod3sense {
            Bod3sense(0)
        }
    }
    impl core::fmt::Debug for Bod3sense {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bod3sense")
                .field("avddboden", &self.avddboden())
                .field("vddio0boden", &self.vddio0boden())
                .field("vddio1boden", &self.vddio1boden())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bod3sense {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bod3sense {{ avddboden: {=bool:?}, vddio0boden: {=bool:?}, vddio1boden: {=bool:?} }}",
                self.avddboden(),
                self.vddio0boden(),
                self.vddio1boden()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "EM4 unlatch."]
        #[must_use]
        #[inline(always)]
        pub const fn em4unlatch(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 unlatch."]
        #[inline(always)]
        pub const fn set_em4unlatch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Temperature Average Request."]
        #[must_use]
        #[inline(always)]
        pub const fn tempavgreq(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Average Request."]
        #[inline(always)]
        pub const fn set_tempavgreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Scale voltage to Vscale1."]
        #[must_use]
        #[inline(always)]
        pub const fn em01vscale1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Scale voltage to Vscale1."]
        #[inline(always)]
        pub const fn set_em01vscale1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Scale voltage to Vscale2."]
        #[must_use]
        #[inline(always)]
        pub const fn em01vscale2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Scale voltage to Vscale2."]
        #[inline(always)]
        pub const fn set_em01vscale2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Reset Cause Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn rstcauseclr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Cause Clear."]
        #[inline(always)]
        pub const fn set_rstcauseclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("em4unlatch", &self.em4unlatch())
                .field("tempavgreq", &self.tempavgreq())
                .field("em01vscale1", &self.em01vscale1())
                .field("em01vscale2", &self.em01vscale2())
                .field("rstcauseclr", &self.rstcauseclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ em4unlatch: {=bool:?}, tempavgreq: {=bool:?}, em01vscale1: {=bool:?}, em01vscale2: {=bool:?}, rstcauseclr: {=bool:?} }}",
                self.em4unlatch(),
                self.tempavgreq(),
                self.em01vscale1(),
                self.em01vscale2(),
                self.rstcauseclr()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Enable debugging in EM2."]
        #[must_use]
        #[inline(always)]
        pub const fn em2dbgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable debugging in EM2."]
        #[inline(always)]
        pub const fn set_em2dbgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Averaged Temperature samples num."]
        #[must_use]
        #[inline(always)]
        pub const fn tempavgnum(&self) -> super::vals::Tempavgnum {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Tempavgnum::from_bits(val as u8)
        }
        #[doc = "Averaged Temperature samples num."]
        #[inline(always)]
        pub const fn set_tempavgnum(&mut self, val: super::vals::Tempavgnum) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "EM2/EM3 Vscale."]
        #[must_use]
        #[inline(always)]
        pub const fn em23vscale(&self) -> super::vals::Em23vscale {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Em23vscale::from_bits(val as u8)
        }
        #[doc = "EM2/EM3 Vscale."]
        #[inline(always)]
        pub const fn set_em23vscale(&mut self, val: super::vals::Em23vscale) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Enable flash on demand wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn flashpwrupondemand(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable flash on demand wakeup."]
        #[inline(always)]
        pub const fn set_flashpwrupondemand(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "EFP Direct Mode Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn efpdirectmodeen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "EFP Direct Mode Enable."]
        #[inline(always)]
        pub const fn set_efpdirectmodeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EFP drives DECOUPLE."]
        #[must_use]
        #[inline(always)]
        pub const fn efpdrvdecouple(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EFP drives DECOUPLE."]
        #[inline(always)]
        pub const fn set_efpdrvdecouple(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EFP drives DVDD."]
        #[must_use]
        #[inline(always)]
        pub const fn efpdrvdvdd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EFP drives DVDD."]
        #[inline(always)]
        pub const fn set_efpdrvdvdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("em2dbgen", &self.em2dbgen())
                .field("tempavgnum", &self.tempavgnum())
                .field("em23vscale", &self.em23vscale())
                .field("flashpwrupondemand", &self.flashpwrupondemand())
                .field("efpdirectmodeen", &self.efpdirectmodeen())
                .field("efpdrvdecouple", &self.efpdrvdecouple())
                .field("efpdrvdvdd", &self.efpdrvdvdd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ em2dbgen: {=bool:?}, tempavgnum: {:?}, em23vscale: {:?}, flashpwrupondemand: {=bool:?}, efpdirectmodeen: {=bool:?}, efpdrvdecouple: {=bool:?}, efpdrvdvdd: {=bool:?} }}",
                self.em2dbgen(),
                self.tempavgnum(),
                self.em23vscale(),
                self.flashpwrupondemand(),
                self.efpdirectmodeen(),
                self.efpdrvdecouple(),
                self.efpdrvdvdd()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Decbod(pub u32);
    impl Decbod {
        #[doc = "DECBOD enable."]
        #[must_use]
        #[inline(always)]
        pub const fn decboden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DECBOD enable."]
        #[inline(always)]
        pub const fn set_decboden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DECBOD Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn decbodmask(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DECBOD Mask."]
        #[inline(always)]
        pub const fn set_decbodmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Over Voltage Monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn decovmboden(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Over Voltage Monitor enable."]
        #[inline(always)]
        pub const fn set_decovmboden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Over Voltage Monitor Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn decovmbodmask(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Over Voltage Monitor Mask."]
        #[inline(always)]
        pub const fn set_decovmbodmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Decbod {
        #[inline(always)]
        fn default() -> Decbod {
            Decbod(0)
        }
    }
    impl core::fmt::Debug for Decbod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Decbod")
                .field("decboden", &self.decboden())
                .field("decbodmask", &self.decbodmask())
                .field("decovmboden", &self.decovmboden())
                .field("decovmbodmask", &self.decovmbodmask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Decbod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Decbod {{ decboden: {=bool:?}, decbodmask: {=bool:?}, decovmboden: {=bool:?}, decovmbodmask: {=bool:?} }}",
                self.decboden(),
                self.decbodmask(),
                self.decovmboden(),
                self.decovmbodmask()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dgien(pub u32);
    impl Dgien {
        #[doc = "EM23 Wake up Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn em23wakeupdgien(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EM23 Wake up Interrupt enable."]
        #[inline(always)]
        pub const fn set_em23wakeupdgien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Temperature Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tempdgien(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Interrupt enable."]
        #[inline(always)]
        pub const fn set_tempdgien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Temperature low Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn templowdgien(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature low Interrupt enable."]
        #[inline(always)]
        pub const fn set_templowdgien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Temperature high Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn temphighdgien(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature high Interrupt enable."]
        #[inline(always)]
        pub const fn set_temphighdgien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dgien {
        #[inline(always)]
        fn default() -> Dgien {
            Dgien(0)
        }
    }
    impl core::fmt::Debug for Dgien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dgien")
                .field("em23wakeupdgien", &self.em23wakeupdgien())
                .field("tempdgien", &self.tempdgien())
                .field("templowdgien", &self.templowdgien())
                .field("temphighdgien", &self.temphighdgien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dgien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dgien {{ em23wakeupdgien: {=bool:?}, tempdgien: {=bool:?}, templowdgien: {=bool:?}, temphighdgien: {=bool:?} }}",
                self.em23wakeupdgien(),
                self.tempdgien(),
                self.templowdgien(),
                self.temphighdgien()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dgif(pub u32);
    impl Dgif {
        #[doc = "EM23 Wake up Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn em23wakeupdgif(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EM23 Wake up Interrupt flag."]
        #[inline(always)]
        pub const fn set_em23wakeupdgif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Temperature Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tempdgif(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Interrupt flag."]
        #[inline(always)]
        pub const fn set_tempdgif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Temperature low Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn templowdgif(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature low Interrupt flag."]
        #[inline(always)]
        pub const fn set_templowdgif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Temperature high Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn temphighdgif(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature high Interrupt flag."]
        #[inline(always)]
        pub const fn set_temphighdgif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dgif {
        #[inline(always)]
        fn default() -> Dgif {
            Dgif(0)
        }
    }
    impl core::fmt::Debug for Dgif {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dgif")
                .field("em23wakeupdgif", &self.em23wakeupdgif())
                .field("tempdgif", &self.tempdgif())
                .field("templowdgif", &self.templowdgif())
                .field("temphighdgif", &self.temphighdgif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dgif {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dgif {{ em23wakeupdgif: {=bool:?}, tempdgif: {=bool:?}, templowdgif: {=bool:?}, temphighdgif: {=bool:?} }}",
                self.em23wakeupdgif(),
                self.tempdgif(),
                self.templowdgif(),
                self.temphighdgif()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Efpien(pub u32);
    impl Efpien {
        #[doc = "EFP Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn efpien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EFP Interrupt enable."]
        #[inline(always)]
        pub const fn set_efpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Efpien {
        #[inline(always)]
        fn default() -> Efpien {
            Efpien(0)
        }
    }
    impl core::fmt::Debug for Efpien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Efpien").field("efpien", &self.efpien()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Efpien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Efpien {{ efpien: {=bool:?} }}", self.efpien())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Efpif(pub u32);
    impl Efpif {
        #[doc = "EFP Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn efpif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EFP Interrupt Flag."]
        #[inline(always)]
        pub const fn set_efpif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Efpif {
        #[inline(always)]
        fn default() -> Efpif {
            Efpif(0)
        }
    }
    impl core::fmt::Debug for Efpif {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Efpif").field("efpif", &self.efpif()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Efpif {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Efpif {{ efpif: {=bool:?} }}", self.efpif())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em4ctrl(pub u32);
    impl Em4ctrl {
        #[doc = "EM4 entry request."]
        #[must_use]
        #[inline(always)]
        pub const fn em4entry(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "EM4 entry request."]
        #[inline(always)]
        pub const fn set_em4entry(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "EM4 IO retention mode."]
        #[must_use]
        #[inline(always)]
        pub const fn em4ioretmode(&self) -> super::vals::Em4ioretmode {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Em4ioretmode::from_bits(val as u8)
        }
        #[doc = "EM4 IO retention mode."]
        #[inline(always)]
        pub const fn set_em4ioretmode(&mut self, val: super::vals::Em4ioretmode) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Set BOD3SENSE as EM4 wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn bod3senseem4wu(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Set BOD3SENSE as EM4 wakeup."]
        #[inline(always)]
        pub const fn set_bod3senseem4wu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Em4ctrl {
        #[inline(always)]
        fn default() -> Em4ctrl {
            Em4ctrl(0)
        }
    }
    impl core::fmt::Debug for Em4ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em4ctrl")
                .field("em4entry", &self.em4entry())
                .field("em4ioretmode", &self.em4ioretmode())
                .field("bod3senseem4wu", &self.bod3senseem4wu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em4ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Em4ctrl {{ em4entry: {=u8:?}, em4ioretmode: {:?}, bod3senseem4wu: {=bool:?} }}",
                self.em4entry(),
                self.em4ioretmode(),
                self.bod3senseem4wu()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "AVDD BOD Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn avddbod(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AVDD BOD Interrupt enable."]
        #[inline(always)]
        pub const fn set_avddbod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VDDIO0 BOD Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iovdd0bod(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO0 BOD Interrupt enable."]
        #[inline(always)]
        pub const fn set_iovdd0bod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EM23 Wake up Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn em23wakeup(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EM23 Wake up Interrupt enable."]
        #[inline(always)]
        pub const fn set_em23wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Vscale done Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vscaledone(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Vscale done Interrupt enable."]
        #[inline(always)]
        pub const fn set_vscaledone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Temperature Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tempavg(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Interrupt enable."]
        #[inline(always)]
        pub const fn set_tempavg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Temperature Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn temp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Interrupt enable."]
        #[inline(always)]
        pub const fn set_temp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Temperature low Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn templow(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature low Interrupt enable."]
        #[inline(always)]
        pub const fn set_templow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Temperature high Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn temphigh(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature high Interrupt enable."]
        #[inline(always)]
        pub const fn set_temphigh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("avddbod", &self.avddbod())
                .field("iovdd0bod", &self.iovdd0bod())
                .field("em23wakeup", &self.em23wakeup())
                .field("vscaledone", &self.vscaledone())
                .field("tempavg", &self.tempavg())
                .field("temp", &self.temp())
                .field("templow", &self.templow())
                .field("temphigh", &self.temphigh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ avddbod: {=bool:?}, iovdd0bod: {=bool:?}, em23wakeup: {=bool:?}, vscaledone: {=bool:?}, tempavg: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}",
                self.avddbod(),
                self.iovdd0bod(),
                self.em23wakeup(),
                self.vscaledone(),
                self.tempavg(),
                self.temp(),
                self.templow(),
                self.temphigh()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "AVDD BOD Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn avddbod(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AVDD BOD Interrupt flag."]
        #[inline(always)]
        pub const fn set_avddbod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VDDIO0 BOD Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iovdd0bod(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO0 BOD Interrupt flag."]
        #[inline(always)]
        pub const fn set_iovdd0bod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EM23 Wake up Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn em23wakeup(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EM23 Wake up Interrupt flag."]
        #[inline(always)]
        pub const fn set_em23wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Vscale done Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn vscaledone(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Vscale done Interrupt flag."]
        #[inline(always)]
        pub const fn set_vscaledone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Temperature Average Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tempavg(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Average Interrupt flag."]
        #[inline(always)]
        pub const fn set_tempavg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Temperature Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn temp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Interrupt flag."]
        #[inline(always)]
        pub const fn set_temp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Temperature low Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn templow(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature low Interrupt flag."]
        #[inline(always)]
        pub const fn set_templow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Temperature high Interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn temphigh(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature high Interrupt flag."]
        #[inline(always)]
        pub const fn set_temphigh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("avddbod", &self.avddbod())
                .field("iovdd0bod", &self.iovdd0bod())
                .field("em23wakeup", &self.em23wakeup())
                .field("vscaledone", &self.vscaledone())
                .field("tempavg", &self.tempavg())
                .field("temp", &self.temp())
                .field("templow", &self.templow())
                .field("temphigh", &self.temphigh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ avddbod: {=bool:?}, iovdd0bod: {=bool:?}, em23wakeup: {=bool:?}, vscaledone: {=bool:?}, tempavg: {=bool:?}, temp: {=bool:?}, templow: {=bool:?}, temphigh: {=bool:?} }}",
                self.avddbod(),
                self.iovdd0bod(),
                self.em23wakeup(),
                self.vscaledone(),
                self.tempavg(),
                self.temp(),
                self.templow(),
                self.temphigh()
            )
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
    pub struct Pd1paretctrl(pub u32);
    impl Pd1paretctrl {
        #[doc = "Disable PD1 Partial Retention."]
        #[must_use]
        #[inline(always)]
        pub const fn pd1paretdis(&self) -> super::vals::Pd1paretdis {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Pd1paretdis::from_bits(val as u16)
        }
        #[doc = "Disable PD1 Partial Retention."]
        #[inline(always)]
        pub const fn set_pd1paretdis(&mut self, val: super::vals::Pd1paretdis) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pd1paretctrl {
        #[inline(always)]
        fn default() -> Pd1paretctrl {
            Pd1paretctrl(0)
        }
    }
    impl core::fmt::Debug for Pd1paretctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pd1paretctrl")
                .field("pd1paretdis", &self.pd1paretdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pd1paretctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pd1paretctrl {{ pd1paretdis: {:?} }}", self.pd1paretdis())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstcause(pub u32);
    impl Rstcause {
        #[doc = "Power On Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn por(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power On Reset."]
        #[inline(always)]
        pub const fn set_por(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Pin Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Pin Reset."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EM4 Wakeup Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn em4(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wakeup Reset."]
        #[inline(always)]
        pub const fn set_em4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Watchdog 0 Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog 0 Reset."]
        #[inline(always)]
        pub const fn set_wdog0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "M33 Core Lockup Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lockup(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "M33 Core Lockup Reset."]
        #[inline(always)]
        pub const fn set_lockup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "M33 Core Sys Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sysreq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "M33 Core Sys Reset."]
        #[inline(always)]
        pub const fn set_sysreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "HVBOD Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dvddbod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "HVBOD Reset."]
        #[inline(always)]
        pub const fn set_dvddbod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LEBOD Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dvddlebod(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LEBOD Reset."]
        #[inline(always)]
        pub const fn set_dvddlebod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LVBOD Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn decbod(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LVBOD Reset."]
        #[inline(always)]
        pub const fn set_decbod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LEBOD1 Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn avddbod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LEBOD1 Reset."]
        #[inline(always)]
        pub const fn set_avddbod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LEBOD2 Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn iovdd0bod(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LEBOD2 Reset."]
        #[inline(always)]
        pub const fn set_iovdd0bod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DCI reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dci(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCI reset."]
        #[inline(always)]
        pub const fn set_dci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DCDC VREGIN comparator."]
        #[must_use]
        #[inline(always)]
        pub const fn vregin(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC VREGIN comparator."]
        #[inline(always)]
        pub const fn set_vregin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rstcause {
        #[inline(always)]
        fn default() -> Rstcause {
            Rstcause(0)
        }
    }
    impl core::fmt::Debug for Rstcause {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rstcause")
                .field("por", &self.por())
                .field("pin", &self.pin())
                .field("em4", &self.em4())
                .field("wdog0", &self.wdog0())
                .field("lockup", &self.lockup())
                .field("sysreq", &self.sysreq())
                .field("dvddbod", &self.dvddbod())
                .field("dvddlebod", &self.dvddlebod())
                .field("decbod", &self.decbod())
                .field("avddbod", &self.avddbod())
                .field("iovdd0bod", &self.iovdd0bod())
                .field("dci", &self.dci())
                .field("vregin", &self.vregin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rstcause {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rstcause {{ por: {=bool:?}, pin: {=bool:?}, em4: {=bool:?}, wdog0: {=bool:?}, lockup: {=bool:?}, sysreq: {=bool:?}, dvddbod: {=bool:?}, dvddlebod: {=bool:?}, decbod: {=bool:?}, avddbod: {=bool:?}, iovdd0bod: {=bool:?}, dci: {=bool:?}, vregin: {=bool:?} }}",
                self.por(),
                self.pin(),
                self.em4(),
                self.wdog0(),
                self.lockup(),
                self.sysreq(),
                self.dvddbod(),
                self.dvddlebod(),
                self.decbod(),
                self.avddbod(),
                self.iovdd0bod(),
                self.dci(),
                self.vregin()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rstctrl(pub u32);
    impl Rstctrl {
        #[doc = "Enable WDOG0 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog0rmode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable WDOG0 reset."]
        #[inline(always)]
        pub const fn set_wdog0rmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable M33 System reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sysrmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable M33 System reset."]
        #[inline(always)]
        pub const fn set_sysrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable M33 Lockup reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lockuprmode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable M33 Lockup reset."]
        #[inline(always)]
        pub const fn set_lockuprmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable AVDD BOD reset."]
        #[must_use]
        #[inline(always)]
        pub const fn avddbodrmode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable AVDD BOD reset."]
        #[inline(always)]
        pub const fn set_avddbodrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable VDDIO0 BOD reset."]
        #[must_use]
        #[inline(always)]
        pub const fn iovdd0bodrmode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable VDDIO0 BOD reset."]
        #[inline(always)]
        pub const fn set_iovdd0bodrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable DECBOD reset."]
        #[must_use]
        #[inline(always)]
        pub const fn decbodrmode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DECBOD reset."]
        #[inline(always)]
        pub const fn set_decbodrmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCI System reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dcirmode(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCI System reset."]
        #[inline(always)]
        pub const fn set_dcirmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rstctrl {
        #[inline(always)]
        fn default() -> Rstctrl {
            Rstctrl(0)
        }
    }
    impl core::fmt::Debug for Rstctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rstctrl")
                .field("wdog0rmode", &self.wdog0rmode())
                .field("sysrmode", &self.sysrmode())
                .field("lockuprmode", &self.lockuprmode())
                .field("avddbodrmode", &self.avddbodrmode())
                .field("iovdd0bodrmode", &self.iovdd0bodrmode())
                .field("decbodrmode", &self.decbodrmode())
                .field("dcirmode", &self.dcirmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rstctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rstctrl {{ wdog0rmode: {=bool:?}, sysrmode: {=bool:?}, lockuprmode: {=bool:?}, avddbodrmode: {=bool:?}, iovdd0bodrmode: {=bool:?}, decbodrmode: {=bool:?}, dcirmode: {=bool:?} }}",
                self.wdog0rmode(),
                self.sysrmode(),
                self.lockuprmode(),
                self.avddbodrmode(),
                self.iovdd0bodrmode(),
                self.decbodrmode(),
                self.dcirmode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Lock status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Lock status."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "First Temp done."]
        #[must_use]
        #[inline(always)]
        pub const fn firsttempdone(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "First Temp done."]
        #[inline(always)]
        pub const fn set_firsttempdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Temp active."]
        #[must_use]
        #[inline(always)]
        pub const fn tempactive(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Temp active."]
        #[inline(always)]
        pub const fn set_tempactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Temp Average active."]
        #[must_use]
        #[inline(always)]
        pub const fn tempavgactive(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Temp Average active."]
        #[inline(always)]
        pub const fn set_tempavgactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Vscale busy."]
        #[must_use]
        #[inline(always)]
        pub const fn vscalebusy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Vscale busy."]
        #[inline(always)]
        pub const fn set_vscalebusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Vscale failed."]
        #[must_use]
        #[inline(always)]
        pub const fn vscalefailed(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Vscale failed."]
        #[inline(always)]
        pub const fn set_vscalefailed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Vscale status."]
        #[must_use]
        #[inline(always)]
        pub const fn vscale(&self) -> super::vals::Vscale {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Vscale::from_bits(val as u8)
        }
        #[doc = "Vscale status."]
        #[inline(always)]
        pub const fn set_vscale(&mut self, val: super::vals::Vscale) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "RAC active."]
        #[must_use]
        #[inline(always)]
        pub const fn racactive(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RAC active."]
        #[inline(always)]
        pub const fn set_racactive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EM4 IO retention status."]
        #[must_use]
        #[inline(always)]
        pub const fn em4ioret(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 IO retention status."]
        #[inline(always)]
        pub const fn set_em4ioret(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "EM2 entered."]
        #[must_use]
        #[inline(always)]
        pub const fn em2entered(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "EM2 entered."]
        #[inline(always)]
        pub const fn set_em2entered(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
                .field("lock", &self.lock())
                .field("firsttempdone", &self.firsttempdone())
                .field("tempactive", &self.tempactive())
                .field("tempavgactive", &self.tempavgactive())
                .field("vscalebusy", &self.vscalebusy())
                .field("vscalefailed", &self.vscalefailed())
                .field("vscale", &self.vscale())
                .field("racactive", &self.racactive())
                .field("em4ioret", &self.em4ioret())
                .field("em2entered", &self.em2entered())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ lock: {=bool:?}, firsttempdone: {=bool:?}, tempactive: {=bool:?}, tempavgactive: {=bool:?}, vscalebusy: {=bool:?}, vscalefailed: {=bool:?}, vscale: {:?}, racactive: {=bool:?}, em4ioret: {=bool:?}, em2entered: {=bool:?} }}",
                self.lock(),
                self.firsttempdone(),
                self.tempactive(),
                self.tempavgactive(),
                self.vscalebusy(),
                self.vscalefailed(),
                self.vscale(),
                self.racactive(),
                self.em4ioret(),
                self.em2entered()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Temp(pub u32);
    impl Temp {
        #[doc = "Temperature measured decimal part."]
        #[must_use]
        #[inline(always)]
        pub const fn templsb(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Temperature measured decimal part."]
        #[inline(always)]
        pub const fn set_templsb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Temperature measured."]
        #[must_use]
        #[inline(always)]
        pub const fn temp(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x01ff;
            val as u16
        }
        #[doc = "Temperature measured."]
        #[inline(always)]
        pub const fn set_temp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
        }
        #[doc = "Averaged Temperature."]
        #[must_use]
        #[inline(always)]
        pub const fn tempavg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Averaged Temperature."]
        #[inline(always)]
        pub const fn set_tempavg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Temp {
        #[inline(always)]
        fn default() -> Temp {
            Temp(0)
        }
    }
    impl core::fmt::Debug for Temp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Temp")
                .field("templsb", &self.templsb())
                .field("temp", &self.temp())
                .field("tempavg", &self.tempavg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Temp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Temp {{ templsb: {=u8:?}, temp: {=u16:?}, tempavg: {=u16:?} }}",
                self.templsb(),
                self.temp(),
                self.tempavg()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Templimits(pub u32);
    impl Templimits {
        #[doc = "Temp Low limit."]
        #[must_use]
        #[inline(always)]
        pub const fn templow(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Temp Low limit."]
        #[inline(always)]
        pub const fn set_templow(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Temp High limit."]
        #[must_use]
        #[inline(always)]
        pub const fn temphigh(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Temp High limit."]
        #[inline(always)]
        pub const fn set_temphigh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Templimits {
        #[inline(always)]
        fn default() -> Templimits {
            Templimits(0)
        }
    }
    impl core::fmt::Debug for Templimits {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Templimits")
                .field("templow", &self.templow())
                .field("temphigh", &self.temphigh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Templimits {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Templimits {{ templow: {=u16:?}, temphigh: {=u16:?} }}",
                self.templow(),
                self.temphigh()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vregvddcmpctrl(pub u32);
    impl Vregvddcmpctrl {
        #[doc = "VREGVDD comparator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vregincmpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VREGVDD comparator enable."]
        #[inline(always)]
        pub const fn set_vregincmpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VREGVDD comparator threshold programming."]
        #[must_use]
        #[inline(always)]
        pub const fn thressel(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "VREGVDD comparator threshold programming."]
        #[inline(always)]
        pub const fn set_thressel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
    }
    impl Default for Vregvddcmpctrl {
        #[inline(always)]
        fn default() -> Vregvddcmpctrl {
            Vregvddcmpctrl(0)
        }
    }
    impl core::fmt::Debug for Vregvddcmpctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vregvddcmpctrl")
                .field("vregincmpen", &self.vregincmpen())
                .field("thressel", &self.thressel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vregvddcmpctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vregvddcmpctrl {{ vregincmpen: {=bool:?}, thressel: {=u8:?} }}",
                self.vregincmpen(),
                self.thressel()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em23vscale {
        #[doc = "VSCALE0. 0.9v."]
        Vscale0 = 0x0,
        #[doc = "VSCALE1. 1.0v."]
        Vscale1 = 0x01,
        #[doc = "VSCALE2. 1.1v."]
        Vscale2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Em23vscale {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em23vscale {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em23vscale {
        #[inline(always)]
        fn from(val: u8) -> Em23vscale {
            Em23vscale::from_bits(val)
        }
    }
    impl From<Em23vscale> for u8 {
        #[inline(always)]
        fn from(val: Em23vscale) -> u8 {
            Em23vscale::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em4ioretmode {
        #[doc = "No Retention: Pads enter reset state when entering EM4."]
        Disable = 0x0,
        #[doc = "Retention through EM4: Pads enter reset state when exiting EM4."]
        Em4exit = 0x01,
        #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention."]
        Swunlatch = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Em4ioretmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em4ioretmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em4ioretmode {
        #[inline(always)]
        fn from(val: u8) -> Em4ioretmode {
            Em4ioretmode::from_bits(val)
        }
    }
    impl From<Em4ioretmode> for u8 {
        #[inline(always)]
        fn from(val: Em4ioretmode) -> u8 {
            Em4ioretmode::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Unlock EMU register."]
        pub const Unlock: Self = Self(0xade8);
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
                0xade8 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xade8 => defmt::write!(f, "Unlock"),
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pd1paretdis(u16);
    impl Pd1paretdis {
        #[doc = "Retain associated registers when in EM2/3."]
        pub const Retain: Self = Self(0x0);
        #[doc = "Do not retain associcated registers when in EM2/3."]
        pub const Noretain: Self = Self(0x01);
    }
    impl Pd1paretdis {
        pub const fn from_bits(val: u16) -> Pd1paretdis {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pd1paretdis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Retain"),
                0x01 => f.write_str("Noretain"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pd1paretdis {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Retain"),
                0x01 => defmt::write!(f, "Noretain"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pd1paretdis {
        #[inline(always)]
        fn from(val: u16) -> Pd1paretdis {
            Pd1paretdis::from_bits(val)
        }
    }
    impl From<Pd1paretdis> for u16 {
        #[inline(always)]
        fn from(val: Pd1paretdis) -> u16 {
            Pd1paretdis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tempavgnum {
        #[doc = "16 measurements."]
        N16 = 0x0,
        #[doc = "64 measurements."]
        N64 = 0x01,
    }
    impl Tempavgnum {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tempavgnum {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tempavgnum {
        #[inline(always)]
        fn from(val: u8) -> Tempavgnum {
            Tempavgnum::from_bits(val)
        }
    }
    impl From<Tempavgnum> for u8 {
        #[inline(always)]
        fn from(val: Tempavgnum) -> u8 {
            Tempavgnum::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vscale {
        #[doc = "Voltage scaling set to 0.9v."]
        Vscale0 = 0x0,
        #[doc = "Voltage scaling set to 1.0v."]
        Vscale1 = 0x01,
        #[doc = "Voltage scaling set to 1.1v."]
        Vscale2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Vscale {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vscale {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vscale {
        #[inline(always)]
        fn from(val: u8) -> Vscale {
            Vscale::from_bits(val)
        }
    }
    impl From<Vscale> for u8 {
        #[inline(always)]
        fn from(val: Vscale) -> u8 {
            Vscale::to_bits(val)
        }
    }
}
