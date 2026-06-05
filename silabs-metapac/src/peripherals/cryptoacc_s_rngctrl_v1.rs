#[doc = "CRYPTOACC_S_RNGCTRL peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CryptoaccSRngctrl {
    ptr: *mut u8,
}
unsafe impl Send for CryptoaccSRngctrl {}
unsafe impl Sync for CryptoaccSRngctrl {}
impl CryptoaccSRngctrl {
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
    pub const fn rngctrl(self) -> crate::common::Reg<regs::Rngctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt."]
    #[inline(always)]
    pub const fn fifolevel(self) -> crate::common::Reg<regs::Fifolevel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks."]
    #[inline(always)]
    pub const fn fifothresh(self) -> crate::common::Reg<regs::Fifothresh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth."]
    #[inline(always)]
    pub const fn fifodepth(self) -> crate::common::Reg<regs::Fifodepth, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key0(self) -> crate::common::Reg<regs::Key0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key1(self) -> crate::common::Reg<regs::Key1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key2(self) -> crate::common::Reg<regs::Key2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key3(self) -> crate::common::Reg<regs::Key3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "This register is used to feed known data to the conditioning function or to the continuous tests. See manual."]
    #[inline(always)]
    pub const fn testdata(self) -> crate::common::Reg<regs::Testdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rngstatus(self) -> crate::common::Reg<regs::Rngstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn initwaitval(self) -> crate::common::Reg<regs::Initwaitval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Number of clk cycles to wait before stopping the rings after the FIFO is full."]
    #[inline(always)]
    pub const fn swofftmrval(self) -> crate::common::Reg<regs::Swofftmrval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)."]
    #[inline(always)]
    pub const fn clkdiv(self) -> crate::common::Reg<regs::Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ais31conf0(self) -> crate::common::Reg<regs::Ais31conf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ais31conf1(self) -> crate::common::Reg<regs::Ais31conf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ais31conf2(self) -> crate::common::Reg<regs::Ais31conf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields."]
    #[inline(always)]
    pub const fn ais31status(self) -> crate::common::Reg<regs::Ais31status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ais31conf0(pub u32);
    impl Ais31conf0 {
        #[doc = "Start-up Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn startupthres(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Start-up Threshold."]
        #[inline(always)]
        pub const fn set_startupthres(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Online Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn onlinethresh(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Online Threshold."]
        #[inline(always)]
        pub const fn set_onlinethresh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Ais31conf0 {
        #[inline(always)]
        fn default() -> Ais31conf0 {
            Ais31conf0(0)
        }
    }
    impl core::fmt::Debug for Ais31conf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ais31conf0")
                .field("startupthres", &self.startupthres())
                .field("onlinethresh", &self.onlinethresh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ais31conf0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ais31conf0 {{ startupthres: {=u16:?}, onlinethresh: {=u16:?} }}",
                self.startupthres(),
                self.onlinethresh()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ais31conf1(pub u32);
    impl Ais31conf1 {
        #[doc = "Expected History Value."]
        #[must_use]
        #[inline(always)]
        pub const fn hexpectedvalue(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Expected History Value."]
        #[inline(always)]
        pub const fn set_hexpectedvalue(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Online Repeat Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn onlinerepthresh(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Online Repeat Threshold."]
        #[inline(always)]
        pub const fn set_onlinerepthresh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Ais31conf1 {
        #[inline(always)]
        fn default() -> Ais31conf1 {
            Ais31conf1(0)
        }
    }
    impl core::fmt::Debug for Ais31conf1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ais31conf1")
                .field("hexpectedvalue", &self.hexpectedvalue())
                .field("onlinerepthresh", &self.onlinerepthresh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ais31conf1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ais31conf1 {{ hexpectedvalue: {=u16:?}, onlinerepthresh: {=u16:?} }}",
                self.hexpectedvalue(),
                self.onlinerepthresh()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ais31conf2(pub u32);
    impl Ais31conf2 {
        #[doc = "Minimum Allowed History Value."]
        #[must_use]
        #[inline(always)]
        pub const fn hmin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Minimum Allowed History Value."]
        #[inline(always)]
        pub const fn set_hmin(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Maximum Allowed History Value."]
        #[must_use]
        #[inline(always)]
        pub const fn hmax(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Maximum Allowed History Value."]
        #[inline(always)]
        pub const fn set_hmax(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Ais31conf2 {
        #[inline(always)]
        fn default() -> Ais31conf2 {
            Ais31conf2(0)
        }
    }
    impl core::fmt::Debug for Ais31conf2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ais31conf2")
                .field("hmin", &self.hmin())
                .field("hmax", &self.hmax())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ais31conf2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ais31conf2 {{ hmin: {=u16:?}, hmax: {=u16:?} }}",
                self.hmin(),
                self.hmax()
            )
        }
    }
    #[doc = "This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ais31status(pub u32);
    impl Ais31status {
        #[doc = "Number of preliminary alarms."]
        #[must_use]
        #[inline(always)]
        pub const fn numprelimalarms(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of preliminary alarms."]
        #[inline(always)]
        pub const fn set_numprelimalarms(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Preliminary noise alarm RNG."]
        #[must_use]
        #[inline(always)]
        pub const fn prelimnoisealarmrng(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Preliminary noise alarm RNG."]
        #[inline(always)]
        pub const fn set_prelimnoisealarmrng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Preliminary noise alarm Rep."]
        #[must_use]
        #[inline(always)]
        pub const fn prelimnoisealarmrep(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Preliminary noise alarm Rep."]
        #[inline(always)]
        pub const fn set_prelimnoisealarmrep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ais31status {
        #[inline(always)]
        fn default() -> Ais31status {
            Ais31status(0)
        }
    }
    impl core::fmt::Debug for Ais31status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ais31status")
                .field("numprelimalarms", &self.numprelimalarms())
                .field("prelimnoisealarmrng", &self.prelimnoisealarmrng())
                .field("prelimnoisealarmrep", &self.prelimnoisealarmrep())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ais31status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ais31status {{ numprelimalarms: {=u16:?}, prelimnoisealarmrng: {=bool:?}, prelimnoisealarmrep: {=bool:?} }}",
                self.numprelimalarms(),
                self.prelimnoisealarmrng(),
                self.prelimnoisealarmrep()
            )
        }
    }
    #[doc = "Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkdiv(pub u32);
    impl Clkdiv {
        #[doc = "Sample clock divider."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Sample clock divider."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Clkdiv {
        #[inline(always)]
        fn default() -> Clkdiv {
            Clkdiv(0)
        }
    }
    impl core::fmt::Debug for Clkdiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkdiv").field("value", &self.value()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clkdiv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Clkdiv {{ value: {=u8:?} }}", self.value())
        }
    }
    #[doc = "Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifodepth(pub u32);
    impl Fifodepth {
        #[doc = "FIFO Depth."]
        #[must_use]
        #[inline(always)]
        pub const fn fifodepth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FIFO Depth."]
        #[inline(always)]
        pub const fn set_fifodepth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifodepth {
        #[inline(always)]
        fn default() -> Fifodepth {
            Fifodepth(0)
        }
    }
    impl core::fmt::Debug for Fifodepth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifodepth")
                .field("fifodepth", &self.fifodepth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifodepth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifodepth {{ fifodepth: {=u32:?} }}", self.fifodepth())
        }
    }
    #[doc = "Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifolevel(pub u32);
    impl Fifolevel {
        #[doc = "FIFO Level."]
        #[must_use]
        #[inline(always)]
        pub const fn fifolevel(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FIFO Level."]
        #[inline(always)]
        pub const fn set_fifolevel(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifolevel {
        #[inline(always)]
        fn default() -> Fifolevel {
            Fifolevel(0)
        }
    }
    impl core::fmt::Debug for Fifolevel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifolevel")
                .field("fifolevel", &self.fifolevel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifolevel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifolevel {{ fifolevel: {=u32:?} }}", self.fifolevel())
        }
    }
    #[doc = "FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifothresh(pub u32);
    impl Fifothresh {
        #[doc = "FIFO threshold level."]
        #[must_use]
        #[inline(always)]
        pub const fn fifothresh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FIFO threshold level."]
        #[inline(always)]
        pub const fn set_fifothresh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifothresh {
        #[inline(always)]
        fn default() -> Fifothresh {
            Fifothresh(0)
        }
    }
    impl core::fmt::Debug for Fifothresh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifothresh")
                .field("fifothresh", &self.fifothresh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifothresh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifothresh {{ fifothresh: {=u32:?} }}", self.fifothresh())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Initwaitval(pub u32);
    impl Initwaitval {
        #[doc = "Wait counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn initwaitval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Wait counter value."]
        #[inline(always)]
        pub const fn set_initwaitval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Initwaitval {
        #[inline(always)]
        fn default() -> Initwaitval {
            Initwaitval(0)
        }
    }
    impl core::fmt::Debug for Initwaitval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Initwaitval")
                .field("initwaitval", &self.initwaitval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Initwaitval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Initwaitval {{ initwaitval: {=u16:?} }}", self.initwaitval())
        }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key0(pub u32);
    impl Key0 {
        #[doc = "Key."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Key."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key0 {
        #[inline(always)]
        fn default() -> Key0 {
            Key0(0)
        }
    }
    impl core::fmt::Debug for Key0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key0").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key0 {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key1(pub u32);
    impl Key1 {
        #[doc = "Key."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Key."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key1 {
        #[inline(always)]
        fn default() -> Key1 {
            Key1(0)
        }
    }
    impl core::fmt::Debug for Key1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key1").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key1 {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key2(pub u32);
    impl Key2 {
        #[doc = "Key."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Key."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key2 {
        #[inline(always)]
        fn default() -> Key2 {
            Key2(0)
        }
    }
    impl core::fmt::Debug for Key2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key2").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key2 {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key3(pub u32);
    impl Key3 {
        #[doc = "Key."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Key."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key3 {
        #[inline(always)]
        fn default() -> Key3 {
            Key3(0)
        }
    }
    impl core::fmt::Debug for Key3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Key3").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Key3 {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rngctrl(pub u32);
    impl Rngctrl {
        #[doc = "TRNG Module Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TRNG Module Enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Test Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn testen(&self) -> super::vals::Testen {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Testen::from_bits(val as u8)
        }
        #[doc = "Test Enable."]
        #[inline(always)]
        pub const fn set_testen(&mut self, val: super::vals::Testen) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Conditioning Bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn condbypass(&self) -> super::vals::Condbypass {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Condbypass::from_bits(val as u8)
        }
        #[doc = "Conditioning Bypass."]
        #[inline(always)]
        pub const fn set_condbypass(&mut self, val: super::vals::Condbypass) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "IRQ enable for Repetition Count Test."]
        #[must_use]
        #[inline(always)]
        pub const fn repcountien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ enable for Repetition Count Test."]
        #[inline(always)]
        pub const fn set_repcountien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IRQ enable for APT64IF."]
        #[must_use]
        #[inline(always)]
        pub const fn apt64ien(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ enable for APT64IF."]
        #[inline(always)]
        pub const fn set_apt64ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IRQ enable for APT4096IF."]
        #[must_use]
        #[inline(always)]
        pub const fn apt4096ien(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ enable for APT4096IF."]
        #[inline(always)]
        pub const fn set_apt4096ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IRQ enable for FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn fullien(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ enable for FIFO full."]
        #[inline(always)]
        pub const fn set_fullien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Software Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn softreset(&self) -> super::vals::Softreset {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Softreset::from_bits(val as u8)
        }
        #[doc = "Software Reset."]
        #[inline(always)]
        pub const fn set_softreset(&mut self, val: super::vals::Softreset) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "IRQ enable for AIS31 prelim. noise alarm."]
        #[must_use]
        #[inline(always)]
        pub const fn preien(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ enable for AIS31 prelim. noise alarm."]
        #[inline(always)]
        pub const fn set_preien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IRQ enable for AIS31 noise alarm."]
        #[must_use]
        #[inline(always)]
        pub const fn almien(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IRQ enable for AIS31 noise alarm."]
        #[inline(always)]
        pub const fn set_almien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Oscillator Force Run."]
        #[must_use]
        #[inline(always)]
        pub const fn forcerun(&self) -> super::vals::Forcerun {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Forcerun::from_bits(val as u8)
        }
        #[doc = "Oscillator Force Run."]
        #[inline(always)]
        pub const fn set_forcerun(&mut self, val: super::vals::Forcerun) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "NIST Start-up Test Bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn bypnist(&self) -> super::vals::Bypnist {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Bypnist::from_bits(val as u8)
        }
        #[doc = "NIST Start-up Test Bypass."]
        #[inline(always)]
        pub const fn set_bypnist(&mut self, val: super::vals::Bypnist) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "AIS31 Start-up Test Bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn bypais31(&self) -> super::vals::Bypais31 {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Bypais31::from_bits(val as u8)
        }
        #[doc = "AIS31 Start-up Test Bypass."]
        #[inline(always)]
        pub const fn set_bypais31(&mut self, val: super::vals::Bypais31) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Health test input select."]
        #[must_use]
        #[inline(always)]
        pub const fn healthtestsel(&self) -> super::vals::Healthtestsel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Healthtestsel::from_bits(val as u8)
        }
        #[doc = "Health test input select."]
        #[inline(always)]
        pub const fn set_healthtestsel(&mut self, val: super::vals::Healthtestsel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "AIS31 test input select."]
        #[must_use]
        #[inline(always)]
        pub const fn ais31testsel(&self) -> super::vals::Ais31testsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Ais31testsel::from_bits(val as u8)
        }
        #[doc = "AIS31 test input select."]
        #[inline(always)]
        pub const fn set_ais31testsel(&mut self, val: super::vals::Ais31testsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Number of 128b blocks in AES-CBCMAC."]
        #[must_use]
        #[inline(always)]
        pub const fn nb128bitblocks(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of 128b blocks in AES-CBCMAC."]
        #[inline(always)]
        pub const fn set_nb128bitblocks(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Fifo Write Start Up."]
        #[must_use]
        #[inline(always)]
        pub const fn fifowrstartup(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Fifo Write Start Up."]
        #[inline(always)]
        pub const fn set_fifowrstartup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Rngctrl {
        #[inline(always)]
        fn default() -> Rngctrl {
            Rngctrl(0)
        }
    }
    impl core::fmt::Debug for Rngctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rngctrl")
                .field("enable", &self.enable())
                .field("testen", &self.testen())
                .field("condbypass", &self.condbypass())
                .field("repcountien", &self.repcountien())
                .field("apt64ien", &self.apt64ien())
                .field("apt4096ien", &self.apt4096ien())
                .field("fullien", &self.fullien())
                .field("softreset", &self.softreset())
                .field("preien", &self.preien())
                .field("almien", &self.almien())
                .field("forcerun", &self.forcerun())
                .field("bypnist", &self.bypnist())
                .field("bypais31", &self.bypais31())
                .field("healthtestsel", &self.healthtestsel())
                .field("ais31testsel", &self.ais31testsel())
                .field("nb128bitblocks", &self.nb128bitblocks())
                .field("fifowrstartup", &self.fifowrstartup())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rngctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rngctrl {{ enable: {=bool:?}, testen: {:?}, condbypass: {:?}, repcountien: {=bool:?}, apt64ien: {=bool:?}, apt4096ien: {=bool:?}, fullien: {=bool:?}, softreset: {:?}, preien: {=bool:?}, almien: {=bool:?}, forcerun: {:?}, bypnist: {:?}, bypais31: {:?}, healthtestsel: {:?}, ais31testsel: {:?}, nb128bitblocks: {=u8:?}, fifowrstartup: {=bool:?} }}",
                self.enable(),
                self.testen(),
                self.condbypass(),
                self.repcountien(),
                self.apt64ien(),
                self.apt4096ien(),
                self.fullien(),
                self.softreset(),
                self.preien(),
                self.almien(),
                self.forcerun(),
                self.bypnist(),
                self.bypais31(),
                self.healthtestsel(),
                self.ais31testsel(),
                self.nb128bitblocks(),
                self.fifowrstartup()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rngstatus(pub u32);
    impl Rngstatus {
        #[doc = "Test Data Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn testdatabusy(&self) -> super::vals::Testdatabusy {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Testdatabusy::from_bits(val as u8)
        }
        #[doc = "Test Data Busy."]
        #[inline(always)]
        pub const fn set_testdatabusy(&mut self, val: super::vals::Testdatabusy) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "State of the control FSM."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> super::vals::State {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::State::from_bits(val as u8)
        }
        #[doc = "State of the control FSM."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: super::vals::State) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "Repetition Count Test interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn repcountif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition Count Test interrupt status."]
        #[inline(always)]
        pub const fn set_repcountif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "64-sample window Adaptive Proportion IF."]
        #[must_use]
        #[inline(always)]
        pub const fn apt64if(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "64-sample window Adaptive Proportion IF."]
        #[inline(always)]
        pub const fn set_apt64if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "4096-sample window Adaptive Prop. IF."]
        #[must_use]
        #[inline(always)]
        pub const fn apt4096if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "4096-sample window Adaptive Prop. IF."]
        #[inline(always)]
        pub const fn set_apt4096if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIFO full interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn fullif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO full interrupt status."]
        #[inline(always)]
        pub const fn set_fullif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "AIS31 Preliminary Noise Alarm IF."]
        #[must_use]
        #[inline(always)]
        pub const fn preif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "AIS31 Preliminary Noise Alarm IF."]
        #[inline(always)]
        pub const fn set_preif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AIS31 Noise Alarm interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn almif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "AIS31 Noise Alarm interrupt status."]
        #[inline(always)]
        pub const fn set_almif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Rngstatus {
        #[inline(always)]
        fn default() -> Rngstatus {
            Rngstatus(0)
        }
    }
    impl core::fmt::Debug for Rngstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rngstatus")
                .field("testdatabusy", &self.testdatabusy())
                .field("state", &self.state())
                .field("repcountif", &self.repcountif())
                .field("apt64if", &self.apt64if())
                .field("apt4096if", &self.apt4096if())
                .field("fullif", &self.fullif())
                .field("preif", &self.preif())
                .field("almif", &self.almif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rngstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rngstatus {{ testdatabusy: {:?}, state: {:?}, repcountif: {=bool:?}, apt64if: {=bool:?}, apt4096if: {=bool:?}, fullif: {=bool:?}, preif: {=bool:?}, almif: {=bool:?} }}",
                self.testdatabusy(),
                self.state(),
                self.repcountif(),
                self.apt64if(),
                self.apt4096if(),
                self.fullif(),
                self.preif(),
                self.almif()
            )
        }
    }
    #[doc = "Number of clk cycles to wait before stopping the rings after the FIFO is full."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swofftmrval(pub u32);
    impl Swofftmrval {
        #[doc = "Switch Off Timer Value."]
        #[must_use]
        #[inline(always)]
        pub const fn swofftmrval(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Switch Off Timer Value."]
        #[inline(always)]
        pub const fn set_swofftmrval(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Swofftmrval {
        #[inline(always)]
        fn default() -> Swofftmrval {
            Swofftmrval(0)
        }
    }
    impl core::fmt::Debug for Swofftmrval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swofftmrval")
                .field("swofftmrval", &self.swofftmrval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swofftmrval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Swofftmrval {{ swofftmrval: {=u16:?} }}", self.swofftmrval())
        }
    }
    #[doc = "This register is used to feed known data to the conditioning function or to the continuous tests. See manual."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Testdata(pub u32);
    impl Testdata {
        #[doc = "Test data input to conditioning tests."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Test data input to conditioning tests."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Testdata {
        #[inline(always)]
        fn default() -> Testdata {
            Testdata(0)
        }
    }
    impl core::fmt::Debug for Testdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Testdata").field("value", &self.value()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Testdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Testdata {{ value: {=u32:?} }}", self.value())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ais31testsel {
        #[doc = "Before conditioning."]
        Before = 0x0,
        #[doc = "After conditioning."]
        After = 0x01,
    }
    impl Ais31testsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ais31testsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ais31testsel {
        #[inline(always)]
        fn from(val: u8) -> Ais31testsel {
            Ais31testsel::from_bits(val)
        }
    }
    impl From<Ais31testsel> for u8 {
        #[inline(always)]
        fn from(val: Ais31testsel) -> u8 {
            Ais31testsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bypais31 {
        #[doc = "AIS31 startup test is applied. No data will be written to the FIFO until the test passes."]
        Normal = 0x0,
        #[doc = "AIS31 startup test is bypassed."]
        Bypass = 0x01,
    }
    impl Bypais31 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bypais31 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bypais31 {
        #[inline(always)]
        fn from(val: u8) -> Bypais31 {
            Bypais31::from_bits(val)
        }
    }
    impl From<Bypais31> for u8 {
        #[inline(always)]
        fn from(val: Bypais31) -> u8 {
            Bypais31::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bypnist {
        #[doc = "NIST-800-90B startup test is applied. No data will be written to the FIFO until the test passes."]
        Normal = 0x0,
        #[doc = "NIST-800-90B startup test is bypassed."]
        Bypass = 0x01,
    }
    impl Bypnist {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bypnist {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bypnist {
        #[inline(always)]
        fn from(val: u8) -> Bypnist {
            Bypnist::from_bits(val)
        }
    }
    impl From<Bypnist> for u8 {
        #[inline(always)]
        fn from(val: Bypnist) -> u8 {
            Bypnist::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Condbypass {
        #[doc = "The conditionig function is used."]
        Normal = 0x0,
        #[doc = "The conditioning function is bypassed."]
        Bypass = 0x01,
    }
    impl Condbypass {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Condbypass {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Condbypass {
        #[inline(always)]
        fn from(val: u8) -> Condbypass {
            Condbypass::from_bits(val)
        }
    }
    impl From<Condbypass> for u8 {
        #[inline(always)]
        fn from(val: Condbypass) -> u8 {
            Condbypass::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Forcerun {
        #[doc = "Oscillators will shut down when FIFO is full."]
        Normal = 0x0,
        #[doc = "Oscillators will continue to run even after FIFO is full."]
        Run = 0x01,
    }
    impl Forcerun {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Forcerun {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Forcerun {
        #[inline(always)]
        fn from(val: u8) -> Forcerun {
            Forcerun::from_bits(val)
        }
    }
    impl From<Forcerun> for u8 {
        #[inline(always)]
        fn from(val: Forcerun) -> u8 {
            Forcerun::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Healthtestsel {
        #[doc = "Before conditioning."]
        Before = 0x0,
        #[doc = "After conditioning."]
        After = 0x01,
    }
    impl Healthtestsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Healthtestsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Healthtestsel {
        #[inline(always)]
        fn from(val: u8) -> Healthtestsel {
            Healthtestsel::from_bits(val)
        }
    }
    impl From<Healthtestsel> for u8 {
        #[inline(always)]
        fn from(val: Healthtestsel) -> u8 {
            Healthtestsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Softreset {
        #[doc = "Module not in reset."]
        Normal = 0x0,
        #[doc = "The continuous test, the conditioning function and the FIFO are reset."]
        Reset = 0x01,
    }
    impl Softreset {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Softreset {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Softreset {
        #[inline(always)]
        fn from(val: u8) -> Softreset {
            Softreset::from_bits(val)
        }
    }
    impl From<Softreset> for u8 {
        #[inline(always)]
        fn from(val: Softreset) -> u8 {
            Softreset::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum State {
        #[doc = "RESET State."]
        Reset = 0x0,
        #[doc = "STARTUP State."]
        Startup = 0x01,
        #[doc = "FIFOFULLON State."]
        Fifofullon = 0x02,
        #[doc = "FIFOFULLOFF State."]
        Fifofulloff = 0x03,
        #[doc = "RUNNING State."]
        Running = 0x04,
        #[doc = "ERROR State."]
        Error = 0x05,
        #[doc = "UNUSED."]
        Unused6 = 0x06,
        #[doc = "UNUSED."]
        Unused7 = 0x07,
    }
    impl State {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> State {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for State {
        #[inline(always)]
        fn from(val: u8) -> State {
            State::from_bits(val)
        }
    }
    impl From<State> for u8 {
        #[inline(always)]
        fn from(val: State) -> u8 {
            State::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Testdatabusy {
        #[doc = "TESTDATA write is finished processing or no test in progress."]
        Idle = 0x0,
        #[doc = "TESTDATA write is still being processed."]
        Busy = 0x01,
    }
    impl Testdatabusy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Testdatabusy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Testdatabusy {
        #[inline(always)]
        fn from(val: u8) -> Testdatabusy {
            Testdatabusy::from_bits(val)
        }
    }
    impl From<Testdatabusy> for u8 {
        #[inline(always)]
        fn from(val: Testdatabusy) -> u8 {
            Testdatabusy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Testen {
        #[doc = "Non-determinsitc random number generation."]
        Noise = 0x0,
        #[doc = "Pseudo-random number generation."]
        Testdata = 0x01,
    }
    impl Testen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Testen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Testen {
        #[inline(always)]
        fn from(val: u8) -> Testen {
            Testen::from_bits(val)
        }
    }
    impl From<Testen> for u8 {
        #[inline(always)]
        fn from(val: Testen) -> u8 {
            Testen::to_bits(val)
        }
    }
}
