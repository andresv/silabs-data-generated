#[doc = "SYSRTC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysrtc {
    ptr: *mut u8,
}
unsafe impl Send for Sysrtc {}
unsafe impl Sync for Sysrtc {}
impl Sysrtc {
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
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_if(self) -> crate::common::Reg<regs::Grp0If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_ien(self) -> crate::common::Reg<regs::Grp0Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_ctrl(self) -> crate::common::Reg<regs::Grp0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_cmp0value(self) -> crate::common::Reg<regs::Grp0Cmp0value, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_cmp1value(self) -> crate::common::Reg<regs::Grp0Cmp1value, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_cap0value(self) -> crate::common::Reg<regs::Grp0Cap0value, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn grp0_syncbusy(self) -> crate::common::Reg<regs::Grp0Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
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
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cnt_set(self) -> crate::common::Reg<regs::Cnt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn grp0_if_set(self) -> crate::common::Reg<regs::Grp0If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn grp0_ien_set(self) -> crate::common::Reg<regs::Grp0Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn grp0_ctrl_set(self) -> crate::common::Reg<regs::Grp0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn grp0_cmp0value_set(self) -> crate::common::Reg<regs::Grp0Cmp0value, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn grp0_cmp1value_set(self) -> crate::common::Reg<regs::Grp0Cmp1value, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
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
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cnt_clr(self) -> crate::common::Reg<regs::Cnt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn grp0_if_clr(self) -> crate::common::Reg<regs::Grp0If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn grp0_ien_clr(self) -> crate::common::Reg<regs::Grp0Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn grp0_ctrl_clr(self) -> crate::common::Reg<regs::Grp0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn grp0_cmp0value_clr(self) -> crate::common::Reg<regs::Grp0Cmp0value, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x204cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn grp0_cmp1value_clr(self) -> crate::common::Reg<regs::Grp0Cmp1value, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
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
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cnt_tgl(self) -> crate::common::Reg<regs::Cnt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn grp0_if_tgl(self) -> crate::common::Reg<regs::Grp0If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn grp0_ien_tgl(self) -> crate::common::Reg<regs::Grp0Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn grp0_ctrl_tgl(self) -> crate::common::Reg<regs::Grp0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn grp0_cmp0value_tgl(self) -> crate::common::Reg<regs::Grp0Cmp0value, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn grp0_cmp1value_tgl(self) -> crate::common::Reg<regs::Grp0Cmp1value, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Debug Mode Run Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn debugrun(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Mode Run Enable."]
        #[inline(always)]
        pub const fn set_debugrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cfg").field("debugrun", &self.debugrun()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfg {{ debugrun: {=bool:?} }}", self.debugrun())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Start SYSRTC."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start SYSRTC."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop SYSRTC."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop SYSRTC."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("start", &self.start())
                .field("stop", &self.stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ start: {=bool:?}, stop: {=bool:?} }}",
                self.start(),
                self.stop()
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
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Counter Value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            defmt::write!(f, "Cnt {{ cnt: {=u32:?} }}", self.cnt())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "SYSRTC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSRTC Enable."]
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
    pub struct Grp0Cap0value(pub u32);
    impl Grp0Cap0value {
        #[doc = "Capture 0 Value."]
        #[must_use]
        #[inline(always)]
        pub const fn cap0value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Capture 0 Value."]
        #[inline(always)]
        pub const fn set_cap0value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Grp0Cap0value {
        #[inline(always)]
        fn default() -> Grp0Cap0value {
            Grp0Cap0value(0)
        }
    }
    impl core::fmt::Debug for Grp0Cap0value {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0Cap0value")
                .field("cap0value", &self.cap0value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0Cap0value {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Grp0Cap0value {{ cap0value: {=u32:?} }}", self.cap0value())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grp0Cmp0value(pub u32);
    impl Grp0Cmp0value {
        #[doc = "Compare 0 Value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp0value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare 0 Value."]
        #[inline(always)]
        pub const fn set_cmp0value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Grp0Cmp0value {
        #[inline(always)]
        fn default() -> Grp0Cmp0value {
            Grp0Cmp0value(0)
        }
    }
    impl core::fmt::Debug for Grp0Cmp0value {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0Cmp0value")
                .field("cmp0value", &self.cmp0value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0Cmp0value {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Grp0Cmp0value {{ cmp0value: {=u32:?} }}", self.cmp0value())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grp0Cmp1value(pub u32);
    impl Grp0Cmp1value {
        #[doc = "Compare 1 Value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Compare 1 Value."]
        #[inline(always)]
        pub const fn set_cmp1value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Grp0Cmp1value {
        #[inline(always)]
        fn default() -> Grp0Cmp1value {
            Grp0Cmp1value(0)
        }
    }
    impl core::fmt::Debug for Grp0Cmp1value {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0Cmp1value")
                .field("cmp1value", &self.cmp1value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0Cmp1value {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Grp0Cmp1value {{ cmp1value: {=u32:?} }}", self.cmp1value())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grp0Ctrl(pub u32);
    impl Grp0Ctrl {
        #[doc = "Compare 0 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp0en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 0 Enable."]
        #[inline(always)]
        pub const fn set_cmp0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compare 1 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 Enable."]
        #[inline(always)]
        pub const fn set_cmp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Capture 0 Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cap0en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 0 Enable."]
        #[inline(always)]
        pub const fn set_cap0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare 0 Compare Match Output Action."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp0cmoa(&self) -> super::vals::Cmp0cmoa {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Cmp0cmoa::from_bits(val as u8)
        }
        #[doc = "Compare 0 Compare Match Output Action."]
        #[inline(always)]
        pub const fn set_cmp0cmoa(&mut self, val: super::vals::Cmp0cmoa) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "Compare 1 Compare Match Output Action."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1cmoa(&self) -> super::vals::Cmp1cmoa {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Cmp1cmoa::from_bits(val as u8)
        }
        #[doc = "Compare 1 Compare Match Output Action."]
        #[inline(always)]
        pub const fn set_cmp1cmoa(&mut self, val: super::vals::Cmp1cmoa) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "Capture 0 Edge Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cap0edge(&self) -> super::vals::Cap0edge {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Cap0edge::from_bits(val as u8)
        }
        #[doc = "Capture 0 Edge Select."]
        #[inline(always)]
        pub const fn set_cap0edge(&mut self, val: super::vals::Cap0edge) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
    }
    impl Default for Grp0Ctrl {
        #[inline(always)]
        fn default() -> Grp0Ctrl {
            Grp0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Grp0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0Ctrl")
                .field("cmp0en", &self.cmp0en())
                .field("cmp1en", &self.cmp1en())
                .field("cap0en", &self.cap0en())
                .field("cmp0cmoa", &self.cmp0cmoa())
                .field("cmp1cmoa", &self.cmp1cmoa())
                .field("cap0edge", &self.cap0edge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Grp0Ctrl {{ cmp0en: {=bool:?}, cmp1en: {=bool:?}, cap0en: {=bool:?}, cmp0cmoa: {:?}, cmp1cmoa: {:?}, cap0edge: {:?} }}",
                self.cmp0en(),
                self.cmp1en(),
                self.cap0en(),
                self.cmp0cmoa(),
                self.cmp1cmoa(),
                self.cap0edge()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grp0Ien(pub u32);
    impl Grp0Ien {
        #[doc = "Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compare 0 Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 0 Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cmp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Compare 1 Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture 0 Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cap0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 0 Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cap0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Grp0Ien {
        #[inline(always)]
        fn default() -> Grp0Ien {
            Grp0Ien(0)
        }
    }
    impl core::fmt::Debug for Grp0Ien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0Ien")
                .field("ovf", &self.ovf())
                .field("cmp0", &self.cmp0())
                .field("cmp1", &self.cmp1())
                .field("cap0", &self.cap0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Grp0Ien {{ ovf: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cap0: {=bool:?} }}",
                self.ovf(),
                self.cmp0(),
                self.cmp1(),
                self.cap0()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grp0If(pub u32);
    impl Grp0If {
        #[doc = "Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ovf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Compare 0 Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 0 Interrupt Flag."]
        #[inline(always)]
        pub const fn set_cmp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Compare 1 Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 Interrupt Flag."]
        #[inline(always)]
        pub const fn set_cmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture 0 Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cap0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Capture 0 Interrupt Flag."]
        #[inline(always)]
        pub const fn set_cap0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Grp0If {
        #[inline(always)]
        fn default() -> Grp0If {
            Grp0If(0)
        }
    }
    impl core::fmt::Debug for Grp0If {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0If")
                .field("ovf", &self.ovf())
                .field("cmp0", &self.cmp0())
                .field("cmp1", &self.cmp1())
                .field("cap0", &self.cap0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Grp0If {{ ovf: {=bool:?}, cmp0: {=bool:?}, cmp1: {=bool:?}, cap0: {=bool:?} }}",
                self.ovf(),
                self.cmp0(),
                self.cmp1(),
                self.cap0()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grp0Syncbusy(pub u32);
    impl Grp0Syncbusy {
        #[doc = "Sync busy for CTRL register."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for CTRL register."]
        #[inline(always)]
        pub const fn set_ctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sync busy for CMP0VALUE register."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp0value(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for CMP0VALUE register."]
        #[inline(always)]
        pub const fn set_cmp0value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sync busy for CMP1VALUE register."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1value(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for CMP1VALUE register."]
        #[inline(always)]
        pub const fn set_cmp1value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Grp0Syncbusy {
        #[inline(always)]
        fn default() -> Grp0Syncbusy {
            Grp0Syncbusy(0)
        }
    }
    impl core::fmt::Debug for Grp0Syncbusy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grp0Syncbusy")
                .field("ctrl", &self.ctrl())
                .field("cmp0value", &self.cmp0value())
                .field("cmp1value", &self.cmp1value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grp0Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Grp0Syncbusy {{ ctrl: {=bool:?}, cmp0value: {=bool:?}, cmp1value: {=bool:?} }}",
                self.ctrl(),
                self.cmp0value(),
                self.cmp1value()
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
        #[doc = "SYSRTC running status."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSRTC running status."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lockstatus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Lock Status."]
        #[inline(always)]
        pub const fn set_lockstatus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("running", &self.running())
                .field("lockstatus", &self.lockstatus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ running: {=bool:?}, lockstatus: {=bool:?} }}",
                self.running(),
                self.lockstatus()
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
        #[doc = "Sync busy for START bitfield."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for START bitfield."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sync busy for STOP bitfield."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for STOP bitfield."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sync busy for CNT bitfield."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for CNT bitfield."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("start", &self.start())
                .field("stop", &self.stop())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ start: {=bool:?}, stop: {=bool:?}, cnt: {=bool:?} }}",
                self.start(),
                self.stop(),
                self.cnt()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cap0edge {
        #[doc = "Rising edges detected."]
        Rising = 0x0,
        #[doc = "Falling edges detected."]
        Falling = 0x01,
        #[doc = "Both edges detected."]
        Both = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cap0edge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cap0edge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cap0edge {
        #[inline(always)]
        fn from(val: u8) -> Cap0edge {
            Cap0edge::from_bits(val)
        }
    }
    impl From<Cap0edge> for u8 {
        #[inline(always)]
        fn from(val: Cap0edge) -> u8 {
            Cap0edge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cmp0cmoa {
        #[doc = "Cleared on the next cycle."]
        Clear = 0x0,
        #[doc = "Set on the next cycle."]
        Set = 0x01,
        #[doc = "Set on the next cycle, cleared on the cycle after."]
        Pulse = 0x02,
        #[doc = "Inverted on the next cycle."]
        Toggle = 0x03,
        #[doc = "Export this channel's CMP IF."]
        Cmpif = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cmp0cmoa {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cmp0cmoa {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cmp0cmoa {
        #[inline(always)]
        fn from(val: u8) -> Cmp0cmoa {
            Cmp0cmoa::from_bits(val)
        }
    }
    impl From<Cmp0cmoa> for u8 {
        #[inline(always)]
        fn from(val: Cmp0cmoa) -> u8 {
            Cmp0cmoa::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cmp1cmoa {
        #[doc = "Cleared on the next cycle."]
        Clear = 0x0,
        #[doc = "Set on the next cycle."]
        Set = 0x01,
        #[doc = "Set on the next cycle, cleared on the cycle after."]
        Pulse = 0x02,
        #[doc = "Inverted on the next cycle."]
        Toggle = 0x03,
        #[doc = "Export this channel's CMP IF."]
        Cmpif = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cmp1cmoa {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cmp1cmoa {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cmp1cmoa {
        #[inline(always)]
        fn from(val: u8) -> Cmp1cmoa {
            Cmp1cmoa::from_bits(val)
        }
    }
    impl From<Cmp1cmoa> for u8 {
        #[inline(always)]
        fn from(val: Cmp1cmoa) -> u8 {
            Cmp1cmoa::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Write to unlock SYSRTC lockable registers."]
        pub const Unlock: Self = Self(0x4776);
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
                0x4776 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x4776 => defmt::write!(f, "Unlock"),
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
