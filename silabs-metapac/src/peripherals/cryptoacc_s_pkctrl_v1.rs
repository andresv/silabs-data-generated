#[doc = "CRYPTOACC_S_PKCTRL peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoaccSPkctrl {
    ptr: *mut u8,
}
unsafe impl Send for CryptoaccSPkctrl {}
unsafe impl Sync for CryptoaccSPkctrl {}
impl CryptoaccSPkctrl {
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
    pub const fn pointer(self) -> crate::common::Reg<regs::Pointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn command(self) -> crate::common::Reg<regs::Command, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pkctrl(self) -> crate::common::Reg<regs::Pkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pkstatus(self) -> crate::common::Reg<regs::Pkstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Command(pub u32);
    impl Command {
        #[doc = "Type of Operation."]
        #[must_use]
        #[inline(always)]
        pub const fn operation(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Type of Operation."]
        #[inline(always)]
        pub const fn set_operation(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Field."]
        #[must_use]
        #[inline(always)]
        pub const fn field(&self) -> super::vals::Field {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Field::from_bits(val as u8)
        }
        #[doc = "Field."]
        #[inline(always)]
        pub const fn set_field(&mut self, val: super::vals::Field) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Size of Operands in data memory."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x07ff;
            val as u16
        }
        #[doc = "Size of Operands in data memory."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 8usize)) | (((val as u32) & 0x07ff) << 8usize);
        }
        #[doc = "Select Curve."]
        #[must_use]
        #[inline(always)]
        pub const fn selcurve(&self) -> super::vals::Selcurve {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Selcurve::from_bits(val as u8)
        }
        #[doc = "Select Curve."]
        #[inline(always)]
        pub const fn set_selcurve(&mut self, val: super::vals::Selcurve) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "Edwards Curve Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn edwards(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Edwards Curve Enable."]
        #[inline(always)]
        pub const fn set_edwards(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Buffer Select."]
        #[must_use]
        #[inline(always)]
        pub const fn bufsel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Select."]
        #[inline(always)]
        pub const fn set_bufsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Swap bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn swapbytes(&self) -> super::vals::Swapbytes {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Swapbytes::from_bits(val as u8)
        }
        #[doc = "Swap bytes."]
        #[inline(always)]
        pub const fn set_swapbytes(&mut self, val: super::vals::Swapbytes) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Flag A."]
        #[must_use]
        #[inline(always)]
        pub const fn flaga(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flag A."]
        #[inline(always)]
        pub const fn set_flaga(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Flag B."]
        #[must_use]
        #[inline(always)]
        pub const fn flagb(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Flag B."]
        #[inline(always)]
        pub const fn set_flagb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Calculate R2."]
        #[must_use]
        #[inline(always)]
        pub const fn calcr2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Calculate R2."]
        #[inline(always)]
        pub const fn set_calcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Command {
        #[inline(always)]
        fn default() -> Command {
            Command(0)
        }
    }
    impl core::fmt::Debug for Command {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Command")
                .field("operation", &self.operation())
                .field("field", &self.field())
                .field("size", &self.size())
                .field("selcurve", &self.selcurve())
                .field("edwards", &self.edwards())
                .field("bufsel", &self.bufsel())
                .field("swapbytes", &self.swapbytes())
                .field("flaga", &self.flaga())
                .field("flagb", &self.flagb())
                .field("calcr2", &self.calcr2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Command {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Command {{ operation: {=u8:?}, field: {:?}, size: {=u16:?}, selcurve: {:?}, edwards: {=bool:?}, bufsel: {=bool:?}, swapbytes: {:?}, flaga: {=bool:?}, flagb: {=bool:?}, calcr2: {=bool:?} }}",
                self.operation(),
                self.field(),
                self.size(),
                self.selcurve(),
                self.edwards(),
                self.bufsel(),
                self.swapbytes(),
                self.flaga(),
                self.flagb(),
                self.calcr2()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pkctrl(pub u32);
    impl Pkctrl {
        #[doc = "PK Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pkstart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PK Start."]
        #[inline(always)]
        pub const fn set_pkstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ClearIRQ."]
        #[must_use]
        #[inline(always)]
        pub const fn ifc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ClearIRQ."]
        #[inline(always)]
        pub const fn set_ifc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Pkctrl {
        #[inline(always)]
        fn default() -> Pkctrl {
            Pkctrl(0)
        }
    }
    impl core::fmt::Debug for Pkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pkctrl")
                .field("pkstart", &self.pkstart())
                .field("ifc", &self.ifc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pkctrl {{ pkstart: {=bool:?}, ifc: {=bool:?} }}",
                self.pkstart(),
                self.ifc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pkstatus(pub u32);
    impl Pkstatus {
        #[doc = "Fail Address."]
        #[must_use]
        #[inline(always)]
        pub const fn failaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Fail Address."]
        #[inline(always)]
        pub const fn set_failaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Point Px not on curve."]
        #[must_use]
        #[inline(always)]
        pub const fn notoncurve(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Point Px not on curve."]
        #[inline(always)]
        pub const fn set_notoncurve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Point Px at infinity."]
        #[must_use]
        #[inline(always)]
        pub const fn atinfinity(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Point Px at infinity."]
        #[inline(always)]
        pub const fn set_atinfinity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Couple not valid."]
        #[must_use]
        #[inline(always)]
        pub const fn couplenotvalid(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Couple not valid."]
        #[inline(always)]
        pub const fn set_couplenotvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Param n not valid."]
        #[must_use]
        #[inline(always)]
        pub const fn paramnnotvalid(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Param n not valid."]
        #[inline(always)]
        pub const fn set_paramnnotvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Not implemented."]
        #[must_use]
        #[inline(always)]
        pub const fn notimplemented(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Not implemented."]
        #[inline(always)]
        pub const fn set_notimplemented(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Signature not valid."]
        #[must_use]
        #[inline(always)]
        pub const fn signotvalid(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Signature not valid."]
        #[inline(always)]
        pub const fn set_signotvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Param AB not valid."]
        #[must_use]
        #[inline(always)]
        pub const fn paramabnotvalid(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Param AB not valid."]
        #[inline(always)]
        pub const fn set_paramabnotvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Not invertible."]
        #[must_use]
        #[inline(always)]
        pub const fn notinvertible(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Not invertible."]
        #[inline(always)]
        pub const fn set_notinvertible(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Composite."]
        #[must_use]
        #[inline(always)]
        pub const fn composite(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Composite."]
        #[inline(always)]
        pub const fn set_composite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Not quadratic residue."]
        #[must_use]
        #[inline(always)]
        pub const fn notquad(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Not quadratic residue."]
        #[inline(always)]
        pub const fn set_notquad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PK busy."]
        #[must_use]
        #[inline(always)]
        pub const fn pkbusy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PK busy."]
        #[inline(always)]
        pub const fn set_pkbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn pkif(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt status."]
        #[inline(always)]
        pub const fn set_pkif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Pkstatus {
        #[inline(always)]
        fn default() -> Pkstatus {
            Pkstatus(0)
        }
    }
    impl core::fmt::Debug for Pkstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pkstatus")
                .field("failaddr", &self.failaddr())
                .field("notoncurve", &self.notoncurve())
                .field("atinfinity", &self.atinfinity())
                .field("couplenotvalid", &self.couplenotvalid())
                .field("paramnnotvalid", &self.paramnnotvalid())
                .field("notimplemented", &self.notimplemented())
                .field("signotvalid", &self.signotvalid())
                .field("paramabnotvalid", &self.paramabnotvalid())
                .field("notinvertible", &self.notinvertible())
                .field("composite", &self.composite())
                .field("notquad", &self.notquad())
                .field("pkbusy", &self.pkbusy())
                .field("pkif", &self.pkif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pkstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pkstatus {{ failaddr: {=u8:?}, notoncurve: {=bool:?}, atinfinity: {=bool:?}, couplenotvalid: {=bool:?}, paramnnotvalid: {=bool:?}, notimplemented: {=bool:?}, signotvalid: {=bool:?}, paramabnotvalid: {=bool:?}, notinvertible: {=bool:?}, composite: {=bool:?}, notquad: {=bool:?}, pkbusy: {=bool:?}, pkif: {=bool:?} }}",
                self.failaddr(),
                self.notoncurve(),
                self.atinfinity(),
                self.couplenotvalid(),
                self.paramnnotvalid(),
                self.notimplemented(),
                self.signotvalid(),
                self.paramabnotvalid(),
                self.notinvertible(),
                self.composite(),
                self.notquad(),
                self.pkbusy(),
                self.pkif()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pointer(pub u32);
    impl Pointer {
        #[doc = "OpPtrA."]
        #[must_use]
        #[inline(always)]
        pub const fn opptra(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "OpPtrA."]
        #[inline(always)]
        pub const fn set_opptra(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "OpPtrB."]
        #[must_use]
        #[inline(always)]
        pub const fn opptrb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "OpPtrB."]
        #[inline(always)]
        pub const fn set_opptrb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "OpPtrC."]
        #[must_use]
        #[inline(always)]
        pub const fn opptrc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "OpPtrC."]
        #[inline(always)]
        pub const fn set_opptrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "OpPtrN."]
        #[must_use]
        #[inline(always)]
        pub const fn opptrn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "OpPtrN."]
        #[inline(always)]
        pub const fn set_opptrn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Pointer {
        #[inline(always)]
        fn default() -> Pointer {
            Pointer(0)
        }
    }
    impl core::fmt::Debug for Pointer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pointer")
                .field("opptra", &self.opptra())
                .field("opptrb", &self.opptrb())
                .field("opptrc", &self.opptrc())
                .field("opptrn", &self.opptrn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pointer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pointer {{ opptra: {=u8:?}, opptrb: {=u8:?}, opptrc: {=u8:?}, opptrn: {=u8:?} }}",
                self.opptra(),
                self.opptrb(),
                self.opptrc(),
                self.opptrn()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer(pub u32);
    impl Timer {
        #[doc = "Timer."]
        #[must_use]
        #[inline(always)]
        pub const fn timer(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Timer."]
        #[inline(always)]
        pub const fn set_timer(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Timer {
        #[inline(always)]
        fn default() -> Timer {
            Timer(0)
        }
    }
    impl core::fmt::Debug for Timer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer").field("timer", &self.timer()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timer {{ timer: {=u32:?} }}", self.timer())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Version(pub u32);
    impl Version {
        #[doc = "Software version number."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Software version number."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Hardware version number."]
        #[must_use]
        #[inline(always)]
        pub const fn hw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Hardware version number."]
        #[inline(always)]
        pub const fn set_hw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Version {
        #[inline(always)]
        fn default() -> Version {
            Version(0)
        }
    }
    impl core::fmt::Debug for Version {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Version")
                .field("sw", &self.sw())
                .field("hw", &self.hw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Version {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Version {{ sw: {=u8:?}, hw: {=u8:?} }}", self.sw(), self.hw())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Field {
        #[doc = "Field is GF(p)."]
        Gfp = 0x0,
        #[doc = "Field is GF(2^m)."]
        Gf2m = 0x01,
    }
    impl Field {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Field {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Field {
        #[inline(always)]
        fn from(val: u8) -> Field {
            Field::from_bits(val)
        }
    }
    impl From<Field> for u8 {
        #[inline(always)]
        fn from(val: Field) -> u8 {
            Field::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Selcurve {
        #[doc = "No acceleration."]
        None = 0x0,
        #[doc = "P256."]
        P256 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "P192."]
        P192 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Selcurve {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Selcurve {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Selcurve {
        #[inline(always)]
        fn from(val: u8) -> Selcurve {
            Selcurve::from_bits(val)
        }
    }
    impl From<Selcurve> for u8 {
        #[inline(always)]
        fn from(val: Selcurve) -> u8 {
            Selcurve::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Swapbytes {
        #[doc = "Native format (little endian)."]
        Native = 0x0,
        #[doc = "Byte swapped (big endian)."]
        Swapped = 0x01,
    }
    impl Swapbytes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Swapbytes {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Swapbytes {
        #[inline(always)]
        fn from(val: u8) -> Swapbytes {
            Swapbytes::from_bits(val)
        }
    }
    impl From<Swapbytes> for u8 {
        #[inline(always)]
        fn from(val: Swapbytes) -> u8 {
            Swapbytes::to_bits(val)
        }
    }
}
