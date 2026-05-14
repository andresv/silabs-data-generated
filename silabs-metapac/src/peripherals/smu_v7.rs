#[doc = "SMU peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smu {
    ptr: *mut u8,
}
unsafe impl Send for Smu {}
unsafe impl Sync for Smu {}
impl Smu {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Holds the M33 control settings."]
    #[inline(always)]
    pub const fn m33ctrl(self) -> crate::common::Reg<regs::M33ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppupatd0(self) -> crate::common::Reg<regs::Ppupatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppupatd1(self) -> crate::common::Reg<regs::Ppupatd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppupatd2(self) -> crate::common::Reg<regs::Ppupatd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only."]
    #[inline(always)]
    pub const fn ppusatd0(self) -> crate::common::Reg<regs::Ppusatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only."]
    #[inline(always)]
    pub const fn ppusatd1(self) -> crate::common::Reg<regs::Ppusatd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only."]
    #[inline(always)]
    pub const fn ppusatd2(self) -> crate::common::Reg<regs::Ppusatd2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ppufs(self) -> crate::common::Reg<regs::Ppufs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a privileged master."]
    #[inline(always)]
    pub const fn bmpupatd0(self) -> crate::common::Reg<regs::Bmpupatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a secure master."]
    #[inline(always)]
    pub const fn bmpusatd0(self) -> crate::common::Reg<regs::Bmpusatd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn bmpufs(self) -> crate::common::Reg<regs::Bmpufs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn bmpufsaddr(self) -> crate::common::Reg<regs::Bmpufsaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn esaurtypes0(self) -> crate::common::Reg<regs::Esaurtypes0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn esaurtypes1(self) -> crate::common::Reg<regs::Esaurtypes1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn esaumrb01(self) -> crate::common::Reg<regs::Esaumrb01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn esaumrb12(self) -> crate::common::Reg<regs::Esaumrb12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn esaumrb45(self) -> crate::common::Reg<regs::Esaumrb45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn esaumrb56(self) -> crate::common::Reg<regs::Esaumrb56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Holds the M33 control settings. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn m33ctrl_set(self) -> crate::common::Reg<regs::M33ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ppupatd0_set(self) -> crate::common::Reg<regs::Ppupatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ppupatd1_set(self) -> crate::common::Reg<regs::Ppupatd1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ppupatd2_set(self) -> crate::common::Reg<regs::Ppupatd2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ppusatd0_set(self) -> crate::common::Reg<regs::Ppusatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ppusatd1_set(self) -> crate::common::Reg<regs::Ppusatd1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ppusatd2_set(self) -> crate::common::Reg<regs::Ppusatd2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a privileged master. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bmpupatd0_set(self) -> crate::common::Reg<regs::Bmpupatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a secure master. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bmpusatd0_set(self) -> crate::common::Reg<regs::Bmpusatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn esaurtypes0_set(self) -> crate::common::Reg<regs::Esaurtypes0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1260usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn esaurtypes1_set(self) -> crate::common::Reg<regs::Esaurtypes1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1264usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn esaumrb01_set(self) -> crate::common::Reg<regs::Esaumrb01, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1270usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn esaumrb12_set(self) -> crate::common::Reg<regs::Esaumrb12, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1274usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn esaumrb45_set(self) -> crate::common::Reg<regs::Esaumrb45, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1280usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn esaumrb56_set(self) -> crate::common::Reg<regs::Esaumrb56, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1284usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "Holds the M33 control settings. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn m33ctrl_clr(self) -> crate::common::Reg<regs::M33ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ppupatd0_clr(self) -> crate::common::Reg<regs::Ppupatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ppupatd1_clr(self) -> crate::common::Reg<regs::Ppupatd1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ppupatd2_clr(self) -> crate::common::Reg<regs::Ppupatd2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ppusatd0_clr(self) -> crate::common::Reg<regs::Ppusatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ppusatd1_clr(self) -> crate::common::Reg<regs::Ppusatd1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ppusatd2_clr(self) -> crate::common::Reg<regs::Ppusatd2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a privileged master. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bmpupatd0_clr(self) -> crate::common::Reg<regs::Bmpupatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2150usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a secure master. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bmpusatd0_clr(self) -> crate::common::Reg<regs::Bmpusatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn esaurtypes0_clr(self) -> crate::common::Reg<regs::Esaurtypes0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2260usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn esaurtypes1_clr(self) -> crate::common::Reg<regs::Esaurtypes1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2264usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn esaumrb01_clr(self) -> crate::common::Reg<regs::Esaumrb01, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2270usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn esaumrb12_clr(self) -> crate::common::Reg<regs::Esaumrb12, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2274usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn esaumrb45_clr(self) -> crate::common::Reg<regs::Esaumrb45, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn esaumrb56_clr(self) -> crate::common::Reg<regs::Esaumrb56, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2284usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "Holds the M33 control settings. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn m33ctrl_tgl(self) -> crate::common::Reg<regs::M33ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ppupatd0_tgl(self) -> crate::common::Reg<regs::Ppupatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ppupatd1_tgl(self) -> crate::common::Reg<regs::Ppupatd1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ppupatd2_tgl(self) -> crate::common::Reg<regs::Ppupatd2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ppusatd0_tgl(self) -> crate::common::Reg<regs::Ppusatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ppusatd1_tgl(self) -> crate::common::Reg<regs::Ppusatd1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ppusatd2_tgl(self) -> crate::common::Reg<regs::Ppusatd2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a privileged master. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bmpupatd0_tgl(self) -> crate::common::Reg<regs::Bmpupatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3150usize) as _) }
    }
    #[doc = "Set master bits to 1 to mark as a secure master. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bmpusatd0_tgl(self) -> crate::common::Reg<regs::Bmpusatd0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3170usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn esaurtypes0_tgl(self) -> crate::common::Reg<regs::Esaurtypes0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3260usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn esaurtypes1_tgl(self) -> crate::common::Reg<regs::Esaurtypes1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3264usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn esaumrb01_tgl(self) -> crate::common::Reg<regs::Esaumrb01, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3270usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn esaumrb12_tgl(self) -> crate::common::Reg<regs::Esaumrb12, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3274usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn esaumrb45_tgl(self) -> crate::common::Reg<regs::Esaumrb45, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3280usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn esaumrb56_tgl(self) -> crate::common::Reg<regs::Esaumrb56, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3284usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmpufs(pub u32);
    impl Bmpufs {
        #[doc = "Bus Manager ID."]
        #[must_use]
        #[inline(always)]
        pub const fn bmpufsmasterid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bus Manager ID."]
        #[inline(always)]
        pub const fn set_bmpufsmasterid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Bmpufs {
        #[inline(always)]
        fn default() -> Bmpufs {
            Bmpufs(0)
        }
    }
    impl core::fmt::Debug for Bmpufs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmpufs")
                .field("bmpufsmasterid", &self.bmpufsmasterid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmpufs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bmpufs {{ bmpufsmasterid: {=u8:?} }}", self.bmpufsmasterid())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmpufsaddr(pub u32);
    impl Bmpufsaddr {
        #[doc = "Fault Address."]
        #[must_use]
        #[inline(always)]
        pub const fn bmpufsaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Fault Address."]
        #[inline(always)]
        pub const fn set_bmpufsaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bmpufsaddr {
        #[inline(always)]
        fn default() -> Bmpufsaddr {
            Bmpufsaddr(0)
        }
    }
    impl core::fmt::Debug for Bmpufsaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmpufsaddr")
                .field("bmpufsaddr", &self.bmpufsaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmpufsaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bmpufsaddr {{ bmpufsaddr: {=u32:?} }}", self.bmpufsaddr())
        }
    }
    #[doc = "Set master bits to 1 to mark as a privileged master."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmpupatd0(pub u32);
    impl Bmpupatd0 {
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
    impl Default for Bmpupatd0 {
        #[inline(always)]
        fn default() -> Bmpupatd0 {
            Bmpupatd0(0)
        }
    }
    impl core::fmt::Debug for Bmpupatd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmpupatd0")
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
    impl defmt::Format for Bmpupatd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bmpupatd0 {{ radioaes: {=bool:?}, radiosubsystem: {=bool:?}, ldma: {=bool:?}, rfeca0: {=bool:?}, rfeca1: {=bool:?}, seextdma: {=bool:?} }}",
                self.radioaes(),
                self.radiosubsystem(),
                self.ldma(),
                self.rfeca0(),
                self.rfeca1(),
                self.seextdma()
            )
        }
    }
    #[doc = "Set master bits to 1 to mark as a secure master."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmpusatd0(pub u32);
    impl Bmpusatd0 {
        #[doc = "RADIOAES Secure Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn radioaes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RADIOAES Secure Mode."]
        #[inline(always)]
        pub const fn set_radioaes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RADIOSUBSYSTEM Secure Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn radiosubsystem(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RADIOSUBSYSTEM Secure Mode."]
        #[inline(always)]
        pub const fn set_radiosubsystem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LDMA Secure Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ldma(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LDMA Secure Mode."]
        #[inline(always)]
        pub const fn set_ldma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RFECA0 Secure Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rfeca0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RFECA0 Secure Mode."]
        #[inline(always)]
        pub const fn set_rfeca0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RFECA1 Secure Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rfeca1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RFECA1 Secure Mode."]
        #[inline(always)]
        pub const fn set_rfeca1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SEEXTDMA Secure Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn seextdma(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SEEXTDMA Secure Mode."]
        #[inline(always)]
        pub const fn set_seextdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Bmpusatd0 {
        #[inline(always)]
        fn default() -> Bmpusatd0 {
            Bmpusatd0(0)
        }
    }
    impl core::fmt::Debug for Bmpusatd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmpusatd0")
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
    impl defmt::Format for Bmpusatd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bmpusatd0 {{ radioaes: {=bool:?}, radiosubsystem: {=bool:?}, ldma: {=bool:?}, rfeca0: {=bool:?}, rfeca1: {=bool:?}, seextdma: {=bool:?} }}",
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
    pub struct Esaumrb01(pub u32);
    impl Esaumrb01 {
        #[doc = "Moveable Region Boundary."]
        #[must_use]
        #[inline(always)]
        pub const fn esaumrb01(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Moveable Region Boundary."]
        #[inline(always)]
        pub const fn set_esaumrb01(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
    }
    impl Default for Esaumrb01 {
        #[inline(always)]
        fn default() -> Esaumrb01 {
            Esaumrb01(0)
        }
    }
    impl core::fmt::Debug for Esaumrb01 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esaumrb01")
                .field("esaumrb01", &self.esaumrb01())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esaumrb01 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Esaumrb01 {{ esaumrb01: {=u16:?} }}", self.esaumrb01())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esaumrb12(pub u32);
    impl Esaumrb12 {
        #[doc = "Moveable Region Boundary."]
        #[must_use]
        #[inline(always)]
        pub const fn esaumrb12(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Moveable Region Boundary."]
        #[inline(always)]
        pub const fn set_esaumrb12(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
    }
    impl Default for Esaumrb12 {
        #[inline(always)]
        fn default() -> Esaumrb12 {
            Esaumrb12(0)
        }
    }
    impl core::fmt::Debug for Esaumrb12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esaumrb12")
                .field("esaumrb12", &self.esaumrb12())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esaumrb12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Esaumrb12 {{ esaumrb12: {=u16:?} }}", self.esaumrb12())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esaumrb45(pub u32);
    impl Esaumrb45 {
        #[doc = "Moveable Region Boundary."]
        #[must_use]
        #[inline(always)]
        pub const fn esaumrb45(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Moveable Region Boundary."]
        #[inline(always)]
        pub const fn set_esaumrb45(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
    }
    impl Default for Esaumrb45 {
        #[inline(always)]
        fn default() -> Esaumrb45 {
            Esaumrb45(0)
        }
    }
    impl core::fmt::Debug for Esaumrb45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esaumrb45")
                .field("esaumrb45", &self.esaumrb45())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esaumrb45 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Esaumrb45 {{ esaumrb45: {=u16:?} }}", self.esaumrb45())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esaumrb56(pub u32);
    impl Esaumrb56 {
        #[doc = "Moveable Region Boundary."]
        #[must_use]
        #[inline(always)]
        pub const fn esaumrb56(&self) -> u16 {
            let val = (self.0 >> 12usize) & 0xffff;
            val as u16
        }
        #[doc = "Moveable Region Boundary."]
        #[inline(always)]
        pub const fn set_esaumrb56(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
        }
    }
    impl Default for Esaumrb56 {
        #[inline(always)]
        fn default() -> Esaumrb56 {
            Esaumrb56(0)
        }
    }
    impl core::fmt::Debug for Esaumrb56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esaumrb56")
                .field("esaumrb56", &self.esaumrb56())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esaumrb56 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Esaumrb56 {{ esaumrb56: {=u16:?} }}", self.esaumrb56())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esaurtypes0(pub u32);
    impl Esaurtypes0 {
        #[doc = "Region 3 Non-Secure."]
        #[must_use]
        #[inline(always)]
        pub const fn esaur3ns(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Region 3 Non-Secure."]
        #[inline(always)]
        pub const fn set_esaur3ns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Esaurtypes0 {
        #[inline(always)]
        fn default() -> Esaurtypes0 {
            Esaurtypes0(0)
        }
    }
    impl core::fmt::Debug for Esaurtypes0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esaurtypes0")
                .field("esaur3ns", &self.esaur3ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esaurtypes0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Esaurtypes0 {{ esaur3ns: {=bool:?} }}", self.esaur3ns())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esaurtypes1(pub u32);
    impl Esaurtypes1 {
        #[doc = "Region 11 Non-Secure."]
        #[must_use]
        #[inline(always)]
        pub const fn esaur11ns(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Region 11 Non-Secure."]
        #[inline(always)]
        pub const fn set_esaur11ns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Esaurtypes1 {
        #[inline(always)]
        fn default() -> Esaurtypes1 {
            Esaurtypes1(0)
        }
    }
    impl core::fmt::Debug for Esaurtypes1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esaurtypes1")
                .field("esaur11ns", &self.esaur11ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esaurtypes1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Esaurtypes1 {{ esaur11ns: {=bool:?} }}", self.esaur11ns())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "PPU Privilege Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppupriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PPU Privilege Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ppupriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PPU Instruction Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppuinst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PPU Instruction Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ppuinst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PPU Security Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ppusec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PPU Security Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ppusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BMPU Security Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bmpusec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BMPU Security Interrupt Enable."]
        #[inline(always)]
        pub const fn set_bmpusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("ppupriv", &self.ppupriv())
                .field("ppuinst", &self.ppuinst())
                .field("ppusec", &self.ppusec())
                .field("bmpusec", &self.bmpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ ppupriv: {=bool:?}, ppuinst: {=bool:?}, ppusec: {=bool:?}, bmpusec: {=bool:?} }}",
                self.ppupriv(),
                self.ppuinst(),
                self.ppusec(),
                self.bmpusec()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "PPU Privilege Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ppupriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PPU Privilege Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ppupriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PPU Instruction Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ppuinst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PPU Instruction Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ppuinst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PPU Security Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ppusec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PPU Security Interrupt Flag."]
        #[inline(always)]
        pub const fn set_ppusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BMPU Security Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bmpusec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BMPU Security Interrupt Flag."]
        #[inline(always)]
        pub const fn set_bmpusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("ppupriv", &self.ppupriv())
                .field("ppuinst", &self.ppuinst())
                .field("ppusec", &self.ppusec())
                .field("bmpusec", &self.bmpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ ppupriv: {=bool:?}, ppuinst: {=bool:?}, ppusec: {=bool:?}, bmpusec: {=bool:?} }}",
                self.ppupriv(),
                self.ppuinst(),
                self.ppusec(),
                self.bmpusec()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP Version."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP Version."]
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
        #[doc = "No Description."]
        #[must_use]
        #[inline(always)]
        pub const fn smulockkey(&self) -> super::vals::Smulockkey {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            super::vals::Smulockkey::from_bits(val as u32)
        }
        #[doc = "No Description."]
        #[inline(always)]
        pub const fn set_smulockkey(&mut self, val: super::vals::Smulockkey) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val.to_bits() as u32) & 0x00ff_ffff) << 0usize);
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
            f.debug_struct("Lock").field("smulockkey", &self.smulockkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lock {{ smulockkey: {:?} }}", self.smulockkey())
        }
    }
    #[doc = "Holds the M33 control settings."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M33ctrl(pub u32);
    impl M33ctrl {
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "New BitField."]
        #[inline(always)]
        pub const fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for M33ctrl {
        #[inline(always)]
        fn default() -> M33ctrl {
            M33ctrl(0)
        }
    }
    impl core::fmt::Debug for M33ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M33ctrl")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locknsvtor", &self.locknsvtor())
                .field("locksmpu", &self.locksmpu())
                .field("locknsmpu", &self.locknsmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M33ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M33ctrl {{ locksvtaircr: {=bool:?}, locknsvtor: {=bool:?}, locksmpu: {=bool:?}, locknsmpu: {=bool:?}, locksau: {=bool:?} }}",
                self.locksvtaircr(),
                self.locknsvtor(),
                self.locksmpu(),
                self.locknsmpu(),
                self.locksau()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppufs(pub u32);
    impl Ppufs {
        #[doc = "Peripheral ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ppufsperiphid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Peripheral ID."]
        #[inline(always)]
        pub const fn set_ppufsperiphid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ppufs {
        #[inline(always)]
        fn default() -> Ppufs {
            Ppufs(0)
        }
    }
    impl core::fmt::Debug for Ppufs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppufs")
                .field("ppufsperiphid", &self.ppufsperiphid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppufs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ppufs {{ ppufsperiphid: {=u8:?} }}", self.ppufsperiphid())
        }
    }
    #[doc = "Set peripheral bits to 1 to mark as privileged access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppupatd0(pub u32);
    impl Ppupatd0 {
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
    impl Default for Ppupatd0 {
        #[inline(always)]
        fn default() -> Ppupatd0 {
            Ppupatd0(0)
        }
    }
    impl core::fmt::Debug for Ppupatd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppupatd0")
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
    impl defmt::Format for Ppupatd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppupatd0 {{ emu: {=bool:?}, cmu: {=bool:?}, burtc: {=bool:?}, hfrco0: {=bool:?}, fsrco: {=bool:?}, dpll0: {=bool:?}, lfxo: {=bool:?}, lfrco: {=bool:?}, ulfrco: {=bool:?}, keyscan: {=bool:?}, msc: {=bool:?}, icache0: {=bool:?}, prs: {=bool:?}, gpio: {=bool:?}, ldma: {=bool:?}, ldmaxbar: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, timer2: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, timer5: {=bool:?}, timer6: {=bool:?}, timer7: {=bool:?}, timer8: {=bool:?}, timer9: {=bool:?}, dmem0: {=bool:?}, dmem1: {=bool:?}, syscfgcfgns: {=bool:?}, syscfg: {=bool:?} }}",
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
    pub struct Ppupatd1(pub u32);
    impl Ppupatd1 {
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
    impl Default for Ppupatd1 {
        #[inline(always)]
        fn default() -> Ppupatd1 {
            Ppupatd1(0)
        }
    }
    impl core::fmt::Debug for Ppupatd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppupatd1")
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
    impl defmt::Format for Ppupatd1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppupatd1 {{ buram: {=bool:?}, gpcrc: {=bool:?}, eusart1: {=bool:?}, eusart2: {=bool:?}, eusart3: {=bool:?}, dcdc: {=bool:?}, hostmailbox: {=bool:?}, usart0: {=bool:?}, usart1: {=bool:?}, usart2: {=bool:?}, sysrtc: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, i2c3: {=bool:?}, lcd: {=bool:?}, lcdrf: {=bool:?}, radioaes: {=bool:?}, smu: {=bool:?}, smucfgns: {=bool:?}, letimer0: {=bool:?}, iadc0: {=bool:?}, acmp0: {=bool:?}, acmp1: {=bool:?}, amuxcp0: {=bool:?}, vdac0: {=bool:?}, vdac1: {=bool:?}, pcnt: {=bool:?}, hfrco1: {=bool:?}, hfxo0: {=bool:?}, i2c0: {=bool:?}, wdog0: {=bool:?}, wdog1: {=bool:?} }}",
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
    pub struct Ppupatd2(pub u32);
    impl Ppupatd2 {
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
    impl Default for Ppupatd2 {
        #[inline(always)]
        fn default() -> Ppupatd2 {
            Ppupatd2(0)
        }
    }
    impl core::fmt::Debug for Ppupatd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppupatd2")
                .field("eusart0", &self.eusart0())
                .field("semailbox", &self.semailbox())
                .field("ahbradio", &self.ahbradio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppupatd2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppupatd2 {{ eusart0: {=bool:?}, semailbox: {=bool:?}, ahbradio: {=bool:?} }}",
                self.eusart0(),
                self.semailbox(),
                self.ahbradio()
            )
        }
    }
    #[doc = "Set peripheral bits to 1 to mark as secure access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppusatd0(pub u32);
    impl Ppusatd0 {
        #[doc = "EMU Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn emu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EMU Secure Access."]
        #[inline(always)]
        pub const fn set_emu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CMU Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn cmu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CMU Secure Access."]
        #[inline(always)]
        pub const fn set_cmu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BURTC Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn burtc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BURTC Secure Access."]
        #[inline(always)]
        pub const fn set_burtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HFRCO0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hfrco0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HFRCO0 Secure Access."]
        #[inline(always)]
        pub const fn set_hfrco0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FSRCO Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn fsrco(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FSRCO Secure Access."]
        #[inline(always)]
        pub const fn set_fsrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DPLL0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dpll0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DPLL0 Secure Access."]
        #[inline(always)]
        pub const fn set_dpll0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LFXO Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lfxo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LFXO Secure Access."]
        #[inline(always)]
        pub const fn set_lfxo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LFRCO Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lfrco(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LFRCO Secure Access."]
        #[inline(always)]
        pub const fn set_lfrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ULFRCO Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ulfrco(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ULFRCO Secure Access."]
        #[inline(always)]
        pub const fn set_ulfrco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "KEYSCAN Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn keyscan(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "KEYSCAN Secure Access."]
        #[inline(always)]
        pub const fn set_keyscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSC Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn msc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSC Secure Access."]
        #[inline(always)]
        pub const fn set_msc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ICACHE0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn icache0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE0 Secure Access."]
        #[inline(always)]
        pub const fn set_icache0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PRS Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PRS Secure Access."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIO Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO Secure Access."]
        #[inline(always)]
        pub const fn set_gpio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LDMA Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ldma(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "LDMA Secure Access."]
        #[inline(always)]
        pub const fn set_ldma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "LDMAXBAR Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ldmaxbar(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LDMAXBAR Secure Access."]
        #[inline(always)]
        pub const fn set_ldmaxbar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIMER0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER0 Secure Access."]
        #[inline(always)]
        pub const fn set_timer0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIMER1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER1 Secure Access."]
        #[inline(always)]
        pub const fn set_timer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIMER2 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER2 Secure Access."]
        #[inline(always)]
        pub const fn set_timer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIMER3 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer3(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER3 Secure Access."]
        #[inline(always)]
        pub const fn set_timer3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIMER4 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer4(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER4 Secure Access."]
        #[inline(always)]
        pub const fn set_timer4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TIMER5 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer5(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER5 Secure Access."]
        #[inline(always)]
        pub const fn set_timer5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "TIMER6 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer6(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER6 Secure Access."]
        #[inline(always)]
        pub const fn set_timer6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "TIMER7 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer7(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER7 Secure Access."]
        #[inline(always)]
        pub const fn set_timer7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIMER8 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer8(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER8 Secure Access."]
        #[inline(always)]
        pub const fn set_timer8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "TIMER9 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn timer9(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER9 Secure Access."]
        #[inline(always)]
        pub const fn set_timer9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DMEM0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dmem0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DMEM0 Secure Access."]
        #[inline(always)]
        pub const fn set_dmem0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DMEM1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dmem1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DMEM1 Secure Access."]
        #[inline(always)]
        pub const fn set_dmem1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SYSCFGCFGNS Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgcfgns(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFGCFGNS Secure Access."]
        #[inline(always)]
        pub const fn set_syscfgcfgns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SYSCFG Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG Secure Access."]
        #[inline(always)]
        pub const fn set_syscfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ppusatd0 {
        #[inline(always)]
        fn default() -> Ppusatd0 {
            Ppusatd0(0)
        }
    }
    impl core::fmt::Debug for Ppusatd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppusatd0")
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
    impl defmt::Format for Ppusatd0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppusatd0 {{ emu: {=bool:?}, cmu: {=bool:?}, burtc: {=bool:?}, hfrco0: {=bool:?}, fsrco: {=bool:?}, dpll0: {=bool:?}, lfxo: {=bool:?}, lfrco: {=bool:?}, ulfrco: {=bool:?}, keyscan: {=bool:?}, msc: {=bool:?}, icache0: {=bool:?}, prs: {=bool:?}, gpio: {=bool:?}, ldma: {=bool:?}, ldmaxbar: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, timer2: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, timer5: {=bool:?}, timer6: {=bool:?}, timer7: {=bool:?}, timer8: {=bool:?}, timer9: {=bool:?}, dmem0: {=bool:?}, dmem1: {=bool:?}, syscfgcfgns: {=bool:?}, syscfg: {=bool:?} }}",
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
    #[doc = "Set peripheral bits to 1 to mark as secure access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppusatd1(pub u32);
    impl Ppusatd1 {
        #[doc = "BURAM Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn buram(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BURAM Secure Access."]
        #[inline(always)]
        pub const fn set_buram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPCRC Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn gpcrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPCRC Secure Access."]
        #[inline(always)]
        pub const fn set_gpcrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EUSART1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART1 Secure Access."]
        #[inline(always)]
        pub const fn set_eusart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EUSART2 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART2 Secure Access."]
        #[inline(always)]
        pub const fn set_eusart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EUSART3 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart3(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART3 Secure Access."]
        #[inline(always)]
        pub const fn set_eusart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DCDC Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC Secure Access."]
        #[inline(always)]
        pub const fn set_dcdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HOSTMAILBOX Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hostmailbox(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HOSTMAILBOX Secure Access."]
        #[inline(always)]
        pub const fn set_hostmailbox(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn usart0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART0 Secure Access."]
        #[inline(always)]
        pub const fn set_usart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USART1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Secure Access."]
        #[inline(always)]
        pub const fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USART2 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Secure Access."]
        #[inline(always)]
        pub const fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SYSRTC Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn sysrtc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SYSRTC Secure Access."]
        #[inline(always)]
        pub const fn set_sysrtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I2C1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Secure Access."]
        #[inline(always)]
        pub const fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "I2C2 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Secure Access."]
        #[inline(always)]
        pub const fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C3 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Secure Access."]
        #[inline(always)]
        pub const fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "LCD Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lcd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LCD Secure Access."]
        #[inline(always)]
        pub const fn set_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LCDRF Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn lcdrf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "LCDRF Secure Access."]
        #[inline(always)]
        pub const fn set_lcdrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RADIOAES Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn radioaes(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RADIOAES Secure Access."]
        #[inline(always)]
        pub const fn set_radioaes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SMU Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn smu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SMU Secure Access."]
        #[inline(always)]
        pub const fn set_smu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SMUCFGNS Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn smucfgns(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "SMUCFGNS Secure Access."]
        #[inline(always)]
        pub const fn set_smucfgns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LETIMER0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn letimer0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LETIMER0 Secure Access."]
        #[inline(always)]
        pub const fn set_letimer0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "IADC0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn iadc0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "IADC0 Secure Access."]
        #[inline(always)]
        pub const fn set_iadc0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ACMP0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp0(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ACMP0 Secure Access."]
        #[inline(always)]
        pub const fn set_acmp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ACMP1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn acmp1(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ACMP1 Secure Access."]
        #[inline(always)]
        pub const fn set_acmp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "AMUXCP0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn amuxcp0(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "AMUXCP0 Secure Access."]
        #[inline(always)]
        pub const fn set_amuxcp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "VDAC0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn vdac0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "VDAC0 Secure Access."]
        #[inline(always)]
        pub const fn set_vdac0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VDAC1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn vdac1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VDAC1 Secure Access."]
        #[inline(always)]
        pub const fn set_vdac1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCNT Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn pcnt(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCNT Secure Access."]
        #[inline(always)]
        pub const fn set_pcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "HFRCO1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hfrco1(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HFRCO1 Secure Access."]
        #[inline(always)]
        pub const fn set_hfrco1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "HFXO0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn hfxo0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "HFXO0 Secure Access."]
        #[inline(always)]
        pub const fn set_hfxo0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "I2C0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c0(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "I2C0 Secure Access."]
        #[inline(always)]
        pub const fn set_i2c0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "WDOG0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog0(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG0 Secure Access."]
        #[inline(always)]
        pub const fn set_wdog0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "WDOG1 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "WDOG1 Secure Access."]
        #[inline(always)]
        pub const fn set_wdog1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ppusatd1 {
        #[inline(always)]
        fn default() -> Ppusatd1 {
            Ppusatd1(0)
        }
    }
    impl core::fmt::Debug for Ppusatd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppusatd1")
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
    impl defmt::Format for Ppusatd1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppusatd1 {{ buram: {=bool:?}, gpcrc: {=bool:?}, eusart1: {=bool:?}, eusart2: {=bool:?}, eusart3: {=bool:?}, dcdc: {=bool:?}, hostmailbox: {=bool:?}, usart0: {=bool:?}, usart1: {=bool:?}, usart2: {=bool:?}, sysrtc: {=bool:?}, i2c1: {=bool:?}, i2c2: {=bool:?}, i2c3: {=bool:?}, lcd: {=bool:?}, lcdrf: {=bool:?}, radioaes: {=bool:?}, smu: {=bool:?}, smucfgns: {=bool:?}, letimer0: {=bool:?}, iadc0: {=bool:?}, acmp0: {=bool:?}, acmp1: {=bool:?}, amuxcp0: {=bool:?}, vdac0: {=bool:?}, vdac1: {=bool:?}, pcnt: {=bool:?}, hfrco1: {=bool:?}, hfxo0: {=bool:?}, i2c0: {=bool:?}, wdog0: {=bool:?}, wdog1: {=bool:?} }}",
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
    #[doc = "Set peripheral bits to 1 to mark as secure access only."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ppusatd2(pub u32);
    impl Ppusatd2 {
        #[doc = "EUSART0 Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn eusart0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EUSART0 Secure Access."]
        #[inline(always)]
        pub const fn set_eusart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SEMAILBOX Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn semailbox(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SEMAILBOX Secure Access."]
        #[inline(always)]
        pub const fn set_semailbox(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AHBRADIO Secure Access."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbradio(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AHBRADIO Secure Access."]
        #[inline(always)]
        pub const fn set_ahbradio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ppusatd2 {
        #[inline(always)]
        fn default() -> Ppusatd2 {
            Ppusatd2(0)
        }
    }
    impl core::fmt::Debug for Ppusatd2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ppusatd2")
                .field("eusart0", &self.eusart0())
                .field("semailbox", &self.semailbox())
                .field("ahbradio", &self.ahbradio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ppusatd2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ppusatd2 {{ eusart0: {=bool:?}, semailbox: {=bool:?}, ahbradio: {=bool:?} }}",
                self.eusart0(),
                self.semailbox(),
                self.ahbradio()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "SMU Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn smulock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SMU Lock."]
        #[inline(always)]
        pub const fn set_smulock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SMU Programming Error."]
        #[must_use]
        #[inline(always)]
        pub const fn smuprgerr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SMU Programming Error."]
        #[inline(always)]
        pub const fn set_smuprgerr(&mut self, val: bool) {
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
                .field("smulock", &self.smulock())
                .field("smuprgerr", &self.smuprgerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ smulock: {=bool:?}, smuprgerr: {=bool:?} }}",
                self.smulock(),
                self.smuprgerr()
            )
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smulockkey(u32);
    impl Smulockkey {
        #[doc = "Unlocks Registers."]
        pub const Unlock: Self = Self(0x00ac_ce55);
    }
    impl Smulockkey {
        pub const fn from_bits(val: u32) -> Smulockkey {
            Self(val & 0x00ff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Smulockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x00ac_ce55 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smulockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x00ac_ce55 => defmt::write!(f, "Unlock"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Smulockkey {
        #[inline(always)]
        fn from(val: u32) -> Smulockkey {
            Smulockkey::from_bits(val)
        }
    }
    impl From<Smulockkey> for u32 {
        #[inline(always)]
        fn from(val: Smulockkey) -> u32 {
            Smulockkey::to_bits(val)
        }
    }
}
