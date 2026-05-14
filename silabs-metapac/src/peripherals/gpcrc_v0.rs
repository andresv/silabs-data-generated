#[doc = "GPCRC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpcrc {
    ptr: *mut u8,
}
unsafe impl Send for Gpcrc {}
unsafe impl Sync for Gpcrc {}
impl Gpcrc {
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
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn init(self) -> crate::common::Reg<regs::Init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn poly(self) -> crate::common::Reg<regs::Poly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn inputdata(self) -> crate::common::Reg<regs::Inputdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn inputdatahword(self) -> crate::common::Reg<regs::Inputdatahword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn inputdatabyte(self) -> crate::common::Reg<regs::Inputdatabyte, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn datarev(self) -> crate::common::Reg<regs::Datarev, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn databyterev(self) -> crate::common::Reg<regs::Databyterev, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn init_set(self) -> crate::common::Reg<regs::Init, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn poly_set(self) -> crate::common::Reg<regs::Poly, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn inputdata_set(self) -> crate::common::Reg<regs::Inputdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn inputdatahword_set(self) -> crate::common::Reg<regs::Inputdatahword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn inputdatabyte_set(self) -> crate::common::Reg<regs::Inputdatabyte, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn init_clr(self) -> crate::common::Reg<regs::Init, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn poly_clr(self) -> crate::common::Reg<regs::Poly, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn inputdata_clr(self) -> crate::common::Reg<regs::Inputdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn inputdatahword_clr(self) -> crate::common::Reg<regs::Inputdatahword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn inputdatabyte_clr(self) -> crate::common::Reg<regs::Inputdatabyte, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn init_tgl(self) -> crate::common::Reg<regs::Init, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn poly_tgl(self) -> crate::common::Reg<regs::Poly, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn inputdata_tgl(self) -> crate::common::Reg<regs::Inputdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn inputdatahword_tgl(self) -> crate::common::Reg<regs::Inputdatahword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn inputdatabyte_tgl(self) -> crate::common::Reg<regs::Inputdatabyte, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Initialization Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Enable."]
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("Cmd").field("init", &self.init()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmd {{ init: {=bool:?} }}", self.init())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Polynomial Select."]
        #[must_use]
        #[inline(always)]
        pub const fn polysel(&self) -> super::vals::Polysel {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Polysel::from_bits(val as u8)
        }
        #[doc = "Polynomial Select."]
        #[inline(always)]
        pub const fn set_polysel(&mut self, val: super::vals::Polysel) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Byte Mode Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bytemode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Byte Mode Enable."]
        #[inline(always)]
        pub const fn set_bytemode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Byte-level Bit Reverse Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bitreverse(&self) -> super::vals::Bitreverse {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Bitreverse::from_bits(val as u8)
        }
        #[doc = "Byte-level Bit Reverse Enable."]
        #[inline(always)]
        pub const fn set_bitreverse(&mut self, val: super::vals::Bitreverse) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Byte Reverse Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bytereverse(&self) -> super::vals::Bytereverse {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Bytereverse::from_bits(val as u8)
        }
        #[doc = "Byte Reverse Mode."]
        #[inline(always)]
        pub const fn set_bytereverse(&mut self, val: super::vals::Bytereverse) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Auto Init Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn autoinit(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Auto Init Enable."]
        #[inline(always)]
        pub const fn set_autoinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("polysel", &self.polysel())
                .field("bytemode", &self.bytemode())
                .field("bitreverse", &self.bitreverse())
                .field("bytereverse", &self.bytereverse())
                .field("autoinit", &self.autoinit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ polysel: {:?}, bytemode: {=bool:?}, bitreverse: {:?}, bytereverse: {:?}, autoinit: {=bool:?} }}",
                self.polysel(),
                self.bytemode(),
                self.bitreverse(),
                self.bytereverse(),
                self.autoinit()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "CRC Data Register."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC Data Register."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Databyterev(pub u32);
    impl Databyterev {
        #[doc = "Data Byte Reverse Value."]
        #[must_use]
        #[inline(always)]
        pub const fn databyterev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Byte Reverse Value."]
        #[inline(always)]
        pub const fn set_databyterev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Databyterev {
        #[inline(always)]
        fn default() -> Databyterev {
            Databyterev(0)
        }
    }
    impl core::fmt::Debug for Databyterev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Databyterev")
                .field("databyterev", &self.databyterev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Databyterev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Databyterev {{ databyterev: {=u32:?} }}", self.databyterev())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datarev(pub u32);
    impl Datarev {
        #[doc = "Data Reverse Value."]
        #[must_use]
        #[inline(always)]
        pub const fn datarev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Reverse Value."]
        #[inline(always)]
        pub const fn set_datarev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Datarev {
        #[inline(always)]
        fn default() -> Datarev {
            Datarev(0)
        }
    }
    impl core::fmt::Debug for Datarev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Datarev").field("datarev", &self.datarev()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Datarev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Datarev {{ datarev: {=u32:?} }}", self.datarev())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "CRC Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            f.debug_struct("En").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for En {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "En {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Init(pub u32);
    impl Init {
        #[doc = "CRC Initialization Value."]
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC Initialization Value."]
        #[inline(always)]
        pub const fn set_init(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Init {
        #[inline(always)]
        fn default() -> Init {
            Init(0)
        }
    }
    impl core::fmt::Debug for Init {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Init").field("init", &self.init()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Init {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Init {{ init: {=u32:?} }}", self.init())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inputdata(pub u32);
    impl Inputdata {
        #[doc = "Input Data for 32-bit."]
        #[must_use]
        #[inline(always)]
        pub const fn inputdata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input Data for 32-bit."]
        #[inline(always)]
        pub const fn set_inputdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Inputdata {
        #[inline(always)]
        fn default() -> Inputdata {
            Inputdata(0)
        }
    }
    impl core::fmt::Debug for Inputdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inputdata")
                .field("inputdata", &self.inputdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inputdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Inputdata {{ inputdata: {=u32:?} }}", self.inputdata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inputdatabyte(pub u32);
    impl Inputdatabyte {
        #[doc = "Input Data for 8-bit."]
        #[must_use]
        #[inline(always)]
        pub const fn inputdatabyte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Input Data for 8-bit."]
        #[inline(always)]
        pub const fn set_inputdatabyte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Inputdatabyte {
        #[inline(always)]
        fn default() -> Inputdatabyte {
            Inputdatabyte(0)
        }
    }
    impl core::fmt::Debug for Inputdatabyte {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inputdatabyte")
                .field("inputdatabyte", &self.inputdatabyte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inputdatabyte {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Inputdatabyte {{ inputdatabyte: {=u8:?} }}", self.inputdatabyte())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inputdatahword(pub u32);
    impl Inputdatahword {
        #[doc = "Input Data for 16-bit."]
        #[must_use]
        #[inline(always)]
        pub const fn inputdatahword(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Input Data for 16-bit."]
        #[inline(always)]
        pub const fn set_inputdatahword(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Inputdatahword {
        #[inline(always)]
        fn default() -> Inputdatahword {
            Inputdatahword(0)
        }
    }
    impl core::fmt::Debug for Inputdatahword {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inputdatahword")
                .field("inputdatahword", &self.inputdatahword())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inputdatahword {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Inputdatahword {{ inputdatahword: {=u16:?} }}",
                self.inputdatahword()
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
    pub struct Poly(pub u32);
    impl Poly {
        #[doc = "CRC Polynomial Value."]
        #[must_use]
        #[inline(always)]
        pub const fn poly(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "CRC Polynomial Value."]
        #[inline(always)]
        pub const fn set_poly(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Poly {
        #[inline(always)]
        fn default() -> Poly {
            Poly(0)
        }
    }
    impl core::fmt::Debug for Poly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Poly").field("poly", &self.poly()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Poly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Poly {{ poly: {=u16:?} }}", self.poly())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bitreverse {
        #[doc = "No reverse."]
        Normal = 0x0,
        #[doc = "Reverse bit order in each byte."]
        Reversed = 0x01,
    }
    impl Bitreverse {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bitreverse {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bitreverse {
        #[inline(always)]
        fn from(val: u8) -> Bitreverse {
            Bitreverse::from_bits(val)
        }
    }
    impl From<Bitreverse> for u8 {
        #[inline(always)]
        fn from(val: Bitreverse) -> u8 {
            Bitreverse::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bytereverse {
        #[doc = "No reverse: B3, B2, B1, B0."]
        Normal = 0x0,
        #[doc = "Reverse byte order. For 32-bit: B0, B1, B2, B3; For 16-bit: 0, 0, B0, B1."]
        Reversed = 0x01,
    }
    impl Bytereverse {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bytereverse {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bytereverse {
        #[inline(always)]
        fn from(val: u8) -> Bytereverse {
            Bytereverse::from_bits(val)
        }
    }
    impl From<Bytereverse> for u8 {
        #[inline(always)]
        fn from(val: Bytereverse) -> u8 {
            Bytereverse::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Polysel {
        #[doc = "CRC-32 (0x04C11DB7) polynomial selected."]
        Crc32 = 0x0,
        #[doc = "16-bit CRC programmable polynomial selected."]
        Crc16 = 0x01,
    }
    impl Polysel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polysel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polysel {
        #[inline(always)]
        fn from(val: u8) -> Polysel {
            Polysel::from_bits(val)
        }
    }
    impl From<Polysel> for u8 {
        #[inline(always)]
        fn from(val: Polysel) -> u8 {
            Polysel::to_bits(val)
        }
    }
}
