#[doc = "SEMAILBOX_NS_HOST peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SemailboxNsHost {
    ptr: *mut u8,
}
unsafe impl Send for SemailboxNsHost {}
unsafe impl Sync for SemailboxNsHost {}
impl SemailboxNsHost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TX Status register."]
    #[inline(always)]
    pub const fn tx_status(self) -> crate::common::Reg<regs::TxStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "RX Status register."]
    #[inline(always)]
    pub const fn rx_status(self) -> crate::common::Reg<regs::RxStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "TX Protection register."]
    #[inline(always)]
    pub const fn tx_prot(self) -> crate::common::Reg<regs::TxProt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "RX Protection register."]
    #[inline(always)]
    pub const fn rx_prot(self) -> crate::common::Reg<regs::RxProt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "A write access to this register will be mapped to the TX FIFO (only for header)."]
    #[inline(always)]
    pub const fn tx_header(self) -> crate::common::Reg<regs::TxHeader, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "A read access to this register will be mapped to the RX FIFO (only for the header)."]
    #[inline(always)]
    pub const fn rx_header(self) -> crate::common::Reg<regs::RxHeader, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Configuration register."]
    #[inline(always)]
    pub const fn configuration(self) -> crate::common::Reg<regs::Configuration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
}
pub mod regs {
    #[doc = "Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Configuration(pub u32);
    impl Configuration {
        #[doc = "TXINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txinten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TXINTEN."]
        #[inline(always)]
        pub const fn set_txinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RXINTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn rxinten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RXINTEN."]
        #[inline(always)]
        pub const fn set_rxinten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Configuration {
        #[inline(always)]
        fn default() -> Configuration {
            Configuration(0)
        }
    }
    impl core::fmt::Debug for Configuration {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Configuration")
                .field("txinten", &self.txinten())
                .field("rxinten", &self.rxinten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Configuration {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Configuration {{ txinten: {=bool:?}, rxinten: {=bool:?} }}",
                self.txinten(),
                self.rxinten()
            )
        }
    }
    #[doc = "A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u32);
    impl Fifo {
        #[doc = "FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FIFO."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    impl core::fmt::Debug for Fifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifo").field("fifo", &self.fifo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifo {{ fifo: {=u32:?} }}", self.fifo())
        }
    }
    #[doc = "A read access to this register will be mapped to the RX FIFO (only for the header)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxHeader(pub u32);
    impl RxHeader {
        #[doc = "RXHEADER."]
        #[must_use]
        #[inline(always)]
        pub const fn rxheader(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RXHEADER."]
        #[inline(always)]
        pub const fn set_rxheader(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RxHeader {
        #[inline(always)]
        fn default() -> RxHeader {
            RxHeader(0)
        }
    }
    impl core::fmt::Debug for RxHeader {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxHeader").field("rxheader", &self.rxheader()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxHeader {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RxHeader {{ rxheader: {=u32:?} }}", self.rxheader())
        }
    }
    #[doc = "RX Protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxProt(pub u32);
    impl RxProt {
        #[doc = "UNPROTECTED."]
        #[must_use]
        #[inline(always)]
        pub const fn unprotected(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "UNPROTECTED."]
        #[inline(always)]
        pub const fn set_unprotected(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PRIVILEGED."]
        #[must_use]
        #[inline(always)]
        pub const fn privileged(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PRIVILEGED."]
        #[inline(always)]
        pub const fn set_privileged(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "NONSECURE."]
        #[must_use]
        #[inline(always)]
        pub const fn nonsecure(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "NONSECURE."]
        #[inline(always)]
        pub const fn set_nonsecure(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "USER."]
        #[must_use]
        #[inline(always)]
        pub const fn user(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "USER."]
        #[inline(always)]
        pub const fn set_user(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RxProt {
        #[inline(always)]
        fn default() -> RxProt {
            RxProt(0)
        }
    }
    impl core::fmt::Debug for RxProt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxProt")
                .field("unprotected", &self.unprotected())
                .field("privileged", &self.privileged())
                .field("nonsecure", &self.nonsecure())
                .field("user", &self.user())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxProt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RxProt {{ unprotected: {=bool:?}, privileged: {=bool:?}, nonsecure: {=bool:?}, user: {=u8:?} }}",
                self.unprotected(),
                self.privileged(),
                self.nonsecure(),
                self.user()
            )
        }
    }
    #[doc = "RX Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxStatus(pub u32);
    impl RxStatus {
        #[doc = "REMBYTES."]
        #[must_use]
        #[inline(always)]
        pub const fn rembytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "REMBYTES."]
        #[inline(always)]
        pub const fn set_rembytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "MSGINFO."]
        #[must_use]
        #[inline(always)]
        pub const fn msginfo(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "MSGINFO."]
        #[inline(always)]
        pub const fn set_msginfo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "RXINT."]
        #[must_use]
        #[inline(always)]
        pub const fn rxint(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "RXINT."]
        #[inline(always)]
        pub const fn set_rxint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RXEMPTY."]
        #[must_use]
        #[inline(always)]
        pub const fn rxempty(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RXEMPTY."]
        #[inline(always)]
        pub const fn set_rxempty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "RXHDR."]
        #[must_use]
        #[inline(always)]
        pub const fn rxhdr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "RXHDR."]
        #[inline(always)]
        pub const fn set_rxhdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "RXERROR."]
        #[must_use]
        #[inline(always)]
        pub const fn rxerror(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "RXERROR."]
        #[inline(always)]
        pub const fn set_rxerror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for RxStatus {
        #[inline(always)]
        fn default() -> RxStatus {
            RxStatus(0)
        }
    }
    impl core::fmt::Debug for RxStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxStatus")
                .field("rembytes", &self.rembytes())
                .field("msginfo", &self.msginfo())
                .field("rxint", &self.rxint())
                .field("rxempty", &self.rxempty())
                .field("rxhdr", &self.rxhdr())
                .field("rxerror", &self.rxerror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RxStatus {{ rembytes: {=u16:?}, msginfo: {=u8:?}, rxint: {=bool:?}, rxempty: {=bool:?}, rxhdr: {=bool:?}, rxerror: {=bool:?} }}",
                self.rembytes(),
                self.msginfo(),
                self.rxint(),
                self.rxempty(),
                self.rxhdr(),
                self.rxerror()
            )
        }
    }
    #[doc = "A write access to this register will be mapped to the TX FIFO (only for header)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxHeader(pub u32);
    impl TxHeader {
        #[doc = "TXHEADER."]
        #[must_use]
        #[inline(always)]
        pub const fn txheader(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TXHEADER."]
        #[inline(always)]
        pub const fn set_txheader(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TxHeader {
        #[inline(always)]
        fn default() -> TxHeader {
            TxHeader(0)
        }
    }
    impl core::fmt::Debug for TxHeader {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxHeader").field("txheader", &self.txheader()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxHeader {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TxHeader {{ txheader: {=u32:?} }}", self.txheader())
        }
    }
    #[doc = "TX Protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxProt(pub u32);
    impl TxProt {
        #[doc = "UNPROTECTED."]
        #[must_use]
        #[inline(always)]
        pub const fn unprotected(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "UNPROTECTED."]
        #[inline(always)]
        pub const fn set_unprotected(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PRIVILEGED."]
        #[must_use]
        #[inline(always)]
        pub const fn privileged(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PRIVILEGED."]
        #[inline(always)]
        pub const fn set_privileged(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "NONSECURE."]
        #[must_use]
        #[inline(always)]
        pub const fn nonsecure(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "NONSECURE."]
        #[inline(always)]
        pub const fn set_nonsecure(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "USER."]
        #[must_use]
        #[inline(always)]
        pub const fn user(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "USER."]
        #[inline(always)]
        pub const fn set_user(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TxProt {
        #[inline(always)]
        fn default() -> TxProt {
            TxProt(0)
        }
    }
    impl core::fmt::Debug for TxProt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxProt")
                .field("unprotected", &self.unprotected())
                .field("privileged", &self.privileged())
                .field("nonsecure", &self.nonsecure())
                .field("user", &self.user())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxProt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxProt {{ unprotected: {=bool:?}, privileged: {=bool:?}, nonsecure: {=bool:?}, user: {=u8:?} }}",
                self.unprotected(),
                self.privileged(),
                self.nonsecure(),
                self.user()
            )
        }
    }
    #[doc = "TX Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxStatus(pub u32);
    impl TxStatus {
        #[doc = "REMBYTES."]
        #[must_use]
        #[inline(always)]
        pub const fn rembytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "REMBYTES."]
        #[inline(always)]
        pub const fn set_rembytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "MSGINFO."]
        #[must_use]
        #[inline(always)]
        pub const fn msginfo(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "MSGINFO."]
        #[inline(always)]
        pub const fn set_msginfo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "TXINT."]
        #[must_use]
        #[inline(always)]
        pub const fn txint(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TXINT."]
        #[inline(always)]
        pub const fn set_txint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TXFULL."]
        #[must_use]
        #[inline(always)]
        pub const fn txfull(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TXFULL."]
        #[inline(always)]
        pub const fn set_txfull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TXERROR."]
        #[must_use]
        #[inline(always)]
        pub const fn txerror(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "TXERROR."]
        #[inline(always)]
        pub const fn set_txerror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for TxStatus {
        #[inline(always)]
        fn default() -> TxStatus {
            TxStatus(0)
        }
    }
    impl core::fmt::Debug for TxStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxStatus")
                .field("rembytes", &self.rembytes())
                .field("msginfo", &self.msginfo())
                .field("txint", &self.txint())
                .field("txfull", &self.txfull())
                .field("txerror", &self.txerror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TxStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TxStatus {{ rembytes: {=u16:?}, msginfo: {=u8:?}, txint: {=bool:?}, txfull: {=bool:?}, txerror: {=bool:?} }}",
                self.rembytes(),
                self.msginfo(),
                self.txint(),
                self.txfull(),
                self.txerror()
            )
        }
    }
}
