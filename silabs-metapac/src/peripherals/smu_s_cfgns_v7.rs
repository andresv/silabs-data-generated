#[doc = "SMU_S_CFGNS peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmuSCfgns {
    ptr: *mut u8,
}
unsafe impl Send for SmuSCfgns {}
unsafe impl Sync for SmuSCfgns {}
impl SmuSCfgns {
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
    pub const fn nsstatus(self) -> crate::common::Reg<regs::Nsstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn nslock(self) -> crate::common::Reg<regs::Nslock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn nsif(self) -> crate::common::Reg<regs::Nsif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn nsien(self) -> crate::common::Reg<regs::Nsien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppunspatd0(self) -> crate::common::Reg<regs::Ppunspatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppunspatd1(self) -> crate::common::Reg<regs::Ppunspatd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppunspatd2(self) -> crate::common::Reg<regs::Ppunspatd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ppunsfs(self) -> crate::common::Reg<regs::Ppunsfs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn bmpunspatd0(self) -> crate::common::Reg<regs::Bmpunspatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmpunspatd0(pub u32);
    impl Bmpunspatd0 {
        #[doc = "RADIOAES Privileged Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn radioaes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RADIOAES Privileged Mode."]
        #[inline(always)]
        pub const fn set_radioaes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RADIOSUBSYSTEM Privileged Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn radiosubsystem(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RADIOSUBSYSTEM Privileged Mode."]
        #[inline(always)]
        pub const fn set_radiosubsystem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LDMA Privileged Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ldma(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LDMA Privileged Mode."]
        #[inline(always)]
        pub const fn set_ldma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RFECA0 Privileged Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rfeca0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RFECA0 Privileged Mode."]
        #[inline(always)]
        pub const fn set_rfeca0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RFECA1 Privileged Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rfeca1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RFECA1 Privileged Mode."]
        #[inline(always)]
        pub const fn set_rfeca1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SEEXTDMA Privileged Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn seextdma(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SEEXTDMA Privileged Mode."]
        #[inline(always)]
        pub const fn set_seextdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Bmpunspatd0 {
        #[inline(always)]
        fn default() -> Bmpunspatd0 {
            Bmpunspatd0(0)
        }
    }
    impl core::fmt::Debug for Bmpunspatd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmpunspatd0")
                .field("radioaes", &self.radioaes())
                .field("radiosubsystem", &self.radiosubsystem())
                .field("ldma", &self.ldma())
                .field("rfeca0", &self.rfeca0())
                .field("rfeca1", &self.rfeca1())
                .field("seextdma", &self.seextdma())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmpunspatd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bmpunspatd0 {{ radioaes: {=bool:?}, radiosubsystem: {=bool:?}, ldma: {=bool:?}, rfeca0: {=bool:?}, rfeca1: {=bool:?}, seextdma: {=bool:?} }}",
                self.radioaes(),
                self.radiosubsystem(),
                self.ldma(),
                self.rfeca0(),
                self.rfeca1(),
                self.seextdma()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsien(pub u32);
    impl Nsien {
        #[doc = "PPUNS Privilege Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppunspriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PPUNS Privilege Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ppunspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PPUNS Instruction Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppunsinst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PPUNS Instruction Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ppunsinst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Nsien {
        #[inline(always)]
        fn default() -> Nsien {
            Nsien(0)
        }
    }
    impl core::fmt::Debug for Nsien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsien")
                .field("ppunspriv", &self.ppunspriv())
                .field("ppunsinst", &self.ppunsinst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsien {{ ppunspriv: {=bool:?}, ppunsinst: {=bool:?} }}",
                self.ppunspriv(),
                self.ppunsinst()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsif(pub u32);
    impl Nsif {
        #[doc = "PPUNS Privilege Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ppunspriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PPUNS Privilege Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ppunspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PPUNS Instruction Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ppunsinst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PPUNS Instruction Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ppunsinst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Nsif {
        #[inline(always)]
        fn default() -> Nsif {
            Nsif(0)
        }
    }
    impl core::fmt::Debug for Nsif {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsif")
                .field("ppunspriv", &self.ppunspriv())
                .field("ppunsinst", &self.ppunsinst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsif {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsif {{ ppunspriv: {=bool:?}, ppunsinst: {=bool:?} }}",
                self.ppunspriv(),
                self.ppunsinst()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nslock(pub u32);
    impl Nslock {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn smunslockkey(&self) -> super::vals::Smunslockkey {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            super::vals::Smunslockkey::from_bits(val as u32)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_smunslockkey(&mut self, val: super::vals::Smunslockkey) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val.to_bits() as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Nslock {
        #[inline(always)]
        fn default() -> Nslock {
            Nslock(0)
        }
    }
    impl core::fmt::Debug for Nslock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nslock")
                .field("smunslockkey", &self.smunslockkey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nslock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nslock {{ smunslockkey: {:?} }}", self.smunslockkey())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsstatus(pub u32);
    impl Nsstatus {
        #[doc = "SMUNS Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn smunslock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SMUNS Lock."]
        #[inline(always)]
        pub const fn set_smunslock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Nsstatus {
        #[inline(always)]
        fn default() -> Nsstatus {
            Nsstatus(0)
        }
    }
    impl core::fmt::Debug for Nsstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsstatus")
                .field("smunslock", &self.smunslock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nsstatus {{ smunslock: {=bool:?} }}", self.smunslock())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppunsfs(pub u32);
    impl Ppunsfs {
        #[doc = "Peripheral I."]
        #[must_use]
        #[inline(always)]
        pub const fn ppufsperiphid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Peripheral I."]
        #[inline(always)]
        pub const fn set_ppufsperiphid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ppunsfs {
        #[inline(always)]
        fn default() -> Ppunsfs {
            Ppunsfs(0)
        }
    }
    impl core::fmt::Debug for Ppunsfs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppunsfs")
                .field("ppufsperiphid", &self.ppufsperiphid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppunsfs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ppunsfs {{ ppufsperiphid: {=u8:?} }}", self.ppufsperiphid())
        }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppunspatd0(pub u32);
    impl Ppunspatd0 {
        #[doc = "EMU Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn emu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EMU Privileged Access."]
        #[inline(always)]
        pub const fn set_emu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CMU Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn cmu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CMU Privileged Access."]
        #[inline(always)]
        pub const fn set_cmu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BURTC Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn burtc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BURTC Privileged Access."]
        #[inline(always)]
        pub const fn set_burtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HFRCO0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hfrco0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HFRCO0 Privileged Access."]
        #[inline(always)]
        pub const fn set_hfrco0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FSRCO Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn fsrco(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FSRCO Privileged Access."]
        #[inline(always)]
        pub const fn set_fsrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DPLL0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dpll0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DPLL0 Privileged Access."]
        #[inline(always)]
        pub const fn set_dpll0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LFXO Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lfxo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Privileged Access."]
        #[inline(always)]
        pub const fn set_lfxo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LFRCO Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lfrco(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LFRCO Privileged Access."]
        #[inline(always)]
        pub const fn set_lfrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ULFRCO Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ulfrco(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ULFRCO Privileged Access."]
        #[inline(always)]
        pub const fn set_ulfrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "KEYSCAN Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn keyscan(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "KEYSCAN Privileged Access."]
        #[inline(always)]
        pub const fn set_keyscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSC Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn msc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSC Privileged Access."]
        #[inline(always)]
        pub const fn set_msc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ICACHE0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn icache0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE0 Privileged Access."]
        #[inline(always)]
        pub const fn set_icache0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PRS Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Privileged Access."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIO Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO Privileged Access."]
        #[inline(always)]
        pub const fn set_gpio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LDMA Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ldma(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "LDMA Privileged Access."]
        #[inline(always)]
        pub const fn set_ldma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "LDMAXBAR Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ldmaxbar(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LDMAXBAR Privileged Access."]
        #[inline(always)]
        pub const fn set_ldmaxbar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIMER0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER0 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIMER1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER1 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIMER2 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER2 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIMER3 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer3(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER3 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIMER4 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer4(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER4 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TIMER5 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer5(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER5 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "TIMER6 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer6(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER6 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "TIMER7 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer7(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER7 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIMER8 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer8(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER8 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "TIMER9 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer9(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER9 Privileged Access."]
        #[inline(always)]
        pub const fn set_timer9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DMEM0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dmem0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DMEM0 Privileged Access."]
        #[inline(always)]
        pub const fn set_dmem0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DMEM1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dmem1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DMEM1 Privileged Access."]
        #[inline(always)]
        pub const fn set_dmem1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SYSCFGCFGNS Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgcfgns(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFGCFGNS Privileged Access."]
        #[inline(always)]
        pub const fn set_syscfgcfgns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SYSCFG Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG Privileged Access."]
        #[inline(always)]
        pub const fn set_syscfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ppunspatd0 {
        #[inline(always)]
        fn default() -> Ppunspatd0 {
            Ppunspatd0(0)
        }
    }
    impl core::fmt::Debug for Ppunspatd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppunspatd0")
                .field("emu", &self.emu())
                .field("cmu", &self.cmu())
                .field("burtc", &self.burtc())
                .field("hfrco0", &self.hfrco0())
                .field("fsrco", &self.fsrco())
                .field("dpll0", &self.dpll0())
                .field("lfxo", &self.lfxo())
                .field("lfrco", &self.lfrco())
                .field("ulfrco", &self.ulfrco())
                .field("keyscan", &self.keyscan())
                .field("msc", &self.msc())
                .field("icache0", &self.icache0())
                .field("prs", &self.prs())
                .field("gpio", &self.gpio())
                .field("ldma", &self.ldma())
                .field("ldmaxbar", &self.ldmaxbar())
                .field("timer0", &self.timer0())
                .field("timer1", &self.timer1())
                .field("timer2", &self.timer2())
                .field("timer3", &self.timer3())
                .field("timer4", &self.timer4())
                .field("timer5", &self.timer5())
                .field("timer6", &self.timer6())
                .field("timer7", &self.timer7())
                .field("timer8", &self.timer8())
                .field("timer9", &self.timer9())
                .field("dmem0", &self.dmem0())
                .field("dmem1", &self.dmem1())
                .field("syscfgcfgns", &self.syscfgcfgns())
                .field("syscfg", &self.syscfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppunspatd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppunspatd0 {{ emu: {=bool:?}, cmu: {=bool:?}, burtc: {=bool:?}, hfrco0: {=bool:?}, fsrco: {=bool:?}, dpll0: {=bool:?}, lfxo: {=bool:?}, lfrco: {=bool:?}, ulfrco: {=bool:?}, keyscan: {=bool:?}, msc: {=bool:?}, icache0: {=bool:?}, prs: {=bool:?}, gpio: {=bool:?}, ldma: {=bool:?}, ldmaxbar: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, timer2: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, timer5: {=bool:?}, timer6: {=bool:?}, timer7: {=bool:?}, timer8: {=bool:?}, timer9: {=bool:?}, dmem0: {=bool:?}, dmem1: {=bool:?}, syscfgcfgns: {=bool:?}, syscfg: {=bool:?} }}",
                self.emu(),
                self.cmu(),
                self.burtc(),
                self.hfrco0(),
                self.fsrco(),
                self.dpll0(),
                self.lfxo(),
                self.lfrco(),
                self.ulfrco(),
                self.keyscan(),
                self.msc(),
                self.icache0(),
                self.prs(),
                self.gpio(),
                self.ldma(),
                self.ldmaxbar(),
                self.timer0(),
                self.timer1(),
                self.timer2(),
                self.timer3(),
                self.timer4(),
                self.timer5(),
                self.timer6(),
                self.timer7(),
                self.timer8(),
                self.timer9(),
                self.dmem0(),
                self.dmem1(),
                self.syscfgcfgns(),
                self.syscfg()
            )
        }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppunspatd1(pub u32);
    impl Ppunspatd1 {
        #[doc = "BURAM Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn buram(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BURAM Privileged Access."]
        #[inline(always)]
        pub const fn set_buram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPCRC Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn gpcrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPCRC Privileged Access."]
        #[inline(always)]
        pub const fn set_gpcrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EUSART1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART1 Privileged Access."]
        #[inline(always)]
        pub const fn set_eusart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EUSART2 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART2 Privileged Access."]
        #[inline(always)]
        pub const fn set_eusart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EUSART3 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART3 Privileged Access."]
        #[inline(always)]
        pub const fn set_eusart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DCDC Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Privileged Access."]
        #[inline(always)]
        pub const fn set_dcdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HOSTMAILBOX Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hostmailbox(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HOSTMAILBOX Privileged Access."]
        #[inline(always)]
        pub const fn set_hostmailbox(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn usart0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART0 Privileged Access."]
        #[inline(always)]
        pub const fn set_usart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USART1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Privileged Access."]
        #[inline(always)]
        pub const fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USART2 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Privileged Access."]
        #[inline(always)]
        pub const fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SYSRTC Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn sysrtc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SYSRTC Privileged Access."]
        #[inline(always)]
        pub const fn set_sysrtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I2C1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Privileged Access."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "I2C2 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Privileged Access."]
        #[inline(always)]
        pub const fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C3 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Privileged Access."]
        #[inline(always)]
        pub const fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "LCD Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lcd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LCD Privileged Access."]
        #[inline(always)]
        pub const fn set_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LCDRF Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdrf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "LCDRF Privileged Access."]
        #[inline(always)]
        pub const fn set_lcdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RADIOAES Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn radioaes(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RADIOAES Privileged Access."]
        #[inline(always)]
        pub const fn set_radioaes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SMU Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn smu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SMU Privileged Access."]
        #[inline(always)]
        pub const fn set_smu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SMUCFGNS Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn smucfgns(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "SMUCFGNS Privileged Access."]
        #[inline(always)]
        pub const fn set_smucfgns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LETIMER0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn letimer0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LETIMER0 Privileged Access."]
        #[inline(always)]
        pub const fn set_letimer0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "IADC0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn iadc0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "IADC0 Privileged Access."]
        #[inline(always)]
        pub const fn set_iadc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ACMP0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ACMP0 Privileged Access."]
        #[inline(always)]
        pub const fn set_acmp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ACMP1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ACMP1 Privileged Access."]
        #[inline(always)]
        pub const fn set_acmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "AMUXCP0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn amuxcp0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "AMUXCP0 Privileged Access."]
        #[inline(always)]
        pub const fn set_amuxcp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VDAC0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn vdac0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VDAC0 Privileged Access."]
        #[inline(always)]
        pub const fn set_vdac0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VDAC1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn vdac1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VDAC1 Privileged Access."]
        #[inline(always)]
        pub const fn set_vdac1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCNT Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn pcnt(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCNT Privileged Access."]
        #[inline(always)]
        pub const fn set_pcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "HFRCO1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hfrco1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HFRCO1 Privileged Access."]
        #[inline(always)]
        pub const fn set_hfrco1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "HFXO0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hfxo0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "HFXO0 Privileged Access."]
        #[inline(always)]
        pub const fn set_hfxo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "I2C0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "I2C0 Privileged Access."]
        #[inline(always)]
        pub const fn set_i2c0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "WDOG0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG0 Privileged Access."]
        #[inline(always)]
        pub const fn set_wdog0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "WDOG1 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG1 Privileged Access."]
        #[inline(always)]
        pub const fn set_wdog1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ppunspatd1 {
        #[inline(always)]
        fn default() -> Ppunspatd1 {
            Ppunspatd1(0)
        }
    }
    impl core::fmt::Debug for Ppunspatd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppunspatd1")
                .field("buram", &self.buram())
                .field("gpcrc", &self.gpcrc())
                .field("eusart1", &self.eusart1())
                .field("eusart2", &self.eusart2())
                .field("eusart3", &self.eusart3())
                .field("dcdc", &self.dcdc())
                .field("hostmailbox", &self.hostmailbox())
                .field("usart0", &self.usart0())
                .field("usart1", &self.usart1())
                .field("usart2", &self.usart2())
                .field("sysrtc", &self.sysrtc())
                .field("i2c1", &self.i2c1())
                .field("i2c2", &self.i2c2())
                .field("i2c3", &self.i2c3())
                .field("lcd", &self.lcd())
                .field("lcdrf", &self.lcdrf())
                .field("radioaes", &self.radioaes())
                .field("smu", &self.smu())
                .field("smucfgns", &self.smucfgns())
                .field("letimer0", &self.letimer0())
                .field("iadc0", &self.iadc0())
                .field("acmp0", &self.acmp0())
                .field("acmp1", &self.acmp1())
                .field("amuxcp0", &self.amuxcp0())
                .field("vdac0", &self.vdac0())
                .field("vdac1", &self.vdac1())
                .field("pcnt", &self.pcnt())
                .field("hfrco1", &self.hfrco1())
                .field("hfxo0", &self.hfxo0())
                .field("i2c0", &self.i2c0())
                .field("wdog0", &self.wdog0())
                .field("wdog1", &self.wdog1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppunspatd1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppunspatd1 {{ buram: {=bool:?}, gpcrc: {=bool:?}, eusart1: {=bool:?}, eusart2: {=bool:?}, eusart3: {=bool:?}, dcdc: {=bool:?}, hostmailbox: {=bool:?}, usart0: {=bool:?}, usart1: {=bool:?}, usart2: {=bool:?}, sysrtc: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, i2c3: {=bool:?}, lcd: {=bool:?}, lcdrf: {=bool:?}, radioaes: {=bool:?}, smu: {=bool:?}, smucfgns: {=bool:?}, letimer0: {=bool:?}, iadc0: {=bool:?}, acmp0: {=bool:?}, acmp1: {=bool:?}, amuxcp0: {=bool:?}, vdac0: {=bool:?}, vdac1: {=bool:?}, pcnt: {=bool:?}, hfrco1: {=bool:?}, hfxo0: {=bool:?}, i2c0: {=bool:?}, wdog0: {=bool:?}, wdog1: {=bool:?} }}",
                self.buram(),
                self.gpcrc(),
                self.eusart1(),
                self.eusart2(),
                self.eusart3(),
                self.dcdc(),
                self.hostmailbox(),
                self.usart0(),
                self.usart1(),
                self.usart2(),
                self.sysrtc(),
                self.i2c1(),
                self.i2c2(),
                self.i2c3(),
                self.lcd(),
                self.lcdrf(),
                self.radioaes(),
                self.smu(),
                self.smucfgns(),
                self.letimer0(),
                self.iadc0(),
                self.acmp0(),
                self.acmp1(),
                self.amuxcp0(),
                self.vdac0(),
                self.vdac1(),
                self.pcnt(),
                self.hfrco1(),
                self.hfxo0(),
                self.i2c0(),
                self.wdog0(),
                self.wdog1()
            )
        }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppunspatd2(pub u32);
    impl Ppunspatd2 {
        #[doc = "EUSART0 Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART0 Privileged Access."]
        #[inline(always)]
        pub const fn set_eusart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SEMAILBOX Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn semailbox(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SEMAILBOX Privileged Access."]
        #[inline(always)]
        pub const fn set_semailbox(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AHBRADIO Privileged Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbradio(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AHBRADIO Privileged Access."]
        #[inline(always)]
        pub const fn set_ahbradio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ppunspatd2 {
        #[inline(always)]
        fn default() -> Ppunspatd2 {
            Ppunspatd2(0)
        }
    }
    impl core::fmt::Debug for Ppunspatd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppunspatd2")
                .field("eusart0", &self.eusart0())
                .field("semailbox", &self.semailbox())
                .field("ahbradio", &self.ahbradio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppunspatd2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppunspatd2 {{ eusart0: {=bool:?}, semailbox: {=bool:?}, ahbradio: {=bool:?} }}",
                self.eusart0(),
                self.semailbox(),
                self.ahbradio()
            )
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smunslockkey(u32);
    impl Smunslockkey {
        #[doc = "Unlocks Registers."]
        pub const Unlock: Self = Self(0x00ac_ce55);
    }
    impl Smunslockkey {
        pub const fn from_bits(val: u32) -> Smunslockkey {
            Self(val & 0x00ff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Smunslockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x00ac_ce55 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smunslockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x00ac_ce55 => defmt::write!(f, "Unlock"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Smunslockkey {
        #[inline(always)]
        fn from(val: u32) -> Smunslockkey {
            Smunslockkey::from_bits(val)
        }
    }
    impl From<Smunslockkey> for u32 {
        #[inline(always)]
        fn from(val: Smunslockkey) -> u32 {
            Smunslockkey::to_bits(val)
        }
    }
}
