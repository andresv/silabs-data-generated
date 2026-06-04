#[doc = "USB_NS_APBS peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbNsApbs {
    ptr: *mut u8,
}
unsafe impl Send for UsbNsApbs {}
unsafe impl Sync for UsbNsApbs {}
impl UsbNsApbs {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cal(self) -> crate::common::Reg<regs::Cal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cal(pub u32);
    impl Cal {
        #[doc = "FS slew."]
        #[must_use]
        #[inline(always)]
        pub const fn fsslewt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "FS slew."]
        #[inline(always)]
        pub const fn set_fsslewt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Cal {
        #[inline(always)]
        fn default() -> Cal {
            Cal(0)
        }
    }
    impl core::fmt::Debug for Cal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cal").field("fsslewt", &self.fsslewt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cal {{ fsslewt: {=u8:?} }}", self.fsslewt())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "TXD SE0 chicken bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txdse0mxsel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TXD SE0 chicken bit."]
        #[inline(always)]
        pub const fn set_txdse0mxsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VBUSSENSE going high - enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vbssnshen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VBUSSENSE going high - enable."]
        #[inline(always)]
        pub const fn set_vbssnshen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VBUSSENSE going low - enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vbssnslen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VBUSSENSE going low - enable."]
        #[inline(always)]
        pub const fn set_vbssnslen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "VBUS override to GPPLL."]
        #[must_use]
        #[inline(always)]
        pub const fn vbusovride(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS override to GPPLL."]
        #[inline(always)]
        pub const fn set_vbusovride(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "FS Slew."]
        #[must_use]
        #[inline(always)]
        pub const fn fsslew(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "FS Slew."]
        #[inline(always)]
        pub const fn set_fsslew(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "Disable TX pull-up resistor."]
        #[must_use]
        #[inline(always)]
        pub const fn disruptx(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable TX pull-up resistor."]
        #[inline(always)]
        pub const fn set_disruptx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bypass VREG signal to USB PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn bypsvreg(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass VREG signal to USB PHY."]
        #[inline(always)]
        pub const fn set_bypsvreg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Add 40pf cap to D+/D- in TX mode."]
        #[must_use]
        #[inline(always)]
        pub const fn endatacap(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Add 40pf cap to D+/D- in TX mode."]
        #[inline(always)]
        pub const fn set_endatacap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("txdse0mxsel", &self.txdse0mxsel())
                .field("vbssnshen", &self.vbssnshen())
                .field("vbssnslen", &self.vbssnslen())
                .field("vbusovride", &self.vbusovride())
                .field("fsslew", &self.fsslew())
                .field("disruptx", &self.disruptx())
                .field("bypsvreg", &self.bypsvreg())
                .field("endatacap", &self.endatacap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ txdse0mxsel: {=bool:?}, vbssnshen: {=bool:?}, vbssnslen: {=bool:?}, vbusovride: {=bool:?}, fsslew: {=u8:?}, disruptx: {=bool:?}, bypsvreg: {=bool:?}, endatacap: {=bool:?} }}",
                self.txdse0mxsel(),
                self.vbssnshen(),
                self.vbssnslen(),
                self.vbusovride(),
                self.fsslew(),
                self.disruptx(),
                self.bypsvreg(),
                self.endatacap()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Module enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Module enable."]
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
        #[doc = "VBUS_SENSE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vbus(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS_SENSE interrupt enable."]
        #[inline(always)]
        pub const fn set_vbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DWC_OTG interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dwcotg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DWC_OTG interrupt enable."]
        #[inline(always)]
        pub const fn set_dwcotg(&mut self, val: bool) {
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
                .field("vbus", &self.vbus())
                .field("dwcotg", &self.dwcotg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ vbus: {=bool:?}, dwcotg: {=bool:?} }}",
                self.vbus(),
                self.dwcotg()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "VBUS_SENSE interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn vbus(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS_SENSE interrupt flag."]
        #[inline(always)]
        pub const fn set_vbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DWC_OTG interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dwcotg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DWC_OTG interrupt flag."]
        #[inline(always)]
        pub const fn set_dwcotg(&mut self, val: bool) {
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
                .field("vbus", &self.vbus())
                .field("dwcotg", &self.dwcotg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ vbus: {=bool:?}, dwcotg: {=bool:?} }}",
                self.vbus(),
                self.dwcotg()
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
        pub const fn ipversion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IP Version ID."]
        #[inline(always)]
        pub const fn set_ipversion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            defmt::write!(f, "Ipversion {{ ipversion: {=bool:?} }}", self.ipversion())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "VBus valid."]
        #[must_use]
        #[inline(always)]
        pub const fn vbusvalid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VBus valid."]
        #[inline(always)]
        pub const fn set_vbusvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Status").field("vbusvalid", &self.vbusvalid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Status {{ vbusvalid: {=bool:?} }}", self.vbusvalid())
        }
    }
}
