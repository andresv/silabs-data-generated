#[doc = "DMEM peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmem {
    ptr: *mut u8,
}
unsafe impl Send for Dmem {}
unsafe impl Sync for Dmem {}
impl Dmem {
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
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn eccerraddr0(self) -> crate::common::Reg<regs::Eccerraddr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn eccerraddr1(self) -> crate::common::Reg<regs::Eccerraddr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn eccmerrind(self) -> crate::common::Reg<regs::Eccmerrind, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Clear ECCERRADDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn cleareccaddr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECCERRADDR0."]
        #[inline(always)]
        pub const fn set_cleareccaddr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECCERRADDR1."]
        #[must_use]
        #[inline(always)]
        pub const fn cleareccaddr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECCERRADDR1."]
        #[inline(always)]
        pub const fn set_cleareccaddr1(&mut self, val: bool) {
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
                .field("cleareccaddr0", &self.cleareccaddr0())
                .field("cleareccaddr1", &self.cleareccaddr1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ cleareccaddr0: {=bool:?}, cleareccaddr1: {=bool:?} }}",
                self.cleareccaddr0(),
                self.cleareccaddr1()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Enable ECC functionality."]
        #[must_use]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable ECC functionality."]
        #[inline(always)]
        pub const fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable ECC syndrome writes."]
        #[must_use]
        #[inline(always)]
        pub const fn eccwen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable ECC syndrome writes."]
        #[inline(always)]
        pub const fn set_eccwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ECC Error bus fault enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerrfaulten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Error bus fault enable."]
        #[inline(always)]
        pub const fn set_eccerrfaulten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AHB port arbitration priority."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbportpriority(&self) -> super::vals::Ahbportpriority {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Ahbportpriority::from_bits(val as u8)
        }
        #[doc = "AHB port arbitration priority."]
        #[inline(always)]
        pub const fn set_ahbportpriority(&mut self, val: super::vals::Ahbportpriority) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "Address fault bus fault enable."]
        #[must_use]
        #[inline(always)]
        pub const fn addrfaulten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Address fault bus fault enable."]
        #[inline(always)]
        pub const fn set_addrfaulten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("eccen", &self.eccen())
                .field("eccwen", &self.eccwen())
                .field("eccerrfaulten", &self.eccerrfaulten())
                .field("ahbportpriority", &self.ahbportpriority())
                .field("addrfaulten", &self.addrfaulten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ eccen: {=bool:?}, eccwen: {=bool:?}, eccerrfaulten: {=bool:?}, ahbportpriority: {:?}, addrfaulten: {=bool:?} }}",
                self.eccen(),
                self.eccwen(),
                self.eccerrfaulten(),
                self.ahbportpriority(),
                self.addrfaulten()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccerraddr0(pub u32);
    impl Eccerraddr0 {
        #[doc = "ECC Error Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC Error Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eccerraddr0 {
        #[inline(always)]
        fn default() -> Eccerraddr0 {
            Eccerraddr0(0)
        }
    }
    impl core::fmt::Debug for Eccerraddr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccerraddr0").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccerraddr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccerraddr0 {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccerraddr1(pub u32);
    impl Eccerraddr1 {
        #[doc = "ECC Error Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC Error Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eccerraddr1 {
        #[inline(always)]
        fn default() -> Eccerraddr1 {
            Eccerraddr1(0)
        }
    }
    impl core::fmt::Debug for Eccerraddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccerraddr1").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccerraddr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccerraddr1 {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccmerrind(pub u32);
    impl Eccmerrind {
        #[doc = "Multiple ECC errors on AHB port 0."]
        #[must_use]
        #[inline(always)]
        pub const fn p0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multiple ECC errors on AHB port 0."]
        #[inline(always)]
        pub const fn set_p0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Multiple ECC errors on AHB port 1."]
        #[must_use]
        #[inline(always)]
        pub const fn p1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Multiple ECC errors on AHB port 1."]
        #[inline(always)]
        pub const fn set_p1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Eccmerrind {
        #[inline(always)]
        fn default() -> Eccmerrind {
            Eccmerrind(0)
        }
    }
    impl core::fmt::Debug for Eccmerrind {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccmerrind")
                .field("p0", &self.p0())
                .field("p1", &self.p1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccmerrind {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccmerrind {{ p0: {=bool:?}, p1: {=bool:?} }}", self.p0(), self.p1())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "AHB0 1-bit ECC Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb0err1b(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AHB0 1-bit ECC Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ahb0err1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AHB1 1-bit ECC Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb1err1b(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 1-bit ECC Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ahb1err1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AHB0 2-bit ECC Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb0err2b(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AHB0 2-bit ECC Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ahb0err2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AHB1 2-bit ECC Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb1err2b(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 2-bit ECC Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ahb1err2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("ahb0err1b", &self.ahb0err1b())
                .field("ahb1err1b", &self.ahb1err1b())
                .field("ahb0err2b", &self.ahb0err2b())
                .field("ahb1err2b", &self.ahb1err2b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ ahb0err1b: {=bool:?}, ahb1err1b: {=bool:?}, ahb0err2b: {=bool:?}, ahb1err2b: {=bool:?} }}",
                self.ahb0err1b(),
                self.ahb1err1b(),
                self.ahb0err2b(),
                self.ahb1err2b()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "AHB0 1-bit ECC Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb0err1b(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AHB0 1-bit ECC Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ahb0err1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AHB1 1-bit ECC Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb1err1b(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 1-bit ECC Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ahb1err1b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AHB0 2-bit ECC Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb0err2b(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AHB0 2-bit ECC Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ahb0err2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AHB1 2-bit ECC Error Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb1err2b(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 2-bit ECC Error Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ahb1err2b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("ahb0err1b", &self.ahb0err1b())
                .field("ahb1err1b", &self.ahb1err1b())
                .field("ahb0err2b", &self.ahb0err2b())
                .field("ahb1err2b", &self.ahb1err2b())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ ahb0err1b: {=bool:?}, ahb1err1b: {=bool:?}, ahb0err2b: {=bool:?}, ahb1err2b: {=bool:?} }}",
                self.ahb0err1b(),
                self.ahb1err1b(),
                self.ahb0err2b(),
                self.ahb1err2b()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_ipversion(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
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
            defmt::write!(f, "Ipversion {{ ipversion: {=u8:?} }}", self.ipversion())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ahbportpriority {
        #[doc = "No AHB port have raised priority."]
        None = 0x0,
        #[doc = "AHB port 0 has raised priority."]
        Port0 = 0x01,
        #[doc = "AHB port 1 has raised priority."]
        Port1 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ahbportpriority {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ahbportpriority {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ahbportpriority {
        #[inline(always)]
        fn from(val: u8) -> Ahbportpriority {
            Ahbportpriority::from_bits(val)
        }
    }
    impl From<Ahbportpriority> for u8 {
        #[inline(always)]
        fn from(val: Ahbportpriority) -> u8 {
            Ahbportpriority::to_bits(val)
        }
    }
}
