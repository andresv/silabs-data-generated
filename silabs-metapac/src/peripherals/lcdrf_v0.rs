#[doc = "LCDRF peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdrf {
    ptr: *mut u8,
}
unsafe impl Send for Lcdrf {}
unsafe impl Sync for Lcdrf {}
impl Lcdrf {
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
    pub const fn rfimlcdctrl(self) -> crate::common::Reg<regs::Rfimlcdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rfimlcdctrl_set(self) -> crate::common::Reg<regs::Rfimlcdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rfimlcdctrl_clr(self) -> crate::common::Reg<regs::Rfimlcdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rfimlcdctrl_tgl(self) -> crate::common::Reg<regs::Rfimlcdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfimlcdctrl(pub u32);
    impl Rfimlcdctrl {
        #[doc = "LCD Charge Pump XO Clock Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdcpxoen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD Charge Pump XO Clock Enable."]
        #[inline(always)]
        pub const fn set_lcdcpxoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LCD Charge Pump XO Select."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdcpxosel(&self) -> super::vals::Lcdcpxosel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lcdcpxosel::from_bits(val as u8)
        }
        #[doc = "LCD Charge Pump XO Select."]
        #[inline(always)]
        pub const fn set_lcdcpxosel(&mut self, val: super::vals::Lcdcpxosel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "LCD Charge Pump XO Retime Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdcpxoretimeen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LCD Charge Pump XO Retime Enable."]
        #[inline(always)]
        pub const fn set_lcdcpxoretimeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LCD Low Noise."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdlownoise(&self) -> super::vals::Lcdlownoise {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Lcdlownoise::from_bits(val as u8)
        }
        #[doc = "LCD Low Noise."]
        #[inline(always)]
        pub const fn set_lcdlownoise(&mut self, val: super::vals::Lcdlownoise) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "LCD Comparator Dout."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdcmpdout(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LCD Comparator Dout."]
        #[inline(always)]
        pub const fn set_lcdcmpdout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Rfimlcdctrl {
        #[inline(always)]
        fn default() -> Rfimlcdctrl {
            Rfimlcdctrl(0)
        }
    }
    impl core::fmt::Debug for Rfimlcdctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfimlcdctrl")
                .field("lcdcpxoen", &self.lcdcpxoen())
                .field("lcdcpxosel", &self.lcdcpxosel())
                .field("lcdcpxoretimeen", &self.lcdcpxoretimeen())
                .field("lcdlownoise", &self.lcdlownoise())
                .field("lcdcmpdout", &self.lcdcmpdout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfimlcdctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rfimlcdctrl {{ lcdcpxoen: {=bool:?}, lcdcpxosel: {:?}, lcdcpxoretimeen: {=bool:?}, lcdlownoise: {:?}, lcdcmpdout: {=bool:?} }}",
                self.lcdcpxoen(),
                self.lcdcpxosel(),
                self.lcdcpxoretimeen(),
                self.lcdlownoise(),
                self.lcdcmpdout()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lcdcpxosel {
        #[doc = "Internal LCD CP 10Mhz RC oscillator."]
        Intrco = 0x0,
        #[doc = "HFXO divided 4 clock."]
        Hfxodiv = 0x01,
    }
    impl Lcdcpxosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lcdcpxosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lcdcpxosel {
        #[inline(always)]
        fn from(val: u8) -> Lcdcpxosel {
            Lcdcpxosel::from_bits(val)
        }
    }
    impl From<Lcdcpxosel> for u8 {
        #[inline(always)]
        fn from(val: Lcdcpxosel) -> u8 {
            Lcdcpxosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lcdlownoise {
        #[doc = "Normal operation."]
        Normal = 0x0,
        #[doc = "slows down slew rate to reduce RF interference at a cost of additional power consumption."]
        Slow = 0x01,
    }
    impl Lcdlownoise {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lcdlownoise {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lcdlownoise {
        #[inline(always)]
        fn from(val: u8) -> Lcdlownoise {
            Lcdlownoise::from_bits(val)
        }
    }
    impl From<Lcdlownoise> for u8 {
        #[inline(always)]
        fn from(val: Lcdlownoise) -> u8 {
            Lcdlownoise::to_bits(val)
        }
    }
}
