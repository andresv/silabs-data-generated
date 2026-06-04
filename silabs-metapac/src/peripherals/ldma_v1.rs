#[doc = "LDMA peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldma {
    ptr: *mut u8,
}
unsafe impl Send for Ldma {}
unsafe impl Sync for Ldma {}
impl Ldma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IP version register."]
    #[inline(always)]
    pub const fn ipversion(self) -> crate::common::Reg<regs::Ipversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Module enable disable Register."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Software Reset Register."]
    #[inline(always)]
    pub const fn swrst(self) -> crate::common::Reg<regs::Swrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Sync Trig Sw Set Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn syncswset(self) -> crate::common::Reg<regs::Syncswset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Sync Trig Sw Clear register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn syncswclr(self) -> crate::common::Reg<regs::Syncswclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Sync HW trigger enable register."]
    #[inline(always)]
    pub const fn synchwen(self) -> crate::common::Reg<regs::Synchwen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Sync HW trigger selection register."]
    #[inline(always)]
    pub const fn synchwsel(self) -> crate::common::Reg<regs::Synchwsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Sync Trigger Status Register."]
    #[inline(always)]
    pub const fn syncstatus(self) -> crate::common::Reg<regs::Syncstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel Enable Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel Disable Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn chdis(self) -> crate::common::Reg<regs::Chdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel Status Register."]
    #[inline(always)]
    pub const fn chstatus(self) -> crate::common::Reg<regs::Chstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Busy Register."]
    #[inline(always)]
    pub const fn chbusy(self) -> crate::common::Reg<regs::Chbusy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn chdone(self) -> crate::common::Reg<regs::Chdone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Channel Debug Halt Register."]
    #[inline(always)]
    pub const fn dbghalt(self) -> crate::common::Reg<regs::Dbghalt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn swreq(self) -> crate::common::Reg<regs::Swreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Channel Request Disable Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn reqdis(self) -> crate::common::Reg<regs::Reqdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Channel Requests Pending Register."]
    #[inline(always)]
    pub const fn reqpend(self) -> crate::common::Reg<regs::Reqpend, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Channel Link Load Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn linkload(self) -> crate::common::Reg<regs::Linkload, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Channel Request Clear Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn reqclear(self) -> crate::common::Reg<regs::Reqclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt Flag Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Done Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch0_cfg(self) -> crate::common::Reg<regs::Ch0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_loop(self) -> crate::common::Reg<regs::Ch0Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_ctrl(self) -> crate::common::Reg<regs::Ch0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_src(self) -> crate::common::Reg<regs::Ch0Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_dst(self) -> crate::common::Reg<regs::Ch0Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_link(self) -> crate::common::Reg<regs::Ch0Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_xctrl(self) -> crate::common::Reg<regs::Ch0Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch0_ilsrc(self) -> crate::common::Reg<regs::Ch0Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch1_cfg(self) -> crate::common::Reg<regs::Ch1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_loop(self) -> crate::common::Reg<regs::Ch1Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_ctrl(self) -> crate::common::Reg<regs::Ch1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_src(self) -> crate::common::Reg<regs::Ch1Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_dst(self) -> crate::common::Reg<regs::Ch1Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_link(self) -> crate::common::Reg<regs::Ch1Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_xctrl(self) -> crate::common::Reg<regs::Ch1Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch1_ilsrc(self) -> crate::common::Reg<regs::Ch1Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch2_cfg(self) -> crate::common::Reg<regs::Ch2Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_loop(self) -> crate::common::Reg<regs::Ch2Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_ctrl(self) -> crate::common::Reg<regs::Ch2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_src(self) -> crate::common::Reg<regs::Ch2Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_dst(self) -> crate::common::Reg<regs::Ch2Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_link(self) -> crate::common::Reg<regs::Ch2Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_xctrl(self) -> crate::common::Reg<regs::Ch2Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch2_ilsrc(self) -> crate::common::Reg<regs::Ch2Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch3_cfg(self) -> crate::common::Reg<regs::Ch3Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_loop(self) -> crate::common::Reg<regs::Ch3Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_ctrl(self) -> crate::common::Reg<regs::Ch3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_src(self) -> crate::common::Reg<regs::Ch3Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_dst(self) -> crate::common::Reg<regs::Ch3Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_link(self) -> crate::common::Reg<regs::Ch3Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_xctrl(self) -> crate::common::Reg<regs::Ch3Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch3_ilsrc(self) -> crate::common::Reg<regs::Ch3Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch4_cfg(self) -> crate::common::Reg<regs::Ch4Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_loop(self) -> crate::common::Reg<regs::Ch4Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_ctrl(self) -> crate::common::Reg<regs::Ch4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_src(self) -> crate::common::Reg<regs::Ch4Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_dst(self) -> crate::common::Reg<regs::Ch4Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_link(self) -> crate::common::Reg<regs::Ch4Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_xctrl(self) -> crate::common::Reg<regs::Ch4Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch4_ilsrc(self) -> crate::common::Reg<regs::Ch4Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch5_cfg(self) -> crate::common::Reg<regs::Ch5Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_loop(self) -> crate::common::Reg<regs::Ch5Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_ctrl(self) -> crate::common::Reg<regs::Ch5Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_src(self) -> crate::common::Reg<regs::Ch5Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_dst(self) -> crate::common::Reg<regs::Ch5Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_link(self) -> crate::common::Reg<regs::Ch5Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_xctrl(self) -> crate::common::Reg<regs::Ch5Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch5_ilsrc(self) -> crate::common::Reg<regs::Ch5Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch6_cfg(self) -> crate::common::Reg<regs::Ch6Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_loop(self) -> crate::common::Reg<regs::Ch6Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_ctrl(self) -> crate::common::Reg<regs::Ch6Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_src(self) -> crate::common::Reg<regs::Ch6Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_dst(self) -> crate::common::Reg<regs::Ch6Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_link(self) -> crate::common::Reg<regs::Ch6Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_xctrl(self) -> crate::common::Reg<regs::Ch6Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch6_ilsrc(self) -> crate::common::Reg<regs::Ch6Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch7_cfg(self) -> crate::common::Reg<regs::Ch7Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_loop(self) -> crate::common::Reg<regs::Ch7Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_ctrl(self) -> crate::common::Reg<regs::Ch7Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_src(self) -> crate::common::Reg<regs::Ch7Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_dst(self) -> crate::common::Reg<regs::Ch7Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_link(self) -> crate::common::Reg<regs::Ch7Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_xctrl(self) -> crate::common::Reg<regs::Ch7Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch7_ilsrc(self) -> crate::common::Reg<regs::Ch7Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch8_cfg(self) -> crate::common::Reg<regs::Ch8Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_loop(self) -> crate::common::Reg<regs::Ch8Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_ctrl(self) -> crate::common::Reg<regs::Ch8Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_src(self) -> crate::common::Reg<regs::Ch8Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_dst(self) -> crate::common::Reg<regs::Ch8Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_link(self) -> crate::common::Reg<regs::Ch8Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_xctrl(self) -> crate::common::Reg<regs::Ch8Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch8_ilsrc(self) -> crate::common::Reg<regs::Ch8Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch9_cfg(self) -> crate::common::Reg<regs::Ch9Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_loop(self) -> crate::common::Reg<regs::Ch9Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_ctrl(self) -> crate::common::Reg<regs::Ch9Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_src(self) -> crate::common::Reg<regs::Ch9Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_dst(self) -> crate::common::Reg<regs::Ch9Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_link(self) -> crate::common::Reg<regs::Ch9Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_xctrl(self) -> crate::common::Reg<regs::Ch9Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch9_ilsrc(self) -> crate::common::Reg<regs::Ch9Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch10_cfg(self) -> crate::common::Reg<regs::Ch10Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_loop(self) -> crate::common::Reg<regs::Ch10Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_ctrl(self) -> crate::common::Reg<regs::Ch10Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_src(self) -> crate::common::Reg<regs::Ch10Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_dst(self) -> crate::common::Reg<regs::Ch10Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_link(self) -> crate::common::Reg<regs::Ch10Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_xctrl(self) -> crate::common::Reg<regs::Ch10Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch10_ilsrc(self) -> crate::common::Reg<regs::Ch10Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch11_cfg(self) -> crate::common::Reg<regs::Ch11Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_loop(self) -> crate::common::Reg<regs::Ch11Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_ctrl(self) -> crate::common::Reg<regs::Ch11Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_src(self) -> crate::common::Reg<regs::Ch11Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_dst(self) -> crate::common::Reg<regs::Ch11Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_link(self) -> crate::common::Reg<regs::Ch11Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_xctrl(self) -> crate::common::Reg<regs::Ch11Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch11_ilsrc(self) -> crate::common::Reg<regs::Ch11Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch12_cfg(self) -> crate::common::Reg<regs::Ch12Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_loop(self) -> crate::common::Reg<regs::Ch12Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_ctrl(self) -> crate::common::Reg<regs::Ch12Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_src(self) -> crate::common::Reg<regs::Ch12Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_dst(self) -> crate::common::Reg<regs::Ch12Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_link(self) -> crate::common::Reg<regs::Ch12Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_xctrl(self) -> crate::common::Reg<regs::Ch12Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch12_ilsrc(self) -> crate::common::Reg<regs::Ch12Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch13_cfg(self) -> crate::common::Reg<regs::Ch13Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_loop(self) -> crate::common::Reg<regs::Ch13Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_ctrl(self) -> crate::common::Reg<regs::Ch13Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_src(self) -> crate::common::Reg<regs::Ch13Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_dst(self) -> crate::common::Reg<regs::Ch13Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_link(self) -> crate::common::Reg<regs::Ch13Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_xctrl(self) -> crate::common::Reg<regs::Ch13Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch13_ilsrc(self) -> crate::common::Reg<regs::Ch13Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch14_cfg(self) -> crate::common::Reg<regs::Ch14Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_loop(self) -> crate::common::Reg<regs::Ch14Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_ctrl(self) -> crate::common::Reg<regs::Ch14Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_src(self) -> crate::common::Reg<regs::Ch14Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_dst(self) -> crate::common::Reg<regs::Ch14Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_link(self) -> crate::common::Reg<regs::Ch14Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_xctrl(self) -> crate::common::Reg<regs::Ch14Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch14_ilsrc(self) -> crate::common::Reg<regs::Ch14Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch15_cfg(self) -> crate::common::Reg<regs::Ch15Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_loop(self) -> crate::common::Reg<regs::Ch15Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_ctrl(self) -> crate::common::Reg<regs::Ch15Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_src(self) -> crate::common::Reg<regs::Ch15Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_dst(self) -> crate::common::Reg<regs::Ch15Dst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_link(self) -> crate::common::Reg<regs::Ch15Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_xctrl(self) -> crate::common::Reg<regs::Ch15Xctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[inline(always)]
    pub const fn ch15_ilsrc(self) -> crate::common::Reg<regs::Ch15Ilsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Module enable disable Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Software Reset Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn swrst_set(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Control Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Sync Trig Sw Set Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn syncswset_set(self) -> crate::common::Reg<regs::Syncswset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "Sync Trig Sw Clear register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn syncswclr_set(self) -> crate::common::Reg<regs::Syncswclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "Sync HW trigger enable register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn synchwen_set(self) -> crate::common::Reg<regs::Synchwen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Sync HW trigger selection register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn synchwsel_set(self) -> crate::common::Reg<regs::Synchwsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Channel Enable Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chen_set(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "Channel Disable Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chdis_set(self) -> crate::common::Reg<regs::Chdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn chdone_set(self) -> crate::common::Reg<regs::Chdone, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "Channel Debug Halt Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dbghalt_set(self) -> crate::common::Reg<regs::Dbghalt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn swreq_set(self) -> crate::common::Reg<regs::Swreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "Channel Request Disable Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn reqdis_set(self) -> crate::common::Reg<regs::Reqdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "Channel Link Load Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn linkload_set(self) -> crate::common::Reg<regs::Linkload, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "Channel Request Clear Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn reqclear_set(self) -> crate::common::Reg<regs::Reqclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "Interrupt Flag Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "Done Interrupt Enable Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_cfg_set(self) -> crate::common::Reg<regs::Ch0Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_loop_set(self) -> crate::common::Reg<regs::Ch0Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_ctrl_set(self) -> crate::common::Reg<regs::Ch0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_src_set(self) -> crate::common::Reg<regs::Ch0Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_dst_set(self) -> crate::common::Reg<regs::Ch0Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_link_set(self) -> crate::common::Reg<regs::Ch0Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_xctrl_set(self) -> crate::common::Reg<regs::Ch0Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch0_ilsrc_set(self) -> crate::common::Reg<regs::Ch0Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_cfg_set(self) -> crate::common::Reg<regs::Ch1Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_loop_set(self) -> crate::common::Reg<regs::Ch1Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_ctrl_set(self) -> crate::common::Reg<regs::Ch1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_src_set(self) -> crate::common::Reg<regs::Ch1Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_dst_set(self) -> crate::common::Reg<regs::Ch1Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_link_set(self) -> crate::common::Reg<regs::Ch1Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_xctrl_set(self) -> crate::common::Reg<regs::Ch1Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch1_ilsrc_set(self) -> crate::common::Reg<regs::Ch1Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_cfg_set(self) -> crate::common::Reg<regs::Ch2Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_loop_set(self) -> crate::common::Reg<regs::Ch2Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_ctrl_set(self) -> crate::common::Reg<regs::Ch2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_src_set(self) -> crate::common::Reg<regs::Ch2Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_dst_set(self) -> crate::common::Reg<regs::Ch2Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_link_set(self) -> crate::common::Reg<regs::Ch2Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_xctrl_set(self) -> crate::common::Reg<regs::Ch2Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch2_ilsrc_set(self) -> crate::common::Reg<regs::Ch2Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_cfg_set(self) -> crate::common::Reg<regs::Ch3Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_loop_set(self) -> crate::common::Reg<regs::Ch3Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_ctrl_set(self) -> crate::common::Reg<regs::Ch3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_src_set(self) -> crate::common::Reg<regs::Ch3Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10fcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_dst_set(self) -> crate::common::Reg<regs::Ch3Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_link_set(self) -> crate::common::Reg<regs::Ch3Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_xctrl_set(self) -> crate::common::Reg<regs::Ch3Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch3_ilsrc_set(self) -> crate::common::Reg<regs::Ch3Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1110usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_cfg_set(self) -> crate::common::Reg<regs::Ch4Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_loop_set(self) -> crate::common::Reg<regs::Ch4Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_ctrl_set(self) -> crate::common::Reg<regs::Ch4Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_src_set(self) -> crate::common::Reg<regs::Ch4Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_dst_set(self) -> crate::common::Reg<regs::Ch4Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_link_set(self) -> crate::common::Reg<regs::Ch4Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_xctrl_set(self) -> crate::common::Reg<regs::Ch4Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch4_ilsrc_set(self) -> crate::common::Reg<regs::Ch4Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_cfg_set(self) -> crate::common::Reg<regs::Ch5Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_loop_set(self) -> crate::common::Reg<regs::Ch5Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_ctrl_set(self) -> crate::common::Reg<regs::Ch5Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1158usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_src_set(self) -> crate::common::Reg<regs::Ch5Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_dst_set(self) -> crate::common::Reg<regs::Ch5Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_link_set(self) -> crate::common::Reg<regs::Ch5Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_xctrl_set(self) -> crate::common::Reg<regs::Ch5Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch5_ilsrc_set(self) -> crate::common::Reg<regs::Ch5Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_cfg_set(self) -> crate::common::Reg<regs::Ch6Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_loop_set(self) -> crate::common::Reg<regs::Ch6Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_ctrl_set(self) -> crate::common::Reg<regs::Ch6Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_src_set(self) -> crate::common::Reg<regs::Ch6Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x118cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_dst_set(self) -> crate::common::Reg<regs::Ch6Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1190usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_link_set(self) -> crate::common::Reg<regs::Ch6Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_xctrl_set(self) -> crate::common::Reg<regs::Ch6Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1198usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch6_ilsrc_set(self) -> crate::common::Reg<regs::Ch6Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_cfg_set(self) -> crate::common::Reg<regs::Ch7Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_loop_set(self) -> crate::common::Reg<regs::Ch7Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_ctrl_set(self) -> crate::common::Reg<regs::Ch7Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_src_set(self) -> crate::common::Reg<regs::Ch7Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_dst_set(self) -> crate::common::Reg<regs::Ch7Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_link_set(self) -> crate::common::Reg<regs::Ch7Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_xctrl_set(self) -> crate::common::Reg<regs::Ch7Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch7_ilsrc_set(self) -> crate::common::Reg<regs::Ch7Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_cfg_set(self) -> crate::common::Reg<regs::Ch8Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_loop_set(self) -> crate::common::Reg<regs::Ch8Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_ctrl_set(self) -> crate::common::Reg<regs::Ch8Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_src_set(self) -> crate::common::Reg<regs::Ch8Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_dst_set(self) -> crate::common::Reg<regs::Ch8Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_link_set(self) -> crate::common::Reg<regs::Ch8Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_xctrl_set(self) -> crate::common::Reg<regs::Ch8Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch8_ilsrc_set(self) -> crate::common::Reg<regs::Ch8Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_cfg_set(self) -> crate::common::Reg<regs::Ch9Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1210usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_loop_set(self) -> crate::common::Reg<regs::Ch9Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1214usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_ctrl_set(self) -> crate::common::Reg<regs::Ch9Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1218usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_src_set(self) -> crate::common::Reg<regs::Ch9Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x121cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_dst_set(self) -> crate::common::Reg<regs::Ch9Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1220usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_link_set(self) -> crate::common::Reg<regs::Ch9Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1224usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_xctrl_set(self) -> crate::common::Reg<regs::Ch9Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1228usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch9_ilsrc_set(self) -> crate::common::Reg<regs::Ch9Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1230usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_cfg_set(self) -> crate::common::Reg<regs::Ch10Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1240usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_loop_set(self) -> crate::common::Reg<regs::Ch10Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1244usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_ctrl_set(self) -> crate::common::Reg<regs::Ch10Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1248usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_src_set(self) -> crate::common::Reg<regs::Ch10Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x124cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_dst_set(self) -> crate::common::Reg<regs::Ch10Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1250usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_link_set(self) -> crate::common::Reg<regs::Ch10Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1254usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_xctrl_set(self) -> crate::common::Reg<regs::Ch10Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1258usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch10_ilsrc_set(self) -> crate::common::Reg<regs::Ch10Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1260usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_cfg_set(self) -> crate::common::Reg<regs::Ch11Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1270usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_loop_set(self) -> crate::common::Reg<regs::Ch11Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1274usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_ctrl_set(self) -> crate::common::Reg<regs::Ch11Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1278usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_src_set(self) -> crate::common::Reg<regs::Ch11Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x127cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_dst_set(self) -> crate::common::Reg<regs::Ch11Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1280usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_link_set(self) -> crate::common::Reg<regs::Ch11Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1284usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_xctrl_set(self) -> crate::common::Reg<regs::Ch11Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1288usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch11_ilsrc_set(self) -> crate::common::Reg<regs::Ch11Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1290usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_cfg_set(self) -> crate::common::Reg<regs::Ch12Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_loop_set(self) -> crate::common::Reg<regs::Ch12Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_ctrl_set(self) -> crate::common::Reg<regs::Ch12Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_src_set(self) -> crate::common::Reg<regs::Ch12Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12acusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_dst_set(self) -> crate::common::Reg<regs::Ch12Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_link_set(self) -> crate::common::Reg<regs::Ch12Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_xctrl_set(self) -> crate::common::Reg<regs::Ch12Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch12_ilsrc_set(self) -> crate::common::Reg<regs::Ch12Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_cfg_set(self) -> crate::common::Reg<regs::Ch13Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_loop_set(self) -> crate::common::Reg<regs::Ch13Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_ctrl_set(self) -> crate::common::Reg<regs::Ch13Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_src_set(self) -> crate::common::Reg<regs::Ch13Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12dcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_dst_set(self) -> crate::common::Reg<regs::Ch13Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_link_set(self) -> crate::common::Reg<regs::Ch13Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_xctrl_set(self) -> crate::common::Reg<regs::Ch13Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch13_ilsrc_set(self) -> crate::common::Reg<regs::Ch13Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12f0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_cfg_set(self) -> crate::common::Reg<regs::Ch14Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1300usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_loop_set(self) -> crate::common::Reg<regs::Ch14Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1304usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_ctrl_set(self) -> crate::common::Reg<regs::Ch14Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1308usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_src_set(self) -> crate::common::Reg<regs::Ch14Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x130cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_dst_set(self) -> crate::common::Reg<regs::Ch14Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1310usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_link_set(self) -> crate::common::Reg<regs::Ch14Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1314usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_xctrl_set(self) -> crate::common::Reg<regs::Ch14Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1318usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch14_ilsrc_set(self) -> crate::common::Reg<regs::Ch14Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1320usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_cfg_set(self) -> crate::common::Reg<regs::Ch15Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1330usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_loop_set(self) -> crate::common::Reg<regs::Ch15Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1334usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_ctrl_set(self) -> crate::common::Reg<regs::Ch15Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1338usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_src_set(self) -> crate::common::Reg<regs::Ch15Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x133cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_dst_set(self) -> crate::common::Reg<regs::Ch15Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1340usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_link_set(self) -> crate::common::Reg<regs::Ch15Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1344usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_xctrl_set(self) -> crate::common::Reg<regs::Ch15Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1348usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ch15_ilsrc_set(self) -> crate::common::Reg<regs::Ch15Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1350usize) as _) }
    }
    #[doc = "Module enable disable Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Software Reset Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn swrst_clr(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Control Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "Sync Trig Sw Set Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn syncswset_clr(self) -> crate::common::Reg<regs::Syncswset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "Sync Trig Sw Clear register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn syncswclr_clr(self) -> crate::common::Reg<regs::Syncswclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "Sync HW trigger enable register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn synchwen_clr(self) -> crate::common::Reg<regs::Synchwen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "Sync HW trigger selection register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn synchwsel_clr(self) -> crate::common::Reg<regs::Synchwsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Channel Enable Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chen_clr(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "Channel Disable Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chdis_clr(self) -> crate::common::Reg<regs::Chdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn chdone_clr(self) -> crate::common::Reg<regs::Chdone, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "Channel Debug Halt Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dbghalt_clr(self) -> crate::common::Reg<regs::Dbghalt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn swreq_clr(self) -> crate::common::Reg<regs::Swreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "Channel Request Disable Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn reqdis_clr(self) -> crate::common::Reg<regs::Reqdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "Channel Link Load Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn linkload_clr(self) -> crate::common::Reg<regs::Linkload, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x204cusize) as _) }
    }
    #[doc = "Channel Request Clear Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn reqclear_clr(self) -> crate::common::Reg<regs::Reqclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "Interrupt Flag Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "Done Interrupt Enable Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_cfg_clr(self) -> crate::common::Reg<regs::Ch0Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_loop_clr(self) -> crate::common::Reg<regs::Ch0Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_ctrl_clr(self) -> crate::common::Reg<regs::Ch0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_src_clr(self) -> crate::common::Reg<regs::Ch0Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x206cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_dst_clr(self) -> crate::common::Reg<regs::Ch0Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_link_clr(self) -> crate::common::Reg<regs::Ch0Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_xctrl_clr(self) -> crate::common::Reg<regs::Ch0Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2078usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch0_ilsrc_clr(self) -> crate::common::Reg<regs::Ch0Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_cfg_clr(self) -> crate::common::Reg<regs::Ch1Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_loop_clr(self) -> crate::common::Reg<regs::Ch1Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2094usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_ctrl_clr(self) -> crate::common::Reg<regs::Ch1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2098usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_src_clr(self) -> crate::common::Reg<regs::Ch1Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x209cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_dst_clr(self) -> crate::common::Reg<regs::Ch1Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_link_clr(self) -> crate::common::Reg<regs::Ch1Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_xctrl_clr(self) -> crate::common::Reg<regs::Ch1Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch1_ilsrc_clr(self) -> crate::common::Reg<regs::Ch1Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_cfg_clr(self) -> crate::common::Reg<regs::Ch2Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_loop_clr(self) -> crate::common::Reg<regs::Ch2Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_ctrl_clr(self) -> crate::common::Reg<regs::Ch2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_src_clr(self) -> crate::common::Reg<regs::Ch2Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20ccusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_dst_clr(self) -> crate::common::Reg<regs::Ch2Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_link_clr(self) -> crate::common::Reg<regs::Ch2Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_xctrl_clr(self) -> crate::common::Reg<regs::Ch2Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch2_ilsrc_clr(self) -> crate::common::Reg<regs::Ch2Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20e0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_cfg_clr(self) -> crate::common::Reg<regs::Ch3Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_loop_clr(self) -> crate::common::Reg<regs::Ch3Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_ctrl_clr(self) -> crate::common::Reg<regs::Ch3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_src_clr(self) -> crate::common::Reg<regs::Ch3Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20fcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_dst_clr(self) -> crate::common::Reg<regs::Ch3Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_link_clr(self) -> crate::common::Reg<regs::Ch3Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2104usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_xctrl_clr(self) -> crate::common::Reg<regs::Ch3Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2108usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch3_ilsrc_clr(self) -> crate::common::Reg<regs::Ch3Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2110usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_cfg_clr(self) -> crate::common::Reg<regs::Ch4Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_loop_clr(self) -> crate::common::Reg<regs::Ch4Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_ctrl_clr(self) -> crate::common::Reg<regs::Ch4Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_src_clr(self) -> crate::common::Reg<regs::Ch4Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x212cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_dst_clr(self) -> crate::common::Reg<regs::Ch4Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2130usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_link_clr(self) -> crate::common::Reg<regs::Ch4Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2134usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_xctrl_clr(self) -> crate::common::Reg<regs::Ch4Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2138usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch4_ilsrc_clr(self) -> crate::common::Reg<regs::Ch4Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_cfg_clr(self) -> crate::common::Reg<regs::Ch5Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2150usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_loop_clr(self) -> crate::common::Reg<regs::Ch5Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2154usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_ctrl_clr(self) -> crate::common::Reg<regs::Ch5Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2158usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_src_clr(self) -> crate::common::Reg<regs::Ch5Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x215cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_dst_clr(self) -> crate::common::Reg<regs::Ch5Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_link_clr(self) -> crate::common::Reg<regs::Ch5Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2164usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_xctrl_clr(self) -> crate::common::Reg<regs::Ch5Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2168usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch5_ilsrc_clr(self) -> crate::common::Reg<regs::Ch5Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_cfg_clr(self) -> crate::common::Reg<regs::Ch6Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_loop_clr(self) -> crate::common::Reg<regs::Ch6Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2184usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_ctrl_clr(self) -> crate::common::Reg<regs::Ch6Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2188usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_src_clr(self) -> crate::common::Reg<regs::Ch6Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x218cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_dst_clr(self) -> crate::common::Reg<regs::Ch6Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2190usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_link_clr(self) -> crate::common::Reg<regs::Ch6Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2194usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_xctrl_clr(self) -> crate::common::Reg<regs::Ch6Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2198usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch6_ilsrc_clr(self) -> crate::common::Reg<regs::Ch6Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_cfg_clr(self) -> crate::common::Reg<regs::Ch7Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_loop_clr(self) -> crate::common::Reg<regs::Ch7Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_ctrl_clr(self) -> crate::common::Reg<regs::Ch7Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_src_clr(self) -> crate::common::Reg<regs::Ch7Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21bcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_dst_clr(self) -> crate::common::Reg<regs::Ch7Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_link_clr(self) -> crate::common::Reg<regs::Ch7Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_xctrl_clr(self) -> crate::common::Reg<regs::Ch7Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch7_ilsrc_clr(self) -> crate::common::Reg<regs::Ch7Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_cfg_clr(self) -> crate::common::Reg<regs::Ch8Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_loop_clr(self) -> crate::common::Reg<regs::Ch8Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_ctrl_clr(self) -> crate::common::Reg<regs::Ch8Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_src_clr(self) -> crate::common::Reg<regs::Ch8Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21ecusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_dst_clr(self) -> crate::common::Reg<regs::Ch8Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_link_clr(self) -> crate::common::Reg<regs::Ch8Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_xctrl_clr(self) -> crate::common::Reg<regs::Ch8Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch8_ilsrc_clr(self) -> crate::common::Reg<regs::Ch8Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_cfg_clr(self) -> crate::common::Reg<regs::Ch9Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2210usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_loop_clr(self) -> crate::common::Reg<regs::Ch9Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2214usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_ctrl_clr(self) -> crate::common::Reg<regs::Ch9Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2218usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_src_clr(self) -> crate::common::Reg<regs::Ch9Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x221cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_dst_clr(self) -> crate::common::Reg<regs::Ch9Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2220usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_link_clr(self) -> crate::common::Reg<regs::Ch9Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2224usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_xctrl_clr(self) -> crate::common::Reg<regs::Ch9Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2228usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch9_ilsrc_clr(self) -> crate::common::Reg<regs::Ch9Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2230usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_cfg_clr(self) -> crate::common::Reg<regs::Ch10Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2240usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_loop_clr(self) -> crate::common::Reg<regs::Ch10Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2244usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_ctrl_clr(self) -> crate::common::Reg<regs::Ch10Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2248usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_src_clr(self) -> crate::common::Reg<regs::Ch10Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x224cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_dst_clr(self) -> crate::common::Reg<regs::Ch10Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2250usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_link_clr(self) -> crate::common::Reg<regs::Ch10Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2254usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_xctrl_clr(self) -> crate::common::Reg<regs::Ch10Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2258usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch10_ilsrc_clr(self) -> crate::common::Reg<regs::Ch10Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2260usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_cfg_clr(self) -> crate::common::Reg<regs::Ch11Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2270usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_loop_clr(self) -> crate::common::Reg<regs::Ch11Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2274usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_ctrl_clr(self) -> crate::common::Reg<regs::Ch11Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2278usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_src_clr(self) -> crate::common::Reg<regs::Ch11Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x227cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_dst_clr(self) -> crate::common::Reg<regs::Ch11Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_link_clr(self) -> crate::common::Reg<regs::Ch11Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2284usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_xctrl_clr(self) -> crate::common::Reg<regs::Ch11Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2288usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch11_ilsrc_clr(self) -> crate::common::Reg<regs::Ch11Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2290usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_cfg_clr(self) -> crate::common::Reg<regs::Ch12Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22a0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_loop_clr(self) -> crate::common::Reg<regs::Ch12Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22a4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_ctrl_clr(self) -> crate::common::Reg<regs::Ch12Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22a8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_src_clr(self) -> crate::common::Reg<regs::Ch12Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22acusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_dst_clr(self) -> crate::common::Reg<regs::Ch12Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22b0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_link_clr(self) -> crate::common::Reg<regs::Ch12Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22b4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_xctrl_clr(self) -> crate::common::Reg<regs::Ch12Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22b8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch12_ilsrc_clr(self) -> crate::common::Reg<regs::Ch12Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22c0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_cfg_clr(self) -> crate::common::Reg<regs::Ch13Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22d0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_loop_clr(self) -> crate::common::Reg<regs::Ch13Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22d4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_ctrl_clr(self) -> crate::common::Reg<regs::Ch13Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22d8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_src_clr(self) -> crate::common::Reg<regs::Ch13Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22dcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_dst_clr(self) -> crate::common::Reg<regs::Ch13Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22e0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_link_clr(self) -> crate::common::Reg<regs::Ch13Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22e4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_xctrl_clr(self) -> crate::common::Reg<regs::Ch13Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22e8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch13_ilsrc_clr(self) -> crate::common::Reg<regs::Ch13Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22f0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_cfg_clr(self) -> crate::common::Reg<regs::Ch14Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_loop_clr(self) -> crate::common::Reg<regs::Ch14Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2304usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_ctrl_clr(self) -> crate::common::Reg<regs::Ch14Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2308usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_src_clr(self) -> crate::common::Reg<regs::Ch14Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x230cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_dst_clr(self) -> crate::common::Reg<regs::Ch14Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2310usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_link_clr(self) -> crate::common::Reg<regs::Ch14Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2314usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_xctrl_clr(self) -> crate::common::Reg<regs::Ch14Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2318usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch14_ilsrc_clr(self) -> crate::common::Reg<regs::Ch14Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2320usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_cfg_clr(self) -> crate::common::Reg<regs::Ch15Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2330usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_loop_clr(self) -> crate::common::Reg<regs::Ch15Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2334usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_ctrl_clr(self) -> crate::common::Reg<regs::Ch15Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2338usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_src_clr(self) -> crate::common::Reg<regs::Ch15Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x233cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_dst_clr(self) -> crate::common::Reg<regs::Ch15Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2340usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_link_clr(self) -> crate::common::Reg<regs::Ch15Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2344usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_xctrl_clr(self) -> crate::common::Reg<regs::Ch15Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2348usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ch15_ilsrc_clr(self) -> crate::common::Reg<regs::Ch15Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2350usize) as _) }
    }
    #[doc = "Module enable disable Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "Software Reset Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn swrst_tgl(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Control Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "Sync Trig Sw Set Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn syncswset_tgl(self) -> crate::common::Reg<regs::Syncswset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "Sync Trig Sw Clear register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn syncswclr_tgl(self) -> crate::common::Reg<regs::Syncswclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "Sync HW trigger enable register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn synchwen_tgl(self) -> crate::common::Reg<regs::Synchwen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "Sync HW trigger selection register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn synchwsel_tgl(self) -> crate::common::Reg<regs::Synchwsel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Channel Enable Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chen_tgl(self) -> crate::common::Reg<regs::Chen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "Channel Disable Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chdis_tgl(self) -> crate::common::Reg<regs::Chdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn chdone_tgl(self) -> crate::common::Reg<regs::Chdone, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "Channel Debug Halt Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dbghalt_tgl(self) -> crate::common::Reg<regs::Dbghalt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn swreq_tgl(self) -> crate::common::Reg<regs::Swreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "Channel Request Disable Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn reqdis_tgl(self) -> crate::common::Reg<regs::Reqdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "Channel Link Load Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn linkload_tgl(self) -> crate::common::Reg<regs::Linkload, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "Channel Request Clear Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn reqclear_tgl(self) -> crate::common::Reg<regs::Reqclear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "Interrupt Flag Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "Done Interrupt Enable Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_cfg_tgl(self) -> crate::common::Reg<regs::Ch0Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_loop_tgl(self) -> crate::common::Reg<regs::Ch0Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_ctrl_tgl(self) -> crate::common::Reg<regs::Ch0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_src_tgl(self) -> crate::common::Reg<regs::Ch0Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_dst_tgl(self) -> crate::common::Reg<regs::Ch0Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_link_tgl(self) -> crate::common::Reg<regs::Ch0Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_xctrl_tgl(self) -> crate::common::Reg<regs::Ch0Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3078usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch0_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch0Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_cfg_tgl(self) -> crate::common::Reg<regs::Ch1Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3090usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_loop_tgl(self) -> crate::common::Reg<regs::Ch1Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3094usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_ctrl_tgl(self) -> crate::common::Reg<regs::Ch1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3098usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_src_tgl(self) -> crate::common::Reg<regs::Ch1Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x309cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_dst_tgl(self) -> crate::common::Reg<regs::Ch1Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_link_tgl(self) -> crate::common::Reg<regs::Ch1Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_xctrl_tgl(self) -> crate::common::Reg<regs::Ch1Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch1_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch1Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_cfg_tgl(self) -> crate::common::Reg<regs::Ch2Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_loop_tgl(self) -> crate::common::Reg<regs::Ch2Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_ctrl_tgl(self) -> crate::common::Reg<regs::Ch2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_src_tgl(self) -> crate::common::Reg<regs::Ch2Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30ccusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_dst_tgl(self) -> crate::common::Reg<regs::Ch2Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_link_tgl(self) -> crate::common::Reg<regs::Ch2Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_xctrl_tgl(self) -> crate::common::Reg<regs::Ch2Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch2_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch2Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30e0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_cfg_tgl(self) -> crate::common::Reg<regs::Ch3Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_loop_tgl(self) -> crate::common::Reg<regs::Ch3Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_ctrl_tgl(self) -> crate::common::Reg<regs::Ch3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_src_tgl(self) -> crate::common::Reg<regs::Ch3Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30fcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_dst_tgl(self) -> crate::common::Reg<regs::Ch3Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3100usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_link_tgl(self) -> crate::common::Reg<regs::Ch3Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3104usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_xctrl_tgl(self) -> crate::common::Reg<regs::Ch3Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3108usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch3_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch3Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3110usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_cfg_tgl(self) -> crate::common::Reg<regs::Ch4Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_loop_tgl(self) -> crate::common::Reg<regs::Ch4Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3124usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_ctrl_tgl(self) -> crate::common::Reg<regs::Ch4Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_src_tgl(self) -> crate::common::Reg<regs::Ch4Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x312cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_dst_tgl(self) -> crate::common::Reg<regs::Ch4Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3130usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_link_tgl(self) -> crate::common::Reg<regs::Ch4Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3134usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_xctrl_tgl(self) -> crate::common::Reg<regs::Ch4Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3138usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch4_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch4Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3140usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_cfg_tgl(self) -> crate::common::Reg<regs::Ch5Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3150usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_loop_tgl(self) -> crate::common::Reg<regs::Ch5Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3154usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_ctrl_tgl(self) -> crate::common::Reg<regs::Ch5Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3158usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_src_tgl(self) -> crate::common::Reg<regs::Ch5Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x315cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_dst_tgl(self) -> crate::common::Reg<regs::Ch5Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3160usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_link_tgl(self) -> crate::common::Reg<regs::Ch5Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3164usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_xctrl_tgl(self) -> crate::common::Reg<regs::Ch5Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3168usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch5_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch5Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3170usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_cfg_tgl(self) -> crate::common::Reg<regs::Ch6Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3180usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_loop_tgl(self) -> crate::common::Reg<regs::Ch6Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3184usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_ctrl_tgl(self) -> crate::common::Reg<regs::Ch6Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3188usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_src_tgl(self) -> crate::common::Reg<regs::Ch6Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x318cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_dst_tgl(self) -> crate::common::Reg<regs::Ch6Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3190usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_link_tgl(self) -> crate::common::Reg<regs::Ch6Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3194usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_xctrl_tgl(self) -> crate::common::Reg<regs::Ch6Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3198usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch6_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch6Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_cfg_tgl(self) -> crate::common::Reg<regs::Ch7Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_loop_tgl(self) -> crate::common::Reg<regs::Ch7Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_ctrl_tgl(self) -> crate::common::Reg<regs::Ch7Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_src_tgl(self) -> crate::common::Reg<regs::Ch7Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31bcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_dst_tgl(self) -> crate::common::Reg<regs::Ch7Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_link_tgl(self) -> crate::common::Reg<regs::Ch7Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_xctrl_tgl(self) -> crate::common::Reg<regs::Ch7Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch7_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch7Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_cfg_tgl(self) -> crate::common::Reg<regs::Ch8Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_loop_tgl(self) -> crate::common::Reg<regs::Ch8Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_ctrl_tgl(self) -> crate::common::Reg<regs::Ch8Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_src_tgl(self) -> crate::common::Reg<regs::Ch8Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31ecusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_dst_tgl(self) -> crate::common::Reg<regs::Ch8Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_link_tgl(self) -> crate::common::Reg<regs::Ch8Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_xctrl_tgl(self) -> crate::common::Reg<regs::Ch8Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch8_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch8Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_cfg_tgl(self) -> crate::common::Reg<regs::Ch9Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3210usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_loop_tgl(self) -> crate::common::Reg<regs::Ch9Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3214usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_ctrl_tgl(self) -> crate::common::Reg<regs::Ch9Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3218usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_src_tgl(self) -> crate::common::Reg<regs::Ch9Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x321cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_dst_tgl(self) -> crate::common::Reg<regs::Ch9Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3220usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_link_tgl(self) -> crate::common::Reg<regs::Ch9Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3224usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_xctrl_tgl(self) -> crate::common::Reg<regs::Ch9Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3228usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch9_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch9Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3230usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_cfg_tgl(self) -> crate::common::Reg<regs::Ch10Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3240usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_loop_tgl(self) -> crate::common::Reg<regs::Ch10Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3244usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_ctrl_tgl(self) -> crate::common::Reg<regs::Ch10Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3248usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_src_tgl(self) -> crate::common::Reg<regs::Ch10Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x324cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_dst_tgl(self) -> crate::common::Reg<regs::Ch10Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3250usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_link_tgl(self) -> crate::common::Reg<regs::Ch10Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3254usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_xctrl_tgl(self) -> crate::common::Reg<regs::Ch10Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3258usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch10_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch10Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3260usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_cfg_tgl(self) -> crate::common::Reg<regs::Ch11Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3270usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_loop_tgl(self) -> crate::common::Reg<regs::Ch11Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3274usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_ctrl_tgl(self) -> crate::common::Reg<regs::Ch11Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3278usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_src_tgl(self) -> crate::common::Reg<regs::Ch11Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x327cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_dst_tgl(self) -> crate::common::Reg<regs::Ch11Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3280usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_link_tgl(self) -> crate::common::Reg<regs::Ch11Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3284usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_xctrl_tgl(self) -> crate::common::Reg<regs::Ch11Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3288usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch11_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch11Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3290usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_cfg_tgl(self) -> crate::common::Reg<regs::Ch12Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32a0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_loop_tgl(self) -> crate::common::Reg<regs::Ch12Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32a4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_ctrl_tgl(self) -> crate::common::Reg<regs::Ch12Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32a8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_src_tgl(self) -> crate::common::Reg<regs::Ch12Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32acusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_dst_tgl(self) -> crate::common::Reg<regs::Ch12Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32b0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_link_tgl(self) -> crate::common::Reg<regs::Ch12Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32b4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_xctrl_tgl(self) -> crate::common::Reg<regs::Ch12Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32b8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch12_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch12Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32c0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_cfg_tgl(self) -> crate::common::Reg<regs::Ch13Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32d0usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_loop_tgl(self) -> crate::common::Reg<regs::Ch13Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32d4usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_ctrl_tgl(self) -> crate::common::Reg<regs::Ch13Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32d8usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_src_tgl(self) -> crate::common::Reg<regs::Ch13Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32dcusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_dst_tgl(self) -> crate::common::Reg<regs::Ch13Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32e0usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_link_tgl(self) -> crate::common::Reg<regs::Ch13Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32e4usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_xctrl_tgl(self) -> crate::common::Reg<regs::Ch13Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32e8usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch13_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch13Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32f0usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_cfg_tgl(self) -> crate::common::Reg<regs::Ch14Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3300usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_loop_tgl(self) -> crate::common::Reg<regs::Ch14Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3304usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_ctrl_tgl(self) -> crate::common::Reg<regs::Ch14Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3308usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_src_tgl(self) -> crate::common::Reg<regs::Ch14Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x330cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_dst_tgl(self) -> crate::common::Reg<regs::Ch14Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3310usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_link_tgl(self) -> crate::common::Reg<regs::Ch14Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3314usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_xctrl_tgl(self) -> crate::common::Reg<regs::Ch14Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3318usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch14_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch14Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3320usize) as _) }
    }
    #[doc = "Channel Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_cfg_tgl(self) -> crate::common::Reg<regs::Ch15Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3330usize) as _) }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_loop_tgl(self) -> crate::common::Reg<regs::Ch15Loop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3334usize) as _) }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_ctrl_tgl(self) -> crate::common::Reg<regs::Ch15Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3338usize) as _) }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_src_tgl(self) -> crate::common::Reg<regs::Ch15Src, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x333cusize) as _) }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_dst_tgl(self) -> crate::common::Reg<regs::Ch15Dst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3340usize) as _) }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_link_tgl(self) -> crate::common::Reg<regs::Ch15Link, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3344usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_xctrl_tgl(self) -> crate::common::Reg<regs::Ch15Xctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3348usize) as _) }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ch15_ilsrc_tgl(self) -> crate::common::Reg<regs::Ch15Ilsrc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3350usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Cfg(pub u32);
    impl Ch0Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch0CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch0CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch0CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch0CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch0CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch0CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch0CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch0CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch0CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch0CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch0CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch0CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch0CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch0CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch0CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch0CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch0CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch0CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch0Cfg {
        #[inline(always)]
        fn default() -> Ch0Cfg {
            Ch0Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch0Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Ctrl(pub u32);
    impl Ch0Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch0CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch0CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch0CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch0CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch0CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch0CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch0CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch0CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch0CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch0CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch0CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch0CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch0CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch0CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch0CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch0CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch0CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch0CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch0CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch0CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch0CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch0CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch0CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch0CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch0Ctrl {
        #[inline(always)]
        fn default() -> Ch0Ctrl {
            Ch0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Dst(pub u32);
    impl Ch0Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch0Dst {
        #[inline(always)]
        fn default() -> Ch0Dst {
            Ch0Dst(0)
        }
    }
    impl core::fmt::Debug for Ch0Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch0Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Ilsrc(pub u32);
    impl Ch0Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch0Ilsrc {
        #[inline(always)]
        fn default() -> Ch0Ilsrc {
            Ch0Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch0Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch0Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Link(pub u32);
    impl Ch0Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch0LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch0LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch0LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch0Link {
        #[inline(always)]
        fn default() -> Ch0Link {
            Ch0Link(0)
        }
    }
    impl core::fmt::Debug for Ch0Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Loop(pub u32);
    impl Ch0Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch0Loop {
        #[inline(always)]
        fn default() -> Ch0Loop {
            Ch0Loop(0)
        }
    }
    impl core::fmt::Debug for Ch0Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch0Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Src(pub u32);
    impl Ch0Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch0Src {
        #[inline(always)]
        fn default() -> Ch0Src {
            Ch0Src(0)
        }
    }
    impl core::fmt::Debug for Ch0Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch0Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch0Xctrl(pub u32);
    impl Ch0Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch0XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch0XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch0XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch0Xctrl {
        #[inline(always)]
        fn default() -> Ch0Xctrl {
            Ch0Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch0Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch0Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch0Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch0Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Cfg(pub u32);
    impl Ch10Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch10CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch10CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch10CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch10CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch10CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch10CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch10CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch10CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch10CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch10CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch10CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch10CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch10CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch10CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch10CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch10CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch10CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch10CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch10Cfg {
        #[inline(always)]
        fn default() -> Ch10Cfg {
            Ch10Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch10Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Ctrl(pub u32);
    impl Ch10Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch10CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch10CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch10CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch10CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch10CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch10CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch10CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch10CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch10CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch10CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch10CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch10CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch10CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch10CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch10CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch10CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch10CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch10CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch10CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch10CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch10CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch10CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch10CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch10CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch10Ctrl {
        #[inline(always)]
        fn default() -> Ch10Ctrl {
            Ch10Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch10Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Dst(pub u32);
    impl Ch10Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch10Dst {
        #[inline(always)]
        fn default() -> Ch10Dst {
            Ch10Dst(0)
        }
    }
    impl core::fmt::Debug for Ch10Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch10Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Ilsrc(pub u32);
    impl Ch10Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch10Ilsrc {
        #[inline(always)]
        fn default() -> Ch10Ilsrc {
            Ch10Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch10Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch10Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Link(pub u32);
    impl Ch10Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch10LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch10LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch10LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch10Link {
        #[inline(always)]
        fn default() -> Ch10Link {
            Ch10Link(0)
        }
    }
    impl core::fmt::Debug for Ch10Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Loop(pub u32);
    impl Ch10Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch10Loop {
        #[inline(always)]
        fn default() -> Ch10Loop {
            Ch10Loop(0)
        }
    }
    impl core::fmt::Debug for Ch10Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch10Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Src(pub u32);
    impl Ch10Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch10Src {
        #[inline(always)]
        fn default() -> Ch10Src {
            Ch10Src(0)
        }
    }
    impl core::fmt::Debug for Ch10Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch10Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch10Xctrl(pub u32);
    impl Ch10Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch10XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch10XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch10XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch10Xctrl {
        #[inline(always)]
        fn default() -> Ch10Xctrl {
            Ch10Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch10Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch10Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch10Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch10Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Cfg(pub u32);
    impl Ch11Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch11CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch11CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch11CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch11CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch11CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch11CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch11CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch11CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch11CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch11CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch11CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch11CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch11CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch11CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch11CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch11CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch11CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch11CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch11Cfg {
        #[inline(always)]
        fn default() -> Ch11Cfg {
            Ch11Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch11Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Ctrl(pub u32);
    impl Ch11Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch11CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch11CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch11CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch11CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch11CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch11CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch11CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch11CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch11CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch11CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch11CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch11CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch11CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch11CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch11CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch11CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch11CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch11CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch11CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch11CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch11CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch11CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch11CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch11CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch11Ctrl {
        #[inline(always)]
        fn default() -> Ch11Ctrl {
            Ch11Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch11Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Dst(pub u32);
    impl Ch11Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch11Dst {
        #[inline(always)]
        fn default() -> Ch11Dst {
            Ch11Dst(0)
        }
    }
    impl core::fmt::Debug for Ch11Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch11Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Ilsrc(pub u32);
    impl Ch11Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch11Ilsrc {
        #[inline(always)]
        fn default() -> Ch11Ilsrc {
            Ch11Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch11Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch11Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Link(pub u32);
    impl Ch11Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch11LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch11LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch11LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch11Link {
        #[inline(always)]
        fn default() -> Ch11Link {
            Ch11Link(0)
        }
    }
    impl core::fmt::Debug for Ch11Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Loop(pub u32);
    impl Ch11Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch11Loop {
        #[inline(always)]
        fn default() -> Ch11Loop {
            Ch11Loop(0)
        }
    }
    impl core::fmt::Debug for Ch11Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch11Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Src(pub u32);
    impl Ch11Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch11Src {
        #[inline(always)]
        fn default() -> Ch11Src {
            Ch11Src(0)
        }
    }
    impl core::fmt::Debug for Ch11Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch11Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch11Xctrl(pub u32);
    impl Ch11Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch11XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch11XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch11XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch11Xctrl {
        #[inline(always)]
        fn default() -> Ch11Xctrl {
            Ch11Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch11Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch11Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch11Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch11Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Cfg(pub u32);
    impl Ch12Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch12CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch12CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch12CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch12CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch12CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch12CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch12CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch12CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch12CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch12CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch12CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch12CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch12CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch12CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch12CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch12CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch12CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch12CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch12Cfg {
        #[inline(always)]
        fn default() -> Ch12Cfg {
            Ch12Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch12Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Ctrl(pub u32);
    impl Ch12Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch12CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch12CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch12CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch12CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch12CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch12CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch12CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch12CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch12CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch12CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch12CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch12CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch12CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch12CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch12CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch12CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch12CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch12CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch12CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch12CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch12CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch12CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch12CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch12CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch12Ctrl {
        #[inline(always)]
        fn default() -> Ch12Ctrl {
            Ch12Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch12Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Dst(pub u32);
    impl Ch12Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch12Dst {
        #[inline(always)]
        fn default() -> Ch12Dst {
            Ch12Dst(0)
        }
    }
    impl core::fmt::Debug for Ch12Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch12Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Ilsrc(pub u32);
    impl Ch12Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch12Ilsrc {
        #[inline(always)]
        fn default() -> Ch12Ilsrc {
            Ch12Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch12Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch12Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Link(pub u32);
    impl Ch12Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch12LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch12LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch12LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch12Link {
        #[inline(always)]
        fn default() -> Ch12Link {
            Ch12Link(0)
        }
    }
    impl core::fmt::Debug for Ch12Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Loop(pub u32);
    impl Ch12Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch12Loop {
        #[inline(always)]
        fn default() -> Ch12Loop {
            Ch12Loop(0)
        }
    }
    impl core::fmt::Debug for Ch12Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch12Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Src(pub u32);
    impl Ch12Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch12Src {
        #[inline(always)]
        fn default() -> Ch12Src {
            Ch12Src(0)
        }
    }
    impl core::fmt::Debug for Ch12Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch12Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch12Xctrl(pub u32);
    impl Ch12Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch12XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch12XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch12XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch12Xctrl {
        #[inline(always)]
        fn default() -> Ch12Xctrl {
            Ch12Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch12Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch12Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch12Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch12Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Cfg(pub u32);
    impl Ch13Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch13CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch13CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch13CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch13CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch13CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch13CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch13CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch13CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch13CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch13CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch13CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch13CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch13CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch13CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch13CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch13CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch13CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch13CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch13Cfg {
        #[inline(always)]
        fn default() -> Ch13Cfg {
            Ch13Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch13Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Ctrl(pub u32);
    impl Ch13Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch13CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch13CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch13CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch13CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch13CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch13CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch13CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch13CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch13CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch13CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch13CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch13CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch13CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch13CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch13CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch13CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch13CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch13CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch13CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch13CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch13CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch13CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch13CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch13CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch13Ctrl {
        #[inline(always)]
        fn default() -> Ch13Ctrl {
            Ch13Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch13Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Dst(pub u32);
    impl Ch13Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch13Dst {
        #[inline(always)]
        fn default() -> Ch13Dst {
            Ch13Dst(0)
        }
    }
    impl core::fmt::Debug for Ch13Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch13Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Ilsrc(pub u32);
    impl Ch13Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch13Ilsrc {
        #[inline(always)]
        fn default() -> Ch13Ilsrc {
            Ch13Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch13Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch13Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Link(pub u32);
    impl Ch13Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch13LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch13LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch13LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch13Link {
        #[inline(always)]
        fn default() -> Ch13Link {
            Ch13Link(0)
        }
    }
    impl core::fmt::Debug for Ch13Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Loop(pub u32);
    impl Ch13Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch13Loop {
        #[inline(always)]
        fn default() -> Ch13Loop {
            Ch13Loop(0)
        }
    }
    impl core::fmt::Debug for Ch13Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch13Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Src(pub u32);
    impl Ch13Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch13Src {
        #[inline(always)]
        fn default() -> Ch13Src {
            Ch13Src(0)
        }
    }
    impl core::fmt::Debug for Ch13Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch13Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch13Xctrl(pub u32);
    impl Ch13Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch13XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch13XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch13XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch13Xctrl {
        #[inline(always)]
        fn default() -> Ch13Xctrl {
            Ch13Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch13Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch13Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch13Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch13Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Cfg(pub u32);
    impl Ch14Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch14CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch14CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch14CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch14CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch14CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch14CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch14CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch14CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch14CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch14CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch14CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch14CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch14CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch14CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch14CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch14CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch14CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch14CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch14Cfg {
        #[inline(always)]
        fn default() -> Ch14Cfg {
            Ch14Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch14Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Ctrl(pub u32);
    impl Ch14Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch14CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch14CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch14CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch14CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch14CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch14CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch14CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch14CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch14CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch14CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch14CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch14CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch14CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch14CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch14CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch14CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch14CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch14CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch14CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch14CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch14CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch14CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch14CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch14CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch14Ctrl {
        #[inline(always)]
        fn default() -> Ch14Ctrl {
            Ch14Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch14Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Dst(pub u32);
    impl Ch14Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch14Dst {
        #[inline(always)]
        fn default() -> Ch14Dst {
            Ch14Dst(0)
        }
    }
    impl core::fmt::Debug for Ch14Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch14Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Ilsrc(pub u32);
    impl Ch14Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch14Ilsrc {
        #[inline(always)]
        fn default() -> Ch14Ilsrc {
            Ch14Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch14Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch14Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Link(pub u32);
    impl Ch14Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch14LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch14LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch14LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch14Link {
        #[inline(always)]
        fn default() -> Ch14Link {
            Ch14Link(0)
        }
    }
    impl core::fmt::Debug for Ch14Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Loop(pub u32);
    impl Ch14Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch14Loop {
        #[inline(always)]
        fn default() -> Ch14Loop {
            Ch14Loop(0)
        }
    }
    impl core::fmt::Debug for Ch14Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch14Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Src(pub u32);
    impl Ch14Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch14Src {
        #[inline(always)]
        fn default() -> Ch14Src {
            Ch14Src(0)
        }
    }
    impl core::fmt::Debug for Ch14Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch14Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch14Xctrl(pub u32);
    impl Ch14Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch14XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch14XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch14XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch14Xctrl {
        #[inline(always)]
        fn default() -> Ch14Xctrl {
            Ch14Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch14Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch14Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch14Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch14Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Cfg(pub u32);
    impl Ch15Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch15CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch15CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch15CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch15CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch15CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch15CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch15CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch15CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch15CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch15CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch15CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch15CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch15CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch15CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch15CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch15CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch15CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch15CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch15Cfg {
        #[inline(always)]
        fn default() -> Ch15Cfg {
            Ch15Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch15Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Ctrl(pub u32);
    impl Ch15Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch15CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch15CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch15CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch15CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch15CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch15CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch15CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch15CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch15CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch15CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch15CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch15CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch15CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch15CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch15CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch15CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch15CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch15CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch15CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch15CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch15CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch15CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch15CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch15CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch15Ctrl {
        #[inline(always)]
        fn default() -> Ch15Ctrl {
            Ch15Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch15Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Dst(pub u32);
    impl Ch15Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch15Dst {
        #[inline(always)]
        fn default() -> Ch15Dst {
            Ch15Dst(0)
        }
    }
    impl core::fmt::Debug for Ch15Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch15Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Ilsrc(pub u32);
    impl Ch15Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch15Ilsrc {
        #[inline(always)]
        fn default() -> Ch15Ilsrc {
            Ch15Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch15Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch15Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Link(pub u32);
    impl Ch15Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch15LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch15LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch15LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch15Link {
        #[inline(always)]
        fn default() -> Ch15Link {
            Ch15Link(0)
        }
    }
    impl core::fmt::Debug for Ch15Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Loop(pub u32);
    impl Ch15Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch15Loop {
        #[inline(always)]
        fn default() -> Ch15Loop {
            Ch15Loop(0)
        }
    }
    impl core::fmt::Debug for Ch15Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch15Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Src(pub u32);
    impl Ch15Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch15Src {
        #[inline(always)]
        fn default() -> Ch15Src {
            Ch15Src(0)
        }
    }
    impl core::fmt::Debug for Ch15Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch15Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch15Xctrl(pub u32);
    impl Ch15Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch15XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch15XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch15XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch15Xctrl {
        #[inline(always)]
        fn default() -> Ch15Xctrl {
            Ch15Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch15Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch15Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch15Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch15Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Cfg(pub u32);
    impl Ch1Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch1CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch1CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch1CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch1CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch1CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch1CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch1CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch1CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch1CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch1CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch1CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch1CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch1CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch1CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch1CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch1CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch1CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch1CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch1Cfg {
        #[inline(always)]
        fn default() -> Ch1Cfg {
            Ch1Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch1Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Ctrl(pub u32);
    impl Ch1Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch1CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch1CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch1CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch1CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch1CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch1CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch1CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch1CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch1CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch1CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch1CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch1CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch1CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch1CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch1CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch1CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch1CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch1CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch1CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch1CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch1CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch1CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch1CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch1CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch1Ctrl {
        #[inline(always)]
        fn default() -> Ch1Ctrl {
            Ch1Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Dst(pub u32);
    impl Ch1Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch1Dst {
        #[inline(always)]
        fn default() -> Ch1Dst {
            Ch1Dst(0)
        }
    }
    impl core::fmt::Debug for Ch1Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Ilsrc(pub u32);
    impl Ch1Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch1Ilsrc {
        #[inline(always)]
        fn default() -> Ch1Ilsrc {
            Ch1Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch1Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Link(pub u32);
    impl Ch1Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch1LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch1LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch1LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch1Link {
        #[inline(always)]
        fn default() -> Ch1Link {
            Ch1Link(0)
        }
    }
    impl core::fmt::Debug for Ch1Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Loop(pub u32);
    impl Ch1Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch1Loop {
        #[inline(always)]
        fn default() -> Ch1Loop {
            Ch1Loop(0)
        }
    }
    impl core::fmt::Debug for Ch1Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Src(pub u32);
    impl Ch1Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch1Src {
        #[inline(always)]
        fn default() -> Ch1Src {
            Ch1Src(0)
        }
    }
    impl core::fmt::Debug for Ch1Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch1Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch1Xctrl(pub u32);
    impl Ch1Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch1XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch1XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch1XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch1Xctrl {
        #[inline(always)]
        fn default() -> Ch1Xctrl {
            Ch1Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch1Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch1Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch1Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch1Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Cfg(pub u32);
    impl Ch2Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch2CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch2CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch2CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch2CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch2CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch2CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch2CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch2CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch2CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch2CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch2CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch2CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch2CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch2CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch2CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch2CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch2CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch2CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch2Cfg {
        #[inline(always)]
        fn default() -> Ch2Cfg {
            Ch2Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch2Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Ctrl(pub u32);
    impl Ch2Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch2CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch2CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch2CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch2CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch2CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch2CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch2CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch2CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch2CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch2CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch2CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch2CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch2CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch2CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch2CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch2CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch2CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch2CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch2CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch2CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch2CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch2CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch2CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch2CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch2Ctrl {
        #[inline(always)]
        fn default() -> Ch2Ctrl {
            Ch2Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch2Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Dst(pub u32);
    impl Ch2Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch2Dst {
        #[inline(always)]
        fn default() -> Ch2Dst {
            Ch2Dst(0)
        }
    }
    impl core::fmt::Debug for Ch2Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Ilsrc(pub u32);
    impl Ch2Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch2Ilsrc {
        #[inline(always)]
        fn default() -> Ch2Ilsrc {
            Ch2Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch2Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Link(pub u32);
    impl Ch2Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch2LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch2LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch2LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch2Link {
        #[inline(always)]
        fn default() -> Ch2Link {
            Ch2Link(0)
        }
    }
    impl core::fmt::Debug for Ch2Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Loop(pub u32);
    impl Ch2Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch2Loop {
        #[inline(always)]
        fn default() -> Ch2Loop {
            Ch2Loop(0)
        }
    }
    impl core::fmt::Debug for Ch2Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Src(pub u32);
    impl Ch2Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch2Src {
        #[inline(always)]
        fn default() -> Ch2Src {
            Ch2Src(0)
        }
    }
    impl core::fmt::Debug for Ch2Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch2Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch2Xctrl(pub u32);
    impl Ch2Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch2XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch2XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch2XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch2Xctrl {
        #[inline(always)]
        fn default() -> Ch2Xctrl {
            Ch2Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch2Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch2Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch2Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch2Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Cfg(pub u32);
    impl Ch3Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch3CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch3CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch3CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch3CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch3CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch3CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch3CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch3CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch3CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch3CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch3CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch3CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch3CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch3CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch3CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch3CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch3CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch3CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch3Cfg {
        #[inline(always)]
        fn default() -> Ch3Cfg {
            Ch3Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch3Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Ctrl(pub u32);
    impl Ch3Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch3CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch3CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch3CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch3CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch3CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch3CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch3CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch3CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch3CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch3CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch3CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch3CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch3CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch3CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch3CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch3CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch3CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch3CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch3CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch3CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch3CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch3CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch3CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch3CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch3Ctrl {
        #[inline(always)]
        fn default() -> Ch3Ctrl {
            Ch3Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch3Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Dst(pub u32);
    impl Ch3Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch3Dst {
        #[inline(always)]
        fn default() -> Ch3Dst {
            Ch3Dst(0)
        }
    }
    impl core::fmt::Debug for Ch3Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Ilsrc(pub u32);
    impl Ch3Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch3Ilsrc {
        #[inline(always)]
        fn default() -> Ch3Ilsrc {
            Ch3Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch3Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Link(pub u32);
    impl Ch3Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch3LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch3LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch3LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch3Link {
        #[inline(always)]
        fn default() -> Ch3Link {
            Ch3Link(0)
        }
    }
    impl core::fmt::Debug for Ch3Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Loop(pub u32);
    impl Ch3Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch3Loop {
        #[inline(always)]
        fn default() -> Ch3Loop {
            Ch3Loop(0)
        }
    }
    impl core::fmt::Debug for Ch3Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Src(pub u32);
    impl Ch3Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch3Src {
        #[inline(always)]
        fn default() -> Ch3Src {
            Ch3Src(0)
        }
    }
    impl core::fmt::Debug for Ch3Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch3Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch3Xctrl(pub u32);
    impl Ch3Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch3XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch3XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch3XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch3Xctrl {
        #[inline(always)]
        fn default() -> Ch3Xctrl {
            Ch3Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch3Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch3Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch3Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch3Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Cfg(pub u32);
    impl Ch4Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch4CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch4CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch4CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch4CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch4CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch4CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch4CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch4CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch4CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch4CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch4CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch4CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch4CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch4CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch4CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch4CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch4CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch4CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch4Cfg {
        #[inline(always)]
        fn default() -> Ch4Cfg {
            Ch4Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch4Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Ctrl(pub u32);
    impl Ch4Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch4CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch4CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch4CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch4CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch4CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch4CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch4CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch4CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch4CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch4CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch4CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch4CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch4CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch4CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch4CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch4CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch4CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch4CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch4CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch4CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch4CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch4CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch4CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch4CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch4Ctrl {
        #[inline(always)]
        fn default() -> Ch4Ctrl {
            Ch4Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch4Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Dst(pub u32);
    impl Ch4Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch4Dst {
        #[inline(always)]
        fn default() -> Ch4Dst {
            Ch4Dst(0)
        }
    }
    impl core::fmt::Debug for Ch4Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch4Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Ilsrc(pub u32);
    impl Ch4Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch4Ilsrc {
        #[inline(always)]
        fn default() -> Ch4Ilsrc {
            Ch4Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch4Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch4Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Link(pub u32);
    impl Ch4Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch4LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch4LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch4LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch4Link {
        #[inline(always)]
        fn default() -> Ch4Link {
            Ch4Link(0)
        }
    }
    impl core::fmt::Debug for Ch4Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Loop(pub u32);
    impl Ch4Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch4Loop {
        #[inline(always)]
        fn default() -> Ch4Loop {
            Ch4Loop(0)
        }
    }
    impl core::fmt::Debug for Ch4Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch4Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Src(pub u32);
    impl Ch4Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch4Src {
        #[inline(always)]
        fn default() -> Ch4Src {
            Ch4Src(0)
        }
    }
    impl core::fmt::Debug for Ch4Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch4Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch4Xctrl(pub u32);
    impl Ch4Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch4XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch4XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch4XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch4Xctrl {
        #[inline(always)]
        fn default() -> Ch4Xctrl {
            Ch4Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch4Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch4Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch4Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch4Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Cfg(pub u32);
    impl Ch5Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch5CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch5CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch5CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch5CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch5CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch5CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch5CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch5CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch5CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch5CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch5CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch5CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch5CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch5CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch5CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch5CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch5CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch5CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch5Cfg {
        #[inline(always)]
        fn default() -> Ch5Cfg {
            Ch5Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch5Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Ctrl(pub u32);
    impl Ch5Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch5CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch5CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch5CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch5CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch5CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch5CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch5CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch5CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch5CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch5CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch5CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch5CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch5CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch5CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch5CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch5CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch5CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch5CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch5CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch5CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch5CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch5CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch5CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch5CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch5Ctrl {
        #[inline(always)]
        fn default() -> Ch5Ctrl {
            Ch5Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch5Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Dst(pub u32);
    impl Ch5Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch5Dst {
        #[inline(always)]
        fn default() -> Ch5Dst {
            Ch5Dst(0)
        }
    }
    impl core::fmt::Debug for Ch5Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch5Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Ilsrc(pub u32);
    impl Ch5Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch5Ilsrc {
        #[inline(always)]
        fn default() -> Ch5Ilsrc {
            Ch5Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch5Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch5Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Link(pub u32);
    impl Ch5Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch5LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch5LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch5LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch5Link {
        #[inline(always)]
        fn default() -> Ch5Link {
            Ch5Link(0)
        }
    }
    impl core::fmt::Debug for Ch5Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Loop(pub u32);
    impl Ch5Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch5Loop {
        #[inline(always)]
        fn default() -> Ch5Loop {
            Ch5Loop(0)
        }
    }
    impl core::fmt::Debug for Ch5Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch5Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Src(pub u32);
    impl Ch5Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch5Src {
        #[inline(always)]
        fn default() -> Ch5Src {
            Ch5Src(0)
        }
    }
    impl core::fmt::Debug for Ch5Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch5Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch5Xctrl(pub u32);
    impl Ch5Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch5XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch5XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch5XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch5Xctrl {
        #[inline(always)]
        fn default() -> Ch5Xctrl {
            Ch5Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch5Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch5Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch5Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch5Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Cfg(pub u32);
    impl Ch6Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch6CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch6CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch6CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch6CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch6CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch6CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch6CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch6CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch6CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch6CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch6CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch6CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch6CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch6CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch6CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch6CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch6CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch6CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch6Cfg {
        #[inline(always)]
        fn default() -> Ch6Cfg {
            Ch6Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch6Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Ctrl(pub u32);
    impl Ch6Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch6CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch6CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch6CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch6CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch6CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch6CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch6CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch6CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch6CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch6CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch6CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch6CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch6CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch6CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch6CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch6CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch6CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch6CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch6CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch6CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch6CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch6CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch6CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch6CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch6Ctrl {
        #[inline(always)]
        fn default() -> Ch6Ctrl {
            Ch6Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch6Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Dst(pub u32);
    impl Ch6Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch6Dst {
        #[inline(always)]
        fn default() -> Ch6Dst {
            Ch6Dst(0)
        }
    }
    impl core::fmt::Debug for Ch6Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch6Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Ilsrc(pub u32);
    impl Ch6Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch6Ilsrc {
        #[inline(always)]
        fn default() -> Ch6Ilsrc {
            Ch6Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch6Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch6Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Link(pub u32);
    impl Ch6Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch6LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch6LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch6LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch6Link {
        #[inline(always)]
        fn default() -> Ch6Link {
            Ch6Link(0)
        }
    }
    impl core::fmt::Debug for Ch6Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Loop(pub u32);
    impl Ch6Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch6Loop {
        #[inline(always)]
        fn default() -> Ch6Loop {
            Ch6Loop(0)
        }
    }
    impl core::fmt::Debug for Ch6Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch6Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Src(pub u32);
    impl Ch6Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch6Src {
        #[inline(always)]
        fn default() -> Ch6Src {
            Ch6Src(0)
        }
    }
    impl core::fmt::Debug for Ch6Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch6Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch6Xctrl(pub u32);
    impl Ch6Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch6XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch6XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch6XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch6Xctrl {
        #[inline(always)]
        fn default() -> Ch6Xctrl {
            Ch6Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch6Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch6Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch6Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch6Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Cfg(pub u32);
    impl Ch7Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch7CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch7CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch7CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch7CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch7CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch7CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch7CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch7CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch7CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch7CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch7CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch7CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch7CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch7CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch7CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch7CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch7CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch7CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch7Cfg {
        #[inline(always)]
        fn default() -> Ch7Cfg {
            Ch7Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch7Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Ctrl(pub u32);
    impl Ch7Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch7CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch7CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch7CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch7CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch7CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch7CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch7CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch7CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch7CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch7CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch7CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch7CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch7CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch7CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch7CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch7CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch7CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch7CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch7CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch7CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch7CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch7CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch7CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch7CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch7Ctrl {
        #[inline(always)]
        fn default() -> Ch7Ctrl {
            Ch7Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch7Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Dst(pub u32);
    impl Ch7Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch7Dst {
        #[inline(always)]
        fn default() -> Ch7Dst {
            Ch7Dst(0)
        }
    }
    impl core::fmt::Debug for Ch7Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch7Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Ilsrc(pub u32);
    impl Ch7Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch7Ilsrc {
        #[inline(always)]
        fn default() -> Ch7Ilsrc {
            Ch7Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch7Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch7Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Link(pub u32);
    impl Ch7Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch7LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch7LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch7LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch7Link {
        #[inline(always)]
        fn default() -> Ch7Link {
            Ch7Link(0)
        }
    }
    impl core::fmt::Debug for Ch7Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Loop(pub u32);
    impl Ch7Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch7Loop {
        #[inline(always)]
        fn default() -> Ch7Loop {
            Ch7Loop(0)
        }
    }
    impl core::fmt::Debug for Ch7Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch7Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Src(pub u32);
    impl Ch7Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch7Src {
        #[inline(always)]
        fn default() -> Ch7Src {
            Ch7Src(0)
        }
    }
    impl core::fmt::Debug for Ch7Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch7Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch7Xctrl(pub u32);
    impl Ch7Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch7XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch7XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch7XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch7Xctrl {
        #[inline(always)]
        fn default() -> Ch7Xctrl {
            Ch7Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch7Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch7Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch7Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch7Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Cfg(pub u32);
    impl Ch8Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch8CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch8CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch8CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch8CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch8CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch8CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch8CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch8CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch8CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch8CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch8CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch8CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch8CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch8CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch8CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch8CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch8CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch8CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch8Cfg {
        #[inline(always)]
        fn default() -> Ch8Cfg {
            Ch8Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch8Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Ctrl(pub u32);
    impl Ch8Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch8CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch8CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch8CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch8CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch8CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch8CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch8CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch8CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch8CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch8CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch8CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch8CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch8CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch8CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch8CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch8CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch8CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch8CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch8CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch8CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch8CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch8CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch8CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch8CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch8Ctrl {
        #[inline(always)]
        fn default() -> Ch8Ctrl {
            Ch8Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch8Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Dst(pub u32);
    impl Ch8Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch8Dst {
        #[inline(always)]
        fn default() -> Ch8Dst {
            Ch8Dst(0)
        }
    }
    impl core::fmt::Debug for Ch8Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch8Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Ilsrc(pub u32);
    impl Ch8Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch8Ilsrc {
        #[inline(always)]
        fn default() -> Ch8Ilsrc {
            Ch8Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch8Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch8Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Link(pub u32);
    impl Ch8Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch8LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch8LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch8LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch8Link {
        #[inline(always)]
        fn default() -> Ch8Link {
            Ch8Link(0)
        }
    }
    impl core::fmt::Debug for Ch8Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Loop(pub u32);
    impl Ch8Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch8Loop {
        #[inline(always)]
        fn default() -> Ch8Loop {
            Ch8Loop(0)
        }
    }
    impl core::fmt::Debug for Ch8Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch8Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Src(pub u32);
    impl Ch8Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch8Src {
        #[inline(always)]
        fn default() -> Ch8Src {
            Ch8Src(0)
        }
    }
    impl core::fmt::Debug for Ch8Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch8Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch8Xctrl(pub u32);
    impl Ch8Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch8XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch8XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch8XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch8Xctrl {
        #[inline(always)]
        fn default() -> Ch8Xctrl {
            Ch8Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch8Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch8Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch8Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch8Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Cfg(pub u32);
    impl Ch9Cfg {
        #[doc = "Arbitration Slot Number Select."]
        #[must_use]
        #[inline(always)]
        pub const fn arbslots(&self) -> super::vals::Ch9CfgArbslots {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Ch9CfgArbslots::from_bits(val as u8)
        }
        #[doc = "Arbitration Slot Number Select."]
        #[inline(always)]
        pub const fn set_arbslots(&mut self, val: super::vals::Ch9CfgArbslots) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Source Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn srcincsign(&self) -> super::vals::Ch9CfgSrcincsign {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Ch9CfgSrcincsign::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Sign."]
        #[inline(always)]
        pub const fn set_srcincsign(&mut self, val: super::vals::Ch9CfgSrcincsign) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Destination Address Increment Sign."]
        #[must_use]
        #[inline(always)]
        pub const fn dstincsign(&self) -> super::vals::Ch9CfgDstincsign {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch9CfgDstincsign::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Sign."]
        #[inline(always)]
        pub const fn set_dstincsign(&mut self, val: super::vals::Ch9CfgDstincsign) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Structure Fetch Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn structbusport(&self) -> super::vals::Ch9CfgStructbusport {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ch9CfgStructbusport::from_bits(val as u8)
        }
        #[doc = "Structure Fetch Bus Port."]
        #[inline(always)]
        pub const fn set_structbusport(&mut self, val: super::vals::Ch9CfgStructbusport) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Source Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusport(&self) -> super::vals::Ch9CfgSrcbusport {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Ch9CfgSrcbusport::from_bits(val as u8)
        }
        #[doc = "Source Bus Port."]
        #[inline(always)]
        pub const fn set_srcbusport(&mut self, val: super::vals::Ch9CfgSrcbusport) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Destination Bus Port."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusport(&self) -> super::vals::Ch9CfgDstbusport {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Ch9CfgDstbusport::from_bits(val as u8)
        }
        #[doc = "Destination Bus Port."]
        #[inline(always)]
        pub const fn set_dstbusport(&mut self, val: super::vals::Ch9CfgDstbusport) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Ch9Cfg {
        #[inline(always)]
        fn default() -> Ch9Cfg {
            Ch9Cfg(0)
        }
    }
    impl core::fmt::Debug for Ch9Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Cfg")
                .field("arbslots", &self.arbslots())
                .field("srcincsign", &self.srcincsign())
                .field("dstincsign", &self.dstincsign())
                .field("structbusport", &self.structbusport())
                .field("srcbusport", &self.srcbusport())
                .field("dstbusport", &self.dstbusport())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Cfg {{ arbslots: {:?}, srcincsign: {:?}, dstincsign: {:?}, structbusport: {:?}, srcbusport: {:?}, dstbusport: {:?} }}",
                self.arbslots(),
                self.srcincsign(),
                self.dstincsign(),
                self.structbusport(),
                self.srcbusport(),
                self.dstbusport()
            )
        }
    }
    #[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Ctrl(pub u32);
    impl Ch9Ctrl {
        #[doc = "DMA Structure Type."]
        #[must_use]
        #[inline(always)]
        pub const fn structtype(&self) -> super::vals::Ch9CtrlStructtype {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Ch9CtrlStructtype::from_bits(val as u8)
        }
        #[doc = "DMA Structure Type."]
        #[inline(always)]
        pub const fn set_structtype(&mut self, val: super::vals::Ch9CtrlStructtype) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Extend."]
        #[must_use]
        #[inline(always)]
        pub const fn extend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Extend."]
        #[inline(always)]
        pub const fn set_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Structure DMA Transfer Request."]
        #[must_use]
        #[inline(always)]
        pub const fn structreq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Structure DMA Transfer Request."]
        #[inline(always)]
        pub const fn set_structreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[must_use]
        #[inline(always)]
        pub const fn xfercnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "DMA Unit Data Transfer Count."]
        #[inline(always)]
        pub const fn set_xfercnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Endian Byte Swap."]
        #[must_use]
        #[inline(always)]
        pub const fn byteswap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endian Byte Swap."]
        #[inline(always)]
        pub const fn set_byteswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Block Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn blocksize(&self) -> super::vals::Ch9CtrlBlocksize {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Ch9CtrlBlocksize::from_bits(val as u8)
        }
        #[doc = "Block Transfer Size."]
        #[inline(always)]
        pub const fn set_blocksize(&mut self, val: super::vals::Ch9CtrlBlocksize) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[must_use]
        #[inline(always)]
        pub const fn doneien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Operation Done Interrupt Flag Set."]
        #[inline(always)]
        pub const fn set_doneien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[must_use]
        #[inline(always)]
        pub const fn reqmode(&self) -> super::vals::Ch9CtrlReqmode {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Ch9CtrlReqmode::from_bits(val as u8)
        }
        #[doc = "DMA Request Transfer Mode Select."]
        #[inline(always)]
        pub const fn set_reqmode(&mut self, val: super::vals::Ch9CtrlReqmode) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Decrement Loop Count."]
        #[must_use]
        #[inline(always)]
        pub const fn decloopcnt(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Decrement Loop Count."]
        #[inline(always)]
        pub const fn set_decloopcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ignore Sreq."]
        #[must_use]
        #[inline(always)]
        pub const fn ignoresreq(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ignore Sreq."]
        #[inline(always)]
        pub const fn set_ignoresreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Source Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn srcinc(&self) -> super::vals::Ch9CtrlSrcinc {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Ch9CtrlSrcinc::from_bits(val as u8)
        }
        #[doc = "Source Address Increment Size."]
        #[inline(always)]
        pub const fn set_srcinc(&mut self, val: super::vals::Ch9CtrlSrcinc) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Unit Data Transfer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Ch9CtrlSize {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ch9CtrlSize::from_bits(val as u8)
        }
        #[doc = "Unit Data Transfer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Ch9CtrlSize) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Destination Address Increment Size."]
        #[must_use]
        #[inline(always)]
        pub const fn dstinc(&self) -> super::vals::Ch9CtrlDstinc {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ch9CtrlDstinc::from_bits(val as u8)
        }
        #[doc = "Destination Address Increment Size."]
        #[inline(always)]
        pub const fn set_dstinc(&mut self, val: super::vals::Ch9CtrlDstinc) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Source Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Ch9CtrlSrcmode {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ch9CtrlSrcmode::from_bits(val as u8)
        }
        #[doc = "Source Addressing Mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Ch9CtrlSrcmode) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Destination Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Ch9CtrlDstmode {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Ch9CtrlDstmode::from_bits(val as u8)
        }
        #[doc = "Destination Addressing Mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Ch9CtrlDstmode) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ch9Ctrl {
        #[inline(always)]
        fn default() -> Ch9Ctrl {
            Ch9Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ch9Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Ctrl")
                .field("structtype", &self.structtype())
                .field("extend", &self.extend())
                .field("structreq", &self.structreq())
                .field("xfercnt", &self.xfercnt())
                .field("byteswap", &self.byteswap())
                .field("blocksize", &self.blocksize())
                .field("doneien", &self.doneien())
                .field("reqmode", &self.reqmode())
                .field("decloopcnt", &self.decloopcnt())
                .field("ignoresreq", &self.ignoresreq())
                .field("srcinc", &self.srcinc())
                .field("size", &self.size())
                .field("dstinc", &self.dstinc())
                .field("srcmode", &self.srcmode())
                .field("dstmode", &self.dstmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Ctrl {{ structtype: {:?}, extend: {=bool:?}, structreq: {=bool:?}, xfercnt: {=u16:?}, byteswap: {=bool:?}, blocksize: {:?}, doneien: {=bool:?}, reqmode: {:?}, decloopcnt: {=bool:?}, ignoresreq: {=bool:?}, srcinc: {:?}, size: {:?}, dstinc: {:?}, srcmode: {:?}, dstmode: {:?} }}",
                self.structtype(),
                self.extend(),
                self.structreq(),
                self.xfercnt(),
                self.byteswap(),
                self.blocksize(),
                self.doneien(),
                self.reqmode(),
                self.decloopcnt(),
                self.ignoresreq(),
                self.srcinc(),
                self.size(),
                self.dstinc(),
                self.srcmode(),
                self.dstmode()
            )
        }
    }
    #[doc = "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Dst(pub u32);
    impl Ch9Dst {
        #[doc = "Destination Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Destination Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch9Dst {
        #[inline(always)]
        fn default() -> Ch9Dst {
            Ch9Dst(0)
        }
    }
    impl core::fmt::Debug for Ch9Dst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Dst").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Dst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch9Dst {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Ilsrc(pub u32);
    impl Ch9Ilsrc {
        #[doc = "Interleave Source Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interleave Source Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch9Ilsrc {
        #[inline(always)]
        fn default() -> Ch9Ilsrc {
            Ch9Ilsrc(0)
        }
    }
    impl core::fmt::Debug for Ch9Ilsrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Ilsrc").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Ilsrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch9Ilsrc {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Link(pub u32);
    impl Ch9Link {
        #[doc = "Link Structure Addressing Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn linkmode(&self) -> super::vals::Ch9LinkLinkmode {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Ch9LinkLinkmode::from_bits(val as u8)
        }
        #[doc = "Link Structure Addressing Mode."]
        #[inline(always)]
        pub const fn set_linkmode(&mut self, val: super::vals::Ch9LinkLinkmode) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Link Next Structure."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link Next Structure."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Link Structure Address."]
        #[must_use]
        #[inline(always)]
        pub const fn linkaddr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Link Structure Address."]
        #[inline(always)]
        pub const fn set_linkaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Ch9Link {
        #[inline(always)]
        fn default() -> Ch9Link {
            Ch9Link(0)
        }
    }
    impl core::fmt::Debug for Ch9Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Link")
                .field("linkmode", &self.linkmode())
                .field("link", &self.link())
                .field("linkaddr", &self.linkaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Link {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Link {{ linkmode: {:?}, link: {=bool:?}, linkaddr: {=u32:?} }}",
                self.linkmode(),
                self.link(),
                self.linkaddr()
            )
        }
    }
    #[doc = "Channel Loop Counter Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Loop(pub u32);
    impl Ch9Loop {
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn loopcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Linked Structure Sequence Loop Counter."]
        #[inline(always)]
        pub const fn set_loopcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ch9Loop {
        #[inline(always)]
        fn default() -> Ch9Loop {
            Ch9Loop(0)
        }
    }
    impl core::fmt::Debug for Ch9Loop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Loop").field("loopcnt", &self.loopcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Loop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch9Loop {{ loopcnt: {=u8:?} }}", self.loopcnt())
        }
    }
    #[doc = "Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Src(pub u32);
    impl Ch9Src {
        #[doc = "Source Data Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Source Data Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ch9Src {
        #[inline(always)]
        fn default() -> Ch9Src {
            Ch9Src(0)
        }
    }
    impl core::fmt::Debug for Ch9Src {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Src").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Src {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ch9Src {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ch9Xctrl(pub u32);
    impl Ch9Xctrl {
        #[doc = "Destination Interleave."]
        #[must_use]
        #[inline(always)]
        pub const fn dstilen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Destination Interleave."]
        #[inline(always)]
        pub const fn set_dstilen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interleave Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ilmode(&self) -> super::vals::Ch9XctrlIlmode {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Ch9XctrlIlmode::from_bits(val as u8)
        }
        #[doc = "Interleave Mode."]
        #[inline(always)]
        pub const fn set_ilmode(&mut self, val: super::vals::Ch9XctrlIlmode) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Allow AHB buffering."]
        #[must_use]
        #[inline(always)]
        pub const fn bufferable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Allow AHB buffering."]
        #[inline(always)]
        pub const fn set_bufferable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ch9Xctrl {
        #[inline(always)]
        fn default() -> Ch9Xctrl {
            Ch9Xctrl(0)
        }
    }
    impl core::fmt::Debug for Ch9Xctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ch9Xctrl")
                .field("dstilen", &self.dstilen())
                .field("ilmode", &self.ilmode())
                .field("bufferable", &self.bufferable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ch9Xctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ch9Xctrl {{ dstilen: {=bool:?}, ilmode: {:?}, bufferable: {=bool:?} }}",
                self.dstilen(),
                self.ilmode(),
                self.bufferable()
            )
        }
    }
    #[doc = "Channel Busy Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chbusy(pub u32);
    impl Chbusy {
        #[doc = "Channels Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channels Busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Chbusy {
        #[inline(always)]
        fn default() -> Chbusy {
            Chbusy(0)
        }
    }
    impl core::fmt::Debug for Chbusy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chbusy").field("busy", &self.busy()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chbusy {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chbusy {{ busy: {=u16:?} }}", self.busy())
        }
    }
    #[doc = "Channel Disable Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chdis(pub u32);
    impl Chdis {
        #[doc = "DMA Channel disable."]
        #[must_use]
        #[inline(always)]
        pub const fn chdis(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Channel disable."]
        #[inline(always)]
        pub const fn set_chdis(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Chdis {
        #[inline(always)]
        fn default() -> Chdis {
            Chdis(0)
        }
    }
    impl core::fmt::Debug for Chdis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chdis").field("chdis", &self.chdis()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chdis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chdis {{ chdis: {=u16:?} }}", self.chdis())
        }
    }
    #[doc = "Channel Linking Done Status Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chdone(pub u32);
    impl Chdone {
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Channel Linking Done Status flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chdone15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Linking Done Status flag."]
        #[inline(always)]
        pub const fn set_chdone15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Chdone {
        #[inline(always)]
        fn default() -> Chdone {
            Chdone(0)
        }
    }
    impl core::fmt::Debug for Chdone {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chdone")
                .field("chdone0", &self.chdone0())
                .field("chdone1", &self.chdone1())
                .field("chdone2", &self.chdone2())
                .field("chdone3", &self.chdone3())
                .field("chdone4", &self.chdone4())
                .field("chdone5", &self.chdone5())
                .field("chdone6", &self.chdone6())
                .field("chdone7", &self.chdone7())
                .field("chdone8", &self.chdone8())
                .field("chdone9", &self.chdone9())
                .field("chdone10", &self.chdone10())
                .field("chdone11", &self.chdone11())
                .field("chdone12", &self.chdone12())
                .field("chdone13", &self.chdone13())
                .field("chdone14", &self.chdone14())
                .field("chdone15", &self.chdone15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chdone {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Chdone {{ chdone0: {=bool:?}, chdone1: {=bool:?}, chdone2: {=bool:?}, chdone3: {=bool:?}, chdone4: {=bool:?}, chdone5: {=bool:?}, chdone6: {=bool:?}, chdone7: {=bool:?}, chdone8: {=bool:?}, chdone9: {=bool:?}, chdone10: {=bool:?}, chdone11: {=bool:?}, chdone12: {=bool:?}, chdone13: {=bool:?}, chdone14: {=bool:?}, chdone15: {=bool:?} }}",
                self.chdone0(),
                self.chdone1(),
                self.chdone2(),
                self.chdone3(),
                self.chdone4(),
                self.chdone5(),
                self.chdone6(),
                self.chdone7(),
                self.chdone8(),
                self.chdone9(),
                self.chdone10(),
                self.chdone11(),
                self.chdone12(),
                self.chdone13(),
                self.chdone14(),
                self.chdone15()
            )
        }
    }
    #[doc = "Channel Enable Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chen(pub u32);
    impl Chen {
        #[doc = "Channel Enables."]
        #[must_use]
        #[inline(always)]
        pub const fn chen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel Enables."]
        #[inline(always)]
        pub const fn set_chen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Chen {
        #[inline(always)]
        fn default() -> Chen {
            Chen(0)
        }
    }
    impl core::fmt::Debug for Chen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chen").field("chen", &self.chen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chen {{ chen: {=u16:?} }}", self.chen())
        }
    }
    #[doc = "Channel Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chstatus(pub u32);
    impl Chstatus {
        #[doc = "DMA Channel Status."]
        #[must_use]
        #[inline(always)]
        pub const fn chstatus(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Channel Status."]
        #[inline(always)]
        pub const fn set_chstatus(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Chstatus {
        #[inline(always)]
        fn default() -> Chstatus {
            Chstatus(0)
        }
    }
    impl core::fmt::Debug for Chstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Chstatus").field("chstatus", &self.chstatus()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Chstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Chstatus {{ chstatus: {=u16:?} }}", self.chstatus())
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Number of Fixed Priority Channels."]
        #[must_use]
        #[inline(always)]
        pub const fn numfixed(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of Fixed Priority Channels."]
        #[inline(always)]
        pub const fn set_numfixed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
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
            f.debug_struct("Ctrl").field("numfixed", &self.numfixed()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ctrl {{ numfixed: {=u8:?} }}", self.numfixed())
        }
    }
    #[doc = "Channel Debug Halt Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbghalt(pub u32);
    impl Dbghalt {
        #[doc = "DMA Debug Halt."]
        #[must_use]
        #[inline(always)]
        pub const fn dbghalt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Debug Halt."]
        #[inline(always)]
        pub const fn set_dbghalt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dbghalt {
        #[inline(always)]
        fn default() -> Dbghalt {
            Dbghalt(0)
        }
    }
    impl core::fmt::Debug for Dbghalt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbghalt").field("dbghalt", &self.dbghalt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbghalt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbghalt {{ dbghalt: {=u16:?} }}", self.dbghalt())
        }
    }
    #[doc = "Module enable disable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Module Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Module Enable."]
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
    #[doc = "Done Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn done15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_done15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Error Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn error(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Error Interrupt Enable."]
        #[inline(always)]
        pub const fn set_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("done0", &self.done0())
                .field("done1", &self.done1())
                .field("done2", &self.done2())
                .field("done3", &self.done3())
                .field("done4", &self.done4())
                .field("done5", &self.done5())
                .field("done6", &self.done6())
                .field("done7", &self.done7())
                .field("done8", &self.done8())
                .field("done9", &self.done9())
                .field("done10", &self.done10())
                .field("done11", &self.done11())
                .field("done12", &self.done12())
                .field("done13", &self.done13())
                .field("done14", &self.done14())
                .field("done15", &self.done15())
                .field("error", &self.error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ done0: {=bool:?}, done1: {=bool:?}, done2: {=bool:?}, done3: {=bool:?}, done4: {=bool:?}, done5: {=bool:?}, done6: {=bool:?}, done7: {=bool:?}, done8: {=bool:?}, done9: {=bool:?}, done10: {=bool:?}, done11: {=bool:?}, done12: {=bool:?}, done13: {=bool:?}, done14: {=bool:?}, done15: {=bool:?}, error: {=bool:?} }}",
                self.done0(),
                self.done1(),
                self.done2(),
                self.done3(),
                self.done4(),
                self.done5(),
                self.done6(),
                self.done7(),
                self.done8(),
                self.done9(),
                self.done10(),
                self.done11(),
                self.done12(),
                self.done13(),
                self.done14(),
                self.done15(),
                self.error()
            )
        }
    }
    #[doc = "Interrupt Flag Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn done15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_done15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Error Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn error(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Error Flag."]
        #[inline(always)]
        pub const fn set_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("done0", &self.done0())
                .field("done1", &self.done1())
                .field("done2", &self.done2())
                .field("done3", &self.done3())
                .field("done4", &self.done4())
                .field("done5", &self.done5())
                .field("done6", &self.done6())
                .field("done7", &self.done7())
                .field("done8", &self.done8())
                .field("done9", &self.done9())
                .field("done10", &self.done10())
                .field("done11", &self.done11())
                .field("done12", &self.done12())
                .field("done13", &self.done13())
                .field("done14", &self.done14())
                .field("done15", &self.done15())
                .field("error", &self.error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ done0: {=bool:?}, done1: {=bool:?}, done2: {=bool:?}, done3: {=bool:?}, done4: {=bool:?}, done5: {=bool:?}, done6: {=bool:?}, done7: {=bool:?}, done8: {=bool:?}, done9: {=bool:?}, done10: {=bool:?}, done11: {=bool:?}, done12: {=bool:?}, done13: {=bool:?}, done14: {=bool:?}, done15: {=bool:?}, error: {=bool:?} }}",
                self.done0(),
                self.done1(),
                self.done2(),
                self.done3(),
                self.done4(),
                self.done5(),
                self.done6(),
                self.done7(),
                self.done8(),
                self.done9(),
                self.done10(),
                self.done11(),
                self.done12(),
                self.done13(),
                self.done14(),
                self.done15(),
                self.error()
            )
        }
    }
    #[doc = "IP version register."]
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
    #[doc = "Channel Link Load Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Linkload(pub u32);
    impl Linkload {
        #[doc = "DMA Link Loads."]
        #[must_use]
        #[inline(always)]
        pub const fn linkload(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Link Loads."]
        #[inline(always)]
        pub const fn set_linkload(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Linkload {
        #[inline(always)]
        fn default() -> Linkload {
            Linkload(0)
        }
    }
    impl core::fmt::Debug for Linkload {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Linkload").field("linkload", &self.linkload()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Linkload {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Linkload {{ linkload: {=u16:?} }}", self.linkload())
        }
    }
    #[doc = "Channel Request Clear Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reqclear(pub u32);
    impl Reqclear {
        #[doc = "DMA Request Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn reqclear(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Request Clear."]
        #[inline(always)]
        pub const fn set_reqclear(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Reqclear {
        #[inline(always)]
        fn default() -> Reqclear {
            Reqclear(0)
        }
    }
    impl core::fmt::Debug for Reqclear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Reqclear").field("reqclear", &self.reqclear()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reqclear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reqclear {{ reqclear: {=u16:?} }}", self.reqclear())
        }
    }
    #[doc = "Channel Request Disable Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reqdis(pub u32);
    impl Reqdis {
        #[doc = "DMA Request Disables."]
        #[must_use]
        #[inline(always)]
        pub const fn reqdis(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Request Disables."]
        #[inline(always)]
        pub const fn set_reqdis(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Reqdis {
        #[inline(always)]
        fn default() -> Reqdis {
            Reqdis(0)
        }
    }
    impl core::fmt::Debug for Reqdis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Reqdis").field("reqdis", &self.reqdis()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reqdis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reqdis {{ reqdis: {=u16:?} }}", self.reqdis())
        }
    }
    #[doc = "Channel Requests Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Reqpend(pub u32);
    impl Reqpend {
        #[doc = "DMA Requests Pending."]
        #[must_use]
        #[inline(always)]
        pub const fn reqpend(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA Requests Pending."]
        #[inline(always)]
        pub const fn set_reqpend(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Reqpend {
        #[inline(always)]
        fn default() -> Reqpend {
            Reqpend(0)
        }
    }
    impl core::fmt::Debug for Reqpend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Reqpend").field("reqpend", &self.reqpend()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Reqpend {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Reqpend {{ reqpend: {=u16:?} }}", self.reqpend())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Any DMA Channel Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn anybusy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Any DMA Channel Busy."]
        #[inline(always)]
        pub const fn set_anybusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Any DMA Channel Request Pending."]
        #[must_use]
        #[inline(always)]
        pub const fn anyreq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Any DMA Channel Request Pending."]
        #[inline(always)]
        pub const fn set_anyreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Granted Channel Number."]
        #[must_use]
        #[inline(always)]
        pub const fn chgrant(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "Granted Channel Number."]
        #[inline(always)]
        pub const fn set_chgrant(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Errant Channel Number."]
        #[must_use]
        #[inline(always)]
        pub const fn cherror(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Errant Channel Number."]
        #[inline(always)]
        pub const fn set_cherror(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Number of Channels."]
        #[must_use]
        #[inline(always)]
        pub const fn chnum(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of Channels."]
        #[inline(always)]
        pub const fn set_chnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
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
                .field("anybusy", &self.anybusy())
                .field("anyreq", &self.anyreq())
                .field("chgrant", &self.chgrant())
                .field("cherror", &self.cherror())
                .field("chnum", &self.chnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ anybusy: {=bool:?}, anyreq: {=bool:?}, chgrant: {=u8:?}, cherror: {=u8:?}, chnum: {=u8:?} }}",
                self.anybusy(),
                self.anyreq(),
                self.chgrant(),
                self.cherror(),
                self.chnum()
            )
        }
    }
    #[doc = "Channel Software Transfer Request (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swreq(pub u32);
    impl Swreq {
        #[doc = "Software Transfer Requests."]
        #[must_use]
        #[inline(always)]
        pub const fn swreq(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Software Transfer Requests."]
        #[inline(always)]
        pub const fn set_swreq(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Swreq {
        #[inline(always)]
        fn default() -> Swreq {
            Swreq(0)
        }
    }
    impl core::fmt::Debug for Swreq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swreq").field("swreq", &self.swreq()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swreq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Swreq {{ swreq: {=u16:?} }}", self.swreq())
        }
    }
    #[doc = "Software Reset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swrst(pub u32);
    impl Swrst {
        #[doc = "Software Reset Command."]
        #[must_use]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset Command."]
        #[inline(always)]
        pub const fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software Reset Busy Status."]
        #[must_use]
        #[inline(always)]
        pub const fn resetting(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset Busy Status."]
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
    #[doc = "Sync HW trigger enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synchwen(pub u32);
    impl Synchwen {
        #[doc = "Hardware Sync Trigger Set Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn syncseten(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Hardware Sync Trigger Set Enable."]
        #[inline(always)]
        pub const fn set_syncseten(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Hardware Sync Trigger Clear Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn syncclren(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Hardware Sync Trigger Clear Enable."]
        #[inline(always)]
        pub const fn set_syncclren(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Synchwen {
        #[inline(always)]
        fn default() -> Synchwen {
            Synchwen(0)
        }
    }
    impl core::fmt::Debug for Synchwen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synchwen")
                .field("syncseten", &self.syncseten())
                .field("syncclren", &self.syncclren())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synchwen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synchwen {{ syncseten: {=u8:?}, syncclren: {=u8:?} }}",
                self.syncseten(),
                self.syncclren()
            )
        }
    }
    #[doc = "Sync HW trigger selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Synchwsel(pub u32);
    impl Synchwsel {
        #[doc = "Hardware Sync Trigger Set Edge Select."]
        #[must_use]
        #[inline(always)]
        pub const fn syncsetedge(&self) -> super::vals::Syncsetedge {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Syncsetedge::from_bits(val as u8)
        }
        #[doc = "Hardware Sync Trigger Set Edge Select."]
        #[inline(always)]
        pub const fn set_syncsetedge(&mut self, val: super::vals::Syncsetedge) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Hardware Sync Trigger Clear Edge Select."]
        #[must_use]
        #[inline(always)]
        pub const fn syncclredge(&self) -> super::vals::Syncclredge {
            let val = (self.0 >> 16usize) & 0xff;
            super::vals::Syncclredge::from_bits(val as u8)
        }
        #[doc = "Hardware Sync Trigger Clear Edge Select."]
        #[inline(always)]
        pub const fn set_syncclredge(&mut self, val: super::vals::Syncclredge) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Synchwsel {
        #[inline(always)]
        fn default() -> Synchwsel {
            Synchwsel(0)
        }
    }
    impl core::fmt::Debug for Synchwsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Synchwsel")
                .field("syncsetedge", &self.syncsetedge())
                .field("syncclredge", &self.syncclredge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Synchwsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Synchwsel {{ syncsetedge: {:?}, syncclredge: {:?} }}",
                self.syncsetedge(),
                self.syncclredge()
            )
        }
    }
    #[doc = "Sync Trigger Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncstatus(pub u32);
    impl Syncstatus {
        #[doc = "sync trig status."]
        #[must_use]
        #[inline(always)]
        pub const fn synctrig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "sync trig status."]
        #[inline(always)]
        pub const fn set_synctrig(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Syncstatus {
        #[inline(always)]
        fn default() -> Syncstatus {
            Syncstatus(0)
        }
    }
    impl core::fmt::Debug for Syncstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Syncstatus")
                .field("synctrig", &self.synctrig())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Syncstatus {{ synctrig: {=u8:?} }}", self.synctrig())
        }
    }
    #[doc = "Sync Trig Sw Clear register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncswclr(pub u32);
    impl Syncswclr {
        #[doc = "DMA SYNC Software Trigger Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn syncswclr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DMA SYNC Software Trigger Clear."]
        #[inline(always)]
        pub const fn set_syncswclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Syncswclr {
        #[inline(always)]
        fn default() -> Syncswclr {
            Syncswclr(0)
        }
    }
    impl core::fmt::Debug for Syncswclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Syncswclr")
                .field("syncswclr", &self.syncswclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncswclr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Syncswclr {{ syncswclr: {=u8:?} }}", self.syncswclr())
        }
    }
    #[doc = "Sync Trig Sw Set Register (Writes will only take effect when EN=1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syncswset(pub u32);
    impl Syncswset {
        #[doc = "DMA SYNC Software Trigger Set."]
        #[must_use]
        #[inline(always)]
        pub const fn syncswset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "DMA SYNC Software Trigger Set."]
        #[inline(always)]
        pub const fn set_syncswset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Syncswset {
        #[inline(always)]
        fn default() -> Syncswset {
            Syncswset(0)
        }
    }
    impl core::fmt::Debug for Syncswset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Syncswset")
                .field("syncswset", &self.syncswset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncswset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Syncswset {{ syncswset: {=u8:?} }}", self.syncswset())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch0CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch0CfgArbslots {
            Ch0CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch0CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch0CfgArbslots) -> u8 {
            Ch0CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch0CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch0CfgDstbusport {
            Ch0CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch0CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch0CfgDstbusport) -> u8 {
            Ch0CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch0CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch0CfgDstincsign {
            Ch0CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch0CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch0CfgDstincsign) -> u8 {
            Ch0CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch0CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch0CfgSrcbusport {
            Ch0CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch0CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch0CfgSrcbusport) -> u8 {
            Ch0CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch0CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch0CfgSrcincsign {
            Ch0CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch0CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch0CfgSrcincsign) -> u8 {
            Ch0CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch0CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch0CfgStructbusport {
            Ch0CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch0CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch0CfgStructbusport) -> u8 {
            Ch0CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch0CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlBlocksize {
            Ch0CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch0CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlBlocksize) -> u8 {
            Ch0CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch0CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlDstinc {
            Ch0CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch0CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlDstinc) -> u8 {
            Ch0CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch0CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlDstmode {
            Ch0CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch0CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlDstmode) -> u8 {
            Ch0CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch0CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlReqmode {
            Ch0CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch0CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlReqmode) -> u8 {
            Ch0CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch0CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlSize {
            Ch0CtrlSize::from_bits(val)
        }
    }
    impl From<Ch0CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlSize) -> u8 {
            Ch0CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch0CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlSrcinc {
            Ch0CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch0CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlSrcinc) -> u8 {
            Ch0CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch0CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlSrcmode {
            Ch0CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch0CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlSrcmode) -> u8 {
            Ch0CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch0CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch0CtrlStructtype {
            Ch0CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch0CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch0CtrlStructtype) -> u8 {
            Ch0CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch0LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch0LinkLinkmode {
            Ch0LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch0LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch0LinkLinkmode) -> u8 {
            Ch0LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch0XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch0XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch0XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch0XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch0XctrlIlmode {
            Ch0XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch0XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch0XctrlIlmode) -> u8 {
            Ch0XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch10CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch10CfgArbslots {
            Ch10CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch10CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch10CfgArbslots) -> u8 {
            Ch10CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch10CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch10CfgDstbusport {
            Ch10CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch10CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch10CfgDstbusport) -> u8 {
            Ch10CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch10CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch10CfgDstincsign {
            Ch10CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch10CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch10CfgDstincsign) -> u8 {
            Ch10CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch10CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch10CfgSrcbusport {
            Ch10CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch10CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch10CfgSrcbusport) -> u8 {
            Ch10CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch10CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch10CfgSrcincsign {
            Ch10CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch10CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch10CfgSrcincsign) -> u8 {
            Ch10CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch10CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch10CfgStructbusport {
            Ch10CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch10CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch10CfgStructbusport) -> u8 {
            Ch10CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch10CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlBlocksize {
            Ch10CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch10CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlBlocksize) -> u8 {
            Ch10CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch10CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlDstinc {
            Ch10CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch10CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlDstinc) -> u8 {
            Ch10CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch10CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlDstmode {
            Ch10CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch10CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlDstmode) -> u8 {
            Ch10CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch10CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlReqmode {
            Ch10CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch10CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlReqmode) -> u8 {
            Ch10CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch10CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlSize {
            Ch10CtrlSize::from_bits(val)
        }
    }
    impl From<Ch10CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlSize) -> u8 {
            Ch10CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch10CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlSrcinc {
            Ch10CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch10CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlSrcinc) -> u8 {
            Ch10CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch10CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlSrcmode {
            Ch10CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch10CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlSrcmode) -> u8 {
            Ch10CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch10CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch10CtrlStructtype {
            Ch10CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch10CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch10CtrlStructtype) -> u8 {
            Ch10CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch10LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch10LinkLinkmode {
            Ch10LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch10LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch10LinkLinkmode) -> u8 {
            Ch10LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch10XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch10XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch10XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch10XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch10XctrlIlmode {
            Ch10XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch10XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch10XctrlIlmode) -> u8 {
            Ch10XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch11CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch11CfgArbslots {
            Ch11CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch11CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch11CfgArbslots) -> u8 {
            Ch11CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch11CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch11CfgDstbusport {
            Ch11CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch11CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch11CfgDstbusport) -> u8 {
            Ch11CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch11CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch11CfgDstincsign {
            Ch11CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch11CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch11CfgDstincsign) -> u8 {
            Ch11CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch11CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch11CfgSrcbusport {
            Ch11CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch11CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch11CfgSrcbusport) -> u8 {
            Ch11CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch11CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch11CfgSrcincsign {
            Ch11CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch11CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch11CfgSrcincsign) -> u8 {
            Ch11CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch11CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch11CfgStructbusport {
            Ch11CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch11CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch11CfgStructbusport) -> u8 {
            Ch11CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch11CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlBlocksize {
            Ch11CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch11CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlBlocksize) -> u8 {
            Ch11CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch11CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlDstinc {
            Ch11CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch11CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlDstinc) -> u8 {
            Ch11CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch11CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlDstmode {
            Ch11CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch11CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlDstmode) -> u8 {
            Ch11CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch11CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlReqmode {
            Ch11CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch11CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlReqmode) -> u8 {
            Ch11CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch11CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlSize {
            Ch11CtrlSize::from_bits(val)
        }
    }
    impl From<Ch11CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlSize) -> u8 {
            Ch11CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch11CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlSrcinc {
            Ch11CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch11CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlSrcinc) -> u8 {
            Ch11CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch11CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlSrcmode {
            Ch11CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch11CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlSrcmode) -> u8 {
            Ch11CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch11CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch11CtrlStructtype {
            Ch11CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch11CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch11CtrlStructtype) -> u8 {
            Ch11CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch11LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch11LinkLinkmode {
            Ch11LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch11LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch11LinkLinkmode) -> u8 {
            Ch11LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch11XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch11XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch11XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch11XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch11XctrlIlmode {
            Ch11XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch11XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch11XctrlIlmode) -> u8 {
            Ch11XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch12CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch12CfgArbslots {
            Ch12CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch12CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch12CfgArbslots) -> u8 {
            Ch12CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch12CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch12CfgDstbusport {
            Ch12CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch12CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch12CfgDstbusport) -> u8 {
            Ch12CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch12CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch12CfgDstincsign {
            Ch12CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch12CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch12CfgDstincsign) -> u8 {
            Ch12CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch12CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch12CfgSrcbusport {
            Ch12CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch12CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch12CfgSrcbusport) -> u8 {
            Ch12CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch12CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch12CfgSrcincsign {
            Ch12CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch12CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch12CfgSrcincsign) -> u8 {
            Ch12CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch12CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch12CfgStructbusport {
            Ch12CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch12CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch12CfgStructbusport) -> u8 {
            Ch12CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch12CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlBlocksize {
            Ch12CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch12CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlBlocksize) -> u8 {
            Ch12CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch12CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlDstinc {
            Ch12CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch12CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlDstinc) -> u8 {
            Ch12CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch12CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlDstmode {
            Ch12CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch12CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlDstmode) -> u8 {
            Ch12CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch12CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlReqmode {
            Ch12CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch12CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlReqmode) -> u8 {
            Ch12CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch12CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlSize {
            Ch12CtrlSize::from_bits(val)
        }
    }
    impl From<Ch12CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlSize) -> u8 {
            Ch12CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch12CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlSrcinc {
            Ch12CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch12CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlSrcinc) -> u8 {
            Ch12CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch12CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlSrcmode {
            Ch12CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch12CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlSrcmode) -> u8 {
            Ch12CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch12CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch12CtrlStructtype {
            Ch12CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch12CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch12CtrlStructtype) -> u8 {
            Ch12CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch12LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch12LinkLinkmode {
            Ch12LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch12LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch12LinkLinkmode) -> u8 {
            Ch12LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch12XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch12XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch12XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch12XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch12XctrlIlmode {
            Ch12XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch12XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch12XctrlIlmode) -> u8 {
            Ch12XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch13CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch13CfgArbslots {
            Ch13CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch13CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch13CfgArbslots) -> u8 {
            Ch13CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch13CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch13CfgDstbusport {
            Ch13CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch13CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch13CfgDstbusport) -> u8 {
            Ch13CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch13CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch13CfgDstincsign {
            Ch13CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch13CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch13CfgDstincsign) -> u8 {
            Ch13CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch13CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch13CfgSrcbusport {
            Ch13CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch13CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch13CfgSrcbusport) -> u8 {
            Ch13CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch13CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch13CfgSrcincsign {
            Ch13CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch13CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch13CfgSrcincsign) -> u8 {
            Ch13CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch13CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch13CfgStructbusport {
            Ch13CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch13CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch13CfgStructbusport) -> u8 {
            Ch13CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch13CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlBlocksize {
            Ch13CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch13CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlBlocksize) -> u8 {
            Ch13CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch13CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlDstinc {
            Ch13CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch13CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlDstinc) -> u8 {
            Ch13CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch13CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlDstmode {
            Ch13CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch13CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlDstmode) -> u8 {
            Ch13CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch13CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlReqmode {
            Ch13CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch13CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlReqmode) -> u8 {
            Ch13CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch13CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlSize {
            Ch13CtrlSize::from_bits(val)
        }
    }
    impl From<Ch13CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlSize) -> u8 {
            Ch13CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch13CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlSrcinc {
            Ch13CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch13CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlSrcinc) -> u8 {
            Ch13CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch13CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlSrcmode {
            Ch13CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch13CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlSrcmode) -> u8 {
            Ch13CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch13CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch13CtrlStructtype {
            Ch13CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch13CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch13CtrlStructtype) -> u8 {
            Ch13CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch13LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch13LinkLinkmode {
            Ch13LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch13LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch13LinkLinkmode) -> u8 {
            Ch13LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch13XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch13XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch13XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch13XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch13XctrlIlmode {
            Ch13XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch13XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch13XctrlIlmode) -> u8 {
            Ch13XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch14CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch14CfgArbslots {
            Ch14CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch14CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch14CfgArbslots) -> u8 {
            Ch14CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch14CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch14CfgDstbusport {
            Ch14CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch14CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch14CfgDstbusport) -> u8 {
            Ch14CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch14CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch14CfgDstincsign {
            Ch14CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch14CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch14CfgDstincsign) -> u8 {
            Ch14CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch14CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch14CfgSrcbusport {
            Ch14CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch14CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch14CfgSrcbusport) -> u8 {
            Ch14CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch14CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch14CfgSrcincsign {
            Ch14CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch14CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch14CfgSrcincsign) -> u8 {
            Ch14CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch14CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch14CfgStructbusport {
            Ch14CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch14CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch14CfgStructbusport) -> u8 {
            Ch14CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch14CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlBlocksize {
            Ch14CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch14CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlBlocksize) -> u8 {
            Ch14CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch14CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlDstinc {
            Ch14CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch14CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlDstinc) -> u8 {
            Ch14CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch14CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlDstmode {
            Ch14CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch14CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlDstmode) -> u8 {
            Ch14CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch14CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlReqmode {
            Ch14CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch14CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlReqmode) -> u8 {
            Ch14CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch14CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlSize {
            Ch14CtrlSize::from_bits(val)
        }
    }
    impl From<Ch14CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlSize) -> u8 {
            Ch14CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch14CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlSrcinc {
            Ch14CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch14CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlSrcinc) -> u8 {
            Ch14CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch14CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlSrcmode {
            Ch14CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch14CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlSrcmode) -> u8 {
            Ch14CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch14CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch14CtrlStructtype {
            Ch14CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch14CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch14CtrlStructtype) -> u8 {
            Ch14CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch14LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch14LinkLinkmode {
            Ch14LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch14LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch14LinkLinkmode) -> u8 {
            Ch14LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch14XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch14XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch14XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch14XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch14XctrlIlmode {
            Ch14XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch14XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch14XctrlIlmode) -> u8 {
            Ch14XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch15CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch15CfgArbslots {
            Ch15CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch15CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch15CfgArbslots) -> u8 {
            Ch15CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch15CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch15CfgDstbusport {
            Ch15CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch15CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch15CfgDstbusport) -> u8 {
            Ch15CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch15CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch15CfgDstincsign {
            Ch15CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch15CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch15CfgDstincsign) -> u8 {
            Ch15CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch15CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch15CfgSrcbusport {
            Ch15CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch15CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch15CfgSrcbusport) -> u8 {
            Ch15CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch15CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch15CfgSrcincsign {
            Ch15CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch15CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch15CfgSrcincsign) -> u8 {
            Ch15CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch15CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch15CfgStructbusport {
            Ch15CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch15CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch15CfgStructbusport) -> u8 {
            Ch15CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch15CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlBlocksize {
            Ch15CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch15CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlBlocksize) -> u8 {
            Ch15CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch15CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlDstinc {
            Ch15CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch15CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlDstinc) -> u8 {
            Ch15CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch15CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlDstmode {
            Ch15CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch15CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlDstmode) -> u8 {
            Ch15CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch15CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlReqmode {
            Ch15CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch15CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlReqmode) -> u8 {
            Ch15CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch15CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlSize {
            Ch15CtrlSize::from_bits(val)
        }
    }
    impl From<Ch15CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlSize) -> u8 {
            Ch15CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch15CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlSrcinc {
            Ch15CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch15CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlSrcinc) -> u8 {
            Ch15CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch15CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlSrcmode {
            Ch15CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch15CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlSrcmode) -> u8 {
            Ch15CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch15CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch15CtrlStructtype {
            Ch15CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch15CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch15CtrlStructtype) -> u8 {
            Ch15CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch15LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch15LinkLinkmode {
            Ch15LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch15LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch15LinkLinkmode) -> u8 {
            Ch15LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch15XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch15XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch15XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch15XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch15XctrlIlmode {
            Ch15XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch15XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch15XctrlIlmode) -> u8 {
            Ch15XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch1CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch1CfgArbslots {
            Ch1CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch1CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch1CfgArbslots) -> u8 {
            Ch1CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch1CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch1CfgDstbusport {
            Ch1CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch1CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch1CfgDstbusport) -> u8 {
            Ch1CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch1CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch1CfgDstincsign {
            Ch1CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch1CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch1CfgDstincsign) -> u8 {
            Ch1CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch1CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch1CfgSrcbusport {
            Ch1CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch1CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch1CfgSrcbusport) -> u8 {
            Ch1CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch1CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch1CfgSrcincsign {
            Ch1CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch1CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch1CfgSrcincsign) -> u8 {
            Ch1CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch1CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch1CfgStructbusport {
            Ch1CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch1CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch1CfgStructbusport) -> u8 {
            Ch1CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch1CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlBlocksize {
            Ch1CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch1CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlBlocksize) -> u8 {
            Ch1CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch1CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlDstinc {
            Ch1CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch1CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlDstinc) -> u8 {
            Ch1CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch1CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlDstmode {
            Ch1CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch1CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlDstmode) -> u8 {
            Ch1CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch1CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlReqmode {
            Ch1CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch1CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlReqmode) -> u8 {
            Ch1CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch1CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlSize {
            Ch1CtrlSize::from_bits(val)
        }
    }
    impl From<Ch1CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlSize) -> u8 {
            Ch1CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch1CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlSrcinc {
            Ch1CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch1CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlSrcinc) -> u8 {
            Ch1CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch1CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlSrcmode {
            Ch1CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch1CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlSrcmode) -> u8 {
            Ch1CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch1CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch1CtrlStructtype {
            Ch1CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch1CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch1CtrlStructtype) -> u8 {
            Ch1CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch1LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch1LinkLinkmode {
            Ch1LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch1LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch1LinkLinkmode) -> u8 {
            Ch1LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch1XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch1XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch1XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch1XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch1XctrlIlmode {
            Ch1XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch1XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch1XctrlIlmode) -> u8 {
            Ch1XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch2CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch2CfgArbslots {
            Ch2CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch2CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch2CfgArbslots) -> u8 {
            Ch2CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch2CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch2CfgDstbusport {
            Ch2CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch2CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch2CfgDstbusport) -> u8 {
            Ch2CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch2CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch2CfgDstincsign {
            Ch2CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch2CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch2CfgDstincsign) -> u8 {
            Ch2CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch2CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch2CfgSrcbusport {
            Ch2CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch2CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch2CfgSrcbusport) -> u8 {
            Ch2CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch2CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch2CfgSrcincsign {
            Ch2CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch2CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch2CfgSrcincsign) -> u8 {
            Ch2CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch2CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch2CfgStructbusport {
            Ch2CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch2CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch2CfgStructbusport) -> u8 {
            Ch2CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch2CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlBlocksize {
            Ch2CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch2CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlBlocksize) -> u8 {
            Ch2CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch2CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlDstinc {
            Ch2CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch2CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlDstinc) -> u8 {
            Ch2CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch2CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlDstmode {
            Ch2CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch2CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlDstmode) -> u8 {
            Ch2CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch2CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlReqmode {
            Ch2CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch2CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlReqmode) -> u8 {
            Ch2CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch2CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlSize {
            Ch2CtrlSize::from_bits(val)
        }
    }
    impl From<Ch2CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlSize) -> u8 {
            Ch2CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch2CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlSrcinc {
            Ch2CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch2CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlSrcinc) -> u8 {
            Ch2CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch2CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlSrcmode {
            Ch2CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch2CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlSrcmode) -> u8 {
            Ch2CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch2CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch2CtrlStructtype {
            Ch2CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch2CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch2CtrlStructtype) -> u8 {
            Ch2CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch2LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch2LinkLinkmode {
            Ch2LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch2LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch2LinkLinkmode) -> u8 {
            Ch2LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch2XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch2XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch2XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch2XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch2XctrlIlmode {
            Ch2XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch2XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch2XctrlIlmode) -> u8 {
            Ch2XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch3CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch3CfgArbslots {
            Ch3CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch3CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch3CfgArbslots) -> u8 {
            Ch3CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch3CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch3CfgDstbusport {
            Ch3CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch3CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch3CfgDstbusport) -> u8 {
            Ch3CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch3CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch3CfgDstincsign {
            Ch3CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch3CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch3CfgDstincsign) -> u8 {
            Ch3CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch3CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch3CfgSrcbusport {
            Ch3CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch3CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch3CfgSrcbusport) -> u8 {
            Ch3CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch3CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch3CfgSrcincsign {
            Ch3CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch3CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch3CfgSrcincsign) -> u8 {
            Ch3CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch3CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch3CfgStructbusport {
            Ch3CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch3CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch3CfgStructbusport) -> u8 {
            Ch3CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch3CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlBlocksize {
            Ch3CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch3CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlBlocksize) -> u8 {
            Ch3CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch3CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlDstinc {
            Ch3CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch3CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlDstinc) -> u8 {
            Ch3CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch3CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlDstmode {
            Ch3CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch3CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlDstmode) -> u8 {
            Ch3CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch3CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlReqmode {
            Ch3CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch3CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlReqmode) -> u8 {
            Ch3CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch3CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlSize {
            Ch3CtrlSize::from_bits(val)
        }
    }
    impl From<Ch3CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlSize) -> u8 {
            Ch3CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch3CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlSrcinc {
            Ch3CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch3CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlSrcinc) -> u8 {
            Ch3CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch3CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlSrcmode {
            Ch3CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch3CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlSrcmode) -> u8 {
            Ch3CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch3CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch3CtrlStructtype {
            Ch3CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch3CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch3CtrlStructtype) -> u8 {
            Ch3CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch3LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch3LinkLinkmode {
            Ch3LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch3LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch3LinkLinkmode) -> u8 {
            Ch3LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch3XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch3XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch3XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch3XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch3XctrlIlmode {
            Ch3XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch3XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch3XctrlIlmode) -> u8 {
            Ch3XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch4CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch4CfgArbslots {
            Ch4CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch4CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch4CfgArbslots) -> u8 {
            Ch4CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch4CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch4CfgDstbusport {
            Ch4CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch4CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch4CfgDstbusport) -> u8 {
            Ch4CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch4CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch4CfgDstincsign {
            Ch4CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch4CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch4CfgDstincsign) -> u8 {
            Ch4CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch4CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch4CfgSrcbusport {
            Ch4CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch4CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch4CfgSrcbusport) -> u8 {
            Ch4CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch4CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch4CfgSrcincsign {
            Ch4CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch4CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch4CfgSrcincsign) -> u8 {
            Ch4CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch4CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch4CfgStructbusport {
            Ch4CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch4CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch4CfgStructbusport) -> u8 {
            Ch4CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch4CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlBlocksize {
            Ch4CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch4CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlBlocksize) -> u8 {
            Ch4CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch4CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlDstinc {
            Ch4CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch4CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlDstinc) -> u8 {
            Ch4CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch4CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlDstmode {
            Ch4CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch4CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlDstmode) -> u8 {
            Ch4CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch4CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlReqmode {
            Ch4CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch4CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlReqmode) -> u8 {
            Ch4CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch4CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlSize {
            Ch4CtrlSize::from_bits(val)
        }
    }
    impl From<Ch4CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlSize) -> u8 {
            Ch4CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch4CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlSrcinc {
            Ch4CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch4CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlSrcinc) -> u8 {
            Ch4CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch4CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlSrcmode {
            Ch4CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch4CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlSrcmode) -> u8 {
            Ch4CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch4CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch4CtrlStructtype {
            Ch4CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch4CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch4CtrlStructtype) -> u8 {
            Ch4CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch4LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch4LinkLinkmode {
            Ch4LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch4LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch4LinkLinkmode) -> u8 {
            Ch4LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch4XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch4XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch4XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch4XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch4XctrlIlmode {
            Ch4XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch4XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch4XctrlIlmode) -> u8 {
            Ch4XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch5CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch5CfgArbslots {
            Ch5CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch5CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch5CfgArbslots) -> u8 {
            Ch5CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch5CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch5CfgDstbusport {
            Ch5CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch5CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch5CfgDstbusport) -> u8 {
            Ch5CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch5CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch5CfgDstincsign {
            Ch5CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch5CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch5CfgDstincsign) -> u8 {
            Ch5CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch5CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch5CfgSrcbusport {
            Ch5CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch5CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch5CfgSrcbusport) -> u8 {
            Ch5CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch5CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch5CfgSrcincsign {
            Ch5CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch5CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch5CfgSrcincsign) -> u8 {
            Ch5CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch5CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch5CfgStructbusport {
            Ch5CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch5CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch5CfgStructbusport) -> u8 {
            Ch5CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch5CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlBlocksize {
            Ch5CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch5CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlBlocksize) -> u8 {
            Ch5CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch5CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlDstinc {
            Ch5CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch5CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlDstinc) -> u8 {
            Ch5CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch5CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlDstmode {
            Ch5CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch5CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlDstmode) -> u8 {
            Ch5CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch5CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlReqmode {
            Ch5CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch5CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlReqmode) -> u8 {
            Ch5CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch5CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlSize {
            Ch5CtrlSize::from_bits(val)
        }
    }
    impl From<Ch5CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlSize) -> u8 {
            Ch5CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch5CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlSrcinc {
            Ch5CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch5CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlSrcinc) -> u8 {
            Ch5CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch5CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlSrcmode {
            Ch5CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch5CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlSrcmode) -> u8 {
            Ch5CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch5CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch5CtrlStructtype {
            Ch5CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch5CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch5CtrlStructtype) -> u8 {
            Ch5CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch5LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch5LinkLinkmode {
            Ch5LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch5LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch5LinkLinkmode) -> u8 {
            Ch5LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch5XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch5XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch5XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch5XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch5XctrlIlmode {
            Ch5XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch5XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch5XctrlIlmode) -> u8 {
            Ch5XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch6CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch6CfgArbslots {
            Ch6CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch6CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch6CfgArbslots) -> u8 {
            Ch6CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch6CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch6CfgDstbusport {
            Ch6CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch6CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch6CfgDstbusport) -> u8 {
            Ch6CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch6CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch6CfgDstincsign {
            Ch6CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch6CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch6CfgDstincsign) -> u8 {
            Ch6CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch6CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch6CfgSrcbusport {
            Ch6CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch6CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch6CfgSrcbusport) -> u8 {
            Ch6CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch6CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch6CfgSrcincsign {
            Ch6CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch6CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch6CfgSrcincsign) -> u8 {
            Ch6CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch6CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch6CfgStructbusport {
            Ch6CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch6CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch6CfgStructbusport) -> u8 {
            Ch6CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch6CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlBlocksize {
            Ch6CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch6CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlBlocksize) -> u8 {
            Ch6CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch6CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlDstinc {
            Ch6CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch6CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlDstinc) -> u8 {
            Ch6CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch6CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlDstmode {
            Ch6CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch6CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlDstmode) -> u8 {
            Ch6CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch6CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlReqmode {
            Ch6CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch6CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlReqmode) -> u8 {
            Ch6CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch6CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlSize {
            Ch6CtrlSize::from_bits(val)
        }
    }
    impl From<Ch6CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlSize) -> u8 {
            Ch6CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch6CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlSrcinc {
            Ch6CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch6CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlSrcinc) -> u8 {
            Ch6CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch6CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlSrcmode {
            Ch6CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch6CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlSrcmode) -> u8 {
            Ch6CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch6CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch6CtrlStructtype {
            Ch6CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch6CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch6CtrlStructtype) -> u8 {
            Ch6CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch6LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch6LinkLinkmode {
            Ch6LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch6LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch6LinkLinkmode) -> u8 {
            Ch6LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch6XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch6XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch6XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch6XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch6XctrlIlmode {
            Ch6XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch6XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch6XctrlIlmode) -> u8 {
            Ch6XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch7CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch7CfgArbslots {
            Ch7CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch7CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch7CfgArbslots) -> u8 {
            Ch7CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch7CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch7CfgDstbusport {
            Ch7CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch7CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch7CfgDstbusport) -> u8 {
            Ch7CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch7CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch7CfgDstincsign {
            Ch7CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch7CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch7CfgDstincsign) -> u8 {
            Ch7CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch7CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch7CfgSrcbusport {
            Ch7CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch7CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch7CfgSrcbusport) -> u8 {
            Ch7CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch7CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch7CfgSrcincsign {
            Ch7CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch7CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch7CfgSrcincsign) -> u8 {
            Ch7CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch7CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch7CfgStructbusport {
            Ch7CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch7CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch7CfgStructbusport) -> u8 {
            Ch7CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch7CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlBlocksize {
            Ch7CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch7CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlBlocksize) -> u8 {
            Ch7CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch7CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlDstinc {
            Ch7CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch7CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlDstinc) -> u8 {
            Ch7CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch7CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlDstmode {
            Ch7CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch7CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlDstmode) -> u8 {
            Ch7CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch7CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlReqmode {
            Ch7CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch7CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlReqmode) -> u8 {
            Ch7CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch7CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlSize {
            Ch7CtrlSize::from_bits(val)
        }
    }
    impl From<Ch7CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlSize) -> u8 {
            Ch7CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch7CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlSrcinc {
            Ch7CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch7CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlSrcinc) -> u8 {
            Ch7CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch7CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlSrcmode {
            Ch7CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch7CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlSrcmode) -> u8 {
            Ch7CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch7CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch7CtrlStructtype {
            Ch7CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch7CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch7CtrlStructtype) -> u8 {
            Ch7CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch7LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch7LinkLinkmode {
            Ch7LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch7LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch7LinkLinkmode) -> u8 {
            Ch7LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch7XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch7XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch7XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch7XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch7XctrlIlmode {
            Ch7XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch7XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch7XctrlIlmode) -> u8 {
            Ch7XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch8CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch8CfgArbslots {
            Ch8CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch8CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch8CfgArbslots) -> u8 {
            Ch8CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch8CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch8CfgDstbusport {
            Ch8CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch8CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch8CfgDstbusport) -> u8 {
            Ch8CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch8CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch8CfgDstincsign {
            Ch8CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch8CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch8CfgDstincsign) -> u8 {
            Ch8CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch8CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch8CfgSrcbusport {
            Ch8CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch8CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch8CfgSrcbusport) -> u8 {
            Ch8CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch8CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch8CfgSrcincsign {
            Ch8CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch8CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch8CfgSrcincsign) -> u8 {
            Ch8CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch8CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch8CfgStructbusport {
            Ch8CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch8CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch8CfgStructbusport) -> u8 {
            Ch8CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch8CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlBlocksize {
            Ch8CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch8CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlBlocksize) -> u8 {
            Ch8CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch8CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlDstinc {
            Ch8CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch8CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlDstinc) -> u8 {
            Ch8CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch8CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlDstmode {
            Ch8CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch8CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlDstmode) -> u8 {
            Ch8CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch8CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlReqmode {
            Ch8CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch8CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlReqmode) -> u8 {
            Ch8CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch8CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlSize {
            Ch8CtrlSize::from_bits(val)
        }
    }
    impl From<Ch8CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlSize) -> u8 {
            Ch8CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch8CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlSrcinc {
            Ch8CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch8CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlSrcinc) -> u8 {
            Ch8CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch8CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlSrcmode {
            Ch8CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch8CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlSrcmode) -> u8 {
            Ch8CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch8CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch8CtrlStructtype {
            Ch8CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch8CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch8CtrlStructtype) -> u8 {
            Ch8CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch8LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch8LinkLinkmode {
            Ch8LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch8LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch8LinkLinkmode) -> u8 {
            Ch8LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch8XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch8XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch8XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch8XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch8XctrlIlmode {
            Ch8XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch8XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch8XctrlIlmode) -> u8 {
            Ch8XctrlIlmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CfgArbslots {
        #[doc = "One arbitration slot selected."]
        One = 0x0,
        #[doc = "Two arbitration slots selected."]
        Two = 0x01,
        #[doc = "Four arbitration slots selected."]
        Four = 0x02,
        #[doc = "Eight arbitration slots selected."]
        Eight = 0x03,
    }
    impl Ch9CfgArbslots {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CfgArbslots {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CfgArbslots {
        #[inline(always)]
        fn from(val: u8) -> Ch9CfgArbslots {
            Ch9CfgArbslots::from_bits(val)
        }
    }
    impl From<Ch9CfgArbslots> for u8 {
        #[inline(always)]
        fn from(val: Ch9CfgArbslots) -> u8 {
            Ch9CfgArbslots::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CfgDstbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch9CfgDstbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CfgDstbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CfgDstbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch9CfgDstbusport {
            Ch9CfgDstbusport::from_bits(val)
        }
    }
    impl From<Ch9CfgDstbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch9CfgDstbusport) -> u8 {
            Ch9CfgDstbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CfgDstincsign {
        #[doc = "Increment destination address."]
        Positive = 0x0,
        #[doc = "Decrement destination address."]
        Negative = 0x01,
    }
    impl Ch9CfgDstincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CfgDstincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CfgDstincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch9CfgDstincsign {
            Ch9CfgDstincsign::from_bits(val)
        }
    }
    impl From<Ch9CfgDstincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch9CfgDstincsign) -> u8 {
            Ch9CfgDstincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CfgSrcbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch9CfgSrcbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CfgSrcbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CfgSrcbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch9CfgSrcbusport {
            Ch9CfgSrcbusport::from_bits(val)
        }
    }
    impl From<Ch9CfgSrcbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch9CfgSrcbusport) -> u8 {
            Ch9CfgSrcbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CfgSrcincsign {
        #[doc = "Increment source address."]
        Positive = 0x0,
        #[doc = "Decrement source address."]
        Negative = 0x01,
    }
    impl Ch9CfgSrcincsign {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CfgSrcincsign {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CfgSrcincsign {
        #[inline(always)]
        fn from(val: u8) -> Ch9CfgSrcincsign {
            Ch9CfgSrcincsign::from_bits(val)
        }
    }
    impl From<Ch9CfgSrcincsign> for u8 {
        #[inline(always)]
        fn from(val: Ch9CfgSrcincsign) -> u8 {
            Ch9CfgSrcincsign::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CfgStructbusport {
        #[doc = "AHBM0."]
        Ahbm0 = 0x0,
        #[doc = "AHBM1."]
        Ahbm1 = 0x01,
    }
    impl Ch9CfgStructbusport {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CfgStructbusport {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CfgStructbusport {
        #[inline(always)]
        fn from(val: u8) -> Ch9CfgStructbusport {
            Ch9CfgStructbusport::from_bits(val)
        }
    }
    impl From<Ch9CfgStructbusport> for u8 {
        #[inline(always)]
        fn from(val: Ch9CfgStructbusport) -> u8 {
            Ch9CfgStructbusport::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlBlocksize {
        #[doc = "1 unit transfer per arbitration."]
        Unit1 = 0x0,
        #[doc = "2 unit transfers per arbitration."]
        Unit2 = 0x01,
        #[doc = "3 unit transfers per arbitration."]
        Unit3 = 0x02,
        #[doc = "4 unit transfers per arbitration."]
        Unit4 = 0x03,
        #[doc = "6 unit transfers per arbitration."]
        Unit6 = 0x04,
        #[doc = "8 unit transfers per arbitration."]
        Unit8 = 0x05,
        #[doc = "12 unit transfers per arbitration."]
        Unit12 = 0x06,
        #[doc = "16 unit transfers per arbitration."]
        Unit16 = 0x07,
        #[doc = "24 unit transfers per arbitration."]
        Unit24 = 0x08,
        #[doc = "32 unit transfers per arbitration."]
        Unit32 = 0x09,
        #[doc = "64 unit transfers per arbitration."]
        Unit64 = 0x0a,
        #[doc = "128 unit transfers per arbitration."]
        Unit128 = 0x0b,
        #[doc = "256 unit transfers per arbitration."]
        Unit256 = 0x0c,
        #[doc = "512 unit transfers per arbitration."]
        Unit512 = 0x0d,
        #[doc = "1024 unit transfers per arbitration."]
        Unit1024 = 0x0e,
        #[doc = "Transfer all units as specified by the XFRCNT field."]
        All = 0x0f,
    }
    impl Ch9CtrlBlocksize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlBlocksize {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlBlocksize {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlBlocksize {
            Ch9CtrlBlocksize::from_bits(val)
        }
    }
    impl From<Ch9CtrlBlocksize> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlBlocksize) -> u8 {
            Ch9CtrlBlocksize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlDstinc {
        #[doc = "Increment destination address by one unit data size after each write."]
        One = 0x0,
        #[doc = "Increment destination address by two unit data sizes after each write."]
        Two = 0x01,
        #[doc = "Increment destination address by four unit data sizes after each write."]
        Four = 0x02,
        #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
        None = 0x03,
    }
    impl Ch9CtrlDstinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlDstinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlDstinc {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlDstinc {
            Ch9CtrlDstinc::from_bits(val)
        }
    }
    impl From<Ch9CtrlDstinc> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlDstinc) -> u8 {
            Ch9CtrlDstinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlDstmode {
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
        Absolute = 0x0,
        #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
        Relative = 0x01,
    }
    impl Ch9CtrlDstmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlDstmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlDstmode {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlDstmode {
            Ch9CtrlDstmode::from_bits(val)
        }
    }
    impl From<Ch9CtrlDstmode> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlDstmode) -> u8 {
            Ch9CtrlDstmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlReqmode {
        #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
        Block = 0x0,
        #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
        All = 0x01,
    }
    impl Ch9CtrlReqmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlReqmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlReqmode {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlReqmode {
            Ch9CtrlReqmode::from_bits(val)
        }
    }
    impl From<Ch9CtrlReqmode> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlReqmode) -> u8 {
            Ch9CtrlReqmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlSize {
        #[doc = "Each unit transfer is a byte."]
        Byte = 0x0,
        #[doc = "Each unit transfer is a half-word."]
        Halfword = 0x01,
        #[doc = "Each unit transfer is a word."]
        Word = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch9CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlSize {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlSize {
            Ch9CtrlSize::from_bits(val)
        }
    }
    impl From<Ch9CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlSize) -> u8 {
            Ch9CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlSrcinc {
        #[doc = "Increment source address by one unit data size after each read."]
        One = 0x0,
        #[doc = "Increment source address by two unit data sizes after each read."]
        Two = 0x01,
        #[doc = "Increment source address by four unit data sizes after each read."]
        Four = 0x02,
        #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
        None = 0x03,
    }
    impl Ch9CtrlSrcinc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlSrcinc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlSrcinc {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlSrcinc {
            Ch9CtrlSrcinc::from_bits(val)
        }
    }
    impl From<Ch9CtrlSrcinc> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlSrcinc) -> u8 {
            Ch9CtrlSrcinc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlSrcmode {
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
        Absolute = 0x0,
        #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
        Relative = 0x01,
    }
    impl Ch9CtrlSrcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlSrcmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlSrcmode {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlSrcmode {
            Ch9CtrlSrcmode::from_bits(val)
        }
    }
    impl From<Ch9CtrlSrcmode> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlSrcmode) -> u8 {
            Ch9CtrlSrcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9CtrlStructtype {
        #[doc = "DMA transfer structure type selected."]
        Transfer = 0x0,
        #[doc = "Synchronization structure type selected."]
        Synchronize = 0x01,
        #[doc = "Write immediate value structure type selected."]
        Write = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch9CtrlStructtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9CtrlStructtype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9CtrlStructtype {
        #[inline(always)]
        fn from(val: u8) -> Ch9CtrlStructtype {
            Ch9CtrlStructtype::from_bits(val)
        }
    }
    impl From<Ch9CtrlStructtype> for u8 {
        #[inline(always)]
        fn from(val: Ch9CtrlStructtype) -> u8 {
            Ch9CtrlStructtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9LinkLinkmode {
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor."]
        Absolute = 0x0,
        #[doc = "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor."]
        Relative = 0x01,
    }
    impl Ch9LinkLinkmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9LinkLinkmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9LinkLinkmode {
        #[inline(always)]
        fn from(val: u8) -> Ch9LinkLinkmode {
            Ch9LinkLinkmode::from_bits(val)
        }
    }
    impl From<Ch9LinkLinkmode> for u8 {
        #[inline(always)]
        fn from(val: Ch9LinkLinkmode) -> u8 {
            Ch9LinkLinkmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ch9XctrlIlmode {
        #[doc = "Address determined by value in rules. Size of WORD."]
        Absolute = 0x0,
        #[doc = "Address determined by adding rules to DST. Size of HALFWORD."]
        Relative16 = 0x01,
        #[doc = "Address determined by adding rules to DST. Size of BYTE."]
        Relative8 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ch9XctrlIlmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ch9XctrlIlmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ch9XctrlIlmode {
        #[inline(always)]
        fn from(val: u8) -> Ch9XctrlIlmode {
            Ch9XctrlIlmode::from_bits(val)
        }
    }
    impl From<Ch9XctrlIlmode> for u8 {
        #[inline(always)]
        fn from(val: Ch9XctrlIlmode) -> u8 {
            Ch9XctrlIlmode::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncclredge(u8);
    impl Syncclredge {
        #[doc = "Use rising edge detection."]
        pub const Rise: Self = Self(0x0);
        #[doc = "Use falling edge detection."]
        pub const Fall: Self = Self(0x01);
    }
    impl Syncclredge {
        pub const fn from_bits(val: u8) -> Syncclredge {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Syncclredge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Rise"),
                0x01 => f.write_str("Fall"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncclredge {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Rise"),
                0x01 => defmt::write!(f, "Fall"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Syncclredge {
        #[inline(always)]
        fn from(val: u8) -> Syncclredge {
            Syncclredge::from_bits(val)
        }
    }
    impl From<Syncclredge> for u8 {
        #[inline(always)]
        fn from(val: Syncclredge) -> u8 {
            Syncclredge::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syncsetedge(u8);
    impl Syncsetedge {
        #[doc = "Use rising edge detection."]
        pub const Rise: Self = Self(0x0);
        #[doc = "Use falling edge detection."]
        pub const Fall: Self = Self(0x01);
    }
    impl Syncsetedge {
        pub const fn from_bits(val: u8) -> Syncsetedge {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Syncsetedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Rise"),
                0x01 => f.write_str("Fall"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syncsetedge {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Rise"),
                0x01 => defmt::write!(f, "Fall"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Syncsetedge {
        #[inline(always)]
        fn from(val: u8) -> Syncsetedge {
            Syncsetedge::from_bits(val)
        }
    }
    impl From<Syncsetedge> for u8 {
        #[inline(always)]
        fn from(val: Syncsetedge) -> u8 {
            Syncsetedge::to_bits(val)
        }
    }
}
