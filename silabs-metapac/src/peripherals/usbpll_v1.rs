#[doc = "USBPLL peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbpll {
    ptr: *mut u8,
}
unsafe impl Send for Usbpll {}
unsafe impl Sync for Usbpll {}
impl Usbpll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IPVERSION."]
    #[inline(always)]
    pub const fn ipversion(self) -> crate::common::Reg<regs::Ipversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DAC Controlled Oscillator."]
    #[inline(always)]
    pub const fn dcoctrl(self) -> crate::common::Reg<regs::Dcoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Lock PLL from APBIF access."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt flag."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "DAC Controlled Oscillator. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dcoctrl_set(self) -> crate::common::Reg<regs::Dcoctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Lock PLL from APBIF access. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "Interrupt flag. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "Interrupt enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "Control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "DAC Controlled Oscillator. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dcoctrl_clr(self) -> crate::common::Reg<regs::Dcoctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "Lock PLL from APBIF access. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "Interrupt flag. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "Interrupt enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "Control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "DAC Controlled Oscillator. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dcoctrl_tgl(self) -> crate::common::Reg<regs::Dcoctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "Lock PLL from APBIF access. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "Interrupt flag. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "Interrupt enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control."]
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
        #[doc = "Disable on Demand."]
        #[must_use]
        #[inline(always)]
        pub const fn disondemand(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Disable on Demand."]
        #[inline(always)]
        pub const fn set_disondemand(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Shunt Regulator LP Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn shuntreglpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Shunt Regulator LP Enable."]
        #[inline(always)]
        pub const fn set_shuntreglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Divider R."]
        #[must_use]
        #[inline(always)]
        pub const fn divr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Divider R."]
        #[inline(always)]
        pub const fn set_divr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Divider X."]
        #[must_use]
        #[inline(always)]
        pub const fn divx(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x1f;
            val as u8
        }
        #[doc = "Divider X."]
        #[inline(always)]
        pub const fn set_divx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
        }
        #[doc = "Divider N."]
        #[must_use]
        #[inline(always)]
        pub const fn divn(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x7f;
            val as u8
        }
        #[doc = "Divider N."]
        #[inline(always)]
        pub const fn set_divn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
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
                .field("shuntreglpen", &self.shuntreglpen())
                .field("divr", &self.divr())
                .field("divx", &self.divx())
                .field("divn", &self.divn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ forceen: {=bool:?}, disondemand: {=bool:?}, shuntreglpen: {=bool:?}, divr: {=u8:?}, divx: {=u8:?}, divn: {=u8:?} }}",
                self.forceen(),
                self.disondemand(),
                self.shuntreglpen(),
                self.divr(),
                self.divx(),
                self.divn()
            )
        }
    }
    #[doc = "DAC Controlled Oscillator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcoctrl(pub u32);
    impl Dcoctrl {
        #[doc = "DCO Half bias."]
        #[must_use]
        #[inline(always)]
        pub const fn dcobiashalf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DCO Half bias."]
        #[inline(always)]
        pub const fn set_dcobiashalf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Dcoctrl {
        #[inline(always)]
        fn default() -> Dcoctrl {
            Dcoctrl(0)
        }
    }
    impl core::fmt::Debug for Dcoctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcoctrl")
                .field("dcobiashalf", &self.dcobiashalf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcoctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcoctrl {{ dcobiashalf: {=bool:?} }}", self.dcobiashalf())
        }
    }
    #[doc = "Interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "USBPLL Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USBPLL Lock Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn plllockien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Lock Interrupt Enable."]
        #[inline(always)]
        pub const fn set_plllockien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USBPLL Loss of Lock Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllnolockien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Loss of Lock Interrupt Enable."]
        #[inline(always)]
        pub const fn set_pllnolockien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("pllrdy", &self.pllrdy())
                .field("plllockien", &self.plllockien())
                .field("pllnolockien", &self.pllnolockien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ pllrdy: {=bool:?}, plllockien: {=bool:?}, pllnolockien: {=bool:?} }}",
                self.pllrdy(),
                self.plllockien(),
                self.pllnolockien()
            )
        }
    }
    #[doc = "Interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "USBPLL Ready Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Ready Interrupt Flag."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USBPLL Lock Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn plllockif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Lock Interrupt Flag."]
        #[inline(always)]
        pub const fn set_plllockif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USBPLL Loss of Lock Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pllnolockif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Loss of Lock Interrupt Flag."]
        #[inline(always)]
        pub const fn set_pllnolockif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("pllrdy", &self.pllrdy())
                .field("plllockif", &self.plllockif())
                .field("pllnolockif", &self.pllnolockif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ pllrdy: {=bool:?}, plllockif: {=bool:?}, pllnolockif: {=bool:?} }}",
                self.pllrdy(),
                self.plllockif(),
                self.pllnolockif()
            )
        }
    }
    #[doc = "IPVERSION."]
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
    #[doc = "Lock PLL from APBIF access."]
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
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "USBPLL Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL Ready."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USBPLL is locked."]
        #[must_use]
        #[inline(always)]
        pub const fn plllock(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL is locked."]
        #[inline(always)]
        pub const fn set_plllock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Sync Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn syncbusy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Busy."]
        #[inline(always)]
        pub const fn set_syncbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Locks out registers."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Locks out registers."]
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
                .field("pllrdy", &self.pllrdy())
                .field("plllock", &self.plllock())
                .field("syncbusy", &self.syncbusy())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ pllrdy: {=bool:?}, plllock: {=bool:?}, syncbusy: {=bool:?}, lock: {=bool:?} }}",
                self.pllrdy(),
                self.plllock(),
                self.syncbusy(),
                self.lock()
            )
        }
    }
}
pub mod vals {
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
}
