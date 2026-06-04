#[doc = "DEVINFO peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devinfo {
    ptr: *mut u8,
}
unsafe impl Send for Devinfo {}
unsafe impl Sync for Devinfo {}
impl Devinfo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version of the device info structure being used."]
    #[inline(always)]
    pub const fn info(self) -> crate::common::Reg<regs::Info, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Part description."]
    #[inline(always)]
    pub const fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flash page size and misc. chip information."]
    #[inline(always)]
    pub const fn meminfo(self) -> crate::common::Reg<regs::Meminfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash and SRAM Memory size in kB."]
    #[inline(always)]
    pub const fn msize(self) -> crate::common::Reg<regs::Msize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Miscellaneous device information."]
    #[inline(always)]
    pub const fn pkginfo(self) -> crate::common::Reg<regs::Pkginfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Custom information."]
    #[inline(always)]
    pub const fn custominfo(self) -> crate::common::Reg<regs::Custominfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Used to track s/w workaround info."]
    #[inline(always)]
    pub const fn swfix(self) -> crate::common::Reg<regs::Swfix, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Software Capability Vector 0."]
    #[inline(always)]
    pub const fn swcapa0(self) -> crate::common::Reg<regs::Swcapa0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Software Capability Vector 1."]
    #[inline(always)]
    pub const fn swcapa1(self) -> crate::common::Reg<regs::Swcapa1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "External component description."]
    #[inline(always)]
    pub const fn extinfo(self) -> crate::common::Reg<regs::Extinfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)."]
    #[inline(always)]
    pub const fn eui48l(self) -> crate::common::Reg<regs::Eui48l, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MA-L compliant EUI48 OUI (high bits)."]
    #[inline(always)]
    pub const fn eui48h(self) -> crate::common::Reg<regs::Eui48h, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MA-L compliant EUI64 Unique Identifier (low bits)."]
    #[inline(always)]
    pub const fn eui64l(self) -> crate::common::Reg<regs::Eui64l, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MA-L compliant EUI64 OUI and Unique Identifier (high bits)."]
    #[inline(always)]
    pub const fn eui64h(self) -> crate::common::Reg<regs::Eui64h, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Calibration Temperature Information."]
    #[inline(always)]
    pub const fn caltemp(self) -> crate::common::Reg<regs::Caltemp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "EMU Temperature Sensor Calibration."]
    #[inline(always)]
    pub const fn emutemp(self) -> crate::common::Reg<regs::Emutemp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal0(self) -> crate::common::Reg<regs::Hfrcodpllcal0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal1(self) -> crate::common::Reg<regs::Hfrcodpllcal1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal2(self) -> crate::common::Reg<regs::Hfrcodpllcal2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal3(self) -> crate::common::Reg<regs::Hfrcodpllcal3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal4(self) -> crate::common::Reg<regs::Hfrcodpllcal4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal5(self) -> crate::common::Reg<regs::Hfrcodpllcal5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal6(self) -> crate::common::Reg<regs::Hfrcodpllcal6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal7(self) -> crate::common::Reg<regs::Hfrcodpllcal7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal8(self) -> crate::common::Reg<regs::Hfrcodpllcal8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal9(self) -> crate::common::Reg<regs::Hfrcodpllcal9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal10(self) -> crate::common::Reg<regs::Hfrcodpllcal10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal11(self) -> crate::common::Reg<regs::Hfrcodpllcal11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal12(self) -> crate::common::Reg<regs::Hfrcodpllcal12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal13(self) -> crate::common::Reg<regs::Hfrcodpllcal13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal14(self) -> crate::common::Reg<regs::Hfrcodpllcal14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal15(self) -> crate::common::Reg<regs::Hfrcodpllcal15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal16(self) -> crate::common::Reg<regs::Hfrcodpllcal16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[inline(always)]
    pub const fn hfrcodpllcal17(self) -> crate::common::Reg<regs::Hfrcodpllcal17, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal0(self) -> crate::common::Reg<regs::Hfrcoem23cal0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal1(self) -> crate::common::Reg<regs::Hfrcoem23cal1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal2(self) -> crate::common::Reg<regs::Hfrcoem23cal2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal3(self) -> crate::common::Reg<regs::Hfrcoem23cal3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal4(self) -> crate::common::Reg<regs::Hfrcoem23cal4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal5(self) -> crate::common::Reg<regs::Hfrcoem23cal5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal6(self) -> crate::common::Reg<regs::Hfrcoem23cal6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal7(self) -> crate::common::Reg<regs::Hfrcoem23cal7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal8(self) -> crate::common::Reg<regs::Hfrcoem23cal8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal9(self) -> crate::common::Reg<regs::Hfrcoem23cal9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal10(self) -> crate::common::Reg<regs::Hfrcoem23cal10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal11(self) -> crate::common::Reg<regs::Hfrcoem23cal11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal12(self) -> crate::common::Reg<regs::Hfrcoem23cal12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal13(self) -> crate::common::Reg<regs::Hfrcoem23cal13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal14(self) -> crate::common::Reg<regs::Hfrcoem23cal14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal15(self) -> crate::common::Reg<regs::Hfrcoem23cal15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal16(self) -> crate::common::Reg<regs::Hfrcoem23cal16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[inline(always)]
    pub const fn hfrcoem23cal17(self) -> crate::common::Reg<regs::Hfrcoem23cal17, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Characters 1-4 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename0(self) -> crate::common::Reg<regs::Modulename0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Characters 5-8 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename1(self) -> crate::common::Reg<regs::Modulename1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Characters 9-12 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename2(self) -> crate::common::Reg<regs::Modulename2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Characters 13-16 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename3(self) -> crate::common::Reg<regs::Modulename3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Characters 17-20 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename4(self) -> crate::common::Reg<regs::Modulename4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Characters 21-24 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename5(self) -> crate::common::Reg<regs::Modulename5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Characters 25-26 of Module Name stored as a null terminated string."]
    #[inline(always)]
    pub const fn modulename6(self) -> crate::common::Reg<regs::Modulename6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Module Information."]
    #[inline(always)]
    pub const fn moduleinfo(self) -> crate::common::Reg<regs::Moduleinfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Module Crystal Oscillator Calibration."]
    #[inline(always)]
    pub const fn modxocal(self) -> crate::common::Reg<regs::Modxocal, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "High Frequency Crystal Oscillator Calibration data."]
    #[inline(always)]
    pub const fn hfxocal(self) -> crate::common::Reg<regs::Hfxocal, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "IADC0 Gain Calibration Info."]
    #[inline(always)]
    pub const fn iadc0gain0(self) -> crate::common::Reg<regs::Iadc0gain0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "IADC0 Gain Calibration Info."]
    #[inline(always)]
    pub const fn iadc0gain1(self) -> crate::common::Reg<regs::Iadc0gain1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "IADC0 Offset Calibration Info."]
    #[inline(always)]
    pub const fn iadc0offsetcal0(self) -> crate::common::Reg<regs::Iadc0offsetcal0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "IADC0 Normal Offset Calibration Info."]
    #[inline(always)]
    pub const fn iadc0normaloffsetcal0(self) -> crate::common::Reg<regs::Iadc0normaloffsetcal0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "IADC0 Normal Offset Calibration Info."]
    #[inline(always)]
    pub const fn iadc0normaloffsetcal1(self) -> crate::common::Reg<regs::Iadc0normaloffsetcal1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "IADC High Speed Offset Calibration Info."]
    #[inline(always)]
    pub const fn iadc0hispdoffsetcal0(self) -> crate::common::Reg<regs::Iadc0hispdoffsetcal0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "IADC High Speed Offset Calibration Info."]
    #[inline(always)]
    pub const fn iadc0hispdoffsetcal1(self) -> crate::common::Reg<regs::Iadc0hispdoffsetcal1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "This is the legacy device detection information for tools compatability."]
    #[inline(always)]
    pub const fn legacy(self) -> crate::common::Reg<regs::Legacy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Calibration Temperature Information."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Caltemp(pub u32);
    impl Caltemp {
        #[doc = "Cal Temp."]
        #[must_use]
        #[inline(always)]
        pub const fn temp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Cal Temp."]
        #[inline(always)]
        pub const fn set_temp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Caltemp {
        #[inline(always)]
        fn default() -> Caltemp {
            Caltemp(0)
        }
    }
    impl core::fmt::Debug for Caltemp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Caltemp").field("temp", &self.temp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Caltemp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Caltemp {{ temp: {=u8:?} }}", self.temp())
        }
    }
    #[doc = "Custom information."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Custominfo(pub u32);
    impl Custominfo {
        #[doc = "Part Number."]
        #[must_use]
        #[inline(always)]
        pub const fn partno(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Part Number."]
        #[inline(always)]
        pub const fn set_partno(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Custominfo {
        #[inline(always)]
        fn default() -> Custominfo {
            Custominfo(0)
        }
    }
    impl core::fmt::Debug for Custominfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Custominfo").field("partno", &self.partno()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Custominfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Custominfo {{ partno: {=u16:?} }}", self.partno())
        }
    }
    #[doc = "EMU Temperature Sensor Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Emutemp(pub u32);
    impl Emutemp {
        #[doc = "Emu Room Temperature."]
        #[must_use]
        #[inline(always)]
        pub const fn emutemproom(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x01ff;
            val as u16
        }
        #[doc = "Emu Room Temperature."]
        #[inline(always)]
        pub const fn set_emutemproom(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 2usize)) | (((val as u32) & 0x01ff) << 2usize);
        }
    }
    impl Default for Emutemp {
        #[inline(always)]
        fn default() -> Emutemp {
            Emutemp(0)
        }
    }
    impl core::fmt::Debug for Emutemp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Emutemp")
                .field("emutemproom", &self.emutemproom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Emutemp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Emutemp {{ emutemproom: {=u16:?} }}", self.emutemproom())
        }
    }
    #[doc = "MA-L compliant EUI48 OUI (high bits)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eui48h(pub u32);
    impl Eui48h {
        #[doc = "OUI48H."]
        #[must_use]
        #[inline(always)]
        pub const fn oui48h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "OUI48H."]
        #[inline(always)]
        pub const fn set_oui48h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "RESERVED."]
        #[must_use]
        #[inline(always)]
        pub const fn reserved(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "RESERVED."]
        #[inline(always)]
        pub const fn set_reserved(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Eui48h {
        #[inline(always)]
        fn default() -> Eui48h {
            Eui48h(0)
        }
    }
    impl core::fmt::Debug for Eui48h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eui48h")
                .field("oui48h", &self.oui48h())
                .field("reserved", &self.reserved())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eui48h {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eui48h {{ oui48h: {=u16:?}, reserved: {=u16:?} }}",
                self.oui48h(),
                self.reserved()
            )
        }
    }
    #[doc = "MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eui48l(pub u32);
    impl Eui48l {
        #[doc = "Unique ID."]
        #[must_use]
        #[inline(always)]
        pub const fn uniqueid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Unique ID."]
        #[inline(always)]
        pub const fn set_uniqueid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "OUI48L."]
        #[must_use]
        #[inline(always)]
        pub const fn oui48l(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "OUI48L."]
        #[inline(always)]
        pub const fn set_oui48l(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Eui48l {
        #[inline(always)]
        fn default() -> Eui48l {
            Eui48l(0)
        }
    }
    impl core::fmt::Debug for Eui48l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eui48l")
                .field("uniqueid", &self.uniqueid())
                .field("oui48l", &self.oui48l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eui48l {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eui48l {{ uniqueid: {=u32:?}, oui48l: {=u8:?} }}",
                self.uniqueid(),
                self.oui48l()
            )
        }
    }
    #[doc = "MA-L compliant EUI64 OUI and Unique Identifier (high bits)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eui64h(pub u32);
    impl Eui64h {
        #[doc = "UNIQUEH."]
        #[must_use]
        #[inline(always)]
        pub const fn uniqueh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UNIQUEH."]
        #[inline(always)]
        pub const fn set_uniqueh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "OUI64."]
        #[must_use]
        #[inline(always)]
        pub const fn oui64(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OUI64."]
        #[inline(always)]
        pub const fn set_oui64(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Eui64h {
        #[inline(always)]
        fn default() -> Eui64h {
            Eui64h(0)
        }
    }
    impl core::fmt::Debug for Eui64h {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eui64h")
                .field("uniqueh", &self.uniqueh())
                .field("oui64", &self.oui64())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eui64h {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eui64h {{ uniqueh: {=u8:?}, oui64: {=u32:?} }}",
                self.uniqueh(),
                self.oui64()
            )
        }
    }
    #[doc = "MA-L compliant EUI64 Unique Identifier (low bits)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eui64l(pub u32);
    impl Eui64l {
        #[doc = "UNIQUEL."]
        #[must_use]
        #[inline(always)]
        pub const fn uniquel(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "UNIQUEL."]
        #[inline(always)]
        pub const fn set_uniquel(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Eui64l {
        #[inline(always)]
        fn default() -> Eui64l {
            Eui64l(0)
        }
    }
    impl core::fmt::Debug for Eui64l {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eui64l").field("uniquel", &self.uniquel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eui64l {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eui64l {{ uniquel: {=u32:?} }}", self.uniquel())
        }
    }
    #[doc = "External component description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extinfo(pub u32);
    impl Extinfo {
        #[doc = "Type."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> super::vals::ExtinfoType {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::ExtinfoType::from_bits(val as u8)
        }
        #[doc = "Type."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: super::vals::ExtinfoType) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Connection."]
        #[must_use]
        #[inline(always)]
        pub const fn connection(&self) -> super::vals::Connection {
            let val = (self.0 >> 8usize) & 0xff;
            super::vals::Connection::from_bits(val as u8)
        }
        #[doc = "Connection."]
        #[inline(always)]
        pub const fn set_connection(&mut self, val: super::vals::Connection) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
        }
        #[doc = "Revision."]
        #[must_use]
        #[inline(always)]
        pub const fn rev(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Revision."]
        #[inline(always)]
        pub const fn set_rev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Extinfo {
        #[inline(always)]
        fn default() -> Extinfo {
            Extinfo(0)
        }
    }
    impl core::fmt::Debug for Extinfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extinfo")
                .field("type_", &self.type_())
                .field("connection", &self.connection())
                .field("rev", &self.rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extinfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Extinfo {{ type_: {:?}, connection: {:?}, rev: {=u8:?} }}",
                self.type_(),
                self.connection(),
                self.rev()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal0(pub u32);
    impl Hfrcodpllcal0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal0 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal0 {
            Hfrcodpllcal0(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal0")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal0 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal1(pub u32);
    impl Hfrcodpllcal1 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal1 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal1 {
            Hfrcodpllcal1(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal1")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal1 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal10(pub u32);
    impl Hfrcodpllcal10 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal10 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal10 {
            Hfrcodpllcal10(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal10")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal10 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal11(pub u32);
    impl Hfrcodpllcal11 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal11 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal11 {
            Hfrcodpllcal11(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal11")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal11 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal12(pub u32);
    impl Hfrcodpllcal12 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal12 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal12 {
            Hfrcodpllcal12(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal12")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal12 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal13(pub u32);
    impl Hfrcodpllcal13 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal13 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal13 {
            Hfrcodpllcal13(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal13")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal13 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal14(pub u32);
    impl Hfrcodpllcal14 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal14 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal14 {
            Hfrcodpllcal14(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal14")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal14 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal15(pub u32);
    impl Hfrcodpllcal15 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal15 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal15 {
            Hfrcodpllcal15(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal15")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal15 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal16(pub u32);
    impl Hfrcodpllcal16 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal16 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal16 {
            Hfrcodpllcal16(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal16")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal16 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal17(pub u32);
    impl Hfrcodpllcal17 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal17 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal17 {
            Hfrcodpllcal17(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal17")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal17 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal2(pub u32);
    impl Hfrcodpllcal2 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal2 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal2 {
            Hfrcodpllcal2(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal2")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal2 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal3(pub u32);
    impl Hfrcodpllcal3 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal3 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal3 {
            Hfrcodpllcal3(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal3")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal3 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal4(pub u32);
    impl Hfrcodpllcal4 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal4 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal4 {
            Hfrcodpllcal4(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal4")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal4 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal5(pub u32);
    impl Hfrcodpllcal5 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal5 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal5 {
            Hfrcodpllcal5(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal5")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal5 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal6(pub u32);
    impl Hfrcodpllcal6 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal6 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal6 {
            Hfrcodpllcal6(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal6")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal6 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal7(pub u32);
    impl Hfrcodpllcal7 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal7 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal7 {
            Hfrcodpllcal7(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal7")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal7 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal8(pub u32);
    impl Hfrcodpllcal8 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal8 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal8 {
            Hfrcodpllcal8(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal8")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal8 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCODPLL Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcodpllcal9(pub u32);
    impl Hfrcodpllcal9 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcodpllcal9 {
        #[inline(always)]
        fn default() -> Hfrcodpllcal9 {
            Hfrcodpllcal9(0)
        }
    }
    impl core::fmt::Debug for Hfrcodpllcal9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcodpllcal9")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcodpllcal9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcodpllcal9 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal0(pub u32);
    impl Hfrcoem23cal0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal0 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal0 {
            Hfrcoem23cal0(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal0")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal0 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal1(pub u32);
    impl Hfrcoem23cal1 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal1 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal1 {
            Hfrcoem23cal1(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal1")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal1 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal10(pub u32);
    impl Hfrcoem23cal10 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal10 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal10 {
            Hfrcoem23cal10(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal10")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal10 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal11(pub u32);
    impl Hfrcoem23cal11 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal11 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal11 {
            Hfrcoem23cal11(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal11")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal11 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal12(pub u32);
    impl Hfrcoem23cal12 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal12 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal12 {
            Hfrcoem23cal12(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal12")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal12 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal13(pub u32);
    impl Hfrcoem23cal13 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal13 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal13 {
            Hfrcoem23cal13(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal13")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal13 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal14(pub u32);
    impl Hfrcoem23cal14 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal14 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal14 {
            Hfrcoem23cal14(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal14")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal14 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal15(pub u32);
    impl Hfrcoem23cal15 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal15 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal15 {
            Hfrcoem23cal15(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal15")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal15 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal16(pub u32);
    impl Hfrcoem23cal16 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal16 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal16 {
            Hfrcoem23cal16(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal16")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal16 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal17(pub u32);
    impl Hfrcoem23cal17 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal17 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal17 {
            Hfrcoem23cal17(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal17")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal17 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal2(pub u32);
    impl Hfrcoem23cal2 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal2 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal2 {
            Hfrcoem23cal2(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal2")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal2 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal3(pub u32);
    impl Hfrcoem23cal3 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal3 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal3 {
            Hfrcoem23cal3(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal3")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal3 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal4(pub u32);
    impl Hfrcoem23cal4 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal4 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal4 {
            Hfrcoem23cal4(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal4")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal4 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal5(pub u32);
    impl Hfrcoem23cal5 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal5 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal5 {
            Hfrcoem23cal5(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal5")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal5 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal6(pub u32);
    impl Hfrcoem23cal6 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal6 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal6 {
            Hfrcoem23cal6(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal6")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal6 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal7(pub u32);
    impl Hfrcoem23cal7 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal7 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal7 {
            Hfrcoem23cal7(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal7")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal7 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal8(pub u32);
    impl Hfrcoem23cal8 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal8 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal8 {
            Hfrcoem23cal8(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal8")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal8 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "HFRCOEM23 Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfrcoem23cal9(pub u32);
    impl Hfrcoem23cal9 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn tuning(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_tuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn finetuning(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_finetuning(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ldohp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ldohp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn freqrange(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_freqrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpbias(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpbias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_cmpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn ireftc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_ireftc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hfrcoem23cal9 {
        #[inline(always)]
        fn default() -> Hfrcoem23cal9 {
            Hfrcoem23cal9(0)
        }
    }
    impl core::fmt::Debug for Hfrcoem23cal9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfrcoem23cal9")
                .field("tuning", &self.tuning())
                .field("finetuning", &self.finetuning())
                .field("ldohp", &self.ldohp())
                .field("freqrange", &self.freqrange())
                .field("cmpbias", &self.cmpbias())
                .field("clkdiv", &self.clkdiv())
                .field("cmpsel", &self.cmpsel())
                .field("ireftc", &self.ireftc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfrcoem23cal9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfrcoem23cal9 {{ tuning: {=u8:?}, finetuning: {=u8:?}, ldohp: {=bool:?}, freqrange: {=u8:?}, cmpbias: {=u8:?}, clkdiv: {=u8:?}, cmpsel: {=u8:?}, ireftc: {=u8:?} }}",
                self.tuning(),
                self.finetuning(),
                self.ldohp(),
                self.freqrange(),
                self.cmpbias(),
                self.clkdiv(),
                self.cmpsel(),
                self.ireftc()
            )
        }
    }
    #[doc = "High Frequency Crystal Oscillator Calibration data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfxocal(pub u32);
    impl Hfxocal {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn shuntbiasana(&self) -> super::vals::Shuntbiasana {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Shuntbiasana::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_shuntbiasana(&mut self, val: super::vals::Shuntbiasana) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn vtrtrimana(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_vtrtrimana(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn reserved(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_reserved(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Hfxocal {
        #[inline(always)]
        fn default() -> Hfxocal {
            Hfxocal(0)
        }
    }
    impl core::fmt::Debug for Hfxocal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfxocal")
                .field("shuntbiasana", &self.shuntbiasana())
                .field("vtrtrimana", &self.vtrtrimana())
                .field("reserved", &self.reserved())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfxocal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfxocal {{ shuntbiasana: {:?}, vtrtrimana: {=u8:?}, reserved: {=u32:?} }}",
                self.shuntbiasana(),
                self.vtrtrimana(),
                self.reserved()
            )
        }
    }
    #[doc = "IADC0 Gain Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0gain0(pub u32);
    impl Iadc0gain0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn gaincana1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_gaincana1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn gaincana2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_gaincana2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Iadc0gain0 {
        #[inline(always)]
        fn default() -> Iadc0gain0 {
            Iadc0gain0(0)
        }
    }
    impl core::fmt::Debug for Iadc0gain0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0gain0")
                .field("gaincana1", &self.gaincana1())
                .field("gaincana2", &self.gaincana2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0gain0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0gain0 {{ gaincana1: {=u16:?}, gaincana2: {=u16:?} }}",
                self.gaincana1(),
                self.gaincana2()
            )
        }
    }
    #[doc = "IADC0 Gain Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0gain1(pub u32);
    impl Iadc0gain1 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn gaincana3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_gaincana3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn gaincana4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_gaincana4(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Iadc0gain1 {
        #[inline(always)]
        fn default() -> Iadc0gain1 {
            Iadc0gain1(0)
        }
    }
    impl core::fmt::Debug for Iadc0gain1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0gain1")
                .field("gaincana3", &self.gaincana3())
                .field("gaincana4", &self.gaincana4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0gain1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0gain1 {{ gaincana3: {=u16:?}, gaincana4: {=u16:?} }}",
                self.gaincana3(),
                self.gaincana4()
            )
        }
    }
    #[doc = "IADC High Speed Offset Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0hispdoffsetcal0(pub u32);
    impl Iadc0hispdoffsetcal0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana1hispd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana1hispd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana2hispd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana2hispd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Iadc0hispdoffsetcal0 {
        #[inline(always)]
        fn default() -> Iadc0hispdoffsetcal0 {
            Iadc0hispdoffsetcal0(0)
        }
    }
    impl core::fmt::Debug for Iadc0hispdoffsetcal0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0hispdoffsetcal0")
                .field("offsetana1hispd", &self.offsetana1hispd())
                .field("offsetana2hispd", &self.offsetana2hispd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0hispdoffsetcal0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0hispdoffsetcal0 {{ offsetana1hispd: {=u16:?}, offsetana2hispd: {=u16:?} }}",
                self.offsetana1hispd(),
                self.offsetana2hispd()
            )
        }
    }
    #[doc = "IADC High Speed Offset Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0hispdoffsetcal1(pub u32);
    impl Iadc0hispdoffsetcal1 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana3hispd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana3hispd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Iadc0hispdoffsetcal1 {
        #[inline(always)]
        fn default() -> Iadc0hispdoffsetcal1 {
            Iadc0hispdoffsetcal1(0)
        }
    }
    impl core::fmt::Debug for Iadc0hispdoffsetcal1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0hispdoffsetcal1")
                .field("offsetana3hispd", &self.offsetana3hispd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0hispdoffsetcal1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0hispdoffsetcal1 {{ offsetana3hispd: {=u16:?} }}",
                self.offsetana3hispd()
            )
        }
    }
    #[doc = "IADC0 Normal Offset Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0normaloffsetcal0(pub u32);
    impl Iadc0normaloffsetcal0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana1norm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana1norm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana2norm(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana2norm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Iadc0normaloffsetcal0 {
        #[inline(always)]
        fn default() -> Iadc0normaloffsetcal0 {
            Iadc0normaloffsetcal0(0)
        }
    }
    impl core::fmt::Debug for Iadc0normaloffsetcal0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0normaloffsetcal0")
                .field("offsetana1norm", &self.offsetana1norm())
                .field("offsetana2norm", &self.offsetana2norm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0normaloffsetcal0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0normaloffsetcal0 {{ offsetana1norm: {=u16:?}, offsetana2norm: {=u16:?} }}",
                self.offsetana1norm(),
                self.offsetana2norm()
            )
        }
    }
    #[doc = "IADC0 Normal Offset Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0normaloffsetcal1(pub u32);
    impl Iadc0normaloffsetcal1 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana3norm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana3norm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Iadc0normaloffsetcal1 {
        #[inline(always)]
        fn default() -> Iadc0normaloffsetcal1 {
            Iadc0normaloffsetcal1(0)
        }
    }
    impl core::fmt::Debug for Iadc0normaloffsetcal1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0normaloffsetcal1")
                .field("offsetana3norm", &self.offsetana3norm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0normaloffsetcal1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0normaloffsetcal1 {{ offsetana3norm: {=u16:?} }}",
                self.offsetana3norm()
            )
        }
    }
    #[doc = "IADC0 Offset Calibration Info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadc0offsetcal0(pub u32);
    impl Iadc0offsetcal0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetanabase(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetanabase(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn offsetana1hiacc(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_offsetana1hiacc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Iadc0offsetcal0 {
        #[inline(always)]
        fn default() -> Iadc0offsetcal0 {
            Iadc0offsetcal0(0)
        }
    }
    impl core::fmt::Debug for Iadc0offsetcal0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadc0offsetcal0")
                .field("offsetanabase", &self.offsetanabase())
                .field("offsetana1hiacc", &self.offsetana1hiacc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadc0offsetcal0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Iadc0offsetcal0 {{ offsetanabase: {=u16:?}, offsetana1hiacc: {=u16:?} }}",
                self.offsetanabase(),
                self.offsetana1hiacc()
            )
        }
    }
    #[doc = "Version of the device info structure being used."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Info(pub u32);
    impl Info {
        #[doc = "CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "CRC."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Production Revision."]
        #[must_use]
        #[inline(always)]
        pub const fn prodrev(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Production Revision."]
        #[inline(always)]
        pub const fn set_prodrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "DI Page Version."]
        #[must_use]
        #[inline(always)]
        pub const fn devinforev(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "DI Page Version."]
        #[inline(always)]
        pub const fn set_devinforev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Info {
        #[inline(always)]
        fn default() -> Info {
            Info(0)
        }
    }
    impl core::fmt::Debug for Info {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Info")
                .field("crc", &self.crc())
                .field("prodrev", &self.prodrev())
                .field("devinforev", &self.devinforev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Info {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Info {{ crc: {=u16:?}, prodrev: {=u8:?}, devinforev: {=u8:?} }}",
                self.crc(),
                self.prodrev(),
                self.devinforev()
            )
        }
    }
    #[doc = "This is the legacy device detection information for tools compatability."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Legacy(pub u32);
    impl Legacy {
        #[doc = "Device Family."]
        #[must_use]
        #[inline(always)]
        pub const fn devicefamily(&self) -> super::vals::Devicefamily {
            let val = (self.0 >> 16usize) & 0xff;
            super::vals::Devicefamily::from_bits(val as u8)
        }
        #[doc = "Device Family."]
        #[inline(always)]
        pub const fn set_devicefamily(&mut self, val: super::vals::Devicefamily) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Legacy {
        #[inline(always)]
        fn default() -> Legacy {
            Legacy(0)
        }
    }
    impl core::fmt::Debug for Legacy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Legacy")
                .field("devicefamily", &self.devicefamily())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Legacy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Legacy {{ devicefamily: {:?} }}", self.devicefamily())
        }
    }
    #[doc = "Flash page size and misc. chip information."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Meminfo(pub u32);
    impl Meminfo {
        #[doc = "Flash Page Size."]
        #[must_use]
        #[inline(always)]
        pub const fn flashpagesize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Flash Page Size."]
        #[inline(always)]
        pub const fn set_flashpagesize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "User Data Page Size."]
        #[must_use]
        #[inline(always)]
        pub const fn udpagesize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "User Data Page Size."]
        #[inline(always)]
        pub const fn set_udpagesize(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Length of DI Page."]
        #[must_use]
        #[inline(always)]
        pub const fn dilen(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Length of DI Page."]
        #[inline(always)]
        pub const fn set_dilen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Meminfo {
        #[inline(always)]
        fn default() -> Meminfo {
            Meminfo(0)
        }
    }
    impl core::fmt::Debug for Meminfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Meminfo")
                .field("flashpagesize", &self.flashpagesize())
                .field("udpagesize", &self.udpagesize())
                .field("dilen", &self.dilen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Meminfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Meminfo {{ flashpagesize: {=u8:?}, udpagesize: {=u8:?}, dilen: {=u16:?} }}",
                self.flashpagesize(),
                self.udpagesize(),
                self.dilen()
            )
        }
    }
    #[doc = "Module Information."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Moduleinfo(pub u32);
    impl Moduleinfo {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn hwrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_hwrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn antenna(&self) -> super::vals::Antenna {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Antenna::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_antenna(&mut self, val: super::vals::Antenna) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modnumber(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modnumber(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> super::vals::ModuleinfoType {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::ModuleinfoType::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: super::vals::ModuleinfoType) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn lfxo(&self) -> super::vals::Lfxo {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Lfxo::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_lfxo(&mut self, val: super::vals::Lfxo) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn express(&self) -> super::vals::Express {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Express::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_express(&mut self, val: super::vals::Express) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn lfxocalval(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_lfxocalval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn hfxocalval(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_hfxocalval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modnumbermsb(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x01ff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modnumbermsb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val as u32) & 0x01ff) << 20usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn padcdc(&self) -> super::vals::Padcdc {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Padcdc::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_padcdc(&mut self, val: super::vals::Padcdc) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn phylimited(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_phylimited(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn extvalid(&self) -> super::vals::Extvalid {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Extvalid::from_bits(val as u8)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_extvalid(&mut self, val: super::vals::Extvalid) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Moduleinfo {
        #[inline(always)]
        fn default() -> Moduleinfo {
            Moduleinfo(0)
        }
    }
    impl core::fmt::Debug for Moduleinfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Moduleinfo")
                .field("hwrev", &self.hwrev())
                .field("antenna", &self.antenna())
                .field("modnumber", &self.modnumber())
                .field("type_", &self.type_())
                .field("lfxo", &self.lfxo())
                .field("express", &self.express())
                .field("lfxocalval", &self.lfxocalval())
                .field("hfxocalval", &self.hfxocalval())
                .field("modnumbermsb", &self.modnumbermsb())
                .field("padcdc", &self.padcdc())
                .field("phylimited", &self.phylimited())
                .field("extvalid", &self.extvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Moduleinfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Moduleinfo {{ hwrev: {=u8:?}, antenna: {:?}, modnumber: {=u8:?}, type_: {:?}, lfxo: {:?}, express: {:?}, lfxocalval: {=bool:?}, hfxocalval: {=bool:?}, modnumbermsb: {=u16:?}, padcdc: {:?}, phylimited: {=bool:?}, extvalid: {:?} }}",
                self.hwrev(),
                self.antenna(),
                self.modnumber(),
                self.type_(),
                self.lfxo(),
                self.express(),
                self.lfxocalval(),
                self.hfxocalval(),
                self.modnumbermsb(),
                self.padcdc(),
                self.phylimited(),
                self.extvalid()
            )
        }
    }
    #[doc = "Characters 1-4 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename0(pub u32);
    impl Modulename0 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Modulename0 {
        #[inline(always)]
        fn default() -> Modulename0 {
            Modulename0(0)
        }
    }
    impl core::fmt::Debug for Modulename0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename0")
                .field("modchar1", &self.modchar1())
                .field("modchar2", &self.modchar2())
                .field("modchar3", &self.modchar3())
                .field("modchar4", &self.modchar4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename0 {{ modchar1: {=u8:?}, modchar2: {=u8:?}, modchar3: {=u8:?}, modchar4: {=u8:?} }}",
                self.modchar1(),
                self.modchar2(),
                self.modchar3(),
                self.modchar4()
            )
        }
    }
    #[doc = "Characters 5-8 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename1(pub u32);
    impl Modulename1 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar7(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar8(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar8(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Modulename1 {
        #[inline(always)]
        fn default() -> Modulename1 {
            Modulename1(0)
        }
    }
    impl core::fmt::Debug for Modulename1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename1")
                .field("modchar5", &self.modchar5())
                .field("modchar6", &self.modchar6())
                .field("modchar7", &self.modchar7())
                .field("modchar8", &self.modchar8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename1 {{ modchar5: {=u8:?}, modchar6: {=u8:?}, modchar7: {=u8:?}, modchar8: {=u8:?} }}",
                self.modchar5(),
                self.modchar6(),
                self.modchar7(),
                self.modchar8()
            )
        }
    }
    #[doc = "Characters 9-12 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename2(pub u32);
    impl Modulename2 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar9(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar9(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar10(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar11(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar11(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar12(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar12(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Modulename2 {
        #[inline(always)]
        fn default() -> Modulename2 {
            Modulename2(0)
        }
    }
    impl core::fmt::Debug for Modulename2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename2")
                .field("modchar9", &self.modchar9())
                .field("modchar10", &self.modchar10())
                .field("modchar11", &self.modchar11())
                .field("modchar12", &self.modchar12())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename2 {{ modchar9: {=u8:?}, modchar10: {=u8:?}, modchar11: {=u8:?}, modchar12: {=u8:?} }}",
                self.modchar9(),
                self.modchar10(),
                self.modchar11(),
                self.modchar12()
            )
        }
    }
    #[doc = "Characters 13-16 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename3(pub u32);
    impl Modulename3 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar13(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar13(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar14(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar14(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar15(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar15(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar16(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar16(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Modulename3 {
        #[inline(always)]
        fn default() -> Modulename3 {
            Modulename3(0)
        }
    }
    impl core::fmt::Debug for Modulename3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename3")
                .field("modchar13", &self.modchar13())
                .field("modchar14", &self.modchar14())
                .field("modchar15", &self.modchar15())
                .field("modchar16", &self.modchar16())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename3 {{ modchar13: {=u8:?}, modchar14: {=u8:?}, modchar15: {=u8:?}, modchar16: {=u8:?} }}",
                self.modchar13(),
                self.modchar14(),
                self.modchar15(),
                self.modchar16()
            )
        }
    }
    #[doc = "Characters 17-20 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename4(pub u32);
    impl Modulename4 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar17(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar17(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar18(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar18(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar19(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar19(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar20(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar20(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Modulename4 {
        #[inline(always)]
        fn default() -> Modulename4 {
            Modulename4(0)
        }
    }
    impl core::fmt::Debug for Modulename4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename4")
                .field("modchar17", &self.modchar17())
                .field("modchar18", &self.modchar18())
                .field("modchar19", &self.modchar19())
                .field("modchar20", &self.modchar20())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename4 {{ modchar17: {=u8:?}, modchar18: {=u8:?}, modchar19: {=u8:?}, modchar20: {=u8:?} }}",
                self.modchar17(),
                self.modchar18(),
                self.modchar19(),
                self.modchar20()
            )
        }
    }
    #[doc = "Characters 21-24 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename5(pub u32);
    impl Modulename5 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar21(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar21(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar22(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar22(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar23(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar23(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar24(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar24(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Modulename5 {
        #[inline(always)]
        fn default() -> Modulename5 {
            Modulename5(0)
        }
    }
    impl core::fmt::Debug for Modulename5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename5")
                .field("modchar21", &self.modchar21())
                .field("modchar22", &self.modchar22())
                .field("modchar23", &self.modchar23())
                .field("modchar24", &self.modchar24())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename5 {{ modchar21: {=u8:?}, modchar22: {=u8:?}, modchar23: {=u8:?}, modchar24: {=u8:?} }}",
                self.modchar21(),
                self.modchar22(),
                self.modchar23(),
                self.modchar24()
            )
        }
    }
    #[doc = "Characters 25-26 of Module Name stored as a null terminated string."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modulename6(pub u32);
    impl Modulename6 {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar25(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar25(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn modchar26(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_modchar26(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Modulename6 {
        #[inline(always)]
        fn default() -> Modulename6 {
            Modulename6(0)
        }
    }
    impl core::fmt::Debug for Modulename6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modulename6")
                .field("modchar25", &self.modchar25())
                .field("modchar26", &self.modchar26())
                .field("rsv", &self.rsv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modulename6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modulename6 {{ modchar25: {=u8:?}, modchar26: {=u8:?}, rsv: {=u16:?} }}",
                self.modchar25(),
                self.modchar26(),
                self.rsv()
            )
        }
    }
    #[doc = "Module Crystal Oscillator Calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Modxocal(pub u32);
    impl Modxocal {
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn hfxoctunexiana(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_hfxoctunexiana(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn hfxoctunexoana(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_hfxoctunexoana(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn lfxocaptune(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_lfxocaptune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Modxocal {
        #[inline(always)]
        fn default() -> Modxocal {
            Modxocal(0)
        }
    }
    impl core::fmt::Debug for Modxocal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Modxocal")
                .field("hfxoctunexiana", &self.hfxoctunexiana())
                .field("hfxoctunexoana", &self.hfxoctunexoana())
                .field("lfxocaptune", &self.lfxocaptune())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Modxocal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Modxocal {{ hfxoctunexiana: {=u8:?}, hfxoctunexoana: {=u8:?}, lfxocaptune: {=u8:?} }}",
                self.hfxoctunexiana(),
                self.hfxoctunexoana(),
                self.lfxocaptune()
            )
        }
    }
    #[doc = "Flash and SRAM Memory size in kB."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msize(pub u32);
    impl Msize {
        #[doc = "Flash Size."]
        #[must_use]
        #[inline(always)]
        pub const fn flash(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Flash Size."]
        #[inline(always)]
        pub const fn set_flash(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Sram Size."]
        #[must_use]
        #[inline(always)]
        pub const fn sram(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Sram Size."]
        #[inline(always)]
        pub const fn set_sram(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Msize {
        #[inline(always)]
        fn default() -> Msize {
            Msize(0)
        }
    }
    impl core::fmt::Debug for Msize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msize")
                .field("flash", &self.flash())
                .field("sram", &self.sram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Msize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Msize {{ flash: {=u16:?}, sram: {=u16:?} }}",
                self.flash(),
                self.sram()
            )
        }
    }
    #[doc = "Part description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Part(pub u32);
    impl Part {
        #[doc = "Device Number."]
        #[must_use]
        #[inline(always)]
        pub const fn devicenum(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Device Number."]
        #[inline(always)]
        pub const fn set_devicenum(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Device Family."]
        #[must_use]
        #[inline(always)]
        pub const fn familynum(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Device Family."]
        #[inline(always)]
        pub const fn set_familynum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Device Family."]
        #[must_use]
        #[inline(always)]
        pub const fn family(&self) -> super::vals::Family {
            let val = (self.0 >> 24usize) & 0x3f;
            super::vals::Family::from_bits(val as u8)
        }
        #[doc = "Device Family."]
        #[inline(always)]
        pub const fn set_family(&mut self, val: super::vals::Family) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Part {
        #[inline(always)]
        fn default() -> Part {
            Part(0)
        }
    }
    impl core::fmt::Debug for Part {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Part")
                .field("devicenum", &self.devicenum())
                .field("familynum", &self.familynum())
                .field("family", &self.family())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Part {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Part {{ devicenum: {=u16:?}, familynum: {=u8:?}, family: {:?} }}",
                self.devicenum(),
                self.familynum(),
                self.family()
            )
        }
    }
    #[doc = "Miscellaneous device information."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pkginfo(pub u32);
    impl Pkginfo {
        #[doc = "Temperature Grade."]
        #[must_use]
        #[inline(always)]
        pub const fn tempgrade(&self) -> super::vals::Tempgrade {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Tempgrade::from_bits(val as u8)
        }
        #[doc = "Temperature Grade."]
        #[inline(always)]
        pub const fn set_tempgrade(&mut self, val: super::vals::Tempgrade) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Package Type."]
        #[must_use]
        #[inline(always)]
        pub const fn pkgtype(&self) -> super::vals::Pkgtype {
            let val = (self.0 >> 8usize) & 0xff;
            super::vals::Pkgtype::from_bits(val as u8)
        }
        #[doc = "Package Type."]
        #[inline(always)]
        pub const fn set_pkgtype(&mut self, val: super::vals::Pkgtype) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
        }
        #[doc = "Pin Count."]
        #[must_use]
        #[inline(always)]
        pub const fn pincount(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Pin Count."]
        #[inline(always)]
        pub const fn set_pincount(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Pkginfo {
        #[inline(always)]
        fn default() -> Pkginfo {
            Pkginfo(0)
        }
    }
    impl core::fmt::Debug for Pkginfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pkginfo")
                .field("tempgrade", &self.tempgrade())
                .field("pkgtype", &self.pkgtype())
                .field("pincount", &self.pincount())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pkginfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pkginfo {{ tempgrade: {:?}, pkgtype: {:?}, pincount: {=u8:?} }}",
                self.tempgrade(),
                self.pkgtype(),
                self.pincount()
            )
        }
    }
    #[doc = "Software Capability Vector 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swcapa0(pub u32);
    impl Swcapa0 {
        #[doc = "Zigbee Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn zigbee(&self) -> super::vals::Zigbee {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Zigbee::from_bits(val as u8)
        }
        #[doc = "Zigbee Capability."]
        #[inline(always)]
        pub const fn set_zigbee(&mut self, val: super::vals::Zigbee) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Thread Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn thread(&self) -> super::vals::Thread {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Thread::from_bits(val as u8)
        }
        #[doc = "Thread Capability."]
        #[inline(always)]
        pub const fn set_thread(&mut self, val: super::vals::Thread) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "RF4CE Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn rf4ce(&self) -> super::vals::Rf4ce {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rf4ce::from_bits(val as u8)
        }
        #[doc = "RF4CE Capability."]
        #[inline(always)]
        pub const fn set_rf4ce(&mut self, val: super::vals::Rf4ce) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Bluetooth Smart Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn btsmart(&self) -> super::vals::Btsmart {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Btsmart::from_bits(val as u8)
        }
        #[doc = "Bluetooth Smart Capability."]
        #[inline(always)]
        pub const fn set_btsmart(&mut self, val: super::vals::Btsmart) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Connect Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn connect(&self) -> super::vals::Connect {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Connect::from_bits(val as u8)
        }
        #[doc = "Connect Capability."]
        #[inline(always)]
        pub const fn set_connect(&mut self, val: super::vals::Connect) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "RAIL Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn sri(&self) -> super::vals::Sri {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Sri::from_bits(val as u8)
        }
        #[doc = "RAIL Capability."]
        #[inline(always)]
        pub const fn set_sri(&mut self, val: super::vals::Sri) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Z-Wave Capability."]
        #[must_use]
        #[inline(always)]
        pub const fn zwave(&self) -> super::vals::Zwave {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Zwave::from_bits(val as u8)
        }
        #[doc = "Z-Wave Capability."]
        #[inline(always)]
        pub const fn set_zwave(&mut self, val: super::vals::Zwave) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Swcapa0 {
        #[inline(always)]
        fn default() -> Swcapa0 {
            Swcapa0(0)
        }
    }
    impl core::fmt::Debug for Swcapa0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swcapa0")
                .field("zigbee", &self.zigbee())
                .field("thread", &self.thread())
                .field("rf4ce", &self.rf4ce())
                .field("btsmart", &self.btsmart())
                .field("connect", &self.connect())
                .field("sri", &self.sri())
                .field("zwave", &self.zwave())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swcapa0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swcapa0 {{ zigbee: {:?}, thread: {:?}, rf4ce: {:?}, btsmart: {:?}, connect: {:?}, sri: {:?}, zwave: {:?} }}",
                self.zigbee(),
                self.thread(),
                self.rf4ce(),
                self.btsmart(),
                self.connect(),
                self.sri(),
                self.zwave()
            )
        }
    }
    #[doc = "Software Capability Vector 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swcapa1(pub u32);
    impl Swcapa1 {
        #[doc = "RF-MCU."]
        #[must_use]
        #[inline(always)]
        pub const fn rfmcuen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RF-MCU."]
        #[inline(always)]
        pub const fn set_rfmcuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NCP."]
        #[must_use]
        #[inline(always)]
        pub const fn ncpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "NCP."]
        #[inline(always)]
        pub const fn set_ncpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Gateway."]
        #[must_use]
        #[inline(always)]
        pub const fn gwen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Gateway."]
        #[inline(always)]
        pub const fn set_gwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "XOUT."]
        #[must_use]
        #[inline(always)]
        pub const fn xout(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "XOUT."]
        #[inline(always)]
        pub const fn set_xout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Swcapa1 {
        #[inline(always)]
        fn default() -> Swcapa1 {
            Swcapa1(0)
        }
    }
    impl core::fmt::Debug for Swcapa1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swcapa1")
                .field("rfmcuen", &self.rfmcuen())
                .field("ncpen", &self.ncpen())
                .field("gwen", &self.gwen())
                .field("xout", &self.xout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swcapa1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swcapa1 {{ rfmcuen: {=bool:?}, ncpen: {=bool:?}, gwen: {=bool:?}, xout: {=bool:?} }}",
                self.rfmcuen(),
                self.ncpen(),
                self.gwen(),
                self.xout()
            )
        }
    }
    #[doc = "Used to track s/w workaround info."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swfix(pub u32);
    impl Swfix {
        #[doc = "Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn rsv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Reserved."]
        #[inline(always)]
        pub const fn set_rsv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Swfix {
        #[inline(always)]
        fn default() -> Swfix {
            Swfix(0)
        }
    }
    impl core::fmt::Debug for Swfix {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swfix").field("rsv", &self.rsv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swfix {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Swfix {{ rsv: {=u32:?} }}", self.rsv())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Antenna {
        #[doc = "BUILTIN."]
        Builtin = 0x0,
        #[doc = "CONNECTOR."]
        Connector = 0x01,
        #[doc = "RFPAD."]
        Rfpad = 0x02,
        #[doc = "INVERTEDF."]
        Invertedf = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Antenna {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Antenna {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Antenna {
        #[inline(always)]
        fn from(val: u8) -> Antenna {
            Antenna::from_bits(val)
        }
    }
    impl From<Antenna> for u8 {
        #[inline(always)]
        fn from(val: Antenna) -> u8 {
            Antenna::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Btsmart {
        #[doc = "Bluetooth SMART stack capability not available."]
        Level0 = 0x0,
        #[doc = "Bluetooth SMART enabled."]
        Level1 = 0x01,
        #[doc = "N/A."]
        Level2 = 0x02,
        #[doc = "N/A."]
        Level3 = 0x03,
    }
    impl Btsmart {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Btsmart {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Btsmart {
        #[inline(always)]
        fn from(val: u8) -> Btsmart {
            Btsmart::from_bits(val)
        }
    }
    impl From<Btsmart> for u8 {
        #[inline(always)]
        fn from(val: Btsmart) -> u8 {
            Btsmart::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Connect {
        #[doc = "Connect stack capability not available."]
        Level0 = 0x0,
        #[doc = "Connect enabled."]
        Level1 = 0x01,
        #[doc = "N/A."]
        Level2 = 0x02,
        #[doc = "N/A."]
        Level3 = 0x03,
    }
    impl Connect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Connect {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Connect {
        #[inline(always)]
        fn from(val: u8) -> Connect {
            Connect::from_bits(val)
        }
    }
    impl From<Connect> for u8 {
        #[inline(always)]
        fn from(val: Connect) -> u8 {
            Connect::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Connection(u8);
    impl Connection {
        #[doc = "SPI control interface."]
        pub const Spi: Self = Self(0x0);
        #[doc = "No interface."]
        pub const None: Self = Self(0xff);
    }
    impl Connection {
        pub const fn from_bits(val: u8) -> Connection {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Connection {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Spi"),
                0xff => f.write_str("None"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Connection {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Spi"),
                0xff => defmt::write!(f, "None"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Connection {
        #[inline(always)]
        fn from(val: u8) -> Connection {
            Connection::from_bits(val)
        }
    }
    impl From<Connection> for u8 {
        #[inline(always)]
        fn from(val: Connection) -> u8 {
            Connection::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Devicefamily(u8);
    impl Devicefamily {
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 1."]
        pub const Efr32mg1p: Self = Self(0x10);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 1."]
        pub const Efr32mg1b: Self = Self(0x11);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 1."]
        pub const Efr32mg1v: Self = Self(0x12);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 1."]
        pub const Efr32bg1p: Self = Self(0x13);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 1."]
        pub const Efr32bg1b: Self = Self(0x14);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 1."]
        pub const Efr32bg1v: Self = Self(0x15);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 1."]
        pub const Efr32fg1p: Self = Self(0x19);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 1."]
        pub const Efr32fg1b: Self = Self(0x1a);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 1."]
        pub const Efr32fg1v: Self = Self(0x1b);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 2."]
        pub const Efr32mg12p: Self = Self(0x1c);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 2."]
        pub const Efr32mg12b: Self = Self(0x1d);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 2."]
        pub const Efr32mg12v: Self = Self(0x1e);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 2."]
        pub const Efr32bg12p: Self = Self(0x1f);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 2."]
        pub const Efr32bg12b: Self = Self(0x20);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 2."]
        pub const Efr32bg12v: Self = Self(0x21);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 2."]
        pub const Efr32fg12p: Self = Self(0x25);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 2."]
        pub const Efr32fg12b: Self = Self(0x26);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 2."]
        pub const Efr32fg12v: Self = Self(0x27);
        #[doc = "EFR32 Mighty Gecko Family Series 13 Device Config 3."]
        pub const Efr32mg13p: Self = Self(0x28);
        #[doc = "EFR32 Mighty Gecko Family Series 13 Device Config 3."]
        pub const Efr32mg13b: Self = Self(0x29);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 3."]
        pub const Efr32mg13v: Self = Self(0x2a);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 3."]
        pub const Efr32bg13p: Self = Self(0x2b);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 3."]
        pub const Efr32bg13b: Self = Self(0x2c);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 3."]
        pub const Efr32bg13v: Self = Self(0x2d);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 3."]
        pub const Efr32fg13p: Self = Self(0x31);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 3."]
        pub const Efr32fg13b: Self = Self(0x32);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 3."]
        pub const Efr32fg13v: Self = Self(0x33);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 4."]
        pub const Efr32mg14p: Self = Self(0x34);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 4."]
        pub const Efr32mg14b: Self = Self(0x35);
        #[doc = "EFR32 Mighty Gecko Family Series 1 Device Config 4."]
        pub const Efr32mg14v: Self = Self(0x36);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 4."]
        pub const Efr32bg14p: Self = Self(0x37);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 4."]
        pub const Efr32bg14b: Self = Self(0x38);
        #[doc = "EFR32 Blue Gecko Family Series 1 Device Config 4."]
        pub const Efr32bg14v: Self = Self(0x39);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 4."]
        pub const Efr32fg14p: Self = Self(0x3d);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 4."]
        pub const Efr32fg14b: Self = Self(0x3e);
        #[doc = "EFR32 Flex Gecko Family Series 1 Device Config 4."]
        pub const Efr32fg14v: Self = Self(0x3f);
        #[doc = "EFM32 Gecko Device Family."]
        pub const Efm32g: Self = Self(0x47);
        #[doc = "EFM32 Giant Gecko Device Family."]
        pub const Efm32gg: Self = Self(0x48);
        #[doc = "EFM32 Tiny Gecko Device Family."]
        pub const Efm32tg: Self = Self(0x49);
        #[doc = "EFM32 Leopard Gecko Device Family."]
        pub const Efm32lg: Self = Self(0x4a);
        #[doc = "EFM32 Wonder Gecko Device Family."]
        pub const Efm32wg: Self = Self(0x4b);
        #[doc = "EFM32 Zero Gecko Device Family."]
        pub const Efm32zg: Self = Self(0x4c);
        #[doc = "EFM32 Happy Gecko Device Family."]
        pub const Efm32hg: Self = Self(0x4d);
        #[doc = "EFM32 Pearl Gecko Device Family Series 1 Device Config 1."]
        pub const Efm32pg1b: Self = Self(0x51);
        #[doc = "EFM32 Jade Gecko Device Family Series 1 Device Config 1."]
        pub const Efm32jg1b: Self = Self(0x53);
        #[doc = "EFM32 Pearl Gecko Device Family Series 1 Device Config 2."]
        pub const Efm32pg12b: Self = Self(0x55);
        #[doc = "EFM32 Jade Gecko Device Family Series 1 Device Config 2."]
        pub const Efm32jg12b: Self = Self(0x57);
        #[doc = "EFM32 Pearl Gecko Device Family Series 1 Device Config 3."]
        pub const Efm32pg13b: Self = Self(0x59);
        #[doc = "EFM32 Jade Gecko Device Family Series 1 Device Config 3."]
        pub const Efm32jg13b: Self = Self(0x5b);
        #[doc = "EFM32 Giant Gecko Device Family Series 1 Device Config 1."]
        pub const Efm32gg11b: Self = Self(0x64);
        #[doc = "EFM32 Giant Gecko Device Family Series 1 Device Config 1."]
        pub const Efm32tg11b: Self = Self(0x67);
        #[doc = "EZR32 Leopard Gecko Device Family."]
        pub const Ezr32lg: Self = Self(0x78);
        #[doc = "EZR32 Wonder Gecko Device Family."]
        pub const Ezr32wg: Self = Self(0x79);
        #[doc = "EZR32 Happy Gecko Device Family."]
        pub const Ezr32hg: Self = Self(0x7a);
        #[doc = "DI page is encoded with the series 2 layout. Check alternate location."]
        pub const Series2v0: Self = Self(0x80);
    }
    impl Devicefamily {
        pub const fn from_bits(val: u8) -> Devicefamily {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Devicefamily {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x10 => f.write_str("Efr32mg1p"),
                0x11 => f.write_str("Efr32mg1b"),
                0x12 => f.write_str("Efr32mg1v"),
                0x13 => f.write_str("Efr32bg1p"),
                0x14 => f.write_str("Efr32bg1b"),
                0x15 => f.write_str("Efr32bg1v"),
                0x19 => f.write_str("Efr32fg1p"),
                0x1a => f.write_str("Efr32fg1b"),
                0x1b => f.write_str("Efr32fg1v"),
                0x1c => f.write_str("Efr32mg12p"),
                0x1d => f.write_str("Efr32mg12b"),
                0x1e => f.write_str("Efr32mg12v"),
                0x1f => f.write_str("Efr32bg12p"),
                0x20 => f.write_str("Efr32bg12b"),
                0x21 => f.write_str("Efr32bg12v"),
                0x25 => f.write_str("Efr32fg12p"),
                0x26 => f.write_str("Efr32fg12b"),
                0x27 => f.write_str("Efr32fg12v"),
                0x28 => f.write_str("Efr32mg13p"),
                0x29 => f.write_str("Efr32mg13b"),
                0x2a => f.write_str("Efr32mg13v"),
                0x2b => f.write_str("Efr32bg13p"),
                0x2c => f.write_str("Efr32bg13b"),
                0x2d => f.write_str("Efr32bg13v"),
                0x31 => f.write_str("Efr32fg13p"),
                0x32 => f.write_str("Efr32fg13b"),
                0x33 => f.write_str("Efr32fg13v"),
                0x34 => f.write_str("Efr32mg14p"),
                0x35 => f.write_str("Efr32mg14b"),
                0x36 => f.write_str("Efr32mg14v"),
                0x37 => f.write_str("Efr32bg14p"),
                0x38 => f.write_str("Efr32bg14b"),
                0x39 => f.write_str("Efr32bg14v"),
                0x3d => f.write_str("Efr32fg14p"),
                0x3e => f.write_str("Efr32fg14b"),
                0x3f => f.write_str("Efr32fg14v"),
                0x47 => f.write_str("Efm32g"),
                0x48 => f.write_str("Efm32gg"),
                0x49 => f.write_str("Efm32tg"),
                0x4a => f.write_str("Efm32lg"),
                0x4b => f.write_str("Efm32wg"),
                0x4c => f.write_str("Efm32zg"),
                0x4d => f.write_str("Efm32hg"),
                0x51 => f.write_str("Efm32pg1b"),
                0x53 => f.write_str("Efm32jg1b"),
                0x55 => f.write_str("Efm32pg12b"),
                0x57 => f.write_str("Efm32jg12b"),
                0x59 => f.write_str("Efm32pg13b"),
                0x5b => f.write_str("Efm32jg13b"),
                0x64 => f.write_str("Efm32gg11b"),
                0x67 => f.write_str("Efm32tg11b"),
                0x78 => f.write_str("Ezr32lg"),
                0x79 => f.write_str("Ezr32wg"),
                0x7a => f.write_str("Ezr32hg"),
                0x80 => f.write_str("Series2v0"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devicefamily {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x10 => defmt::write!(f, "Efr32mg1p"),
                0x11 => defmt::write!(f, "Efr32mg1b"),
                0x12 => defmt::write!(f, "Efr32mg1v"),
                0x13 => defmt::write!(f, "Efr32bg1p"),
                0x14 => defmt::write!(f, "Efr32bg1b"),
                0x15 => defmt::write!(f, "Efr32bg1v"),
                0x19 => defmt::write!(f, "Efr32fg1p"),
                0x1a => defmt::write!(f, "Efr32fg1b"),
                0x1b => defmt::write!(f, "Efr32fg1v"),
                0x1c => defmt::write!(f, "Efr32mg12p"),
                0x1d => defmt::write!(f, "Efr32mg12b"),
                0x1e => defmt::write!(f, "Efr32mg12v"),
                0x1f => defmt::write!(f, "Efr32bg12p"),
                0x20 => defmt::write!(f, "Efr32bg12b"),
                0x21 => defmt::write!(f, "Efr32bg12v"),
                0x25 => defmt::write!(f, "Efr32fg12p"),
                0x26 => defmt::write!(f, "Efr32fg12b"),
                0x27 => defmt::write!(f, "Efr32fg12v"),
                0x28 => defmt::write!(f, "Efr32mg13p"),
                0x29 => defmt::write!(f, "Efr32mg13b"),
                0x2a => defmt::write!(f, "Efr32mg13v"),
                0x2b => defmt::write!(f, "Efr32bg13p"),
                0x2c => defmt::write!(f, "Efr32bg13b"),
                0x2d => defmt::write!(f, "Efr32bg13v"),
                0x31 => defmt::write!(f, "Efr32fg13p"),
                0x32 => defmt::write!(f, "Efr32fg13b"),
                0x33 => defmt::write!(f, "Efr32fg13v"),
                0x34 => defmt::write!(f, "Efr32mg14p"),
                0x35 => defmt::write!(f, "Efr32mg14b"),
                0x36 => defmt::write!(f, "Efr32mg14v"),
                0x37 => defmt::write!(f, "Efr32bg14p"),
                0x38 => defmt::write!(f, "Efr32bg14b"),
                0x39 => defmt::write!(f, "Efr32bg14v"),
                0x3d => defmt::write!(f, "Efr32fg14p"),
                0x3e => defmt::write!(f, "Efr32fg14b"),
                0x3f => defmt::write!(f, "Efr32fg14v"),
                0x47 => defmt::write!(f, "Efm32g"),
                0x48 => defmt::write!(f, "Efm32gg"),
                0x49 => defmt::write!(f, "Efm32tg"),
                0x4a => defmt::write!(f, "Efm32lg"),
                0x4b => defmt::write!(f, "Efm32wg"),
                0x4c => defmt::write!(f, "Efm32zg"),
                0x4d => defmt::write!(f, "Efm32hg"),
                0x51 => defmt::write!(f, "Efm32pg1b"),
                0x53 => defmt::write!(f, "Efm32jg1b"),
                0x55 => defmt::write!(f, "Efm32pg12b"),
                0x57 => defmt::write!(f, "Efm32jg12b"),
                0x59 => defmt::write!(f, "Efm32pg13b"),
                0x5b => defmt::write!(f, "Efm32jg13b"),
                0x64 => defmt::write!(f, "Efm32gg11b"),
                0x67 => defmt::write!(f, "Efm32tg11b"),
                0x78 => defmt::write!(f, "Ezr32lg"),
                0x79 => defmt::write!(f, "Ezr32wg"),
                0x7a => defmt::write!(f, "Ezr32hg"),
                0x80 => defmt::write!(f, "Series2v0"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Devicefamily {
        #[inline(always)]
        fn from(val: u8) -> Devicefamily {
            Devicefamily::from_bits(val)
        }
    }
    impl From<Devicefamily> for u8 {
        #[inline(always)]
        fn from(val: Devicefamily) -> u8 {
            Devicefamily::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Express {
        #[doc = "SUPPORTED."]
        Supported = 0x0,
        #[doc = "NONE."]
        None = 0x01,
    }
    impl Express {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Express {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Express {
        #[inline(always)]
        fn from(val: u8) -> Express {
            Express::from_bits(val)
        }
    }
    impl From<Express> for u8 {
        #[inline(always)]
        fn from(val: Express) -> u8 {
            Express::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ExtinfoType(u8);
    impl ExtinfoType {
        #[doc = "NONE."]
        pub const None: Self = Self(0xff);
    }
    impl ExtinfoType {
        pub const fn from_bits(val: u8) -> ExtinfoType {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for ExtinfoType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xff => f.write_str("None"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExtinfoType {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xff => defmt::write!(f, "None"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for ExtinfoType {
        #[inline(always)]
        fn from(val: u8) -> ExtinfoType {
            ExtinfoType::from_bits(val)
        }
    }
    impl From<ExtinfoType> for u8 {
        #[inline(always)]
        fn from(val: ExtinfoType) -> u8 {
            ExtinfoType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extvalid {
        #[doc = "EXTUSED."]
        Extused = 0x0,
        #[doc = "EXTUNUSED."]
        Extunused = 0x01,
    }
    impl Extvalid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extvalid {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extvalid {
        #[inline(always)]
        fn from(val: u8) -> Extvalid {
            Extvalid::from_bits(val)
        }
    }
    impl From<Extvalid> for u8 {
        #[inline(always)]
        fn from(val: Extvalid) -> u8 {
            Extvalid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Family {
        #[doc = "Flex Gecko."]
        Fg = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Family {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Family {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Family {
        #[inline(always)]
        fn from(val: u8) -> Family {
            Family::from_bits(val)
        }
    }
    impl From<Family> for u8 {
        #[inline(always)]
        fn from(val: Family) -> u8 {
            Family::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lfxo {
        #[doc = "NONE."]
        None = 0x0,
        #[doc = "PRESENT."]
        Present = 0x01,
    }
    impl Lfxo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lfxo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lfxo {
        #[inline(always)]
        fn from(val: u8) -> Lfxo {
            Lfxo::from_bits(val)
        }
    }
    impl From<Lfxo> for u8 {
        #[inline(always)]
        fn from(val: Lfxo) -> u8 {
            Lfxo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ModuleinfoType {
        #[doc = "PCB."]
        Pcb = 0x0,
        #[doc = "SIP."]
        Sip = 0x01,
    }
    impl ModuleinfoType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ModuleinfoType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ModuleinfoType {
        #[inline(always)]
        fn from(val: u8) -> ModuleinfoType {
            ModuleinfoType::from_bits(val)
        }
    }
    impl From<ModuleinfoType> for u8 {
        #[inline(always)]
        fn from(val: ModuleinfoType) -> u8 {
            ModuleinfoType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Padcdc {
        #[doc = "VDCDC."]
        Vdcdc = 0x0,
        #[doc = "OTHER."]
        Other = 0x01,
    }
    impl Padcdc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Padcdc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Padcdc {
        #[inline(always)]
        fn from(val: u8) -> Padcdc {
            Padcdc::from_bits(val)
        }
    }
    impl From<Padcdc> for u8 {
        #[inline(always)]
        fn from(val: Padcdc) -> u8 {
            Padcdc::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pkgtype(u8);
    impl Pkgtype {
        #[doc = "WLCSP package."]
        pub const Wlcsp: Self = Self(0x4a);
        #[doc = "BGA package."]
        pub const Bga: Self = Self(0x4c);
        #[doc = "QFN package."]
        pub const Qfn: Self = Self(0x4d);
        #[doc = "QFP package."]
        pub const Qfp: Self = Self(0x51);
    }
    impl Pkgtype {
        pub const fn from_bits(val: u8) -> Pkgtype {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pkgtype {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x4a => f.write_str("Wlcsp"),
                0x4c => f.write_str("Bga"),
                0x4d => f.write_str("Qfn"),
                0x51 => f.write_str("Qfp"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pkgtype {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x4a => defmt::write!(f, "Wlcsp"),
                0x4c => defmt::write!(f, "Bga"),
                0x4d => defmt::write!(f, "Qfn"),
                0x51 => defmt::write!(f, "Qfp"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pkgtype {
        #[inline(always)]
        fn from(val: u8) -> Pkgtype {
            Pkgtype::from_bits(val)
        }
    }
    impl From<Pkgtype> for u8 {
        #[inline(always)]
        fn from(val: Pkgtype) -> u8 {
            Pkgtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rf4ce {
        #[doc = "Thread stack capability not available."]
        Level0 = 0x0,
        #[doc = "Thread stack enabled."]
        Level1 = 0x01,
        #[doc = "N/A."]
        Level2 = 0x02,
        #[doc = "N/A."]
        Level3 = 0x03,
    }
    impl Rf4ce {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rf4ce {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rf4ce {
        #[inline(always)]
        fn from(val: u8) -> Rf4ce {
            Rf4ce::from_bits(val)
        }
    }
    impl From<Rf4ce> for u8 {
        #[inline(always)]
        fn from(val: Rf4ce) -> u8 {
            Rf4ce::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Shuntbiasana {
        #[doc = "I20UA."]
        I20ua = 0x0,
        #[doc = "I30UA."]
        I30ua = 0x01,
        #[doc = "I40UA."]
        I40ua = 0x02,
        #[doc = "I50UA."]
        I50ua = 0x03,
        #[doc = "I60UA."]
        I60ua = 0x04,
        #[doc = "I70UA."]
        I70ua = 0x05,
        #[doc = "I80UA."]
        I80ua = 0x06,
        #[doc = "I90UA."]
        I90ua = 0x07,
        #[doc = "I100UA."]
        I100ua = 0x08,
        #[doc = "I110UA."]
        I110ua = 0x09,
        #[doc = "I120UA."]
        I120ua = 0x0a,
        #[doc = "I130UA."]
        I130ua = 0x0b,
        #[doc = "I140UA."]
        I140ua = 0x0c,
        #[doc = "I150UA."]
        I150ua = 0x0d,
        #[doc = "I160UA."]
        I160ua = 0x0e,
        #[doc = "I170UA."]
        I170ua = 0x0f,
    }
    impl Shuntbiasana {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Shuntbiasana {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Shuntbiasana {
        #[inline(always)]
        fn from(val: u8) -> Shuntbiasana {
            Shuntbiasana::from_bits(val)
        }
    }
    impl From<Shuntbiasana> for u8 {
        #[inline(always)]
        fn from(val: Shuntbiasana) -> u8 {
            Shuntbiasana::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sri {
        #[doc = "RAIL capability not available."]
        Level0 = 0x0,
        #[doc = "RAIL enabled."]
        Level1 = 0x01,
        #[doc = "N/A."]
        Level2 = 0x02,
        #[doc = "N/A."]
        Level3 = 0x03,
    }
    impl Sri {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sri {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sri {
        #[inline(always)]
        fn from(val: u8) -> Sri {
            Sri::from_bits(val)
        }
    }
    impl From<Sri> for u8 {
        #[inline(always)]
        fn from(val: Sri) -> u8 {
            Sri::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tempgrade(u8);
    impl Tempgrade {
        #[doc = "-40 to 85 degC."]
        pub const N40to85: Self = Self(0x0);
        #[doc = "-40 to 125 degC."]
        pub const N40to125: Self = Self(0x01);
        #[doc = "-40 to 105 degC."]
        pub const N40to105: Self = Self(0x02);
        #[doc = "0 to 70 degC."]
        pub const N0to70: Self = Self(0x03);
    }
    impl Tempgrade {
        pub const fn from_bits(val: u8) -> Tempgrade {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Tempgrade {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("N40to85"),
                0x01 => f.write_str("N40to125"),
                0x02 => f.write_str("N40to105"),
                0x03 => f.write_str("N0to70"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tempgrade {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "N40to85"),
                0x01 => defmt::write!(f, "N40to125"),
                0x02 => defmt::write!(f, "N40to105"),
                0x03 => defmt::write!(f, "N0to70"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Tempgrade {
        #[inline(always)]
        fn from(val: u8) -> Tempgrade {
            Tempgrade::from_bits(val)
        }
    }
    impl From<Tempgrade> for u8 {
        #[inline(always)]
        fn from(val: Tempgrade) -> u8 {
            Tempgrade::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Thread {
        #[doc = "RF4CE stack capability not available."]
        Level0 = 0x0,
        #[doc = "RF4CE stack enabled."]
        Level1 = 0x01,
        #[doc = "N/A."]
        Level2 = 0x02,
        #[doc = "N/A."]
        Level3 = 0x03,
    }
    impl Thread {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Thread {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Thread {
        #[inline(always)]
        fn from(val: u8) -> Thread {
            Thread::from_bits(val)
        }
    }
    impl From<Thread> for u8 {
        #[inline(always)]
        fn from(val: Thread) -> u8 {
            Thread::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Zigbee {
        #[doc = "ZigBee stack capability not available."]
        Level0 = 0x0,
        #[doc = "GreenPower only."]
        Level1 = 0x01,
        #[doc = "ZigBee and GreenPower."]
        Level2 = 0x02,
        #[doc = "ZigBee Only."]
        Level3 = 0x03,
    }
    impl Zigbee {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Zigbee {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Zigbee {
        #[inline(always)]
        fn from(val: u8) -> Zigbee {
            Zigbee::from_bits(val)
        }
    }
    impl From<Zigbee> for u8 {
        #[inline(always)]
        fn from(val: Zigbee) -> u8 {
            Zigbee::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Zwave {
        #[doc = "Z-Wave stack capability not available."]
        Level0 = 0x0,
        #[doc = "Z-Wave Gateway."]
        Level1 = 0x01,
        #[doc = "Z-Wave End Device."]
        Level2 = 0x02,
        #[doc = "Z-Wave Sensor."]
        Level3 = 0x03,
        #[doc = "Z-Wave Lighting."]
        Level4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Zwave {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Zwave {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Zwave {
        #[inline(always)]
        fn from(val: u8) -> Zwave {
            Zwave::from_bits(val)
        }
    }
    impl From<Zwave> for u8 {
        #[inline(always)]
        fn from(val: Zwave) -> u8 {
            Zwave::to_bits(val)
        }
    }
}
