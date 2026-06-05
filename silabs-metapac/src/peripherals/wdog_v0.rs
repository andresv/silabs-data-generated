#[doc = "WDOG peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdog {
    ptr: *mut u8,
}
unsafe impl Send for Wdog {}
unsafe impl Sync for Wdog {}
impl Wdog {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
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
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "WDOG Clear Source."]
        #[must_use]
        #[inline(always)]
        pub const fn clrsrc(&self) -> super::vals::Clrsrc {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Clrsrc::from_bits(val as u8)
        }
        #[doc = "WDOG Clear Source."]
        #[inline(always)]
        pub const fn set_clrsrc(&mut self, val: super::vals::Clrsrc) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "EM2 Run."]
        #[must_use]
        #[inline(always)]
        pub const fn em2run(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EM2 Run."]
        #[inline(always)]
        pub const fn set_em2run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EM3 Run."]
        #[must_use]
        #[inline(always)]
        pub const fn em3run(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EM3 Run."]
        #[inline(always)]
        pub const fn set_em3run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EM4 Block."]
        #[must_use]
        #[inline(always)]
        pub const fn em4block(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Block."]
        #[inline(always)]
        pub const fn set_em4block(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Debug Mode Run."]
        #[must_use]
        #[inline(always)]
        pub const fn debugrun(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Mode Run."]
        #[inline(always)]
        pub const fn set_debugrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WDOG Reset Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn wdogrstdis(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Reset Disable."]
        #[inline(always)]
        pub const fn set_wdogrstdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PRS Src0 Missing Event WDOG Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn prs0missrsten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Src0 Missing Event WDOG Reset."]
        #[inline(always)]
        pub const fn set_prs0missrsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PRS Src1 Missing Event WDOG Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn prs1missrsten(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Src1 Missing Event WDOG Reset."]
        #[inline(always)]
        pub const fn set_prs1missrsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "WDOG Timeout Period Select."]
        #[must_use]
        #[inline(always)]
        pub const fn persel(&self) -> super::vals::Persel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Persel::from_bits(val as u8)
        }
        #[doc = "WDOG Timeout Period Select."]
        #[inline(always)]
        pub const fn set_persel(&mut self, val: super::vals::Persel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "WDOG Warning Period Select."]
        #[must_use]
        #[inline(always)]
        pub const fn warnsel(&self) -> super::vals::Warnsel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Warnsel::from_bits(val as u8)
        }
        #[doc = "WDOG Warning Period Select."]
        #[inline(always)]
        pub const fn set_warnsel(&mut self, val: super::vals::Warnsel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "WDOG Illegal Window Select."]
        #[must_use]
        #[inline(always)]
        pub const fn winsel(&self) -> super::vals::Winsel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Winsel::from_bits(val as u8)
        }
        #[doc = "WDOG Illegal Window Select."]
        #[inline(always)]
        pub const fn set_winsel(&mut self, val: super::vals::Winsel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
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
                .field("clrsrc", &self.clrsrc())
                .field("em2run", &self.em2run())
                .field("em3run", &self.em3run())
                .field("em4block", &self.em4block())
                .field("debugrun", &self.debugrun())
                .field("wdogrstdis", &self.wdogrstdis())
                .field("prs0missrsten", &self.prs0missrsten())
                .field("prs1missrsten", &self.prs1missrsten())
                .field("persel", &self.persel())
                .field("warnsel", &self.warnsel())
                .field("winsel", &self.winsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ clrsrc: {:?}, em2run: {=bool:?}, em3run: {=bool:?}, em4block: {=bool:?}, debugrun: {=bool:?}, wdogrstdis: {=bool:?}, prs0missrsten: {=bool:?}, prs1missrsten: {=bool:?}, persel: {:?}, warnsel: {:?}, winsel: {:?} }}",
                self.clrsrc(),
                self.em2run(),
                self.em3run(),
                self.em4block(),
                self.debugrun(),
                self.wdogrstdis(),
                self.prs0missrsten(),
                self.prs1missrsten(),
                self.persel(),
                self.warnsel(),
                self.winsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "WDOG Timer Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> super::vals::Clear {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Clear::from_bits(val as u8)
        }
        #[doc = "WDOG Timer Clear."]
        #[inline(always)]
        pub const fn set_clear(&mut self, val: super::vals::Clear) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cmd").field("clear", &self.clear()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmd {{ clear: {:?} }}", self.clear())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Module Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Module Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("En").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for En {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "En {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "WDOG Timeout Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Timeout Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "WDOG Warning Timeout Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn warn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Warning Timeout Interrupt Enable."]
        #[inline(always)]
        pub const fn set_warn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WDOG Window Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn win(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Window Interrupt Enable."]
        #[inline(always)]
        pub const fn set_win(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PRS Src0 Event Missing Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pem0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Src0 Event Missing Interrupt Enable."]
        #[inline(always)]
        pub const fn set_pem0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PRS Src1 Event Missing Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pem1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Src1 Event Missing Interrupt Enable."]
        #[inline(always)]
        pub const fn set_pem1(&mut self, val: bool) {
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
                .field("tout", &self.tout())
                .field("warn", &self.warn())
                .field("win", &self.win())
                .field("pem0", &self.pem0())
                .field("pem1", &self.pem1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ tout: {=bool:?}, warn: {=bool:?}, win: {=bool:?}, pem0: {=bool:?}, pem1: {=bool:?} }}",
                self.tout(),
                self.warn(),
                self.win(),
                self.pem0(),
                self.pem1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "WDOG Timeout Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Timeout Interrupt Flag."]
        #[inline(always)]
        pub const fn set_tout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "WDOG Warning Timeout Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn warn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Warning Timeout Interrupt Flag."]
        #[inline(always)]
        pub const fn set_warn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WDOG Window Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn win(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Window Interrupt Flag."]
        #[inline(always)]
        pub const fn set_win(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PRS Src0 Event Missing Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pem0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Src0 Event Missing Interrupt Flag."]
        #[inline(always)]
        pub const fn set_pem0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PRS Src1 Event Missing Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pem1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Src1 Event Missing Interrupt Flag."]
        #[inline(always)]
        pub const fn set_pem1(&mut self, val: bool) {
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
                .field("tout", &self.tout())
                .field("warn", &self.warn())
                .field("win", &self.win())
                .field("pem0", &self.pem0())
                .field("pem1", &self.pem1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ tout: {=bool:?}, warn: {=bool:?}, win: {=bool:?}, pem0: {=bool:?}, pem1: {=bool:?} }}",
                self.tout(),
                self.warn(),
                self.win(),
                self.pem0(),
                self.pem1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP Version."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP Version."]
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
        #[doc = "WDOG Configuration Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockkey(&self) -> super::vals::Lockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Lockkey::from_bits(val as u16)
        }
        #[doc = "WDOG Configuration Lock."]
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
        #[doc = "WDOG Configuration Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG Configuration Lock Status."]
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
            f.debug_struct("Status").field("lock", &self.lock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status {{ lock: {=bool:?} }}", self.lock())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "Sync Busy for Cmd Register."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Busy for Cmd Register."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: bool) {
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
            f.debug_struct("Syncbusy").field("cmd", &self.cmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Syncbusy {{ cmd: {=bool:?} }}", self.cmd())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clear {
        #[doc = "WDOG timer is unchanged."]
        Unchanged = 0x0,
        #[doc = "WDOG timer is cleared to 0."]
        Cleared = 0x01,
    }
    impl Clear {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clear {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clear {
        #[inline(always)]
        fn from(val: u8) -> Clear {
            Clear::from_bits(val)
        }
    }
    impl From<Clear> for u8 {
        #[inline(always)]
        fn from(val: Clear) -> u8 {
            Clear::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clrsrc {
        #[doc = "A write to the clear bit will clear the WDOG counter."]
        Sw = 0x0,
        #[doc = "A rising edge on the PRS Source 0 will clear the WDOG counter."]
        Prssrc0 = 0x01,
    }
    impl Clrsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clrsrc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clrsrc {
        #[inline(always)]
        fn from(val: u8) -> Clrsrc {
            Clrsrc::from_bits(val)
        }
    }
    impl From<Clrsrc> for u8 {
        #[inline(always)]
        fn from(val: Clrsrc) -> u8 {
            Clrsrc::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Lock WDOG lockable registers."]
        pub const Lock: Self = Self(0x0);
        #[doc = "Unlock WDOG lockable registers."]
        pub const Unlock: Self = Self(0xabe8);
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
                0xabe8 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Lock"),
                0xabe8 => defmt::write!(f, "Unlock"),
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
    pub enum Persel {
        #[doc = "Timeout period of 9 wdog cycles."]
        Sel0 = 0x0,
        #[doc = "Timeout period of 17 wdog cycles."]
        Sel1 = 0x01,
        #[doc = "Timeout period of 33 wdog cycles."]
        Sel2 = 0x02,
        #[doc = "Timeout period of 65 wdog cycles."]
        Sel3 = 0x03,
        #[doc = "Timeout period of 129 wdog cycles."]
        Sel4 = 0x04,
        #[doc = "Timeout period of 257 wdog cycles."]
        Sel5 = 0x05,
        #[doc = "Timeout period of 513 wdog cycles."]
        Sel6 = 0x06,
        #[doc = "Timeout period of 1k wdog cycles."]
        Sel7 = 0x07,
        #[doc = "Timeout period of 2k wdog cycles."]
        Sel8 = 0x08,
        #[doc = "Timeout period of 4k wdog cycles."]
        Sel9 = 0x09,
        #[doc = "Timeout period of 8k wdog cycles."]
        Sel10 = 0x0a,
        #[doc = "Timeout period of 16k wdog cycles."]
        Sel11 = 0x0b,
        #[doc = "Timeout period of 32k wdog cycles."]
        Sel12 = 0x0c,
        #[doc = "Timeout period of 64k wdog cycles."]
        Sel13 = 0x0d,
        #[doc = "Timeout period of 128k wdog cycles."]
        Sel14 = 0x0e,
        #[doc = "Timeout period of 256k wdog cycles."]
        Sel15 = 0x0f,
    }
    impl Persel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Persel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Persel {
        #[inline(always)]
        fn from(val: u8) -> Persel {
            Persel::from_bits(val)
        }
    }
    impl From<Persel> for u8 {
        #[inline(always)]
        fn from(val: Persel) -> u8 {
            Persel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Warnsel {
        #[doc = "Disable."]
        Dis = 0x0,
        #[doc = "Warning timeout is 25% of the Timeout."]
        Sel1 = 0x01,
        #[doc = "Warning timeout is 50% of the Timeout."]
        Sel2 = 0x02,
        #[doc = "Warning timeout is 75% of the Timeout."]
        Sel3 = 0x03,
    }
    impl Warnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Warnsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Warnsel {
        #[inline(always)]
        fn from(val: u8) -> Warnsel {
            Warnsel::from_bits(val)
        }
    }
    impl From<Warnsel> for u8 {
        #[inline(always)]
        fn from(val: Warnsel) -> u8 {
            Warnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Winsel {
        #[doc = "Disabled."]
        Dis = 0x0,
        #[doc = "Window timeout is 12.5% of the Timeout."]
        Sel1 = 0x01,
        #[doc = "Window timeout is 25% of the Timeout."]
        Sel2 = 0x02,
        #[doc = "Window timeout is 37.5% of the Timeout."]
        Sel3 = 0x03,
        #[doc = "Window timeout is 50% of the Timeout."]
        Sel4 = 0x04,
        #[doc = "Window timeout is 62.5% of the Timeout."]
        Sel5 = 0x05,
        #[doc = "Window timeout is 75.5% of the Timeout."]
        Sel6 = 0x06,
        #[doc = "Window timeout is 87.5% of the Timeout."]
        Sel7 = 0x07,
    }
    impl Winsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Winsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Winsel {
        #[inline(always)]
        fn from(val: u8) -> Winsel {
            Winsel::from_bits(val)
        }
    }
    impl From<Winsel> for u8 {
        #[inline(always)]
        fn from(val: Winsel) -> u8 {
            Winsel::to_bits(val)
        }
    }
}
