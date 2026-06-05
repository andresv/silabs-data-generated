#[doc = "PRORTC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prortc {
    ptr: *mut u8,
}
unsafe impl Send for Prortc {}
unsafe impl Sync for Prortc {}
impl Prortc {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn precnt(self) -> crate::common::Reg<regs::Precnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn combcnt(self) -> crate::common::Reg<regs::Combcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn syncbusy(self) -> crate::common::Reg<regs::Syncbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cc0_ctrl(self) -> crate::common::Reg<regs::Cc0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cc0_ocvalue(self) -> crate::common::Reg<regs::Cc0Ocvalue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cc0_icvalue(self) -> crate::common::Reg<regs::Cc0Icvalue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cc1_ctrl(self) -> crate::common::Reg<regs::Cc1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cc1_ocvalue(self) -> crate::common::Reg<regs::Cc1Ocvalue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cc1_icvalue(self) -> crate::common::Reg<regs::Cc1Icvalue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc0Ctrl(pub u32);
    impl Cc0Ctrl {
        #[doc = "CC Channel Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Cc0CtrlMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Cc0CtrlMode::from_bits(val as u8)
        }
        #[doc = "CC Channel Mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Cc0CtrlMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Compare Match Output Action."]
        #[must_use]
        #[inline(always)]
        pub const fn cmoa(&self) -> super::vals::Cc0CtrlCmoa {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Cc0CtrlCmoa::from_bits(val as u8)
        }
        #[doc = "Compare Match Output Action."]
        #[inline(always)]
        pub const fn set_cmoa(&mut self, val: super::vals::Cc0CtrlCmoa) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Capture compare channel comparison base."]
        #[must_use]
        #[inline(always)]
        pub const fn compbase(&self) -> super::vals::Cc0CtrlCompbase {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Cc0CtrlCompbase::from_bits(val as u8)
        }
        #[doc = "Capture compare channel comparison base."]
        #[inline(always)]
        pub const fn set_compbase(&mut self, val: super::vals::Cc0CtrlCompbase) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Input Capture Edge Select."]
        #[must_use]
        #[inline(always)]
        pub const fn icedge(&self) -> super::vals::Cc0CtrlIcedge {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cc0CtrlIcedge::from_bits(val as u8)
        }
        #[doc = "Input Capture Edge Select."]
        #[inline(always)]
        pub const fn set_icedge(&mut self, val: super::vals::Cc0CtrlIcedge) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Cc0Ctrl {
        #[inline(always)]
        fn default() -> Cc0Ctrl {
            Cc0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Cc0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc0Ctrl")
                .field("mode", &self.mode())
                .field("cmoa", &self.cmoa())
                .field("compbase", &self.compbase())
                .field("icedge", &self.icedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cc0Ctrl {{ mode: {:?}, cmoa: {:?}, compbase: {:?}, icedge: {:?} }}",
                self.mode(),
                self.cmoa(),
                self.compbase(),
                self.icedge()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc0Icvalue(pub u32);
    impl Cc0Icvalue {
        #[doc = "Input Capture Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ic(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input Capture Value."]
        #[inline(always)]
        pub const fn set_ic(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc0Icvalue {
        #[inline(always)]
        fn default() -> Cc0Icvalue {
            Cc0Icvalue(0)
        }
    }
    impl core::fmt::Debug for Cc0Icvalue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc0Icvalue").field("ic", &self.ic()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc0Icvalue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc0Icvalue {{ ic: {=u32:?} }}", self.ic())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc0Ocvalue(pub u32);
    impl Cc0Ocvalue {
        #[doc = "Output Compare Value."]
        #[must_use]
        #[inline(always)]
        pub const fn oc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output Compare Value."]
        #[inline(always)]
        pub const fn set_oc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc0Ocvalue {
        #[inline(always)]
        fn default() -> Cc0Ocvalue {
            Cc0Ocvalue(0)
        }
    }
    impl core::fmt::Debug for Cc0Ocvalue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc0Ocvalue").field("oc", &self.oc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc0Ocvalue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc0Ocvalue {{ oc: {=u32:?} }}", self.oc())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc1Ctrl(pub u32);
    impl Cc1Ctrl {
        #[doc = "CC Channel Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Cc1CtrlMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Cc1CtrlMode::from_bits(val as u8)
        }
        #[doc = "CC Channel Mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Cc1CtrlMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Compare Match Output Action."]
        #[must_use]
        #[inline(always)]
        pub const fn cmoa(&self) -> super::vals::Cc1CtrlCmoa {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Cc1CtrlCmoa::from_bits(val as u8)
        }
        #[doc = "Compare Match Output Action."]
        #[inline(always)]
        pub const fn set_cmoa(&mut self, val: super::vals::Cc1CtrlCmoa) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Capture compare channel comparison base."]
        #[must_use]
        #[inline(always)]
        pub const fn compbase(&self) -> super::vals::Cc1CtrlCompbase {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Cc1CtrlCompbase::from_bits(val as u8)
        }
        #[doc = "Capture compare channel comparison base."]
        #[inline(always)]
        pub const fn set_compbase(&mut self, val: super::vals::Cc1CtrlCompbase) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Input Capture Edge Select."]
        #[must_use]
        #[inline(always)]
        pub const fn icedge(&self) -> super::vals::Cc1CtrlIcedge {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cc1CtrlIcedge::from_bits(val as u8)
        }
        #[doc = "Input Capture Edge Select."]
        #[inline(always)]
        pub const fn set_icedge(&mut self, val: super::vals::Cc1CtrlIcedge) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
    }
    impl Default for Cc1Ctrl {
        #[inline(always)]
        fn default() -> Cc1Ctrl {
            Cc1Ctrl(0)
        }
    }
    impl core::fmt::Debug for Cc1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc1Ctrl")
                .field("mode", &self.mode())
                .field("cmoa", &self.cmoa())
                .field("compbase", &self.compbase())
                .field("icedge", &self.icedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc1Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cc1Ctrl {{ mode: {:?}, cmoa: {:?}, compbase: {:?}, icedge: {:?} }}",
                self.mode(),
                self.cmoa(),
                self.compbase(),
                self.icedge()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc1Icvalue(pub u32);
    impl Cc1Icvalue {
        #[doc = "Input Capture Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ic(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input Capture Value."]
        #[inline(always)]
        pub const fn set_ic(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc1Icvalue {
        #[inline(always)]
        fn default() -> Cc1Icvalue {
            Cc1Icvalue(0)
        }
    }
    impl core::fmt::Debug for Cc1Icvalue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc1Icvalue").field("ic", &self.ic()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc1Icvalue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc1Icvalue {{ ic: {=u32:?} }}", self.ic())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc1Ocvalue(pub u32);
    impl Cc1Ocvalue {
        #[doc = "Output Compare Value."]
        #[must_use]
        #[inline(always)]
        pub const fn oc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output Compare Value."]
        #[inline(always)]
        pub const fn set_oc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc1Ocvalue {
        #[inline(always)]
        fn default() -> Cc1Ocvalue {
            Cc1Ocvalue(0)
        }
    }
    impl core::fmt::Debug for Cc1Ocvalue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc1Ocvalue").field("oc", &self.oc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc1Ocvalue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc1Ocvalue {{ oc: {=u32:?} }}", self.oc())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Debug Mode Run Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn debugrun(&self) -> super::vals::Debugrun {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Debugrun::from_bits(val as u8)
        }
        #[doc = "Debug Mode Run Enable."]
        #[inline(always)]
        pub const fn set_debugrun(&mut self, val: super::vals::Debugrun) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Pre-counter CCV0 top value enable."]
        #[must_use]
        #[inline(always)]
        pub const fn precntccv0top(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Pre-counter CCV0 top value enable."]
        #[inline(always)]
        pub const fn set_precntccv0top(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CCV1 top value enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cntccv1top(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CCV1 top value enable."]
        #[inline(always)]
        pub const fn set_cntccv1top(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Counter prescaler mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cnttick(&self) -> super::vals::Cnttick {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Cnttick::from_bits(val as u8)
        }
        #[doc = "Counter prescaler mode."]
        #[inline(always)]
        pub const fn set_cnttick(&mut self, val: super::vals::Cnttick) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Counter prescaler value."]
        #[must_use]
        #[inline(always)]
        pub const fn cntpresc(&self) -> super::vals::Cntpresc {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Cntpresc::from_bits(val as u8)
        }
        #[doc = "Counter prescaler value."]
        #[inline(always)]
        pub const fn set_cntpresc(&mut self, val: super::vals::Cntpresc) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
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
                .field("debugrun", &self.debugrun())
                .field("precntccv0top", &self.precntccv0top())
                .field("cntccv1top", &self.cntccv1top())
                .field("cnttick", &self.cnttick())
                .field("cntpresc", &self.cntpresc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ debugrun: {:?}, precntccv0top: {=bool:?}, cntccv1top: {=bool:?}, cnttick: {:?}, cntpresc: {:?} }}",
                self.debugrun(),
                self.precntccv0top(),
                self.cntccv1top(),
                self.cnttick(),
                self.cntpresc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Start RTCC main counter."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start RTCC main counter."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Stop RTCC main counter."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Stop RTCC main counter."]
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
    pub struct Combcnt(pub u32);
    impl Combcnt {
        #[doc = "Pre-Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn precnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Pre-Counter Value."]
        #[inline(always)]
        pub const fn set_precnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn cntlsb(&self) -> u32 {
            let val = (self.0 >> 15usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Counter Value."]
        #[inline(always)]
        pub const fn set_cntlsb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
        }
    }
    impl Default for Combcnt {
        #[inline(always)]
        fn default() -> Combcnt {
            Combcnt(0)
        }
    }
    impl core::fmt::Debug for Combcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Combcnt")
                .field("precnt", &self.precnt())
                .field("cntlsb", &self.cntlsb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Combcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Combcnt {{ precnt: {=u16:?}, cntlsb: {=u32:?} }}",
                self.precnt(),
                self.cntlsb()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "RTCC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTCC Enable."]
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
        #[doc = "OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CNTTICK Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cnttick(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CNTTICK Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cnttick(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC Channel n Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CC Channel n Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CC Channel n Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CC Channel n Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("of", &self.of())
                .field("cnttick", &self.cnttick())
                .field("cc0", &self.cc0())
                .field("cc1", &self.cc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ of: {=bool:?}, cnttick: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?} }}",
                self.of(),
                self.cnttick(),
                self.cc0(),
                self.cc1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn of(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Main counter tick."]
        #[must_use]
        #[inline(always)]
        pub const fn cnttick(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Main counter tick."]
        #[inline(always)]
        pub const fn set_cnttick(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC Channel n Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CC Channel n Interrupt Flag."]
        #[inline(always)]
        pub const fn set_cc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CC Channel n Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CC Channel n Interrupt Flag."]
        #[inline(always)]
        pub const fn set_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("of", &self.of())
                .field("cnttick", &self.cnttick())
                .field("cc0", &self.cc0())
                .field("cc1", &self.cc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ of: {=bool:?}, cnttick: {=bool:?}, cc0: {=bool:?}, cc1: {=bool:?} }}",
                self.of(),
                self.cnttick(),
                self.cc0(),
                self.cc1()
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
    pub struct Precnt(pub u32);
    impl Precnt {
        #[doc = "Pre-Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn precnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Pre-Counter Value."]
        #[inline(always)]
        pub const fn set_precnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for Precnt {
        #[inline(always)]
        fn default() -> Precnt {
            Precnt(0)
        }
    }
    impl core::fmt::Debug for Precnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Precnt").field("precnt", &self.precnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Precnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Precnt {{ precnt: {=u16:?} }}", self.precnt())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "RTCC running status."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTCC running status."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcclockstatus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Lock Status."]
        #[inline(always)]
        pub const fn set_rtcclockstatus(&mut self, val: bool) {
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
                .field("rtcclockstatus", &self.rtcclockstatus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ running: {=bool:?}, rtcclockstatus: {=bool:?} }}",
                self.running(),
                self.rtcclockstatus()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncbusy(pub u32);
    impl Syncbusy {
        #[doc = "Sync busy for START."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for START."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sync busy for STOP."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for STOP."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sync busy for PRECNT."]
        #[must_use]
        #[inline(always)]
        pub const fn precnt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for PRECNT."]
        #[inline(always)]
        pub const fn set_precnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Sync busy for CNT."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Sync busy for CNT."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("precnt", &self.precnt())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syncbusy {{ start: {=bool:?}, stop: {=bool:?}, precnt: {=bool:?}, cnt: {=bool:?} }}",
                self.start(),
                self.stop(),
                self.precnt(),
                self.cnt()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc0CtrlCmoa {
        #[doc = "A single clock cycle pulse is generated on output."]
        Pulse = 0x0,
        #[doc = "Toggle output on compare match."]
        Toggle = 0x01,
        #[doc = "Clear output on compare match."]
        Clear = 0x02,
        #[doc = "Set output on compare match."]
        Set = 0x03,
    }
    impl Cc0CtrlCmoa {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc0CtrlCmoa {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc0CtrlCmoa {
        #[inline(always)]
        fn from(val: u8) -> Cc0CtrlCmoa {
            Cc0CtrlCmoa::from_bits(val)
        }
    }
    impl From<Cc0CtrlCmoa> for u8 {
        #[inline(always)]
        fn from(val: Cc0CtrlCmoa) -> u8 {
            Cc0CtrlCmoa::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc0CtrlCompbase {
        #[doc = "RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
        Cnt = 0x0,
        #[doc = "Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
        Precnt = 0x01,
    }
    impl Cc0CtrlCompbase {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc0CtrlCompbase {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc0CtrlCompbase {
        #[inline(always)]
        fn from(val: u8) -> Cc0CtrlCompbase {
            Cc0CtrlCompbase::from_bits(val)
        }
    }
    impl From<Cc0CtrlCompbase> for u8 {
        #[inline(always)]
        fn from(val: Cc0CtrlCompbase) -> u8 {
            Cc0CtrlCompbase::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc0CtrlIcedge {
        #[doc = "Rising edges detected."]
        Rising = 0x0,
        #[doc = "Falling edges detected."]
        Falling = 0x01,
        #[doc = "Both edges detected."]
        Both = 0x02,
        #[doc = "No edge detection, signal is left as it is."]
        None = 0x03,
    }
    impl Cc0CtrlIcedge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc0CtrlIcedge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc0CtrlIcedge {
        #[inline(always)]
        fn from(val: u8) -> Cc0CtrlIcedge {
            Cc0CtrlIcedge::from_bits(val)
        }
    }
    impl From<Cc0CtrlIcedge> for u8 {
        #[inline(always)]
        fn from(val: Cc0CtrlIcedge) -> u8 {
            Cc0CtrlIcedge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc0CtrlMode {
        #[doc = "Compare/Capture channel turned off."]
        Off = 0x0,
        #[doc = "Input capture."]
        Inputcapture = 0x01,
        #[doc = "Output compare."]
        Outputcompare = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cc0CtrlMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc0CtrlMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc0CtrlMode {
        #[inline(always)]
        fn from(val: u8) -> Cc0CtrlMode {
            Cc0CtrlMode::from_bits(val)
        }
    }
    impl From<Cc0CtrlMode> for u8 {
        #[inline(always)]
        fn from(val: Cc0CtrlMode) -> u8 {
            Cc0CtrlMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc1CtrlCmoa {
        #[doc = "A single clock cycle pulse is generated on output."]
        Pulse = 0x0,
        #[doc = "Toggle output on compare match."]
        Toggle = 0x01,
        #[doc = "Clear output on compare match."]
        Clear = 0x02,
        #[doc = "Set output on compare match."]
        Set = 0x03,
    }
    impl Cc1CtrlCmoa {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc1CtrlCmoa {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc1CtrlCmoa {
        #[inline(always)]
        fn from(val: u8) -> Cc1CtrlCmoa {
            Cc1CtrlCmoa::from_bits(val)
        }
    }
    impl From<Cc1CtrlCmoa> for u8 {
        #[inline(always)]
        fn from(val: Cc1CtrlCmoa) -> u8 {
            Cc1CtrlCmoa::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc1CtrlCompbase {
        #[doc = "RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
        Cnt = 0x0,
        #[doc = "Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
        Precnt = 0x01,
    }
    impl Cc1CtrlCompbase {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc1CtrlCompbase {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc1CtrlCompbase {
        #[inline(always)]
        fn from(val: u8) -> Cc1CtrlCompbase {
            Cc1CtrlCompbase::from_bits(val)
        }
    }
    impl From<Cc1CtrlCompbase> for u8 {
        #[inline(always)]
        fn from(val: Cc1CtrlCompbase) -> u8 {
            Cc1CtrlCompbase::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc1CtrlIcedge {
        #[doc = "Rising edges detected."]
        Rising = 0x0,
        #[doc = "Falling edges detected."]
        Falling = 0x01,
        #[doc = "Both edges detected."]
        Both = 0x02,
        #[doc = "No edge detection, signal is left as it is."]
        None = 0x03,
    }
    impl Cc1CtrlIcedge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc1CtrlIcedge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc1CtrlIcedge {
        #[inline(always)]
        fn from(val: u8) -> Cc1CtrlIcedge {
            Cc1CtrlIcedge::from_bits(val)
        }
    }
    impl From<Cc1CtrlIcedge> for u8 {
        #[inline(always)]
        fn from(val: Cc1CtrlIcedge) -> u8 {
            Cc1CtrlIcedge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cc1CtrlMode {
        #[doc = "Compare/Capture channel turned off."]
        Off = 0x0,
        #[doc = "Input capture."]
        Inputcapture = 0x01,
        #[doc = "Output compare."]
        Outputcompare = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cc1CtrlMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cc1CtrlMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cc1CtrlMode {
        #[inline(always)]
        fn from(val: u8) -> Cc1CtrlMode {
            Cc1CtrlMode::from_bits(val)
        }
    }
    impl From<Cc1CtrlMode> for u8 {
        #[inline(always)]
        fn from(val: Cc1CtrlMode) -> u8 {
            Cc1CtrlMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cntpresc {
        #[doc = "CLK_CNT = (RTCC LF CLK)/1."]
        Div1 = 0x0,
        #[doc = "CLK_CNT = (RTCC LF CLK)/2."]
        Div2 = 0x01,
        #[doc = "CLK_CNT = (RTCC LF CLK)/4."]
        Div4 = 0x02,
        #[doc = "CLK_CNT = (RTCC LF CLK)/8."]
        Div8 = 0x03,
        #[doc = "CLK_CNT = (RTCC LF CLK)/16."]
        Div16 = 0x04,
        #[doc = "CLK_CNT = (RTCC LF CLK)/32."]
        Div32 = 0x05,
        #[doc = "CLK_CNT = (RTCC LF CLK)/64."]
        Div64 = 0x06,
        #[doc = "CLK_CNT = (RTCC LF CLK)/128."]
        Div128 = 0x07,
        #[doc = "CLK_CNT = (RTCC LF CLK)/256."]
        Div256 = 0x08,
        #[doc = "CLK_CNT = (RTCC LF CLK)/512."]
        Div512 = 0x09,
        #[doc = "CLK_CNT = (RTCC LF CLK)/1024."]
        Div1024 = 0x0a,
        #[doc = "CLK_CNT = (RTCC LF CLK)/2048."]
        Div2048 = 0x0b,
        #[doc = "CLK_CNT = (RTCC LF CLK)/4096."]
        Div4096 = 0x0c,
        #[doc = "CLK_CNT = (RTCC LF CLK)/8192."]
        Div8192 = 0x0d,
        #[doc = "CLK_CNT = (RTCC LF CLK)/16384."]
        Div16384 = 0x0e,
        #[doc = "CLK_CNT = (RTCC LF CLK)/32768."]
        Div32768 = 0x0f,
    }
    impl Cntpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cntpresc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cntpresc {
        #[inline(always)]
        fn from(val: u8) -> Cntpresc {
            Cntpresc::from_bits(val)
        }
    }
    impl From<Cntpresc> for u8 {
        #[inline(always)]
        fn from(val: Cntpresc) -> u8 {
            Cntpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cnttick {
        #[doc = "CNT register ticks according to configuration in CNTPRESC."]
        Presc = 0x0,
        #[doc = "CNT register ticks when PRECNT matches RTCC_CC0_OC\\[14:0\\]."]
        Ccv0match = 0x01,
    }
    impl Cnttick {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cnttick {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cnttick {
        #[inline(always)]
        fn from(val: u8) -> Cnttick {
            Cnttick::from_bits(val)
        }
    }
    impl From<Cnttick> for u8 {
        #[inline(always)]
        fn from(val: Cnttick) -> u8 {
            Cnttick::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Debugrun {
        #[doc = "RTCC is frozen in debug mode."]
        X0 = 0x0,
        #[doc = "RTCC is running in debug mode."]
        X1 = 0x01,
    }
    impl Debugrun {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Debugrun {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Debugrun {
        #[inline(always)]
        fn from(val: u8) -> Debugrun {
            Debugrun::from_bits(val)
        }
    }
    impl From<Debugrun> for u8 {
        #[inline(always)]
        fn from(val: Debugrun) -> u8 {
            Debugrun::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Write to unlock RTCC lockable registers."]
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
}
