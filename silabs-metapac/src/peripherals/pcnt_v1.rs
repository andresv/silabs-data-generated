#[doc = "PCNT peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnt {
    ptr: *mut u8,
}
unsafe impl Send for Pcnt {}
unsafe impl Sync for Pcnt {}
impl Pcnt {
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
    pub const fn swrst(self) -> crate::common::Reg<regs::Swrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn auxcnt(self) -> crate::common::Reg<regs::Auxcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn top(self) -> crate::common::Reg<regs::Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn topb(self) -> crate::common::Reg<regs::Topb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ovsctrl(self) -> crate::common::Reg<regs::Ovsctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn swrst_set(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn top_set(self) -> crate::common::Reg<regs::Top, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn topb_set(self) -> crate::common::Reg<regs::Topb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ovsctrl_set(self) -> crate::common::Reg<regs::Ovsctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn swrst_clr(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn top_clr(self) -> crate::common::Reg<regs::Top, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn topb_clr(self) -> crate::common::Reg<regs::Topb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ovsctrl_clr(self) -> crate::common::Reg<regs::Ovsctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn swrst_tgl(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn top_tgl(self) -> crate::common::Reg<regs::Top, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn topb_tgl(self) -> crate::common::Reg<regs::Topb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ovsctrl_tgl(self) -> crate::common::Reg<regs::Ovsctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Auxcnt(pub u32);
    impl Auxcnt {
        #[doc = "Auxiliary Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn auxcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auxiliary Counter Value."]
        #[inline(always)]
        pub const fn set_auxcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Auxcnt {
        #[inline(always)]
        fn default() -> Auxcnt {
            Auxcnt(0)
        }
    }
    impl core::fmt::Debug for Auxcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Auxcnt").field("auxcnt", &self.auxcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Auxcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Auxcnt {{ auxcnt: {=u16:?} }}", self.auxcnt())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Mode Select."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Debug Mode Halt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn debughalt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Mode Halt Enable."]
        #[inline(always)]
        pub const fn set_debughalt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Digital Pulse Width Filter."]
        #[must_use]
        #[inline(always)]
        pub const fn filten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Digital Pulse Width Filter."]
        #[inline(always)]
        pub const fn set_filten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Hysteresis."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Hysteresis."]
        #[inline(always)]
        pub const fn set_hyst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "S0IN PRS Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn s0prsen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "S0IN PRS Enable."]
        #[inline(always)]
        pub const fn set_s0prsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "S1IN PRS Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn s1prsen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "S1IN PRS Enable."]
        #[inline(always)]
        pub const fn set_s1prsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("mode", &self.mode())
                .field("debughalt", &self.debughalt())
                .field("filten", &self.filten())
                .field("hyst", &self.hyst())
                .field("s0prsen", &self.s0prsen())
                .field("s1prsen", &self.s1prsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ mode: {:?}, debughalt: {=bool:?}, filten: {=bool:?}, hyst: {=bool:?}, s0prsen: {=bool:?}, s1prsen: {=bool:?} }}",
                self.mode(),
                self.debughalt(),
                self.filten(),
                self.hyst(),
                self.s0prsen(),
                self.s1prsen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "PCNT Clock Domain Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn corerst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PCNT Clock Domain Reset."]
        #[inline(always)]
        pub const fn set_corerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CNT Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CNT Reset."]
        #[inline(always)]
        pub const fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AUXCNT Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn auxcntrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AUXCNT Reset."]
        #[inline(always)]
        pub const fn set_auxcntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Load CNT Immediately."]
        #[must_use]
        #[inline(always)]
        pub const fn lcntim(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Load CNT Immediately."]
        #[inline(always)]
        pub const fn set_lcntim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Start Main Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn startcnt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Start Main Counter."]
        #[inline(always)]
        pub const fn set_startcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Start Aux Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn startauxcnt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Start Aux Counter."]
        #[inline(always)]
        pub const fn set_startauxcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Stop Main Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn stopcnt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Main Counter."]
        #[inline(always)]
        pub const fn set_stopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Stop Aux Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn stopauxcnt(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Stop Aux Counter."]
        #[inline(always)]
        pub const fn set_stopauxcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("corerst", &self.corerst())
                .field("cntrst", &self.cntrst())
                .field("auxcntrst", &self.auxcntrst())
                .field("lcntim", &self.lcntim())
                .field("startcnt", &self.startcnt())
                .field("startauxcnt", &self.startauxcnt())
                .field("stopcnt", &self.stopcnt())
                .field("stopauxcnt", &self.stopauxcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ corerst: {=bool:?}, cntrst: {=bool:?}, auxcntrst: {=bool:?}, lcntim: {=bool:?}, startcnt: {=bool:?}, startauxcnt: {=bool:?}, stopcnt: {=bool:?}, stopauxcnt: {=bool:?} }}",
                self.corerst(),
                self.cntrst(),
                self.auxcntrst(),
                self.lcntim(),
                self.startcnt(),
                self.startauxcnt(),
                self.stopcnt(),
                self.stopauxcnt()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter Value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Count Direction Determined By S1."]
        #[must_use]
        #[inline(always)]
        pub const fn s1cdir(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Count Direction Determined By S1."]
        #[inline(always)]
        pub const fn set_s1cdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-Quadrature Mode Counter Direction Co."]
        #[must_use]
        #[inline(always)]
        pub const fn cntdir(&self) -> super::vals::Cntdir {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cntdir::from_bits(val as u8)
        }
        #[doc = "Non-Quadrature Mode Counter Direction Co."]
        #[inline(always)]
        pub const fn set_cntdir(&mut self, val: super::vals::Cntdir) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Edge Select."]
        #[must_use]
        #[inline(always)]
        pub const fn edge(&self) -> super::vals::Edge {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Edge::from_bits(val as u8)
        }
        #[doc = "Edge Select."]
        #[inline(always)]
        pub const fn set_edge(&mut self, val: super::vals::Edge) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Controls When the Counter Counts."]
        #[must_use]
        #[inline(always)]
        pub const fn cntev(&self) -> super::vals::Cntev {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Cntev::from_bits(val as u8)
        }
        #[doc = "Controls When the Counter Counts."]
        #[inline(always)]
        pub const fn set_cntev(&mut self, val: super::vals::Cntev) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Controls When the Aux Counter Counts."]
        #[must_use]
        #[inline(always)]
        pub const fn auxcntev(&self) -> super::vals::Auxcntev {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Auxcntev::from_bits(val as u8)
        }
        #[doc = "Controls When the Aux Counter Counts."]
        #[inline(always)]
        pub const fn set_auxcntev(&mut self, val: super::vals::Auxcntev) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
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
                .field("s1cdir", &self.s1cdir())
                .field("cntdir", &self.cntdir())
                .field("edge", &self.edge())
                .field("cntev", &self.cntev())
                .field("auxcntev", &self.auxcntev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ s1cdir: {=bool:?}, cntdir: {:?}, edge: {:?}, cntev: {:?}, auxcntev: {:?} }}",
                self.s1cdir(),
                self.cntdir(),
                self.edge(),
                self.cntev(),
                self.auxcntev()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "PCNT Module Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PCNT Module Enable."]
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
        #[doc = "Underflow Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overflow Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Direction Change Detect Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dircng(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Direction Change Detect Interrupt Flag."]
        #[inline(always)]
        pub const fn set_dircng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Auxiliary Overflow Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn auxof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Overflow Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_auxof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Oversampling Quad State Err Int Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn oqsterr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Oversampling Quad State Err Int Flag."]
        #[inline(always)]
        pub const fn set_oqsterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("uf", &self.uf())
                .field("of", &self.of())
                .field("dircng", &self.dircng())
                .field("auxof", &self.auxof())
                .field("oqsterr", &self.oqsterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ uf: {=bool:?}, of: {=bool:?}, dircng: {=bool:?}, auxof: {=bool:?}, oqsterr: {=bool:?} }}",
                self.uf(),
                self.of(),
                self.dircng(),
                self.auxof(),
                self.oqsterr()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Underflow Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overflow Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Direction Change Detect Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dircng(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Direction Change Detect Interrupt Flag."]
        #[inline(always)]
        pub const fn set_dircng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Auxiliary Overflow Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn auxof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auxiliary Overflow Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_auxof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Oversampling Quad State Err Int Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn oqsterr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Oversampling Quad State Err Int Flag."]
        #[inline(always)]
        pub const fn set_oqsterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("uf", &self.uf())
                .field("of", &self.of())
                .field("dircng", &self.dircng())
                .field("auxof", &self.auxof())
                .field("oqsterr", &self.oqsterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ uf: {=bool:?}, of: {=bool:?}, dircng: {=bool:?}, auxof: {=bool:?}, oqsterr: {=bool:?} }}",
                self.uf(),
                self.of(),
                self.dircng(),
                self.auxof(),
                self.oqsterr()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP VERSION."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP VERSION."]
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
        pub const fn pcntlockkey(&self) -> super::vals::Pcntlockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Pcntlockkey::from_bits(val as u16)
        }
        #[doc = "Configuration Lock Key."]
        #[inline(always)]
        pub const fn set_pcntlockkey(&mut self, val: super::vals::Pcntlockkey) {
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
            f.debug_struct("Lock")
                .field("pcntlockkey", &self.pcntlockkey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lock {{ pcntlockkey: {:?} }}", self.pcntlockkey())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ovsctrl(pub u32);
    impl Ovsctrl {
        #[doc = "Configure Filter Length for Inputs S0IN."]
        #[must_use]
        #[inline(always)]
        pub const fn filtlen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Configure Filter Length for Inputs S0IN."]
        #[inline(always)]
        pub const fn set_filtlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Flutter Remove."]
        #[must_use]
        #[inline(always)]
        pub const fn flutterrm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Flutter Remove."]
        #[inline(always)]
        pub const fn set_flutterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ovsctrl {
        #[inline(always)]
        fn default() -> Ovsctrl {
            Ovsctrl(0)
        }
    }
    impl core::fmt::Debug for Ovsctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ovsctrl")
                .field("filtlen", &self.filtlen())
                .field("flutterrm", &self.flutterrm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ovsctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ovsctrl {{ filtlen: {=u8:?}, flutterrm: {=bool:?} }}",
                self.filtlen(),
                self.flutterrm()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Current Counter Direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Current Counter Direction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "TOP Buffer Valid."]
        #[must_use]
        #[inline(always)]
        pub const fn topbv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TOP Buffer Valid."]
        #[inline(always)]
        pub const fn set_topbv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn pcntlockstatus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Lock Status."]
        #[inline(always)]
        pub const fn set_pcntlockstatus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Main Counter running status."]
        #[must_use]
        #[inline(always)]
        pub const fn cntrunning(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Main Counter running status."]
        #[inline(always)]
        pub const fn set_cntrunning(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Aux Counter running status."]
        #[must_use]
        #[inline(always)]
        pub const fn auxcntrunning(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Aux Counter running status."]
        #[inline(always)]
        pub const fn set_auxcntrunning(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("dir", &self.dir())
                .field("topbv", &self.topbv())
                .field("pcntlockstatus", &self.pcntlockstatus())
                .field("cntrunning", &self.cntrunning())
                .field("auxcntrunning", &self.auxcntrunning())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ dir: {:?}, topbv: {=bool:?}, pcntlockstatus: {=bool:?}, cntrunning: {=bool:?}, auxcntrunning: {=bool:?} }}",
                self.dir(),
                self.topbv(),
                self.pcntlockstatus(),
                self.cntrunning(),
                self.auxcntrunning()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swrst(pub u32);
    impl Swrst {
        #[doc = "Software reset command."]
        #[must_use]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset command."]
        #[inline(always)]
        pub const fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software reset busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn resetting(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset busy status."]
        #[inline(always)]
        pub const fn set_resetting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Swrst {
        #[inline(always)]
        fn default() -> Swrst {
            Swrst(0)
        }
    }
    impl core::fmt::Debug for Swrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swrst")
                .field("swrst", &self.swrst())
                .field("resetting", &self.resetting())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swrst {{ swrst: {=bool:?}, resetting: {=bool:?} }}",
                self.swrst(),
                self.resetting()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "CTRL Register Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CTRL Register Busy."]
        #[inline(always)]
        pub const fn set_ctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CMD Register Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CMD Register Busy."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TOP Register Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn top(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TOP Register Busy."]
        #[inline(always)]
        pub const fn set_top(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TOPB Register Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn topb(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TOPB Register Busy."]
        #[inline(always)]
        pub const fn set_topb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OVSCTRL Register Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn ovsctrl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OVSCTRL Register Busy."]
        #[inline(always)]
        pub const fn set_ovsctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("cmd", &self.cmd())
                .field("top", &self.top())
                .field("topb", &self.topb())
                .field("ovsctrl", &self.ovsctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ ctrl: {=bool:?}, cmd: {=bool:?}, top: {=bool:?}, topb: {=bool:?}, ovsctrl: {=bool:?} }}",
                self.ctrl(),
                self.cmd(),
                self.top(),
                self.topb(),
                self.ovsctrl()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Top(pub u32);
    impl Top {
        #[doc = "Counter Top Value."]
        #[must_use]
        #[inline(always)]
        pub const fn top(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter Top Value."]
        #[inline(always)]
        pub const fn set_top(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Top {
        #[inline(always)]
        fn default() -> Top {
            Top(0)
        }
    }
    impl core::fmt::Debug for Top {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Top").field("top", &self.top()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Top {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Top {{ top: {=u16:?} }}", self.top())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Topb(pub u32);
    impl Topb {
        #[doc = "Counter Top Buffer Register."]
        #[must_use]
        #[inline(always)]
        pub const fn topb(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter Top Buffer Register."]
        #[inline(always)]
        pub const fn set_topb(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Topb {
        #[inline(always)]
        fn default() -> Topb {
            Topb(0)
        }
    }
    impl core::fmt::Debug for Topb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Topb").field("topb", &self.topb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Topb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Topb {{ topb: {=u16:?} }}", self.topb())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Auxcntev {
        #[doc = "Counts up on both up-count and down-count events."]
        Both = 0x0,
        #[doc = "Counts up on up-count events."]
        Up = 0x01,
        #[doc = "Counts up on down-count events."]
        Down = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Auxcntev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Auxcntev {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Auxcntev {
        #[inline(always)]
        fn from(val: u8) -> Auxcntev {
            Auxcntev::from_bits(val)
        }
    }
    impl From<Auxcntev> for u8 {
        #[inline(always)]
        fn from(val: Auxcntev) -> u8 {
            Auxcntev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cntdir {
        #[doc = "Up counter mode."]
        Up = 0x0,
        #[doc = "Down counter mode."]
        Down = 0x01,
    }
    impl Cntdir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cntdir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cntdir {
        #[inline(always)]
        fn from(val: u8) -> Cntdir {
            Cntdir::from_bits(val)
        }
    }
    impl From<Cntdir> for u8 {
        #[inline(always)]
        fn from(val: Cntdir) -> u8 {
            Cntdir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cntev {
        #[doc = "Counts up on up-count and down on down-count events."]
        Both = 0x0,
        #[doc = "Only counts up on up-count events."]
        Up = 0x01,
        #[doc = "Only counts down on down-count events."]
        Down = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cntev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cntev {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cntev {
        #[inline(always)]
        fn from(val: u8) -> Cntev {
            Cntev::from_bits(val)
        }
    }
    impl From<Cntev> for u8 {
        #[inline(always)]
        fn from(val: Cntev) -> u8 {
            Cntev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "Up counter mode (clockwise in EXTCLKQUAD mode with the EDGE bit in PCNTn_CTRL set to 0)."]
        Up = 0x0,
        #[doc = "Down counter mode."]
        Down = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Edge {
        #[doc = "Positive edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Does not invert PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes."]
        Pos = 0x0,
        #[doc = "Negative edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Inverts the PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes."]
        Neg = 0x01,
    }
    impl Edge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Edge {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Edge {
        #[inline(always)]
        fn from(val: u8) -> Edge {
            Edge::from_bits(val)
        }
    }
    impl From<Edge> for u8 {
        #[inline(always)]
        fn from(val: Edge) -> u8 {
            Edge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Single input EM23GRPACLK oversampling mode (available in EM0-EM3)."]
        Ovssingle = 0x0,
        #[doc = "Externally clocked single input counter mode (available in EM0-EM3)."]
        Extclksingle = 0x01,
        #[doc = "Externally clocked quadrature decoder mode (available in EM0-EM3)."]
        Extclkquad = 0x02,
        #[doc = "EM23GRPACLK oversampling quadrature decoder 1X mode (available in EM0-EM3)."]
        Ovsquad1x = 0x03,
        #[doc = "EM23GRPACLK oversampling quadrature decoder 2X mode (available in EM0-EM3)."]
        Ovsquad2x = 0x04,
        #[doc = "EM23GRPACLK oversampling quadrature decoder 4X mode (available in EM0-EM3)."]
        Ovsquad4x = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcntlockkey(u16);
    impl Pcntlockkey {
        #[doc = "Write to unock PCNT lockable registers."]
        pub const Unlock: Self = Self(0xa7e0);
    }
    impl Pcntlockkey {
        pub const fn from_bits(val: u16) -> Pcntlockkey {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pcntlockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xa7e0 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcntlockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xa7e0 => defmt::write!(f, "Unlock"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pcntlockkey {
        #[inline(always)]
        fn from(val: u16) -> Pcntlockkey {
            Pcntlockkey::from_bits(val)
        }
    }
    impl From<Pcntlockkey> for u16 {
        #[inline(always)]
        fn from(val: Pcntlockkey) -> u16 {
            Pcntlockkey::to_bits(val)
        }
    }
}
