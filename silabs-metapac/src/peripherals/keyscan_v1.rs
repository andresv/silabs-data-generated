#[doc = "KEYSCAN peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyscan {
    ptr: *mut u8,
}
unsafe impl Send for Keyscan {}
unsafe impl Sync for Keyscan {}
impl Keyscan {
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
    #[doc = "Enable."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn swrst(self) -> crate::common::Reg<regs::Swrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Config."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Command."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Delay."]
    #[inline(always)]
    pub const fn delay(self) -> crate::common::Reg<regs::Delay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Flags."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Interrupt Enables."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Software Reset. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn swrst_set(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Config. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Command. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Delay. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn delay_set(self) -> crate::common::Reg<regs::Delay, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Interrupt Enables. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Software Reset. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn swrst_clr(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Config. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "Command. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "Delay. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn delay_clr(self) -> crate::common::Reg<regs::Delay, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "Interrupt Enables. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "Software Reset. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn swrst_tgl(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Config. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "Command. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "Delay. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn delay_tgl(self) -> crate::common::Reg<regs::Delay, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "Interrupt Enables. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
}
pub mod regs {
    #[doc = "Config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Clock Divider."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Clock Divider."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "Single Press."]
        #[must_use]
        #[inline(always)]
        pub const fn singlepress(&self) -> super::vals::Singlepress {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Singlepress::from_bits(val as u8)
        }
        #[doc = "Single Press."]
        #[inline(always)]
        pub const fn set_singlepress(&mut self, val: super::vals::Singlepress) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Automatically Start."]
        #[must_use]
        #[inline(always)]
        pub const fn autostart(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically Start."]
        #[inline(always)]
        pub const fn set_autostart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Number of Rows."]
        #[must_use]
        #[inline(always)]
        pub const fn numrows(&self) -> super::vals::Numrows {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Numrows::from_bits(val as u8)
        }
        #[doc = "Number of Rows."]
        #[inline(always)]
        pub const fn set_numrows(&mut self, val: super::vals::Numrows) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Number of Columns."]
        #[must_use]
        #[inline(always)]
        pub const fn numcols(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Number of Columns."]
        #[inline(always)]
        pub const fn set_numcols(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("clkdiv", &self.clkdiv())
                .field("singlepress", &self.singlepress())
                .field("autostart", &self.autostart())
                .field("numrows", &self.numrows())
                .field("numcols", &self.numcols())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ clkdiv: {=u32:?}, singlepress: {:?}, autostart: {=bool:?}, numrows: {:?}, numcols: {=u8:?} }}",
                self.clkdiv(),
                self.singlepress(),
                self.autostart(),
                self.numrows(),
                self.numcols()
            )
        }
    }
    #[doc = "Command."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Keyscan Start."]
        #[must_use]
        #[inline(always)]
        pub const fn keyscanstart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Keyscan Start."]
        #[inline(always)]
        pub const fn set_keyscanstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Keyscan Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn keyscanstop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Keyscan Stop."]
        #[inline(always)]
        pub const fn set_keyscanstop(&mut self, val: bool) {
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
                .field("keyscanstart", &self.keyscanstart())
                .field("keyscanstop", &self.keyscanstop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ keyscanstart: {=bool:?}, keyscanstop: {=bool:?} }}",
                self.keyscanstart(),
                self.keyscanstop()
            )
        }
    }
    #[doc = "Delay."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Delay(pub u32);
    impl Delay {
        #[doc = "Scan Delay."]
        #[must_use]
        #[inline(always)]
        pub const fn scandly(&self) -> super::vals::Scandly {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Scandly::from_bits(val as u8)
        }
        #[doc = "Scan Delay."]
        #[inline(always)]
        pub const fn set_scandly(&mut self, val: super::vals::Scandly) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Debounce Delay."]
        #[must_use]
        #[inline(always)]
        pub const fn debdly(&self) -> super::vals::Debdly {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Debdly::from_bits(val as u8)
        }
        #[doc = "Debounce Delay."]
        #[inline(always)]
        pub const fn set_debdly(&mut self, val: super::vals::Debdly) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Row stable Delay."]
        #[must_use]
        #[inline(always)]
        pub const fn stabdly(&self) -> super::vals::Stabdly {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Stabdly::from_bits(val as u8)
        }
        #[doc = "Row stable Delay."]
        #[inline(always)]
        pub const fn set_stabdly(&mut self, val: super::vals::Stabdly) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Delay {
        #[inline(always)]
        fn default() -> Delay {
            Delay(0)
        }
    }
    impl core::fmt::Debug for Delay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Delay")
                .field("scandly", &self.scandly())
                .field("debdly", &self.debdly())
                .field("stabdly", &self.stabdly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Delay {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Delay {{ scandly: {:?}, debdly: {:?}, stabdly: {:?} }}",
                self.scandly(),
                self.debdly(),
                self.stabdly()
            )
        }
    }
    #[doc = "Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Disablement busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn disabling(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Disablement busy status."]
        #[inline(always)]
        pub const fn set_disabling(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            f.debug_struct("En")
                .field("en", &self.en())
                .field("disabling", &self.disabling())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for En {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "En {{ en: {=bool:?}, disabling: {=bool:?} }}",
                self.en(),
                self.disabling()
            )
        }
    }
    #[doc = "Interrupt Enables."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "No Key was pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn nokey(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No Key was pressed."]
        #[inline(always)]
        pub const fn set_nokey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "A Key was pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "A Key was pressed."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Completed Scanning."]
        #[must_use]
        #[inline(always)]
        pub const fn scanned(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Completed Scanning."]
        #[inline(always)]
        pub const fn set_scanned(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wake up."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wake up."]
        #[inline(always)]
        pub const fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("nokey", &self.nokey())
                .field("key", &self.key())
                .field("scanned", &self.scanned())
                .field("wakeup", &self.wakeup())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ nokey: {=bool:?}, key: {=bool:?}, scanned: {=bool:?}, wakeup: {=bool:?} }}",
                self.nokey(),
                self.key(),
                self.scanned(),
                self.wakeup()
            )
        }
    }
    #[doc = "Interrupt Flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "No key was pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn nokey(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No key was pressed."]
        #[inline(always)]
        pub const fn set_nokey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "A key was pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "A key was pressed."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Completed scan."]
        #[must_use]
        #[inline(always)]
        pub const fn scanned(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Completed scan."]
        #[inline(always)]
        pub const fn set_scanned(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wake up."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wake up."]
        #[inline(always)]
        pub const fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("nokey", &self.nokey())
                .field("key", &self.key())
                .field("scanned", &self.scanned())
                .field("wakeup", &self.wakeup())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ nokey: {=bool:?}, key: {=bool:?}, scanned: {=bool:?}, wakeup: {=bool:?} }}",
                self.nokey(),
                self.key(),
                self.scanned(),
                self.wakeup()
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
        #[doc = "Row detection."]
        #[must_use]
        #[inline(always)]
        pub const fn row(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Row detection."]
        #[inline(always)]
        pub const fn set_row(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Running."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Running."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Column Latched."]
        #[must_use]
        #[inline(always)]
        pub const fn col(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Column Latched."]
        #[inline(always)]
        pub const fn set_col(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "No Key pressed status."]
        #[must_use]
        #[inline(always)]
        pub const fn nokey(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No Key pressed status."]
        #[inline(always)]
        pub const fn set_nokey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Sync Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn syncbusy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Busy."]
        #[inline(always)]
        pub const fn set_syncbusy(&mut self, val: bool) {
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
                .field("row", &self.row())
                .field("running", &self.running())
                .field("col", &self.col())
                .field("nokey", &self.nokey())
                .field("syncbusy", &self.syncbusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ row: {=u8:?}, running: {=bool:?}, col: {=u8:?}, nokey: {=bool:?}, syncbusy: {=bool:?} }}",
                self.row(),
                self.running(),
                self.col(),
                self.nokey(),
                self.syncbusy()
            )
        }
    }
    #[doc = "Software Reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swrst(pub u32);
    impl Swrst {
        #[doc = "Software reset command."]
        #[must_use]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset command."]
        #[inline(always)]
        pub const fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software reset busy status."]
        #[must_use]
        #[inline(always)]
        pub const fn resetting(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset busy status."]
        #[inline(always)]
        pub const fn set_resetting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Swrst {
        #[inline(always)]
        fn default() -> Swrst {
            Swrst(0)
        }
    }
    impl core::fmt::Debug for Swrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swrst")
                .field("swrst", &self.swrst())
                .field("resetting", &self.resetting())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swrst {{ swrst: {=bool:?}, resetting: {=bool:?} }}",
                self.swrst(),
                self.resetting()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Debdly {
        #[doc = "2ms Debounce Delay."]
        Debdly2 = 0x0,
        #[doc = "4ms Debounce Delay."]
        Debdly4 = 0x01,
        #[doc = "6ms Debounce Delay."]
        Debdly6 = 0x02,
        #[doc = "8ms Debounce Delay."]
        Debdly8 = 0x03,
        #[doc = "10ms Debounce Delay."]
        Debdly10 = 0x04,
        #[doc = "12ms Debounce Delay."]
        Debdly12 = 0x05,
        #[doc = "14ms Debounce Delay."]
        Debdly14 = 0x06,
        #[doc = "16ms Debounce Delay."]
        Debdly16 = 0x07,
        #[doc = "18ms Debounce Delay."]
        Debdly18 = 0x08,
        #[doc = "20ms Debounce Delay."]
        Debdly20 = 0x09,
        #[doc = "22ms Debounce Delay."]
        Debdly22 = 0x0a,
        #[doc = "24ms Debounce Delay."]
        Debdly24 = 0x0b,
        #[doc = "26ms Debounce Delay."]
        Debdly26 = 0x0c,
        #[doc = "28ms Debounce Delay."]
        Debdly28 = 0x0d,
        #[doc = "30ms Debounce Delay."]
        Debdly30 = 0x0e,
        #[doc = "32ms Debounce Delay."]
        Debdly32 = 0x0f,
    }
    impl Debdly {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Debdly {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Debdly {
        #[inline(always)]
        fn from(val: u8) -> Debdly {
            Debdly::from_bits(val)
        }
    }
    impl From<Debdly> for u8 {
        #[inline(always)]
        fn from(val: Debdly) -> u8 {
            Debdly::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Numrows {
        #[doc = "1 Row is not supported; defaults to 3 instead."]
        Rsv1 = 0x0,
        #[doc = "2 Rows are not supported; defaults to 3 instead."]
        Rsv2 = 0x01,
        #[doc = "3 Rows."]
        Row3 = 0x02,
        #[doc = "4 Rows."]
        Row4 = 0x03,
        #[doc = "5 Rows."]
        Row5 = 0x04,
        #[doc = "6 Rows."]
        Row6 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Numrows {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Numrows {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Numrows {
        #[inline(always)]
        fn from(val: u8) -> Numrows {
            Numrows::from_bits(val)
        }
    }
    impl From<Numrows> for u8 {
        #[inline(always)]
        fn from(val: Numrows) -> u8 {
            Numrows::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scandly {
        #[doc = "2ms Scan Delay."]
        Scandly2 = 0x0,
        #[doc = "4ms Scan Delay."]
        Scandly4 = 0x01,
        #[doc = "6ms Scan Delay."]
        Scandly6 = 0x02,
        #[doc = "8ms Scan Delay."]
        Scandly8 = 0x03,
        #[doc = "10ms Scan Delay."]
        Scandly10 = 0x04,
        #[doc = "12ms Scan Delay."]
        Scandly12 = 0x05,
        #[doc = "14ms Scan Delay."]
        Scandly14 = 0x06,
        #[doc = "16ms Scan Delay."]
        Scandly16 = 0x07,
        #[doc = "18ms Scan Delay."]
        Scandly18 = 0x08,
        #[doc = "20ms Scan Delay."]
        Scandly20 = 0x09,
        #[doc = "22ms Scan Delay."]
        Scandly22 = 0x0a,
        #[doc = "24ms Scan Delay."]
        Scandly24 = 0x0b,
        #[doc = "26ms Scan Delay."]
        Scandly26 = 0x0c,
        #[doc = "28ms Scan Delay."]
        Scandly28 = 0x0d,
        #[doc = "30ms Scan Delay."]
        Scandly30 = 0x0e,
        #[doc = "32ms Scan Delay."]
        Scandly32 = 0x0f,
    }
    impl Scandly {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scandly {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scandly {
        #[inline(always)]
        fn from(val: u8) -> Scandly {
            Scandly::from_bits(val)
        }
    }
    impl From<Scandly> for u8 {
        #[inline(always)]
        fn from(val: Scandly) -> u8 {
            Scandly::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Singlepress {
        #[doc = "After KEYIF is set and then cleared, scanning will continue. This can give multiple interrupts for the same key press, but allow multiple key presses to be detected. To use this mode for multi-key detection, the ISR should update a section of memory of COLNUM bytes on each interrupt, until key release is detected. After key release, the section of memory where key presses are recorded can be processed."]
        Multipress = 0x0,
        #[doc = "After KEYIF has been set and cleared, it will not set again until no key press is detected. This allows faster response since the ISR can start processing data as soon as the KEYIF is set."]
        Singlepress = 0x01,
    }
    impl Singlepress {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Singlepress {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Singlepress {
        #[inline(always)]
        fn from(val: u8) -> Singlepress {
            Singlepress::from_bits(val)
        }
    }
    impl From<Singlepress> for u8 {
        #[inline(always)]
        fn from(val: Singlepress) -> u8 {
            Singlepress::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stabdly {
        #[doc = "2ms Row Stable Delay."]
        Stabdly2 = 0x0,
        #[doc = "4ms Row Stable Delay."]
        Stabdly4 = 0x01,
        #[doc = "6ms Row Stable Delay."]
        Stabdly6 = 0x02,
        #[doc = "8ms Row Stable Delay."]
        Stabdly8 = 0x03,
        #[doc = "10ms Row Stable Delay."]
        Stabdly10 = 0x04,
        #[doc = "12ms Row Stable Delay."]
        Stabdly12 = 0x05,
        #[doc = "14ms Row Stable Delay."]
        Stabdly14 = 0x06,
        #[doc = "16ms Row Stable Delay."]
        Stabdly16 = 0x07,
        #[doc = "18ms Row Stable Delay."]
        Stabdly18 = 0x08,
        #[doc = "20ms Row Stable Delay."]
        Stabdly20 = 0x09,
        #[doc = "22ms Row Stable Delay."]
        Stabdly22 = 0x0a,
        #[doc = "24ms Row Stable Delay."]
        Stabdly24 = 0x0b,
        #[doc = "26ms Row Stable Delay."]
        Stabdly26 = 0x0c,
        #[doc = "28ms Row Stable Delay."]
        Stabdly28 = 0x0d,
        #[doc = "30ms Row Stable Delay."]
        Stabdly30 = 0x0e,
        #[doc = "32ms Row Stable Delay."]
        Stabdly32 = 0x0f,
    }
    impl Stabdly {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stabdly {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stabdly {
        #[inline(always)]
        fn from(val: u8) -> Stabdly {
            Stabdly::from_bits(val)
        }
    }
    impl From<Stabdly> for u8 {
        #[inline(always)]
        fn from(val: Stabdly) -> u8 {
            Stabdly::to_bits(val)
        }
    }
}
