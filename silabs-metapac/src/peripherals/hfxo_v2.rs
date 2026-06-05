#[doc = "HFXO peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxo {
    ptr: *mut u8,
}
unsafe impl Send for Hfxo {}
unsafe impl Sync for Hfxo {}
impl Hfxo {
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
    pub const fn xtalcfg(self) -> crate::common::Reg<regs::Xtalcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn xtalctrl(self) -> crate::common::Reg<regs::Xtalctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn xtalcfg_set(self) -> crate::common::Reg<regs::Xtalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn xtalctrl_set(self) -> crate::common::Reg<regs::Xtalctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn xtalcfg_clr(self) -> crate::common::Reg<regs::Xtalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn xtalctrl_clr(self) -> crate::common::Reg<regs::Xtalctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn xtalcfg_tgl(self) -> crate::common::Reg<regs::Xtalcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn xtalctrl_tgl(self) -> crate::common::Reg<regs::Xtalctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Crystal Oscillator Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Crystal Oscillator Mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable XI Internal DC Bias."]
        #[must_use]
        #[inline(always)]
        pub const fn enxidcbiasana(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable XI Internal DC Bias."]
        #[inline(always)]
        pub const fn set_enxidcbiasana(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Squaring Buffer Schmitt Trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn sqbufschtrgana(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Squaring Buffer Schmitt Trigger."]
        #[inline(always)]
        pub const fn set_sqbufschtrgana(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("enxidcbiasana", &self.enxidcbiasana())
                .field("sqbufschtrgana", &self.sqbufschtrgana())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ mode: {:?}, enxidcbiasana: {=bool:?}, sqbufschtrgana: {=bool:?} }}",
                self.mode(),
                self.enxidcbiasana(),
                self.sqbufschtrgana()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Core Bias Optimizaton."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasopt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Core Bias Optimizaton."]
        #[inline(always)]
        pub const fn set_corebiasopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Manual Override."]
        #[must_use]
        #[inline(always)]
        pub const fn manualoverride(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Manual Override."]
        #[inline(always)]
        pub const fn set_manualoverride(&mut self, val: bool) {
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
                .field("corebiasopt", &self.corebiasopt())
                .field("manualoverride", &self.manualoverride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ corebiasopt: {=bool:?}, manualoverride: {=bool:?} }}",
                self.corebiasopt(),
                self.manualoverride()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Force Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn forceen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force Enable."]
        #[inline(always)]
        pub const fn set_forceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Disable On-demand Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn disondemand(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Disable On-demand Mode."]
        #[inline(always)]
        pub const fn set_disondemand(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Keep Warm."]
        #[must_use]
        #[inline(always)]
        pub const fn keepwarm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Keep Warm."]
        #[inline(always)]
        pub const fn set_keepwarm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force XI Pin to Ground."]
        #[must_use]
        #[inline(always)]
        pub const fn forcexi2gndana(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force XI Pin to Ground."]
        #[inline(always)]
        pub const fn set_forcexi2gndana(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force XO Pin to Ground."]
        #[must_use]
        #[inline(always)]
        pub const fn forcexo2gndana(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force XO Pin to Ground."]
        #[inline(always)]
        pub const fn set_forcexo2gndana(&mut self, val: bool) {
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
                .field("forceen", &self.forceen())
                .field("disondemand", &self.disondemand())
                .field("keepwarm", &self.keepwarm())
                .field("forcexi2gndana", &self.forcexi2gndana())
                .field("forcexo2gndana", &self.forcexo2gndana())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ forceen: {=bool:?}, disondemand: {=bool:?}, keepwarm: {=bool:?}, forcexi2gndana: {=bool:?}, forcexo2gndana: {=bool:?} }}",
                self.forceen(),
                self.disondemand(),
                self.keepwarm(),
                self.forcexi2gndana(),
                self.forcexo2gndana()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ready Interrupt."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Core Bias Optimization Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasoptrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Core Bias Optimization Ready Interrupt."]
        #[inline(always)]
        pub const fn set_corebiasoptrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Did Not Start Error Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn dnserr(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Did Not Start Error Interrupt."]
        #[inline(always)]
        pub const fn set_dnserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Core Bias Optimization Error Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasopterr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Core Bias Optimization Error Interrupt."]
        #[inline(always)]
        pub const fn set_corebiasopterr(&mut self, val: bool) {
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
                .field("rdy", &self.rdy())
                .field("corebiasoptrdy", &self.corebiasoptrdy())
                .field("dnserr", &self.dnserr())
                .field("corebiasopterr", &self.corebiasopterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ rdy: {=bool:?}, corebiasoptrdy: {=bool:?}, dnserr: {=bool:?}, corebiasopterr: {=bool:?} }}",
                self.rdy(),
                self.corebiasoptrdy(),
                self.dnserr(),
                self.corebiasopterr()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Ready Interrupt."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Core Bias Optimization Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasoptrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Core Bias Optimization Ready Interrupt."]
        #[inline(always)]
        pub const fn set_corebiasoptrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Did Not Start Error Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn dnserr(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Did Not Start Error Interrupt."]
        #[inline(always)]
        pub const fn set_dnserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Core Bias Optimization Error Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasopterr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Core Bias Optimization Error Interrupt."]
        #[inline(always)]
        pub const fn set_corebiasopterr(&mut self, val: bool) {
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
                .field("rdy", &self.rdy())
                .field("corebiasoptrdy", &self.corebiasoptrdy())
                .field("dnserr", &self.dnserr())
                .field("corebiasopterr", &self.corebiasopterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ rdy: {=bool:?}, corebiasoptrdy: {=bool:?}, dnserr: {=bool:?}, corebiasopterr: {=bool:?} }}",
                self.rdy(),
                self.corebiasoptrdy(),
                self.dnserr(),
                self.corebiasopterr()
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
        #[doc = "Core Bias Optimization Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasoptrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Core Bias Optimization Ready."]
        #[inline(always)]
        pub const fn set_corebiasoptrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "Oscillator Requested by Hardware."]
        #[must_use]
        #[inline(always)]
        pub const fn hwreq(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Oscillator Requested by Hardware."]
        #[inline(always)]
        pub const fn set_hwreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Oscillator Is Kept Warm."]
        #[must_use]
        #[inline(always)]
        pub const fn iswarm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Oscillator Is Kept Warm."]
        #[inline(always)]
        pub const fn set_iswarm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "FSM Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn fsmlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "FSM Lock Status."]
        #[inline(always)]
        pub const fn set_fsmlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Configuration Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Lock Status."]
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
                .field("corebiasoptrdy", &self.corebiasoptrdy())
                .field("ens", &self.ens())
                .field("hwreq", &self.hwreq())
                .field("iswarm", &self.iswarm())
                .field("fsmlock", &self.fsmlock())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ rdy: {=bool:?}, corebiasoptrdy: {=bool:?}, ens: {=bool:?}, hwreq: {=bool:?}, iswarm: {=bool:?}, fsmlock: {=bool:?}, lock: {=bool:?} }}",
                self.rdy(),
                self.corebiasoptrdy(),
                self.ens(),
                self.hwreq(),
                self.iswarm(),
                self.fsmlock(),
                self.lock()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalcfg(pub u32);
    impl Xtalcfg {
        #[doc = "Intermediate Startup Core Bias Current."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasstartupi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Intermediate Startup Core Bias Current."]
        #[inline(always)]
        pub const fn set_corebiasstartupi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Startup Core Bias Current."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasstartup(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "Startup Core Bias Current."]
        #[inline(always)]
        pub const fn set_corebiasstartup(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
        }
        #[doc = "Startup Tuning Capacitance on XI."]
        #[must_use]
        #[inline(always)]
        pub const fn ctunexistartup(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Startup Tuning Capacitance on XI."]
        #[inline(always)]
        pub const fn set_ctunexistartup(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Startup Tuning Capacitance on XO."]
        #[must_use]
        #[inline(always)]
        pub const fn ctunexostartup(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Startup Tuning Capacitance on XO."]
        #[inline(always)]
        pub const fn set_ctunexostartup(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Steady State Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn timeoutsteady(&self) -> super::vals::Timeoutsteady {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::Timeoutsteady::from_bits(val as u8)
        }
        #[doc = "Steady State Timeout."]
        #[inline(always)]
        pub const fn set_timeoutsteady(&mut self, val: super::vals::Timeoutsteady) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
        }
        #[doc = "Core Bias LSB Change Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn timeoutcblsb(&self) -> super::vals::Timeoutcblsb {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Timeoutcblsb::from_bits(val as u8)
        }
        #[doc = "Core Bias LSB Change Timeout."]
        #[inline(always)]
        pub const fn set_timeoutcblsb(&mut self, val: super::vals::Timeoutcblsb) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Xtalcfg {
        #[inline(always)]
        fn default() -> Xtalcfg {
            Xtalcfg(0)
        }
    }
    impl core::fmt::Debug for Xtalcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalcfg")
                .field("corebiasstartupi", &self.corebiasstartupi())
                .field("corebiasstartup", &self.corebiasstartup())
                .field("ctunexistartup", &self.ctunexistartup())
                .field("ctunexostartup", &self.ctunexostartup())
                .field("timeoutsteady", &self.timeoutsteady())
                .field("timeoutcblsb", &self.timeoutcblsb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xtalcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xtalcfg {{ corebiasstartupi: {=u8:?}, corebiasstartup: {=u8:?}, ctunexistartup: {=u8:?}, ctunexostartup: {=u8:?}, timeoutsteady: {:?}, timeoutcblsb: {:?} }}",
                self.corebiasstartupi(),
                self.corebiasstartup(),
                self.ctunexistartup(),
                self.ctunexostartup(),
                self.timeoutsteady(),
                self.timeoutcblsb()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtalctrl(pub u32);
    impl Xtalctrl {
        #[doc = "Core Bias Current."]
        #[must_use]
        #[inline(always)]
        pub const fn corebiasana(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Core Bias Current."]
        #[inline(always)]
        pub const fn set_corebiasana(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Tuning Capacitance on XI."]
        #[must_use]
        #[inline(always)]
        pub const fn ctunexiana(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Tuning Capacitance on XI."]
        #[inline(always)]
        pub const fn set_ctunexiana(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Tuning Capacitance on XO."]
        #[must_use]
        #[inline(always)]
        pub const fn ctunexoana(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Tuning Capacitance on XO."]
        #[inline(always)]
        pub const fn set_ctunexoana(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Fixed Tuning Capacitance."]
        #[must_use]
        #[inline(always)]
        pub const fn ctunefixana(&self) -> super::vals::Ctunefixana {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ctunefixana::from_bits(val as u8)
        }
        #[doc = "Fixed Tuning Capacitance."]
        #[inline(always)]
        pub const fn set_ctunefixana(&mut self, val: super::vals::Ctunefixana) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Core Degeneration."]
        #[must_use]
        #[inline(always)]
        pub const fn coredgenana(&self) -> super::vals::Coredgenana {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Coredgenana::from_bits(val as u8)
        }
        #[doc = "Core Degeneration."]
        #[inline(always)]
        pub const fn set_coredgenana(&mut self, val: super::vals::Coredgenana) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Skip Core Bias Optimization."]
        #[must_use]
        #[inline(always)]
        pub const fn skipcorebiasopt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Skip Core Bias Optimization."]
        #[inline(always)]
        pub const fn set_skipcorebiasopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Xtalctrl {
        #[inline(always)]
        fn default() -> Xtalctrl {
            Xtalctrl(0)
        }
    }
    impl core::fmt::Debug for Xtalctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xtalctrl")
                .field("corebiasana", &self.corebiasana())
                .field("ctunexiana", &self.ctunexiana())
                .field("ctunexoana", &self.ctunexoana())
                .field("ctunefixana", &self.ctunefixana())
                .field("coredgenana", &self.coredgenana())
                .field("skipcorebiasopt", &self.skipcorebiasopt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xtalctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xtalctrl {{ corebiasana: {=u8:?}, ctunexiana: {=u8:?}, ctunexoana: {=u8:?}, ctunefixana: {:?}, coredgenana: {:?}, skipcorebiasopt: {=bool:?} }}",
                self.corebiasana(),
                self.ctunexiana(),
                self.ctunexoana(),
                self.ctunefixana(),
                self.coredgenana(),
                self.skipcorebiasopt()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Coredgenana {
        #[doc = "Do not apply core degeneration resistence."]
        None = 0x0,
        #[doc = "Apply 33 ohm core degeneration resistence."]
        Dgen33 = 0x01,
        #[doc = "Apply 50 ohm core degeneration resistence."]
        Dgen50 = 0x02,
        #[doc = "Apply 100 ohm core degeneration resistence."]
        Dgen100 = 0x03,
    }
    impl Coredgenana {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Coredgenana {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Coredgenana {
        #[inline(always)]
        fn from(val: u8) -> Coredgenana {
            Coredgenana::from_bits(val)
        }
    }
    impl From<Coredgenana> for u8 {
        #[inline(always)]
        fn from(val: Coredgenana) -> u8 {
            Coredgenana::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ctunefixana {
        #[doc = "Remove fixed capacitance on XI and XO nodes."]
        None = 0x0,
        #[doc = "Adds fixed capacitance on XI node."]
        Xi = 0x01,
        #[doc = "Adds fixed capacitance on XO node."]
        Xo = 0x02,
        #[doc = "Adds fixed capacitance on both XI and XO nodes."]
        Both = 0x03,
    }
    impl Ctunefixana {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ctunefixana {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ctunefixana {
        #[inline(always)]
        fn from(val: u8) -> Ctunefixana {
            Ctunefixana::from_bits(val)
        }
    }
    impl From<Ctunefixana> for u8 {
        #[inline(always)]
        fn from(val: Ctunefixana) -> u8 {
            Ctunefixana::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Write this value to unlock."]
        pub const Unlock: Self = Self(0x580e);
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
                0x580e => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x580e => defmt::write!(f, "Unlock"),
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
    pub enum Mode {
        #[doc = "crystal oscillator."]
        Xtal = 0x0,
        #[doc = "external sinusoidal clock can be supplied on XI pin."]
        Extclk = 0x01,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Timeoutcblsb {
        #[doc = "The core bias LSB change timeout is set to 8 us minimum. The maximum can be +40%."]
        T8us = 0x0,
        #[doc = "The core bias LSB change timeout is set to 20 us minimum. The maximum can be +40%."]
        T20us = 0x01,
        #[doc = "The core bias LSB change timeout is set to 41 us minimum. The maximum can be +40%."]
        T41us = 0x02,
        #[doc = "The core bias LSB change timeout is set to 62 us minimum. The maximum can be +40%."]
        T62us = 0x03,
        #[doc = "The core bias LSB change timeout is set to 83 us minimum. The maximum can be +40%."]
        T83us = 0x04,
        #[doc = "The core bias LSB change timeout is set to 104 us minimum. The maximum can be +40%."]
        T104us = 0x05,
        #[doc = "The core bias LSB change timeout is set to 125 us minimum. The maximum can be +40%."]
        T125us = 0x06,
        #[doc = "The core bias LSB change timeout is set to 166 us minimum. The maximum can be +40%."]
        T166us = 0x07,
        #[doc = "The core bias LSB change timeout is set to 208 us minimum. The maximum can be +40%."]
        T208us = 0x08,
        #[doc = "The core bias LSB change timeout is set to 250 us minimum. The maximum can be +40%."]
        T250us = 0x09,
        #[doc = "The core bias LSB change timeout is set to 333 us minimum. The maximum can be +40%."]
        T333us = 0x0a,
        #[doc = "The core bias LSB change timeout is set to 416 us minimum. The maximum can be +40%."]
        T416us = 0x0b,
        #[doc = "The core bias LSB change timeout is set to 833 us minimum. The maximum can be +40%."]
        T833us = 0x0c,
        #[doc = "The core bias LSB change timeout is set to 1250 us minimum. The maximum can be +40%."]
        T1250us = 0x0d,
        #[doc = "The core bias LSB change timeout is set to 2083 us minimum. The maximum can be +40%."]
        T2083us = 0x0e,
        #[doc = "The core bias LSB change timeout is set to 3750 us minimum. The maximum can be +40%."]
        T3750us = 0x0f,
    }
    impl Timeoutcblsb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timeoutcblsb {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timeoutcblsb {
        #[inline(always)]
        fn from(val: u8) -> Timeoutcblsb {
            Timeoutcblsb::from_bits(val)
        }
    }
    impl From<Timeoutcblsb> for u8 {
        #[inline(always)]
        fn from(val: Timeoutcblsb) -> u8 {
            Timeoutcblsb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Timeoutsteady {
        #[doc = "The steady state timeout is set to 16 us minimum. The maximum can be +40%."]
        T16us = 0x0,
        #[doc = "The steady state timeout is set to 41 us minimum. The maximum can be +40%."]
        T41us = 0x01,
        #[doc = "The steady state timeout is set to 83 us minimum. The maximum can be +40%."]
        T83us = 0x02,
        #[doc = "The steady state timeout is set to 125 us minimum. The maximum can be +40%."]
        T125us = 0x03,
        #[doc = "The steady state timeout is set to 166 us minimum. The maximum can be +40%."]
        T166us = 0x04,
        #[doc = "The steady state timeout is set to 208 us minimum. The maximum can be +40%."]
        T208us = 0x05,
        #[doc = "The steady state timeout is set to 250 us minimum. The maximum can be +40%."]
        T250us = 0x06,
        #[doc = "The steady state timeout is set to 333 us minimum. The maximum can be +40%."]
        T333us = 0x07,
        #[doc = "The steady state timeout is set to 416 us minimum. The maximum can be +40%."]
        T416us = 0x08,
        #[doc = "The steady state timeout is set to 500 us minimum. The maximum can be +40%."]
        T500us = 0x09,
        #[doc = "The steady state timeout is set to 666 us minimum. The maximum can be +40%."]
        T666us = 0x0a,
        #[doc = "The steady state timeout is set to 833 us minimum. The maximum can be +40%."]
        T833us = 0x0b,
        #[doc = "The steady state timeout is set to 1666 us minimum. The maximum can be +40%."]
        T1666us = 0x0c,
        #[doc = "The steady state timeout is set to 2500 us minimum. The maximum can be +40%."]
        T2500us = 0x0d,
        #[doc = "The steady state timeout is set to 4166 us minimum. The maximum can be +40%."]
        T4166us = 0x0e,
        #[doc = "The steady state timeout is set to 7500 us minimum. The maximum can be +40%."]
        T7500us = 0x0f,
    }
    impl Timeoutsteady {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timeoutsteady {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timeoutsteady {
        #[inline(always)]
        fn from(val: u8) -> Timeoutsteady {
            Timeoutsteady::from_bits(val)
        }
    }
    impl From<Timeoutsteady> for u8 {
        #[inline(always)]
        fn from(val: Timeoutsteady) -> u8 {
            Timeoutsteady::to_bits(val)
        }
    }
}
