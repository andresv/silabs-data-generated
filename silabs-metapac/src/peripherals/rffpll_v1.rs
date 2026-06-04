#[doc = "RFFPLL peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rffpll {
    ptr: *mut u8,
}
unsafe impl Send for Rffpll {}
unsafe impl Sync for Rffpll {}
impl Rffpll {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rffpllctrl1(self) -> crate::common::Reg<regs::Rffpllctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rffpllctrl1_set(self) -> crate::common::Reg<regs::Rffpllctrl1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1084usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rffpllctrl1_clr(self) -> crate::common::Reg<regs::Rffpllctrl1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2084usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rffpllctrl1_tgl(self) -> crate::common::Reg<regs::Rffpllctrl1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3084usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b0usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Force PLL Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn forceen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force PLL Enable."]
        #[inline(always)]
        pub const fn set_forceen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Disable On-demand request."]
        #[must_use]
        #[inline(always)]
        pub const fn disondemand(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable On-demand request."]
        #[inline(always)]
        pub const fn set_disondemand(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ forceen: {=bool:?}, disondemand: {=bool:?} }}",
                self.forceen(),
                self.disondemand()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Radio Output Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rffpllradiordy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Radio Output Ready Interrupt."]
        #[inline(always)]
        pub const fn set_rffpllradiordy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Digital System Output Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rffpllsysrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digital System Output Ready Interrupt."]
        #[inline(always)]
        pub const fn set_rffpllsysrdy(&mut self, val: bool) {
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
                .field("rffpllradiordy", &self.rffpllradiordy())
                .field("rffpllsysrdy", &self.rffpllsysrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ rffpllradiordy: {=bool:?}, rffpllsysrdy: {=bool:?} }}",
                self.rffpllradiordy(),
                self.rffpllsysrdy()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Radio Output Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rffpllradiordy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Radio Output Ready Interrupt."]
        #[inline(always)]
        pub const fn set_rffpllradiordy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Digital System Output Ready Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rffpllsysrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digital System Output Ready Interrupt."]
        #[inline(always)]
        pub const fn set_rffpllsysrdy(&mut self, val: bool) {
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
                .field("rffpllradiordy", &self.rffpllradiordy())
                .field("rffpllsysrdy", &self.rffpllsysrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ rffpllradiordy: {=bool:?}, rffpllsysrdy: {=bool:?} }}",
                self.rffpllradiordy(),
                self.rffpllsysrdy()
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
    pub struct Rffpllctrl1(pub u32);
    impl Rffpllctrl1 {
        #[doc = "Feedback Divider Ratio."]
        #[must_use]
        #[inline(always)]
        pub const fn divn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Feedback Divider Ratio."]
        #[inline(always)]
        pub const fn set_divn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Clock Divider Ratio for Radio."]
        #[must_use]
        #[inline(always)]
        pub const fn divx(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock Divider Ratio for Radio."]
        #[inline(always)]
        pub const fn set_divx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Clock Divider Ratio for digital."]
        #[must_use]
        #[inline(always)]
        pub const fn divy(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Clock Divider Ratio for digital."]
        #[inline(always)]
        pub const fn set_divy(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Rffpllctrl1 {
        #[inline(always)]
        fn default() -> Rffpllctrl1 {
            Rffpllctrl1(0)
        }
    }
    impl core::fmt::Debug for Rffpllctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rffpllctrl1")
                .field("divn", &self.divn())
                .field("divx", &self.divx())
                .field("divy", &self.divy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rffpllctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rffpllctrl1 {{ divn: {=u8:?}, divx: {=u8:?}, divy: {=u8:?} }}",
                self.divn(),
                self.divx(),
                self.divy()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Radio Clock Output Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn rffpllradiordy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Radio Clock Output Ready."]
        #[inline(always)]
        pub const fn set_rffpllradiordy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Digital System Clock Output Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn rffpllsysrdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Digital System Clock Output Ready."]
        #[inline(always)]
        pub const fn set_rffpllsysrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enabled Status."]
        #[must_use]
        #[inline(always)]
        pub const fn ens(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enabled Status."]
        #[inline(always)]
        pub const fn set_ens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLL Radio Output Requested."]
        #[must_use]
        #[inline(always)]
        pub const fn hwreqradio(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Radio Output Requested."]
        #[inline(always)]
        pub const fn set_hwreqradio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL Digital System Output Requested."]
        #[must_use]
        #[inline(always)]
        pub const fn hwreqsys(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Digital System Output Requested."]
        #[inline(always)]
        pub const fn set_hwreqsys(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("rffpllradiordy", &self.rffpllradiordy())
                .field("rffpllsysrdy", &self.rffpllsysrdy())
                .field("ens", &self.ens())
                .field("hwreqradio", &self.hwreqradio())
                .field("hwreqsys", &self.hwreqsys())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ rffpllradiordy: {=bool:?}, rffpllsysrdy: {=bool:?}, ens: {=bool:?}, hwreqradio: {=bool:?}, hwreqsys: {=bool:?}, lock: {=bool:?} }}",
                self.rffpllradiordy(),
                self.rffpllsysrdy(),
                self.ens(),
                self.hwreqradio(),
                self.hwreqsys(),
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
        pub const Unlock: Self = Self(0x85a9);
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
                0x85a9 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x85a9 => defmt::write!(f, "Unlock"),
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
