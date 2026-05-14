#[doc = "CMU peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmu {
    ptr: *mut u8,
}
unsafe impl Send for Cmu {}
unsafe impl Sync for Cmu {}
impl Cmu {
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
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn wdoglock(self) -> crate::common::Reg<regs::Wdoglock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn calcmd(self) -> crate::common::Reg<regs::Calcmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn calctrl(self) -> crate::common::Reg<regs::Calctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn calcnt(self) -> crate::common::Reg<regs::Calcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn clken0(self) -> crate::common::Reg<regs::Clken0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn clken1(self) -> crate::common::Reg<regs::Clken1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn clken2(self) -> crate::common::Reg<regs::Clken2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sysclkctrl(self) -> crate::common::Reg<regs::Sysclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn traceclkctrl(self) -> crate::common::Reg<regs::Traceclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn exportclkctrl(self) -> crate::common::Reg<regs::Exportclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn dpllrefclkctrl(self) -> crate::common::Reg<regs::Dpllrefclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em01grpaclkctrl(self) -> crate::common::Reg<regs::Em01grpaclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em01grpcclkctrl(self) -> crate::common::Reg<regs::Em01grpcclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em23grpaclkctrl(self) -> crate::common::Reg<regs::Em23grpaclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4grpaclkctrl(self) -> crate::common::Reg<regs::Em4grpaclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn iadcclkctrl(self) -> crate::common::Reg<regs::Iadcclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn wdog0clkctrl(self) -> crate::common::Reg<regs::Wdog0clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn wdog1clkctrl(self) -> crate::common::Reg<regs::Wdog1clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn eusart0clkctrl(self) -> crate::common::Reg<regs::Eusart0clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sysrtc0clkctrl(self) -> crate::common::Reg<regs::Sysrtc0clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lcdclkctrl(self) -> crate::common::Reg<regs::Lcdclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn vdac0clkctrl(self) -> crate::common::Reg<regs::Vdac0clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pcnt0clkctrl(self) -> crate::common::Reg<regs::Pcnt0clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn radioclkctrl(self) -> crate::common::Reg<regs::Radioclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn vdac1clkctrl(self) -> crate::common::Reg<regs::Vdac1clkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn wdoglock_set(self) -> crate::common::Reg<regs::Wdoglock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn calcmd_set(self) -> crate::common::Reg<regs::Calcmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn calctrl_set(self) -> crate::common::Reg<regs::Calctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn clken0_set(self) -> crate::common::Reg<regs::Clken0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn clken1_set(self) -> crate::common::Reg<regs::Clken1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn clken2_set(self) -> crate::common::Reg<regs::Clken2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sysclkctrl_set(self) -> crate::common::Reg<regs::Sysclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn traceclkctrl_set(self) -> crate::common::Reg<regs::Traceclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn exportclkctrl_set(self) -> crate::common::Reg<regs::Exportclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dpllrefclkctrl_set(self) -> crate::common::Reg<regs::Dpllrefclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em01grpaclkctrl_set(self) -> crate::common::Reg<regs::Em01grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em01grpcclkctrl_set(self) -> crate::common::Reg<regs::Em01grpcclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em23grpaclkctrl_set(self) -> crate::common::Reg<regs::Em23grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4grpaclkctrl_set(self) -> crate::common::Reg<regs::Em4grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn iadcclkctrl_set(self) -> crate::common::Reg<regs::Iadcclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn wdog0clkctrl_set(self) -> crate::common::Reg<regs::Wdog0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn wdog1clkctrl_set(self) -> crate::common::Reg<regs::Wdog1clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1208usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0clkctrl_set(self) -> crate::common::Reg<regs::Eusart0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1220usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sysrtc0clkctrl_set(self) -> crate::common::Reg<regs::Sysrtc0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1240usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lcdclkctrl_set(self) -> crate::common::Reg<regs::Lcdclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1250usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn vdac0clkctrl_set(self) -> crate::common::Reg<regs::Vdac0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1260usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pcnt0clkctrl_set(self) -> crate::common::Reg<regs::Pcnt0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1270usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn radioclkctrl_set(self) -> crate::common::Reg<regs::Radioclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1280usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn vdac1clkctrl_set(self) -> crate::common::Reg<regs::Vdac1clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1294usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn wdoglock_clr(self) -> crate::common::Reg<regs::Wdoglock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn calcmd_clr(self) -> crate::common::Reg<regs::Calcmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn calctrl_clr(self) -> crate::common::Reg<regs::Calctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn clken0_clr(self) -> crate::common::Reg<regs::Clken0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn clken1_clr(self) -> crate::common::Reg<regs::Clken1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn clken2_clr(self) -> crate::common::Reg<regs::Clken2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x206cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sysclkctrl_clr(self) -> crate::common::Reg<regs::Sysclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn traceclkctrl_clr(self) -> crate::common::Reg<regs::Traceclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn exportclkctrl_clr(self) -> crate::common::Reg<regs::Exportclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dpllrefclkctrl_clr(self) -> crate::common::Reg<regs::Dpllrefclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em01grpaclkctrl_clr(self) -> crate::common::Reg<regs::Em01grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em01grpcclkctrl_clr(self) -> crate::common::Reg<regs::Em01grpcclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em23grpaclkctrl_clr(self) -> crate::common::Reg<regs::Em23grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4grpaclkctrl_clr(self) -> crate::common::Reg<regs::Em4grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn iadcclkctrl_clr(self) -> crate::common::Reg<regs::Iadcclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn wdog0clkctrl_clr(self) -> crate::common::Reg<regs::Wdog0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn wdog1clkctrl_clr(self) -> crate::common::Reg<regs::Wdog1clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2208usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0clkctrl_clr(self) -> crate::common::Reg<regs::Eusart0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2220usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sysrtc0clkctrl_clr(self) -> crate::common::Reg<regs::Sysrtc0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2240usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lcdclkctrl_clr(self) -> crate::common::Reg<regs::Lcdclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2250usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn vdac0clkctrl_clr(self) -> crate::common::Reg<regs::Vdac0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2260usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pcnt0clkctrl_clr(self) -> crate::common::Reg<regs::Pcnt0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2270usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn radioclkctrl_clr(self) -> crate::common::Reg<regs::Radioclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn vdac1clkctrl_clr(self) -> crate::common::Reg<regs::Vdac1clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2294usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn wdoglock_tgl(self) -> crate::common::Reg<regs::Wdoglock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn calcmd_tgl(self) -> crate::common::Reg<regs::Calcmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn calctrl_tgl(self) -> crate::common::Reg<regs::Calctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn clken0_tgl(self) -> crate::common::Reg<regs::Clken0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn clken1_tgl(self) -> crate::common::Reg<regs::Clken1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn clken2_tgl(self) -> crate::common::Reg<regs::Clken2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sysclkctrl_tgl(self) -> crate::common::Reg<regs::Sysclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn traceclkctrl_tgl(self) -> crate::common::Reg<regs::Traceclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn exportclkctrl_tgl(self) -> crate::common::Reg<regs::Exportclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dpllrefclkctrl_tgl(self) -> crate::common::Reg<regs::Dpllrefclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em01grpaclkctrl_tgl(self) -> crate::common::Reg<regs::Em01grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em01grpcclkctrl_tgl(self) -> crate::common::Reg<regs::Em01grpcclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em23grpaclkctrl_tgl(self) -> crate::common::Reg<regs::Em23grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3140usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4grpaclkctrl_tgl(self) -> crate::common::Reg<regs::Em4grpaclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3160usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn iadcclkctrl_tgl(self) -> crate::common::Reg<regs::Iadcclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3180usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn wdog0clkctrl_tgl(self) -> crate::common::Reg<regs::Wdog0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn wdog1clkctrl_tgl(self) -> crate::common::Reg<regs::Wdog1clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3208usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0clkctrl_tgl(self) -> crate::common::Reg<regs::Eusart0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3220usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sysrtc0clkctrl_tgl(self) -> crate::common::Reg<regs::Sysrtc0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3240usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lcdclkctrl_tgl(self) -> crate::common::Reg<regs::Lcdclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3250usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn vdac0clkctrl_tgl(self) -> crate::common::Reg<regs::Vdac0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3260usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pcnt0clkctrl_tgl(self) -> crate::common::Reg<regs::Pcnt0clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3270usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn radioclkctrl_tgl(self) -> crate::common::Reg<regs::Radioclkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3280usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn vdac1clkctrl_tgl(self) -> crate::common::Reg<regs::Vdac1clkctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3294usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calcmd(pub u32);
    impl Calcmd {
        #[doc = "Calibration Start."]
        #[must_use]
        #[inline(always)]
        pub const fn calstart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Start."]
        #[inline(always)]
        pub const fn set_calstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Calibration Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn calstop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Stop."]
        #[inline(always)]
        pub const fn set_calstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Calcmd {
        #[inline(always)]
        fn default() -> Calcmd {
            Calcmd(0)
        }
    }
    impl core::fmt::Debug for Calcmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calcmd")
                .field("calstart", &self.calstart())
                .field("calstop", &self.calstop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calcmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calcmd {{ calstart: {=bool:?}, calstop: {=bool:?} }}",
                self.calstart(),
                self.calstop()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calcnt(pub u32);
    impl Calcnt {
        #[doc = "Calibration Result Counter Value."]
        #[must_use]
        #[inline(always)]
        pub const fn calcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Calibration Result Counter Value."]
        #[inline(always)]
        pub const fn set_calcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Calcnt {
        #[inline(always)]
        fn default() -> Calcnt {
            Calcnt(0)
        }
    }
    impl core::fmt::Debug for Calcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calcnt").field("calcnt", &self.calcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Calcnt {{ calcnt: {=u32:?} }}", self.calcnt())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calctrl(pub u32);
    impl Calctrl {
        #[doc = "Calibration Counter Top Value."]
        #[must_use]
        #[inline(always)]
        pub const fn caltop(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Calibration Counter Top Value."]
        #[inline(always)]
        pub const fn set_caltop(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Continuous Calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous Calibration."]
        #[inline(always)]
        pub const fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Calibration Up-counter Select."]
        #[must_use]
        #[inline(always)]
        pub const fn upsel(&self) -> super::vals::Upsel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Upsel::from_bits(val as u8)
        }
        #[doc = "Calibration Up-counter Select."]
        #[inline(always)]
        pub const fn set_upsel(&mut self, val: super::vals::Upsel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Calibration Down-counter Select."]
        #[must_use]
        #[inline(always)]
        pub const fn downsel(&self) -> super::vals::Downsel {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Downsel::from_bits(val as u8)
        }
        #[doc = "Calibration Down-counter Select."]
        #[inline(always)]
        pub const fn set_downsel(&mut self, val: super::vals::Downsel) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Calctrl {
        #[inline(always)]
        fn default() -> Calctrl {
            Calctrl(0)
        }
    }
    impl core::fmt::Debug for Calctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calctrl")
                .field("caltop", &self.caltop())
                .field("cont", &self.cont())
                .field("upsel", &self.upsel())
                .field("downsel", &self.downsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calctrl {{ caltop: {=u32:?}, cont: {=bool:?}, upsel: {:?}, downsel: {:?} }}",
                self.caltop(),
                self.cont(),
                self.upsel(),
                self.downsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clken0(pub u32);
    impl Clken0 {
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn ldma(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_ldma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn ldmaxbar(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_ldmaxbar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn radioaes(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_radioaes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn gpcrc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_gpcrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn usart0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_usart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn iadc0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_iadc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn amuxcp0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_amuxcp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn letimer0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_letimer0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog0(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_wdog0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c0(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_i2c0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_syscfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn dpll0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_dpll0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn hfrco0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_hfrco0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn hfrcoem23(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_hfrcoem23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn hfxo0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_hfxo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn fsrco(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_fsrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn lfrco(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_lfrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn lfxo(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_lfxo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn ulfrco(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_ulfrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_gpio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn buram(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_buram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn burtc(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_burtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn sysrtc0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_sysrtc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_dcdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Clken0 {
        #[inline(always)]
        fn default() -> Clken0 {
            Clken0(0)
        }
    }
    impl core::fmt::Debug for Clken0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clken0")
                .field("ldma", &self.ldma())
                .field("ldmaxbar", &self.ldmaxbar())
                .field("radioaes", &self.radioaes())
                .field("gpcrc", &self.gpcrc())
                .field("timer0", &self.timer0())
                .field("timer1", &self.timer1())
                .field("timer2", &self.timer2())
                .field("timer3", &self.timer3())
                .field("timer4", &self.timer4())
                .field("usart0", &self.usart0())
                .field("iadc0", &self.iadc0())
                .field("amuxcp0", &self.amuxcp0())
                .field("letimer0", &self.letimer0())
                .field("wdog0", &self.wdog0())
                .field("i2c0", &self.i2c0())
                .field("i2c1", &self.i2c1())
                .field("syscfg", &self.syscfg())
                .field("dpll0", &self.dpll0())
                .field("hfrco0", &self.hfrco0())
                .field("hfrcoem23", &self.hfrcoem23())
                .field("hfxo0", &self.hfxo0())
                .field("fsrco", &self.fsrco())
                .field("lfrco", &self.lfrco())
                .field("lfxo", &self.lfxo())
                .field("ulfrco", &self.ulfrco())
                .field("gpio", &self.gpio())
                .field("prs", &self.prs())
                .field("buram", &self.buram())
                .field("burtc", &self.burtc())
                .field("sysrtc0", &self.sysrtc0())
                .field("dcdc", &self.dcdc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clken0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clken0 {{ ldma: {=bool:?}, ldmaxbar: {=bool:?}, radioaes: {=bool:?}, gpcrc: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, timer2: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, usart0: {=bool:?}, iadc0: {=bool:?}, amuxcp0: {=bool:?}, letimer0: {=bool:?}, wdog0: {=bool:?}, i2c0: {=bool:?}, i2c1: {=bool:?}, syscfg: {=bool:?}, dpll0: {=bool:?}, hfrco0: {=bool:?}, hfrcoem23: {=bool:?}, hfxo0: {=bool:?}, fsrco: {=bool:?}, lfrco: {=bool:?}, lfxo: {=bool:?}, ulfrco: {=bool:?}, gpio: {=bool:?}, prs: {=bool:?}, buram: {=bool:?}, burtc: {=bool:?}, sysrtc0: {=bool:?}, dcdc: {=bool:?} }}",
                self.ldma(),
                self.ldmaxbar(),
                self.radioaes(),
                self.gpcrc(),
                self.timer0(),
                self.timer1(),
                self.timer2(),
                self.timer3(),
                self.timer4(),
                self.usart0(),
                self.iadc0(),
                self.amuxcp0(),
                self.letimer0(),
                self.wdog0(),
                self.i2c0(),
                self.i2c1(),
                self.syscfg(),
                self.dpll0(),
                self.hfrco0(),
                self.hfrcoem23(),
                self.hfxo0(),
                self.fsrco(),
                self.lfrco(),
                self.lfxo(),
                self.ulfrco(),
                self.gpio(),
                self.prs(),
                self.buram(),
                self.burtc(),
                self.sysrtc0(),
                self.dcdc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clken1(pub u32);
    impl Clken1 {
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn agc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_agc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn modem(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_modem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rfcrc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_rfcrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn frc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_frc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn protimer(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_protimer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rac(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_rac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn synth(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_synth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rfscratchpad(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_rfscratchpad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn hostmailbox(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_hostmailbox(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rfmailbox(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_rfmailbox(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn semailboxhost(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_semailboxhost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn bufc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_bufc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn lcd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn keyscan(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_keyscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn smu(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_smu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn icache0(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_icache0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn msc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_msc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_wdog1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_acmp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_acmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn vdac0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_vdac0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn pcnt0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_pcnt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart0(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_eusart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_eusart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rfeca0(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_rfeca0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rfeca1(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_rfeca1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn dmem(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_dmem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn ecaifadc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_ecaifadc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn vdac1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_vdac1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn mvp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_mvp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Clken1 {
        #[inline(always)]
        fn default() -> Clken1 {
            Clken1(0)
        }
    }
    impl core::fmt::Debug for Clken1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clken1")
                .field("agc", &self.agc())
                .field("modem", &self.modem())
                .field("rfcrc", &self.rfcrc())
                .field("frc", &self.frc())
                .field("protimer", &self.protimer())
                .field("rac", &self.rac())
                .field("synth", &self.synth())
                .field("rfscratchpad", &self.rfscratchpad())
                .field("hostmailbox", &self.hostmailbox())
                .field("rfmailbox", &self.rfmailbox())
                .field("semailboxhost", &self.semailboxhost())
                .field("bufc", &self.bufc())
                .field("lcd", &self.lcd())
                .field("keyscan", &self.keyscan())
                .field("smu", &self.smu())
                .field("icache0", &self.icache0())
                .field("msc", &self.msc())
                .field("wdog1", &self.wdog1())
                .field("acmp0", &self.acmp0())
                .field("acmp1", &self.acmp1())
                .field("vdac0", &self.vdac0())
                .field("pcnt0", &self.pcnt0())
                .field("eusart0", &self.eusart0())
                .field("eusart1", &self.eusart1())
                .field("rfeca0", &self.rfeca0())
                .field("rfeca1", &self.rfeca1())
                .field("dmem", &self.dmem())
                .field("ecaifadc", &self.ecaifadc())
                .field("vdac1", &self.vdac1())
                .field("mvp", &self.mvp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clken1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clken1 {{ agc: {=bool:?}, modem: {=bool:?}, rfcrc: {=bool:?}, frc: {=bool:?}, protimer: {=bool:?}, rac: {=bool:?}, synth: {=bool:?}, rfscratchpad: {=bool:?}, hostmailbox: {=bool:?}, rfmailbox: {=bool:?}, semailboxhost: {=bool:?}, bufc: {=bool:?}, lcd: {=bool:?}, keyscan: {=bool:?}, smu: {=bool:?}, icache0: {=bool:?}, msc: {=bool:?}, wdog1: {=bool:?}, acmp0: {=bool:?}, acmp1: {=bool:?}, vdac0: {=bool:?}, pcnt0: {=bool:?}, eusart0: {=bool:?}, eusart1: {=bool:?}, rfeca0: {=bool:?}, rfeca1: {=bool:?}, dmem: {=bool:?}, ecaifadc: {=bool:?}, vdac1: {=bool:?}, mvp: {=bool:?} }}",
                self.agc(),
                self.modem(),
                self.rfcrc(),
                self.frc(),
                self.protimer(),
                self.rac(),
                self.synth(),
                self.rfscratchpad(),
                self.hostmailbox(),
                self.rfmailbox(),
                self.semailboxhost(),
                self.bufc(),
                self.lcd(),
                self.keyscan(),
                self.smu(),
                self.icache0(),
                self.msc(),
                self.wdog1(),
                self.acmp0(),
                self.acmp1(),
                self.vdac0(),
                self.pcnt0(),
                self.eusart0(),
                self.eusart1(),
                self.rfeca0(),
                self.rfeca1(),
                self.dmem(),
                self.ecaifadc(),
                self.vdac1(),
                self.mvp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clken2(pub u32);
    impl Clken2 {
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer5(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer6(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer7(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer8(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn timer9(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_timer9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_eusart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_eusart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable Bus Clock."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Bus Clock."]
        #[inline(always)]
        pub const fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Clken2 {
        #[inline(always)]
        fn default() -> Clken2 {
            Clken2(0)
        }
    }
    impl core::fmt::Debug for Clken2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clken2")
                .field("timer5", &self.timer5())
                .field("timer6", &self.timer6())
                .field("timer7", &self.timer7())
                .field("timer8", &self.timer8())
                .field("timer9", &self.timer9())
                .field("eusart2", &self.eusart2())
                .field("eusart3", &self.eusart3())
                .field("usart1", &self.usart1())
                .field("usart2", &self.usart2())
                .field("i2c2", &self.i2c2())
                .field("i2c3", &self.i2c3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clken2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clken2 {{ timer5: {=bool:?}, timer6: {=bool:?}, timer7: {=bool:?}, timer8: {=bool:?}, timer9: {=bool:?}, eusart2: {=bool:?}, eusart3: {=bool:?}, usart1: {=bool:?}, usart2: {=bool:?}, i2c2: {=bool:?}, i2c3: {=bool:?} }}",
                self.timer5(),
                self.timer6(),
                self.timer7(),
                self.timer8(),
                self.timer9(),
                self.eusart2(),
                self.eusart3(),
                self.usart1(),
                self.usart2(),
                self.i2c2(),
                self.i2c3()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dpllrefclkctrl(pub u32);
    impl Dpllrefclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::DpllrefclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::DpllrefclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::DpllrefclkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Dpllrefclkctrl {
        #[inline(always)]
        fn default() -> Dpllrefclkctrl {
            Dpllrefclkctrl(0)
        }
    }
    impl core::fmt::Debug for Dpllrefclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dpllrefclkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dpllrefclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dpllrefclkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em01grpaclkctrl(pub u32);
    impl Em01grpaclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Em01grpaclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Em01grpaclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Em01grpaclkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Em01grpaclkctrl {
        #[inline(always)]
        fn default() -> Em01grpaclkctrl {
            Em01grpaclkctrl(0)
        }
    }
    impl core::fmt::Debug for Em01grpaclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em01grpaclkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em01grpaclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Em01grpaclkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em01grpcclkctrl(pub u32);
    impl Em01grpcclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Em01grpcclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Em01grpcclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Em01grpcclkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Em01grpcclkctrl {
        #[inline(always)]
        fn default() -> Em01grpcclkctrl {
            Em01grpcclkctrl(0)
        }
    }
    impl core::fmt::Debug for Em01grpcclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em01grpcclkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em01grpcclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Em01grpcclkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em23grpaclkctrl(pub u32);
    impl Em23grpaclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Em23grpaclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Em23grpaclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Em23grpaclkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Em23grpaclkctrl {
        #[inline(always)]
        fn default() -> Em23grpaclkctrl {
            Em23grpaclkctrl(0)
        }
    }
    impl core::fmt::Debug for Em23grpaclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em23grpaclkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em23grpaclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Em23grpaclkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em4grpaclkctrl(pub u32);
    impl Em4grpaclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Em4grpaclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Em4grpaclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Em4grpaclkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Em4grpaclkctrl {
        #[inline(always)]
        fn default() -> Em4grpaclkctrl {
            Em4grpaclkctrl(0)
        }
    }
    impl core::fmt::Debug for Em4grpaclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em4grpaclkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em4grpaclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Em4grpaclkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0clkctrl(pub u32);
    impl Eusart0clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Eusart0clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Eusart0clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Eusart0clkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Eusart0clkctrl {
        #[inline(always)]
        fn default() -> Eusart0clkctrl {
            Eusart0clkctrl(0)
        }
    }
    impl core::fmt::Debug for Eusart0clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0clkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eusart0clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exportclkctrl(pub u32);
    impl Exportclkctrl {
        #[doc = "Clock Output Select 0."]
        #[must_use]
        #[inline(always)]
        pub const fn clkoutsel0(&self) -> super::vals::Clkoutsel0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Clkoutsel0::from_bits(val as u8)
        }
        #[doc = "Clock Output Select 0."]
        #[inline(always)]
        pub const fn set_clkoutsel0(&mut self, val: super::vals::Clkoutsel0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Clock Output Select 1."]
        #[must_use]
        #[inline(always)]
        pub const fn clkoutsel1(&self) -> super::vals::Clkoutsel1 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Clkoutsel1::from_bits(val as u8)
        }
        #[doc = "Clock Output Select 1."]
        #[inline(always)]
        pub const fn set_clkoutsel1(&mut self, val: super::vals::Clkoutsel1) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Clock Output Select 2."]
        #[must_use]
        #[inline(always)]
        pub const fn clkoutsel2(&self) -> super::vals::Clkoutsel2 {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Clkoutsel2::from_bits(val as u8)
        }
        #[doc = "Clock Output Select 2."]
        #[inline(always)]
        pub const fn set_clkoutsel2(&mut self, val: super::vals::Clkoutsel2) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "EXPORTCLK Prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "EXPORTCLK Prescaler."]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Exportclkctrl {
        #[inline(always)]
        fn default() -> Exportclkctrl {
            Exportclkctrl(0)
        }
    }
    impl core::fmt::Debug for Exportclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exportclkctrl")
                .field("clkoutsel0", &self.clkoutsel0())
                .field("clkoutsel1", &self.clkoutsel1())
                .field("clkoutsel2", &self.clkoutsel2())
                .field("presc", &self.presc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exportclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exportclkctrl {{ clkoutsel0: {:?}, clkoutsel1: {:?}, clkoutsel2: {:?}, presc: {=u8:?} }}",
                self.clkoutsel0(),
                self.clkoutsel1(),
                self.clkoutsel2(),
                self.presc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iadcclkctrl(pub u32);
    impl Iadcclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::IadcclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::IadcclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::IadcclkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Iadcclkctrl {
        #[inline(always)]
        fn default() -> Iadcclkctrl {
            Iadcclkctrl(0)
        }
    }
    impl core::fmt::Debug for Iadcclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iadcclkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iadcclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iadcclkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Calibration Ready Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn calrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Ready Interrupt Enable."]
        #[inline(always)]
        pub const fn set_calrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Calibration Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn calof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_calof(&mut self, val: bool) {
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
                .field("calrdy", &self.calrdy())
                .field("calof", &self.calof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ calrdy: {=bool:?}, calof: {=bool:?} }}",
                self.calrdy(),
                self.calof()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Calibration Ready Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn calrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Ready Interrupt Flag."]
        #[inline(always)]
        pub const fn set_calrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Calibration Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn calof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_calof(&mut self, val: bool) {
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
                .field("calrdy", &self.calrdy())
                .field("calof", &self.calof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ calrdy: {=bool:?}, calof: {=bool:?} }}",
                self.calrdy(),
                self.calof()
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
    pub struct Lcdclkctrl(pub u32);
    impl Lcdclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::LcdclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::LcdclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::LcdclkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Lcdclkctrl {
        #[inline(always)]
        fn default() -> Lcdclkctrl {
            Lcdclkctrl(0)
        }
    }
    impl core::fmt::Debug for Lcdclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lcdclkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lcdclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lcdclkctrl {{ clksel: {:?} }}", self.clksel())
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
        pub const fn lockkey(&self) -> super::vals::LockLockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::LockLockkey::from_bits(val as u16)
        }
        #[doc = "Configuration Lock Key."]
        #[inline(always)]
        pub const fn set_lockkey(&mut self, val: super::vals::LockLockkey) {
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
    pub struct Pcnt0clkctrl(pub u32);
    impl Pcnt0clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Pcnt0clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pcnt0clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Pcnt0clkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Pcnt0clkctrl {
        #[inline(always)]
        fn default() -> Pcnt0clkctrl {
            Pcnt0clkctrl(0)
        }
    }
    impl core::fmt::Debug for Pcnt0clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnt0clkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcnt0clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pcnt0clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radioclkctrl(pub u32);
    impl Radioclkctrl {
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
        #[doc = "Enable Clock for Debugger."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgclk(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Clock for Debugger."]
        #[inline(always)]
        pub const fn set_dbgclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Radioclkctrl {
        #[inline(always)]
        fn default() -> Radioclkctrl {
            Radioclkctrl(0)
        }
    }
    impl core::fmt::Debug for Radioclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Radioclkctrl")
                .field("en", &self.en())
                .field("dbgclk", &self.dbgclk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Radioclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Radioclkctrl {{ en: {=bool:?}, dbgclk: {=bool:?} }}",
                self.en(),
                self.dbgclk()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Calibration Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn calrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration Ready."]
        #[inline(always)]
        pub const fn set_calrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Configuration Lock Status for WDOG."]
        #[must_use]
        #[inline(always)]
        pub const fn wdoglock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration Lock Status for WDOG."]
        #[inline(always)]
        pub const fn set_wdoglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("calrdy", &self.calrdy())
                .field("wdoglock", &self.wdoglock())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ calrdy: {=bool:?}, wdoglock: {=bool:?}, lock: {=bool:?} }}",
                self.calrdy(),
                self.wdoglock(),
                self.lock()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sysclkctrl(pub u32);
    impl Sysclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::SysclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SysclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::SysclkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "PCLK Prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn pclkpresc(&self) -> super::vals::Pclkpresc {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Pclkpresc::from_bits(val as u8)
        }
        #[doc = "PCLK Prescaler."]
        #[inline(always)]
        pub const fn set_pclkpresc(&mut self, val: super::vals::Pclkpresc) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "HCLK Prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn hclkpresc(&self) -> super::vals::Hclkpresc {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Hclkpresc::from_bits(val as u8)
        }
        #[doc = "HCLK Prescaler."]
        #[inline(always)]
        pub const fn set_hclkpresc(&mut self, val: super::vals::Hclkpresc) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Radio HCLK Prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn rhclkpresc(&self) -> super::vals::Rhclkpresc {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Rhclkpresc::from_bits(val as u8)
        }
        #[doc = "Radio HCLK Prescaler."]
        #[inline(always)]
        pub const fn set_rhclkpresc(&mut self, val: super::vals::Rhclkpresc) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Sysclkctrl {
        #[inline(always)]
        fn default() -> Sysclkctrl {
            Sysclkctrl(0)
        }
    }
    impl core::fmt::Debug for Sysclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sysclkctrl")
                .field("clksel", &self.clksel())
                .field("pclkpresc", &self.pclkpresc())
                .field("hclkpresc", &self.hclkpresc())
                .field("rhclkpresc", &self.rhclkpresc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sysclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sysclkctrl {{ clksel: {:?}, pclkpresc: {:?}, hclkpresc: {:?}, rhclkpresc: {:?} }}",
                self.clksel(),
                self.pclkpresc(),
                self.hclkpresc(),
                self.rhclkpresc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sysrtc0clkctrl(pub u32);
    impl Sysrtc0clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Sysrtc0clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sysrtc0clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Sysrtc0clkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Sysrtc0clkctrl {
        #[inline(always)]
        fn default() -> Sysrtc0clkctrl {
            Sysrtc0clkctrl(0)
        }
    }
    impl core::fmt::Debug for Sysrtc0clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sysrtc0clkctrl")
                .field("clksel", &self.clksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sysrtc0clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sysrtc0clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Traceclkctrl(pub u32);
    impl Traceclkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::TraceclkctrlClksel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::TraceclkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::TraceclkctrlClksel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "TRACECLK Prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "TRACECLK Prescaler."]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Traceclkctrl {
        #[inline(always)]
        fn default() -> Traceclkctrl {
            Traceclkctrl(0)
        }
    }
    impl core::fmt::Debug for Traceclkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Traceclkctrl")
                .field("clksel", &self.clksel())
                .field("presc", &self.presc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Traceclkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Traceclkctrl {{ clksel: {:?}, presc: {:?} }}",
                self.clksel(),
                self.presc()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vdac0clkctrl(pub u32);
    impl Vdac0clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Vdac0clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Vdac0clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Vdac0clkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Vdac0clkctrl {
        #[inline(always)]
        fn default() -> Vdac0clkctrl {
            Vdac0clkctrl(0)
        }
    }
    impl core::fmt::Debug for Vdac0clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vdac0clkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vdac0clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vdac0clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vdac1clkctrl(pub u32);
    impl Vdac1clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Vdac1clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Vdac1clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Vdac1clkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Vdac1clkctrl {
        #[inline(always)]
        fn default() -> Vdac1clkctrl {
            Vdac1clkctrl(0)
        }
    }
    impl core::fmt::Debug for Vdac1clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vdac1clkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vdac1clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vdac1clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdog0clkctrl(pub u32);
    impl Wdog0clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Wdog0clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Wdog0clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Wdog0clkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Wdog0clkctrl {
        #[inline(always)]
        fn default() -> Wdog0clkctrl {
            Wdog0clkctrl(0)
        }
    }
    impl core::fmt::Debug for Wdog0clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdog0clkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdog0clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdog0clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdog1clkctrl(pub u32);
    impl Wdog1clkctrl {
        #[doc = "Clock Select."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> super::vals::Wdog1clkctrlClksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Wdog1clkctrlClksel::from_bits(val as u8)
        }
        #[doc = "Clock Select."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: super::vals::Wdog1clkctrlClksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Wdog1clkctrl {
        #[inline(always)]
        fn default() -> Wdog1clkctrl {
            Wdog1clkctrl(0)
        }
    }
    impl core::fmt::Debug for Wdog1clkctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdog1clkctrl").field("clksel", &self.clksel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdog1clkctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdog1clkctrl {{ clksel: {:?} }}", self.clksel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdoglock(pub u32);
    impl Wdoglock {
        #[doc = "Configuration Lock Key."]
        #[must_use]
        #[inline(always)]
        pub const fn lockkey(&self) -> super::vals::WdoglockLockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::WdoglockLockkey::from_bits(val as u16)
        }
        #[doc = "Configuration Lock Key."]
        #[inline(always)]
        pub const fn set_lockkey(&mut self, val: super::vals::WdoglockLockkey) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wdoglock {
        #[inline(always)]
        fn default() -> Wdoglock {
            Wdoglock(0)
        }
    }
    impl core::fmt::Debug for Wdoglock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdoglock").field("lockkey", &self.lockkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdoglock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdoglock {{ lockkey: {:?} }}", self.lockkey())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkoutsel0 {
        #[doc = "CLKOUT0 is not clocked."]
        Disabled = 0x0,
        #[doc = "HCLK is clocking CLKOUT0."]
        Hclk = 0x01,
        #[doc = "EXPORTCLK is clocking CLKOUT0."]
        Hfexpclk = 0x02,
        #[doc = "ULFRCO is clocking CLKOUT0."]
        Ulfrco = 0x03,
        #[doc = "LFRCO is clocking CLKOUT0."]
        Lfrco = 0x04,
        #[doc = "LFXO is clocking CLKOUT0."]
        Lfxo = 0x05,
        #[doc = "HFRCODPLL is clocking CLKOUT0."]
        Hfrcodpll = 0x06,
        #[doc = "HFXO is clocking CLKOUT0."]
        Hfxo = 0x07,
        #[doc = "FSRCO is clocking CLKOUT0."]
        Fsrco = 0x08,
        #[doc = "HFRCOEM23 is clocking CLKOUT0."]
        Hfrcoem23 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Clkoutsel0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkoutsel0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkoutsel0 {
        #[inline(always)]
        fn from(val: u8) -> Clkoutsel0 {
            Clkoutsel0::from_bits(val)
        }
    }
    impl From<Clkoutsel0> for u8 {
        #[inline(always)]
        fn from(val: Clkoutsel0) -> u8 {
            Clkoutsel0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkoutsel1 {
        #[doc = "CLKOUT1 is not clocked."]
        Disabled = 0x0,
        #[doc = "HCLK is clocking CLKOUT1."]
        Hclk = 0x01,
        #[doc = "EXPORTCLK is clocking CLKOUT1."]
        Hfexpclk = 0x02,
        #[doc = "ULFRCO is clocking CLKOUT1."]
        Ulfrco = 0x03,
        #[doc = "LFRCO is clocking CLKOUT1."]
        Lfrco = 0x04,
        #[doc = "LFXO is clocking CLKOUT1."]
        Lfxo = 0x05,
        #[doc = "HFRCODPLL is clocking CLKOUT1."]
        Hfrcodpll = 0x06,
        #[doc = "HFXO is clocking CLKOUT1."]
        Hfxo = 0x07,
        #[doc = "FSRCO is clocking CLKOUT1."]
        Fsrco = 0x08,
        #[doc = "HFRCOEM23 is clocking CLKOUT1."]
        Hfrcoem23 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Clkoutsel1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkoutsel1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkoutsel1 {
        #[inline(always)]
        fn from(val: u8) -> Clkoutsel1 {
            Clkoutsel1::from_bits(val)
        }
    }
    impl From<Clkoutsel1> for u8 {
        #[inline(always)]
        fn from(val: Clkoutsel1) -> u8 {
            Clkoutsel1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkoutsel2 {
        #[doc = "CLKOUT2 is not clocked."]
        Disabled = 0x0,
        #[doc = "HCLK is clocking CLKOUT2."]
        Hclk = 0x01,
        #[doc = "EXPORTCLK is clocking CLKOUT2."]
        Hfexpclk = 0x02,
        #[doc = "ULFRCO is clocking CLKOUT2."]
        Ulfrco = 0x03,
        #[doc = "LFRCO is clocking CLKOUT2."]
        Lfrco = 0x04,
        #[doc = "LFXO is clocking CLKOUT2."]
        Lfxo = 0x05,
        #[doc = "HFRCODPLL is clocking CLKOUT2."]
        Hfrcodpll = 0x06,
        #[doc = "HFXO is clocking CLKOUT2."]
        Hfxo = 0x07,
        #[doc = "FSRCO is clocking CLKOUT2."]
        Fsrco = 0x08,
        #[doc = "HFRCOEM23 is clocking CLKOUT2."]
        Hfrcoem23 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Clkoutsel2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkoutsel2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkoutsel2 {
        #[inline(always)]
        fn from(val: u8) -> Clkoutsel2 {
            Clkoutsel2::from_bits(val)
        }
    }
    impl From<Clkoutsel2> for u8 {
        #[inline(always)]
        fn from(val: Clkoutsel2) -> u8 {
            Clkoutsel2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Downsel {
        #[doc = "Down-counter is not clocked."]
        Disabled = 0x0,
        #[doc = "HCLK is clocking down-counter."]
        Hclk = 0x01,
        #[doc = "PRS CMU_CALDN consumer is clocking down-counter."]
        Prs = 0x02,
        #[doc = "HFXO is clocking down-counter."]
        Hfxo = 0x03,
        #[doc = "LFXO is clocking down-counter."]
        Lfxo = 0x04,
        #[doc = "HFRCODPLL is clocking down-counter."]
        Hfrcodpll = 0x05,
        #[doc = "HFRCOEM23 is clocking down-counter."]
        Hfrcoem23 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        #[doc = "FSRCO is clocking down-counter."]
        Fsrco = 0x09,
        #[doc = "LFRCO is clocking down-counter."]
        Lfrco = 0x0a,
        #[doc = "ULFRCO is clocking down-counter."]
        Ulfrco = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Downsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Downsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Downsel {
        #[inline(always)]
        fn from(val: u8) -> Downsel {
            Downsel::from_bits(val)
        }
    }
    impl From<Downsel> for u8 {
        #[inline(always)]
        fn from(val: Downsel) -> u8 {
            Downsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DpllrefclkctrlClksel {
        #[doc = "DPLLREFCLK is not clocked."]
        Disabled = 0x0,
        #[doc = "HFXO is clocking DPLLREFCLK."]
        Hfxo = 0x01,
        #[doc = "LFXO is clocking DPLLREFCLK."]
        Lfxo = 0x02,
        #[doc = "CLKIN0 is clocking DPLLREFCLK."]
        Clkin0 = 0x03,
    }
    impl DpllrefclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DpllrefclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DpllrefclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> DpllrefclkctrlClksel {
            DpllrefclkctrlClksel::from_bits(val)
        }
    }
    impl From<DpllrefclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: DpllrefclkctrlClksel) -> u8 {
            DpllrefclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em01grpaclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "HFRCODPLL is clocking EM01GRPACLK."]
        Hfrcodpll = 0x01,
        #[doc = "HFXO is clocking EM01GRPACLK."]
        Hfxo = 0x02,
        #[doc = "FSRCO is clocking EM01GRPACLK."]
        Fsrco = 0x03,
        #[doc = "HFRCOEM23 is clocking EM01GRPACLK."]
        Hfrcoem23 = 0x04,
        #[doc = "HFRCODPLL (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
        Hfrcodpllrt = 0x05,
        #[doc = "HFXO (retimed) is clocking EM01GRPACLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
        Hfxort = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Em01grpaclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em01grpaclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em01grpaclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Em01grpaclkctrlClksel {
            Em01grpaclkctrlClksel::from_bits(val)
        }
    }
    impl From<Em01grpaclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Em01grpaclkctrlClksel) -> u8 {
            Em01grpaclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em01grpcclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "HFRCODPLL is clocking EM01GRPCCLK."]
        Hfrcodpll = 0x01,
        #[doc = "HFXO is clocking EM01GRPCCLK."]
        Hfxo = 0x02,
        #[doc = "FSRCO is clocking EM01GRPCCLK."]
        Fsrco = 0x03,
        #[doc = "HFRCOEM23 is clocking EM01GRPCCLK."]
        Hfrcoem23 = 0x04,
        #[doc = "HFRCODPLL (retimed) is clocking EM01GRPCCLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
        Hfrcodpllrt = 0x05,
        #[doc = "HFXO (retimed) is clocking EM01GRPCCLK. Check with datasheet for frequency limitation when using retiming with voltage scaling."]
        Hfxort = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Em01grpcclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em01grpcclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em01grpcclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Em01grpcclkctrlClksel {
            Em01grpcclkctrlClksel::from_bits(val)
        }
    }
    impl From<Em01grpcclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Em01grpcclkctrlClksel) -> u8 {
            Em01grpcclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em23grpaclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "LFRCO is clocking EM23GRPACLK."]
        Lfrco = 0x01,
        #[doc = "LFXO is clocking EM23GRPACLK."]
        Lfxo = 0x02,
        #[doc = "ULFRCO is clocking EM23GRPACLK."]
        Ulfrco = 0x03,
    }
    impl Em23grpaclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em23grpaclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em23grpaclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Em23grpaclkctrlClksel {
            Em23grpaclkctrlClksel::from_bits(val)
        }
    }
    impl From<Em23grpaclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Em23grpaclkctrlClksel) -> u8 {
            Em23grpaclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em4grpaclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "LFRCO is clocking EM4GRPACLK."]
        Lfrco = 0x01,
        #[doc = "LFXO is clocking EM4GRPACLK."]
        Lfxo = 0x02,
        #[doc = "ULFRCO is clocking EM4GRPACLK."]
        Ulfrco = 0x03,
    }
    impl Em4grpaclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em4grpaclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em4grpaclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Em4grpaclkctrlClksel {
            Em4grpaclkctrlClksel::from_bits(val)
        }
    }
    impl From<Em4grpaclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Em4grpaclkctrlClksel) -> u8 {
            Em4grpaclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eusart0clkctrlClksel {
        #[doc = "EUSART0 is not clocked."]
        Disabled = 0x0,
        #[doc = "EM01GRPCCLK is clocking EUSART0."]
        Em01grpcclk = 0x01,
        #[doc = "HFRCOEM23 is clocking EUSART0."]
        Hfrcoem23 = 0x02,
        #[doc = "LFRCO is clocking EUSART0."]
        Lfrco = 0x03,
        #[doc = "LFXO is clocking EUSART0."]
        Lfxo = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Eusart0clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eusart0clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eusart0clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Eusart0clkctrlClksel {
            Eusart0clkctrlClksel::from_bits(val)
        }
    }
    impl From<Eusart0clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Eusart0clkctrlClksel) -> u8 {
            Eusart0clkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hclkpresc {
        #[doc = "HCLK is SYSCLK divided by 1."]
        Div1 = 0x0,
        #[doc = "HCLK is SYSCLK divided by 2."]
        Div2 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "HCLK is SYSCLK divided by 4."]
        Div4 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "HCLK is SYSCLK divided by 8."]
        Div8 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "HCLK is SYSCLK divided by 16."]
        Div16 = 0x0f,
    }
    impl Hclkpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hclkpresc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hclkpresc {
        #[inline(always)]
        fn from(val: u8) -> Hclkpresc {
            Hclkpresc::from_bits(val)
        }
    }
    impl From<Hclkpresc> for u8 {
        #[inline(always)]
        fn from(val: Hclkpresc) -> u8 {
            Hclkpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum IadcclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "EM01GRPACLK is clocking IADCCLK."]
        Em01grpaclk = 0x01,
        #[doc = "FSRCO is clocking IADCCLK."]
        Fsrco = 0x02,
        #[doc = "HFRCOEM23 is clocking IADCCLK."]
        Hfrcoem23 = 0x03,
    }
    impl IadcclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IadcclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IadcclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> IadcclkctrlClksel {
            IadcclkctrlClksel::from_bits(val)
        }
    }
    impl From<IadcclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: IadcclkctrlClksel) -> u8 {
            IadcclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LcdclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "LFRCO is clocking LCDCLK."]
        Lfrco = 0x01,
        #[doc = "LFXO is clocking LCDCLK."]
        Lfxo = 0x02,
        #[doc = "ULFRCO is clocking LCDCLK."]
        Ulfrco = 0x03,
    }
    impl LcdclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LcdclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LcdclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> LcdclkctrlClksel {
            LcdclkctrlClksel::from_bits(val)
        }
    }
    impl From<LcdclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: LcdclkctrlClksel) -> u8 {
            LcdclkctrlClksel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LockLockkey(u16);
    impl LockLockkey {
        #[doc = "Write this value to unlock."]
        pub const Unlock: Self = Self(0x93f7);
    }
    impl LockLockkey {
        pub const fn from_bits(val: u16) -> LockLockkey {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for LockLockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x93f7 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LockLockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x93f7 => defmt::write!(f, "Unlock"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for LockLockkey {
        #[inline(always)]
        fn from(val: u16) -> LockLockkey {
            LockLockkey::from_bits(val)
        }
    }
    impl From<LockLockkey> for u16 {
        #[inline(always)]
        fn from(val: LockLockkey) -> u16 {
            LockLockkey::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pclkpresc {
        #[doc = "PCLK is HCLK divided by 1."]
        Div1 = 0x0,
        #[doc = "PCLK is HCLK divided by 2."]
        Div2 = 0x01,
    }
    impl Pclkpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pclkpresc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pclkpresc {
        #[inline(always)]
        fn from(val: u8) -> Pclkpresc {
            Pclkpresc::from_bits(val)
        }
    }
    impl From<Pclkpresc> for u8 {
        #[inline(always)]
        fn from(val: Pclkpresc) -> u8 {
            Pclkpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pcnt0clkctrlClksel {
        #[doc = "PCNT0 is not clocked."]
        Disabled = 0x0,
        #[doc = "EM23GRPACLK is clocking PCNT0."]
        Em23grpaclk = 0x01,
        #[doc = "External pin PCNT_S0 is clocking PCNT0."]
        Pcnts0 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pcnt0clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcnt0clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcnt0clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Pcnt0clkctrlClksel {
            Pcnt0clkctrlClksel::from_bits(val)
        }
    }
    impl From<Pcnt0clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Pcnt0clkctrlClksel) -> u8 {
            Pcnt0clkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Presc {
        #[doc = "TRACECLK is selected clock source divided by 1."]
        Div1 = 0x0,
        #[doc = "TRACECLK is selected clock source divided by 2."]
        Div2 = 0x01,
        #[doc = "TRACECLK is selected clock source divided by 3."]
        Div3 = 0x02,
        #[doc = "TRACECLK is selected clock source divided by 4."]
        Div4 = 0x03,
    }
    impl Presc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Presc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Presc {
        #[inline(always)]
        fn from(val: u8) -> Presc {
            Presc::from_bits(val)
        }
    }
    impl From<Presc> for u8 {
        #[inline(always)]
        fn from(val: Presc) -> u8 {
            Presc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rhclkpresc {
        #[doc = "Radio HCLK is SYSCLK divided by 1."]
        Div1 = 0x0,
        #[doc = "Radio HCLK is SYSCLK divided by 2."]
        Div2 = 0x01,
    }
    impl Rhclkpresc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rhclkpresc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rhclkpresc {
        #[inline(always)]
        fn from(val: u8) -> Rhclkpresc {
            Rhclkpresc::from_bits(val)
        }
    }
    impl From<Rhclkpresc> for u8 {
        #[inline(always)]
        fn from(val: Rhclkpresc) -> u8 {
            Rhclkpresc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SysclkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "FSRCO is clocking SYSCLK."]
        Fsrco = 0x01,
        #[doc = "HFRCODPLL is clocking SYSCLK."]
        Hfrcodpll = 0x02,
        #[doc = "HFXO is clocking SYSCLK."]
        Hfxo = 0x03,
        #[doc = "CLKIN0 is clocking SYSCLK."]
        Clkin0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl SysclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SysclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SysclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> SysclkctrlClksel {
            SysclkctrlClksel::from_bits(val)
        }
    }
    impl From<SysclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: SysclkctrlClksel) -> u8 {
            SysclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sysrtc0clkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "LFRCO is clocking SYSRTC0CLK."]
        Lfrco = 0x01,
        #[doc = "LFXO is clocking SYSRTC0CLK."]
        Lfxo = 0x02,
        #[doc = "ULFRCO is clocking SYSRTC0CLK."]
        Ulfrco = 0x03,
    }
    impl Sysrtc0clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sysrtc0clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sysrtc0clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Sysrtc0clkctrlClksel {
            Sysrtc0clkctrlClksel::from_bits(val)
        }
    }
    impl From<Sysrtc0clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Sysrtc0clkctrlClksel) -> u8 {
            Sysrtc0clkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TraceclkctrlClksel {
        #[doc = "TRACE clock is disable."]
        Disable = 0x0,
        #[doc = "SYSCLK is driving TRACE."]
        Sysclk = 0x01,
        #[doc = "HFRCOEM23 is driving TRACE."]
        Hfrcoem23 = 0x02,
        #[doc = "HFRCODPLLRT is driving TRACE."]
        Hfrcodpllrt = 0x03,
    }
    impl TraceclkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TraceclkctrlClksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TraceclkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> TraceclkctrlClksel {
            TraceclkctrlClksel::from_bits(val)
        }
    }
    impl From<TraceclkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: TraceclkctrlClksel) -> u8 {
            TraceclkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Upsel {
        #[doc = "Up-counter is not clocked."]
        Disabled = 0x0,
        #[doc = "PRS CMU_CALUP consumer is clocking up-counter."]
        Prs = 0x01,
        #[doc = "HFXO is clocking up-counter."]
        Hfxo = 0x02,
        #[doc = "LFXO is clocking up-counter."]
        Lfxo = 0x03,
        #[doc = "HFRCODPLL is clocking up-counter."]
        Hfrcodpll = 0x04,
        #[doc = "HFRCOEM23 is clocking up-counter."]
        Hfrcoem23 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "FSRCO is clocking up-counter."]
        Fsrco = 0x08,
        #[doc = "LFRCO is clocking up-counter."]
        Lfrco = 0x09,
        #[doc = "ULFRCO is clocking up-counter."]
        Ulfrco = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Upsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Upsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Upsel {
        #[inline(always)]
        fn from(val: u8) -> Upsel {
            Upsel::from_bits(val)
        }
    }
    impl From<Upsel> for u8 {
        #[inline(always)]
        fn from(val: Upsel) -> u8 {
            Upsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vdac0clkctrlClksel {
        #[doc = "VDAC is not clocked."]
        Disabled = 0x0,
        #[doc = "EM01GRPACLK is clocking VDAC."]
        Em01grpaclk = 0x01,
        #[doc = "EM23GRPACLK is clocking VDAC."]
        Em23grpaclk = 0x02,
        #[doc = "FSRCO is clocking VDAC."]
        Fsrco = 0x03,
        #[doc = "HFRCOEM23 is clocking VDAC."]
        Hfrcoem23 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Vdac0clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vdac0clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vdac0clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Vdac0clkctrlClksel {
            Vdac0clkctrlClksel::from_bits(val)
        }
    }
    impl From<Vdac0clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Vdac0clkctrlClksel) -> u8 {
            Vdac0clkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vdac1clkctrlClksel {
        #[doc = "VDAC is not clocked."]
        Disabled = 0x0,
        #[doc = "EM01GRPACLK is clocking VDAC."]
        Em01grpaclk = 0x01,
        #[doc = "EM23GRPACLK is clocking VDAC."]
        Em23grpaclk = 0x02,
        #[doc = "FSRCO is clocking VDAC."]
        Fsrco = 0x03,
        #[doc = "HFRCOEM23 is clocking VDAC."]
        Hfrcoem23 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Vdac1clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vdac1clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vdac1clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Vdac1clkctrlClksel {
            Vdac1clkctrlClksel::from_bits(val)
        }
    }
    impl From<Vdac1clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Vdac1clkctrlClksel) -> u8 {
            Vdac1clkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wdog0clkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "LFRCO is clocking WDOG0CLK."]
        Lfrco = 0x01,
        #[doc = "LFXO is clocking WDOG0CLK."]
        Lfxo = 0x02,
        #[doc = "ULFRCO is clocking WDOG0CLK."]
        Ulfrco = 0x03,
        #[doc = "HCLKDIV1024 is clocking WDOG0CLK."]
        Hclkdiv1024 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Wdog0clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wdog0clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wdog0clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Wdog0clkctrlClksel {
            Wdog0clkctrlClksel::from_bits(val)
        }
    }
    impl From<Wdog0clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Wdog0clkctrlClksel) -> u8 {
            Wdog0clkctrlClksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wdog1clkctrlClksel {
        _RESERVED_0 = 0x0,
        #[doc = "LFRCO is clocking WDOG0CLK."]
        Lfrco = 0x01,
        #[doc = "LFXO is clocking WDOG0CLK."]
        Lfxo = 0x02,
        #[doc = "ULFRCO is clocking WDOG0CLK."]
        Ulfrco = 0x03,
        #[doc = "HCLKDIV1024 is clocking WDOG0CLK."]
        Hclkdiv1024 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Wdog1clkctrlClksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wdog1clkctrlClksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wdog1clkctrlClksel {
        #[inline(always)]
        fn from(val: u8) -> Wdog1clkctrlClksel {
            Wdog1clkctrlClksel::from_bits(val)
        }
    }
    impl From<Wdog1clkctrlClksel> for u8 {
        #[inline(always)]
        fn from(val: Wdog1clkctrlClksel) -> u8 {
            Wdog1clkctrlClksel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct WdoglockLockkey(u16);
    impl WdoglockLockkey {
        #[doc = "Write this value to unlock."]
        pub const Unlock: Self = Self(0x93f7);
    }
    impl WdoglockLockkey {
        pub const fn from_bits(val: u16) -> WdoglockLockkey {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for WdoglockLockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x93f7 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdoglockLockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x93f7 => defmt::write!(f, "Unlock"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for WdoglockLockkey {
        #[inline(always)]
        fn from(val: u16) -> WdoglockLockkey {
            WdoglockLockkey::from_bits(val)
        }
    }
    impl From<WdoglockLockkey> for u16 {
        #[inline(always)]
        fn from(val: WdoglockLockkey) -> u16 {
            WdoglockLockkey::to_bits(val)
        }
    }
}
