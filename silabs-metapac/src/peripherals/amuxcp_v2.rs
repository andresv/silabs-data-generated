#[doc = "AMUXCP peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amuxcp {
    ptr: *mut u8,
}
unsafe impl Send for Amuxcp {}
unsafe impl Sync for Amuxcp {}
impl Amuxcp {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Test."]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn trim(self) -> crate::common::Reg<regs::Trim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Test. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn test_set(self) -> crate::common::Reg<regs::Test, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Trim. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn trim_set(self) -> crate::common::Reg<regs::Trim, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Test. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn test_clr(self) -> crate::common::Reg<regs::Test, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "Trim. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn trim_clr(self) -> crate::common::Reg<regs::Trim, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "Control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Test. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn test_tgl(self) -> crate::common::Reg<regs::Test, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "Trim. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn trim_tgl(self) -> crate::common::Reg<regs::Trim, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Force High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn forcehp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force High Power."]
        #[inline(always)]
        pub const fn set_forcehp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn forcelp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force Low Power."]
        #[inline(always)]
        pub const fn set_forcelp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force run."]
        #[must_use]
        #[inline(always)]
        pub const fn forcerun(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force run."]
        #[inline(always)]
        pub const fn set_forcerun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force stop."]
        #[must_use]
        #[inline(always)]
        pub const fn forcestop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Force stop."]
        #[inline(always)]
        pub const fn set_forcestop(&mut self, val: bool) {
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
                .field("forcehp", &self.forcehp())
                .field("forcelp", &self.forcelp())
                .field("forcerun", &self.forcerun())
                .field("forcestop", &self.forcestop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ forcehp: {=bool:?}, forcelp: {=bool:?}, forcerun: {=bool:?}, forcestop: {=bool:?} }}",
                self.forcehp(),
                self.forcelp(),
                self.forcerun(),
                self.forcestop()
            )
        }
    }
    #[doc = "IPVERSION."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IPVERSION."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IPVERSION."]
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
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "running."]
        #[must_use]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "running."]
        #[inline(always)]
        pub const fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "high cap."]
        #[must_use]
        #[inline(always)]
        pub const fn hicap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "high cap."]
        #[inline(always)]
        pub const fn set_hicap(&mut self, val: bool) {
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
                .field("run", &self.run())
                .field("hicap", &self.hicap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ run: {=bool:?}, hicap: {=bool:?} }}",
                self.run(),
                self.hicap()
            )
        }
    }
    #[doc = "Test."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Test(pub u32);
    impl Test {
        #[doc = "Sync Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn syncclk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Clock."]
        #[inline(always)]
        pub const fn set_syncclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sync Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn syncmode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Mode."]
        #[inline(always)]
        pub const fn set_syncmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force Request."]
        #[must_use]
        #[inline(always)]
        pub const fn forcerequest(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Force Request."]
        #[inline(always)]
        pub const fn set_forcerequest(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Force high capacitance driver."]
        #[must_use]
        #[inline(always)]
        pub const fn forcehicap(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Force high capacitance driver."]
        #[inline(always)]
        pub const fn set_forcehicap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Force low capacitance driver."]
        #[must_use]
        #[inline(always)]
        pub const fn forcelocap(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Force low capacitance driver."]
        #[inline(always)]
        pub const fn set_forcelocap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Force Boost On."]
        #[must_use]
        #[inline(always)]
        pub const fn forcebooston(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Force Boost On."]
        #[inline(always)]
        pub const fn set_forcebooston(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force Boost Off."]
        #[must_use]
        #[inline(always)]
        pub const fn forceboostoff(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force Boost Off."]
        #[inline(always)]
        pub const fn set_forceboostoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable write to status bits."]
        #[must_use]
        #[inline(always)]
        pub const fn statusen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable write to status bits."]
        #[inline(always)]
        pub const fn set_statusen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Test {
        #[inline(always)]
        fn default() -> Test {
            Test(0)
        }
    }
    impl core::fmt::Debug for Test {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Test")
                .field("syncclk", &self.syncclk())
                .field("syncmode", &self.syncmode())
                .field("forcerequest", &self.forcerequest())
                .field("forcehicap", &self.forcehicap())
                .field("forcelocap", &self.forcelocap())
                .field("forcebooston", &self.forcebooston())
                .field("forceboostoff", &self.forceboostoff())
                .field("statusen", &self.statusen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Test {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Test {{ syncclk: {=bool:?}, syncmode: {=bool:?}, forcerequest: {=bool:?}, forcehicap: {=bool:?}, forcelocap: {=bool:?}, forcebooston: {=bool:?}, forceboostoff: {=bool:?}, statusen: {=bool:?} }}",
                self.syncclk(),
                self.syncmode(),
                self.forcerequest(),
                self.forcehicap(),
                self.forcelocap(),
                self.forcebooston(),
                self.forceboostoff(),
                self.statusen()
            )
        }
    }
    #[doc = "Trim."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trim(pub u32);
    impl Trim {
        #[doc = "Warm up time."]
        #[must_use]
        #[inline(always)]
        pub const fn warmuptime(&self) -> super::vals::Warmuptime {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Warmuptime::from_bits(val as u8)
        }
        #[doc = "Warm up time."]
        #[inline(always)]
        pub const fn set_warmuptime(&mut self, val: super::vals::Warmuptime) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Float VDDCP Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn floatvddcplo(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Float VDDCP Low Power."]
        #[inline(always)]
        pub const fn set_floatvddcplo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Float VDDCP High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn floatvddcphi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Float VDDCP High Power."]
        #[inline(always)]
        pub const fn set_floatvddcphi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Bypass Div2 Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn bypassdiv2lo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Div2 Low Power."]
        #[inline(always)]
        pub const fn set_bypassdiv2lo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bypass Div2 High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn bypassdiv2hi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass Div2 High Power."]
        #[inline(always)]
        pub const fn set_bypassdiv2hi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Bump 0.5X Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn bump0p5xlo(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Bump 0.5X Low Power."]
        #[inline(always)]
        pub const fn set_bump0p5xlo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Bump 0.5X High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn bump0p5xhi(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Bump 0.5X High Power."]
        #[inline(always)]
        pub const fn set_bump0p5xhi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Bias 2x Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn bias2xlo(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Bias 2x Low Power."]
        #[inline(always)]
        pub const fn set_bias2xlo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Bias 2x High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn bias2xhi(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Bias 2x High Power."]
        #[inline(always)]
        pub const fn set_bias2xhi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Charge Pump Voltage Control Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn voltagectrllo(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Charge Pump Voltage Control Low Power."]
        #[inline(always)]
        pub const fn set_voltagectrllo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Charge Pump Voltage Control High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn voltagectrlhi(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Charge Pump Voltage Control High Power."]
        #[inline(always)]
        pub const fn set_voltagectrlhi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Bias Control Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn biasctrllo(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "Bias Control Low Power."]
        #[inline(always)]
        pub const fn set_biasctrllo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "Bias Control Low Power Continuous."]
        #[must_use]
        #[inline(always)]
        pub const fn biasctrllocont(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "Bias Control Low Power Continuous."]
        #[inline(always)]
        pub const fn set_biasctrllocont(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[doc = "Bias Control High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn biasctrlhi(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "Bias Control High Power."]
        #[inline(always)]
        pub const fn set_biasctrlhi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "Pump Cap Low Power."]
        #[must_use]
        #[inline(always)]
        pub const fn pumpcaplo(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Pump Cap Low Power."]
        #[inline(always)]
        pub const fn set_pumpcaplo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Pump Cap High Power."]
        #[must_use]
        #[inline(always)]
        pub const fn pumpcaphi(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Pump Cap High Power."]
        #[inline(always)]
        pub const fn set_pumpcaphi(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Trim {
        #[inline(always)]
        fn default() -> Trim {
            Trim(0)
        }
    }
    impl core::fmt::Debug for Trim {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trim")
                .field("warmuptime", &self.warmuptime())
                .field("floatvddcplo", &self.floatvddcplo())
                .field("floatvddcphi", &self.floatvddcphi())
                .field("bypassdiv2lo", &self.bypassdiv2lo())
                .field("bypassdiv2hi", &self.bypassdiv2hi())
                .field("bump0p5xlo", &self.bump0p5xlo())
                .field("bump0p5xhi", &self.bump0p5xhi())
                .field("bias2xlo", &self.bias2xlo())
                .field("bias2xhi", &self.bias2xhi())
                .field("voltagectrllo", &self.voltagectrllo())
                .field("voltagectrlhi", &self.voltagectrlhi())
                .field("biasctrllo", &self.biasctrllo())
                .field("biasctrllocont", &self.biasctrllocont())
                .field("biasctrlhi", &self.biasctrlhi())
                .field("pumpcaplo", &self.pumpcaplo())
                .field("pumpcaphi", &self.pumpcaphi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trim {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trim {{ warmuptime: {:?}, floatvddcplo: {=bool:?}, floatvddcphi: {=bool:?}, bypassdiv2lo: {=bool:?}, bypassdiv2hi: {=bool:?}, bump0p5xlo: {=bool:?}, bump0p5xhi: {=bool:?}, bias2xlo: {=bool:?}, bias2xhi: {=bool:?}, voltagectrllo: {=u8:?}, voltagectrlhi: {=u8:?}, biasctrllo: {=u8:?}, biasctrllocont: {=u8:?}, biasctrlhi: {=u8:?}, pumpcaplo: {=u8:?}, pumpcaphi: {=u8:?} }}",
                self.warmuptime(),
                self.floatvddcplo(),
                self.floatvddcphi(),
                self.bypassdiv2lo(),
                self.bypassdiv2hi(),
                self.bump0p5xlo(),
                self.bump0p5xhi(),
                self.bias2xlo(),
                self.bias2xhi(),
                self.voltagectrllo(),
                self.voltagectrlhi(),
                self.biasctrllo(),
                self.biasctrllocont(),
                self.biasctrlhi(),
                self.pumpcaplo(),
                self.pumpcaphi()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Warmuptime {
        #[doc = "Warm up cycle = 72; 3.6us @20 MHz."]
        Wucycles72 = 0x0,
        #[doc = "Warm up cycle = 96; 4.8us @ 20 MHz."]
        Wucycles96 = 0x01,
        #[doc = "Warm up cycle = 128; 6.4us @ 20 MHz."]
        Wucycles128 = 0x02,
        #[doc = "Warm up cycle = 160; 8.0us @ 20 MHz."]
        Wucycles160 = 0x03,
    }
    impl Warmuptime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Warmuptime {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Warmuptime {
        #[inline(always)]
        fn from(val: u8) -> Warmuptime {
            Warmuptime::from_bits(val)
        }
    }
    impl From<Warmuptime> for u8 {
        #[inline(always)]
        fn from(val: Warmuptime) -> u8 {
            Warmuptime::to_bits(val)
        }
    }
}
