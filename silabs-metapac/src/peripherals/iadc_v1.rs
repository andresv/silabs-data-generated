#[doc = "IADC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iadc {
    ptr: *mut u8,
}
unsafe impl Send for Iadc {}
unsafe impl Sync for Iadc {}
impl Iadc {
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
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Command."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Timer."]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Mask Request."]
    #[inline(always)]
    pub const fn maskreq(self) -> crate::common::Reg<regs::Maskreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Scan Table Mask."]
    #[inline(always)]
    pub const fn stmask(self) -> crate::common::Reg<regs::Stmask, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Comparator Threshold."]
    #[inline(always)]
    pub const fn cmpthr(self) -> crate::common::Reg<regs::Cmpthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Trigger."]
    #[inline(always)]
    pub const fn trigger(self) -> crate::common::Reg<regs::Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Configration."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Scale."]
    #[inline(always)]
    pub const fn scale0(self) -> crate::common::Reg<regs::Scale0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Scheduling."]
    #[inline(always)]
    pub const fn sched0(self) -> crate::common::Reg<regs::Sched0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Configration."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Scale."]
    #[inline(always)]
    pub const fn scale1(self) -> crate::common::Reg<regs::Scale1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Scheduling."]
    #[inline(always)]
    pub const fn sched1(self) -> crate::common::Reg<regs::Sched1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Single FIFO Configuration."]
    #[inline(always)]
    pub const fn singlefifocfg(self) -> crate::common::Reg<regs::Singlefifocfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Read the oldest valid data from the single FIFO and pop the FIFO."]
    #[inline(always)]
    pub const fn singlefifodata(self) -> crate::common::Reg<regs::Singlefifodata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Single FIFO status."]
    #[inline(always)]
    pub const fn singlefifostat(self) -> crate::common::Reg<regs::Singlefifostat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "latest single queue conversion data."]
    #[inline(always)]
    pub const fn singledata(self) -> crate::common::Reg<regs::Singledata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Scan FIFO Configuration."]
    #[inline(always)]
    pub const fn scanfifocfg(self) -> crate::common::Reg<regs::Scanfifocfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Read the oldest valid data from the scan FIFO and pop the FIFO."]
    #[inline(always)]
    pub const fn scanfifodata(self) -> crate::common::Reg<regs::Scanfifodata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Scan FIFO status."]
    #[inline(always)]
    pub const fn scanfifostat(self) -> crate::common::Reg<regs::Scanfifostat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Most recent data data from scan queue conversion."]
    #[inline(always)]
    pub const fn scandata(self) -> crate::common::Reg<regs::Scandata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn single(self) -> crate::common::Reg<regs::Single, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan0(self) -> crate::common::Reg<regs::Scan0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan1(self) -> crate::common::Reg<regs::Scan1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan2(self) -> crate::common::Reg<regs::Scan2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan3(self) -> crate::common::Reg<regs::Scan3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan4(self) -> crate::common::Reg<regs::Scan4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan5(self) -> crate::common::Reg<regs::Scan5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan6(self) -> crate::common::Reg<regs::Scan6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan7(self) -> crate::common::Reg<regs::Scan7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan8(self) -> crate::common::Reg<regs::Scan8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan9(self) -> crate::common::Reg<regs::Scan9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan10(self) -> crate::common::Reg<regs::Scan10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan11(self) -> crate::common::Reg<regs::Scan11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan12(self) -> crate::common::Reg<regs::Scan12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan13(self) -> crate::common::Reg<regs::Scan13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan14(self) -> crate::common::Reg<regs::Scan14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn scan15(self) -> crate::common::Reg<regs::Scan15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Command. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Timer. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer_set(self) -> crate::common::Reg<regs::Timer, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "Mask Request. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn maskreq_set(self) -> crate::common::Reg<regs::Maskreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "Comparator Threshold. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmpthr_set(self) -> crate::common::Reg<regs::Cmpthr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "Trigger. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn trigger_set(self) -> crate::common::Reg<regs::Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "Configration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg0_set(self) -> crate::common::Reg<regs::Cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Scale. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scale0_set(self) -> crate::common::Reg<regs::Scale0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "Scheduling. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sched0_set(self) -> crate::common::Reg<regs::Sched0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "Configration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg1_set(self) -> crate::common::Reg<regs::Cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "Scale. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scale1_set(self) -> crate::common::Reg<regs::Scale1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "Scheduling. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sched1_set(self) -> crate::common::Reg<regs::Sched1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "Single FIFO Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn singlefifocfg_set(self) -> crate::common::Reg<regs::Singlefifocfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "Scan FIFO Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scanfifocfg_set(self) -> crate::common::Reg<regs::Scanfifocfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn single_set(self) -> crate::common::Reg<regs::Single, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan0_set(self) -> crate::common::Reg<regs::Scan0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan1_set(self) -> crate::common::Reg<regs::Scan1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan2_set(self) -> crate::common::Reg<regs::Scan2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan3_set(self) -> crate::common::Reg<regs::Scan3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10acusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan4_set(self) -> crate::common::Reg<regs::Scan4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan5_set(self) -> crate::common::Reg<regs::Scan5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan6_set(self) -> crate::common::Reg<regs::Scan6, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan7_set(self) -> crate::common::Reg<regs::Scan7, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10bcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan8_set(self) -> crate::common::Reg<regs::Scan8, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan9_set(self) -> crate::common::Reg<regs::Scan9, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan10_set(self) -> crate::common::Reg<regs::Scan10, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan11_set(self) -> crate::common::Reg<regs::Scan11, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan12_set(self) -> crate::common::Reg<regs::Scan12, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan13_set(self) -> crate::common::Reg<regs::Scan13, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan14_set(self) -> crate::common::Reg<regs::Scan14, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn scan15_set(self) -> crate::common::Reg<regs::Scan15, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10dcusize) as _) }
    }
    #[doc = "Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Command. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "Timer. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer_clr(self) -> crate::common::Reg<regs::Timer, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "Mask Request. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn maskreq_clr(self) -> crate::common::Reg<regs::Maskreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "Comparator Threshold. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmpthr_clr(self) -> crate::common::Reg<regs::Cmpthr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "Trigger. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn trigger_clr(self) -> crate::common::Reg<regs::Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "Configration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg0_clr(self) -> crate::common::Reg<regs::Cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "Scale. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scale0_clr(self) -> crate::common::Reg<regs::Scale0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "Scheduling. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sched0_clr(self) -> crate::common::Reg<regs::Sched0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "Configration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg1_clr(self) -> crate::common::Reg<regs::Cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "Scale. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scale1_clr(self) -> crate::common::Reg<regs::Scale1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "Scheduling. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sched1_clr(self) -> crate::common::Reg<regs::Sched1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "Single FIFO Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn singlefifocfg_clr(self) -> crate::common::Reg<regs::Singlefifocfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "Scan FIFO Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scanfifocfg_clr(self) -> crate::common::Reg<regs::Scanfifocfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn single_clr(self) -> crate::common::Reg<regs::Single, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2098usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan0_clr(self) -> crate::common::Reg<regs::Scan0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan1_clr(self) -> crate::common::Reg<regs::Scan1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan2_clr(self) -> crate::common::Reg<regs::Scan2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan3_clr(self) -> crate::common::Reg<regs::Scan3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20acusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan4_clr(self) -> crate::common::Reg<regs::Scan4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan5_clr(self) -> crate::common::Reg<regs::Scan5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan6_clr(self) -> crate::common::Reg<regs::Scan6, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan7_clr(self) -> crate::common::Reg<regs::Scan7, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20bcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan8_clr(self) -> crate::common::Reg<regs::Scan8, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan9_clr(self) -> crate::common::Reg<regs::Scan9, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan10_clr(self) -> crate::common::Reg<regs::Scan10, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan11_clr(self) -> crate::common::Reg<regs::Scan11, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan12_clr(self) -> crate::common::Reg<regs::Scan12, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan13_clr(self) -> crate::common::Reg<regs::Scan13, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan14_clr(self) -> crate::common::Reg<regs::Scan14, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn scan15_clr(self) -> crate::common::Reg<regs::Scan15, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20dcusize) as _) }
    }
    #[doc = "Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "Control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ctrl_tgl(self) -> crate::common::Reg<regs::Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Command. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "Timer. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer_tgl(self) -> crate::common::Reg<regs::Timer, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "Mask Request. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn maskreq_tgl(self) -> crate::common::Reg<regs::Maskreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "Comparator Threshold. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmpthr_tgl(self) -> crate::common::Reg<regs::Cmpthr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "Trigger. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn trigger_tgl(self) -> crate::common::Reg<regs::Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "Configration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg0_tgl(self) -> crate::common::Reg<regs::Cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "Scale. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scale0_tgl(self) -> crate::common::Reg<regs::Scale0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "Scheduling. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sched0_tgl(self) -> crate::common::Reg<regs::Sched0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "Configration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg1_tgl(self) -> crate::common::Reg<regs::Cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "Scale. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scale1_tgl(self) -> crate::common::Reg<regs::Scale1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "Scheduling. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sched1_tgl(self) -> crate::common::Reg<regs::Sched1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "Single FIFO Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn singlefifocfg_tgl(self) -> crate::common::Reg<regs::Singlefifocfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "Scan FIFO Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scanfifocfg_tgl(self) -> crate::common::Reg<regs::Scanfifocfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn single_tgl(self) -> crate::common::Reg<regs::Single, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3098usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan0_tgl(self) -> crate::common::Reg<regs::Scan0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan1_tgl(self) -> crate::common::Reg<regs::Scan1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan2_tgl(self) -> crate::common::Reg<regs::Scan2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan3_tgl(self) -> crate::common::Reg<regs::Scan3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30acusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan4_tgl(self) -> crate::common::Reg<regs::Scan4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan5_tgl(self) -> crate::common::Reg<regs::Scan5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan6_tgl(self) -> crate::common::Reg<regs::Scan6, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan7_tgl(self) -> crate::common::Reg<regs::Scan7, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30bcusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan8_tgl(self) -> crate::common::Reg<regs::Scan8, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan9_tgl(self) -> crate::common::Reg<regs::Scan9, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan10_tgl(self) -> crate::common::Reg<regs::Scan10, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan11_tgl(self) -> crate::common::Reg<regs::Scan11, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan12_tgl(self) -> crate::common::Reg<regs::Scan12, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan13_tgl(self) -> crate::common::Reg<regs::Scan13, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan14_tgl(self) -> crate::common::Reg<regs::Scan14, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn scan15_tgl(self) -> crate::common::Reg<regs::Scan15, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30dcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Configration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "ADC Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adcmode(&self) -> super::vals::Cfg0Adcmode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Cfg0Adcmode::from_bits(val as u8)
        }
        #[doc = "ADC Mode."]
        #[inline(always)]
        pub const fn set_adcmode(&mut self, val: super::vals::Cfg0Adcmode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "High Speed OSR."]
        #[must_use]
        #[inline(always)]
        pub const fn osrhs(&self) -> super::vals::Cfg0Osrhs {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Cfg0Osrhs::from_bits(val as u8)
        }
        #[doc = "High Speed OSR."]
        #[inline(always)]
        pub const fn set_osrhs(&mut self, val: super::vals::Cfg0Osrhs) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "Analog Gain."]
        #[must_use]
        #[inline(always)]
        pub const fn analoggain(&self) -> super::vals::Cfg0Analoggain {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Cfg0Analoggain::from_bits(val as u8)
        }
        #[doc = "Analog Gain."]
        #[inline(always)]
        pub const fn set_analoggain(&mut self, val: super::vals::Cfg0Analoggain) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "Reference Select."]
        #[must_use]
        #[inline(always)]
        pub const fn refsel(&self) -> super::vals::Cfg0Refsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cfg0Refsel::from_bits(val as u8)
        }
        #[doc = "Reference Select."]
        #[inline(always)]
        pub const fn set_refsel(&mut self, val: super::vals::Cfg0Refsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Digital Averaging."]
        #[must_use]
        #[inline(always)]
        pub const fn digavg(&self) -> super::vals::Cfg0Digavg {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Cfg0Digavg::from_bits(val as u8)
        }
        #[doc = "Digital Averaging."]
        #[inline(always)]
        pub const fn set_digavg(&mut self, val: super::vals::Cfg0Digavg) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Two's Complement."]
        #[must_use]
        #[inline(always)]
        pub const fn twoscompl(&self) -> super::vals::Cfg0Twoscompl {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Cfg0Twoscompl::from_bits(val as u8)
        }
        #[doc = "Two's Complement."]
        #[inline(always)]
        pub const fn set_twoscompl(&mut self, val: super::vals::Cfg0Twoscompl) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    impl core::fmt::Debug for Cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg0")
                .field("adcmode", &self.adcmode())
                .field("osrhs", &self.osrhs())
                .field("analoggain", &self.analoggain())
                .field("refsel", &self.refsel())
                .field("digavg", &self.digavg())
                .field("twoscompl", &self.twoscompl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg0 {{ adcmode: {:?}, osrhs: {:?}, analoggain: {:?}, refsel: {:?}, digavg: {:?}, twoscompl: {:?} }}",
                self.adcmode(),
                self.osrhs(),
                self.analoggain(),
                self.refsel(),
                self.digavg(),
                self.twoscompl()
            )
        }
    }
    #[doc = "Configration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "ADC Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adcmode(&self) -> super::vals::Cfg1Adcmode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Cfg1Adcmode::from_bits(val as u8)
        }
        #[doc = "ADC Mode."]
        #[inline(always)]
        pub const fn set_adcmode(&mut self, val: super::vals::Cfg1Adcmode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "High Speed OSR."]
        #[must_use]
        #[inline(always)]
        pub const fn osrhs(&self) -> super::vals::Cfg1Osrhs {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Cfg1Osrhs::from_bits(val as u8)
        }
        #[doc = "High Speed OSR."]
        #[inline(always)]
        pub const fn set_osrhs(&mut self, val: super::vals::Cfg1Osrhs) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "Analog Gain."]
        #[must_use]
        #[inline(always)]
        pub const fn analoggain(&self) -> super::vals::Cfg1Analoggain {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Cfg1Analoggain::from_bits(val as u8)
        }
        #[doc = "Analog Gain."]
        #[inline(always)]
        pub const fn set_analoggain(&mut self, val: super::vals::Cfg1Analoggain) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "Reference Select."]
        #[must_use]
        #[inline(always)]
        pub const fn refsel(&self) -> super::vals::Cfg1Refsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cfg1Refsel::from_bits(val as u8)
        }
        #[doc = "Reference Select."]
        #[inline(always)]
        pub const fn set_refsel(&mut self, val: super::vals::Cfg1Refsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Digital Averaging."]
        #[must_use]
        #[inline(always)]
        pub const fn digavg(&self) -> super::vals::Cfg1Digavg {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Cfg1Digavg::from_bits(val as u8)
        }
        #[doc = "Digital Averaging."]
        #[inline(always)]
        pub const fn set_digavg(&mut self, val: super::vals::Cfg1Digavg) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Two's Complement."]
        #[must_use]
        #[inline(always)]
        pub const fn twoscompl(&self) -> super::vals::Cfg1Twoscompl {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Cfg1Twoscompl::from_bits(val as u8)
        }
        #[doc = "Two's Complement."]
        #[inline(always)]
        pub const fn set_twoscompl(&mut self, val: super::vals::Cfg1Twoscompl) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    impl core::fmt::Debug for Cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg1")
                .field("adcmode", &self.adcmode())
                .field("osrhs", &self.osrhs())
                .field("analoggain", &self.analoggain())
                .field("refsel", &self.refsel())
                .field("digavg", &self.digavg())
                .field("twoscompl", &self.twoscompl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg1 {{ adcmode: {:?}, osrhs: {:?}, analoggain: {:?}, refsel: {:?}, digavg: {:?}, twoscompl: {:?} }}",
                self.adcmode(),
                self.osrhs(),
                self.analoggain(),
                self.refsel(),
                self.digavg(),
                self.twoscompl()
            )
        }
    }
    #[doc = "Command."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Single Queue Start."]
        #[must_use]
        #[inline(always)]
        pub const fn singlestart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Single Queue Start."]
        #[inline(always)]
        pub const fn set_singlestart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Single Queue Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn singlestop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Single Queue Stop."]
        #[inline(always)]
        pub const fn set_singlestop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Scan Queue Start."]
        #[must_use]
        #[inline(always)]
        pub const fn scanstart(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Queue Start."]
        #[inline(always)]
        pub const fn set_scanstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scan Queue Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn scanstop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Queue Stop."]
        #[inline(always)]
        pub const fn set_scanstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn timeren(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Enable."]
        #[inline(always)]
        pub const fn set_timeren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Timer Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn timerdis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Disable."]
        #[inline(always)]
        pub const fn set_timerdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Flush the Single FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifoflush(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Flush the Single FIFO."]
        #[inline(always)]
        pub const fn set_singlefifoflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Flush the Scan FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifoflush(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Flush the Scan FIFO."]
        #[inline(always)]
        pub const fn set_scanfifoflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("singlestart", &self.singlestart())
                .field("singlestop", &self.singlestop())
                .field("scanstart", &self.scanstart())
                .field("scanstop", &self.scanstop())
                .field("timeren", &self.timeren())
                .field("timerdis", &self.timerdis())
                .field("singlefifoflush", &self.singlefifoflush())
                .field("scanfifoflush", &self.scanfifoflush())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ singlestart: {=bool:?}, singlestop: {=bool:?}, scanstart: {=bool:?}, scanstop: {=bool:?}, timeren: {=bool:?}, timerdis: {=bool:?}, singlefifoflush: {=bool:?}, scanfifoflush: {=bool:?} }}",
                self.singlestart(),
                self.singlestop(),
                self.scanstart(),
                self.scanstop(),
                self.timeren(),
                self.timerdis(),
                self.singlefifoflush(),
                self.scanfifoflush()
            )
        }
    }
    #[doc = "Comparator Threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpthr(pub u32);
    impl Cmpthr {
        #[doc = "ADC Less Than or Equal to Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn adlt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ADC Less Than or Equal to Threshold."]
        #[inline(always)]
        pub const fn set_adlt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "ADC Greater Than or Equal to Threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn adgt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "ADC Greater Than or Equal to Threshold."]
        #[inline(always)]
        pub const fn set_adgt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cmpthr {
        #[inline(always)]
        fn default() -> Cmpthr {
            Cmpthr(0)
        }
    }
    impl core::fmt::Debug for Cmpthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmpthr")
                .field("adlt", &self.adlt())
                .field("adgt", &self.adgt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmpthr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmpthr {{ adlt: {=u16:?}, adgt: {=u16:?} }}",
                self.adlt(),
                self.adgt()
            )
        }
    }
    #[doc = "Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "EM23 Wakeup on Conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn em23wuconvert(&self) -> super::vals::Em23wuconvert {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Em23wuconvert::from_bits(val as u8)
        }
        #[doc = "EM23 Wakeup on Conversion."]
        #[inline(always)]
        pub const fn set_em23wuconvert(&mut self, val: super::vals::Em23wuconvert) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC_CLK Suspend - PRS0."]
        #[must_use]
        #[inline(always)]
        pub const fn adcclksuspend0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC_CLK Suspend - PRS0."]
        #[inline(always)]
        pub const fn set_adcclksuspend0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC_CLK Suspend - PRS1."]
        #[must_use]
        #[inline(always)]
        pub const fn adcclksuspend1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC_CLK Suspend - PRS1."]
        #[inline(always)]
        pub const fn set_adcclksuspend1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Debug Halt."]
        #[must_use]
        #[inline(always)]
        pub const fn dbghalt(&self) -> super::vals::Dbghalt {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Dbghalt::from_bits(val as u8)
        }
        #[doc = "Debug Halt."]
        #[inline(always)]
        pub const fn set_dbghalt(&mut self, val: super::vals::Dbghalt) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Warmup Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn warmupmode(&self) -> super::vals::Warmupmode {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Warmupmode::from_bits(val as u8)
        }
        #[doc = "Warmup Mode."]
        #[inline(always)]
        pub const fn set_warmupmode(&mut self, val: super::vals::Warmupmode) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Time Base."]
        #[must_use]
        #[inline(always)]
        pub const fn timebase(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Time Base."]
        #[inline(always)]
        pub const fn set_timebase(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "High Speed Clock Rate."]
        #[must_use]
        #[inline(always)]
        pub const fn hsclkrate(&self) -> super::vals::Hsclkrate {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Hsclkrate::from_bits(val as u8)
        }
        #[doc = "High Speed Clock Rate."]
        #[inline(always)]
        pub const fn set_hsclkrate(&mut self, val: super::vals::Hsclkrate) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
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
                .field("em23wuconvert", &self.em23wuconvert())
                .field("adcclksuspend0", &self.adcclksuspend0())
                .field("adcclksuspend1", &self.adcclksuspend1())
                .field("dbghalt", &self.dbghalt())
                .field("warmupmode", &self.warmupmode())
                .field("timebase", &self.timebase())
                .field("hsclkrate", &self.hsclkrate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ em23wuconvert: {:?}, adcclksuspend0: {=bool:?}, adcclksuspend1: {=bool:?}, dbghalt: {:?}, warmupmode: {:?}, timebase: {=u8:?}, hsclkrate: {:?} }}",
                self.em23wuconvert(),
                self.adcclksuspend0(),
                self.adcclksuspend1(),
                self.dbghalt(),
                self.warmupmode(),
                self.timebase(),
                self.hsclkrate()
            )
        }
    }
    #[doc = "Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Enable IADC Module."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable IADC Module."]
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
    #[doc = "Interrupt Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Single FIFO Data Valid Level Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifodvl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO Data Valid Level Enable."]
        #[inline(always)]
        pub const fn set_singlefifodvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scan FIFO Data Valid Level Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifodvl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO Data Valid Level Enable."]
        #[inline(always)]
        pub const fn set_scanfifodvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single Result Window Compare Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn singlecmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Single Result Window Compare Enable."]
        #[inline(always)]
        pub const fn set_singlecmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Scan Result Window Compare Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scancmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Result Window Compare Enable."]
        #[inline(always)]
        pub const fn set_scancmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scan Entry Done Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scanentrydone(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Entry Done Enable."]
        #[inline(always)]
        pub const fn set_scanentrydone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Scan Table Done Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scantabledone(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Table Done Enable."]
        #[inline(always)]
        pub const fn set_scantabledone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Single Conversion Done Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn singledone(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Single Conversion Done Enable."]
        #[inline(always)]
        pub const fn set_singledone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Polarity Error Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn polarityerr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity Error Enable."]
        #[inline(always)]
        pub const fn set_polarityerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port Allocation Error Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn portallocerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port Allocation Error Enable."]
        #[inline(always)]
        pub const fn set_portallocerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Single FIFO Overflow Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifoof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO Overflow Enable."]
        #[inline(always)]
        pub const fn set_singlefifoof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Scan FIFO Overflow Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifoof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO Overflow Enable."]
        #[inline(always)]
        pub const fn set_scanfifoof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Single FIFO Underflow Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifouf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO Underflow Enable."]
        #[inline(always)]
        pub const fn set_singlefifouf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Scan FIFO Underflow Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifouf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO Underflow Enable."]
        #[inline(always)]
        pub const fn set_scanfifouf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "EM2/3 Abort Error Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn em23aborterror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EM2/3 Abort Error Enable."]
        #[inline(always)]
        pub const fn set_em23aborterror(&mut self, val: bool) {
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
                .field("singlefifodvl", &self.singlefifodvl())
                .field("scanfifodvl", &self.scanfifodvl())
                .field("singlecmp", &self.singlecmp())
                .field("scancmp", &self.scancmp())
                .field("scanentrydone", &self.scanentrydone())
                .field("scantabledone", &self.scantabledone())
                .field("singledone", &self.singledone())
                .field("polarityerr", &self.polarityerr())
                .field("portallocerr", &self.portallocerr())
                .field("singlefifoof", &self.singlefifoof())
                .field("scanfifoof", &self.scanfifoof())
                .field("singlefifouf", &self.singlefifouf())
                .field("scanfifouf", &self.scanfifouf())
                .field("em23aborterror", &self.em23aborterror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ singlefifodvl: {=bool:?}, scanfifodvl: {=bool:?}, singlecmp: {=bool:?}, scancmp: {=bool:?}, scanentrydone: {=bool:?}, scantabledone: {=bool:?}, singledone: {=bool:?}, polarityerr: {=bool:?}, portallocerr: {=bool:?}, singlefifoof: {=bool:?}, scanfifoof: {=bool:?}, singlefifouf: {=bool:?}, scanfifouf: {=bool:?}, em23aborterror: {=bool:?} }}",
                self.singlefifodvl(),
                self.scanfifodvl(),
                self.singlecmp(),
                self.scancmp(),
                self.scanentrydone(),
                self.scantabledone(),
                self.singledone(),
                self.polarityerr(),
                self.portallocerr(),
                self.singlefifoof(),
                self.scanfifoof(),
                self.singlefifouf(),
                self.scanfifouf(),
                self.em23aborterror()
            )
        }
    }
    #[doc = "Interrupt Flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Single FIFO Data Valid Level."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifodvl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO Data Valid Level."]
        #[inline(always)]
        pub const fn set_singlefifodvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scan FIFO Data Valid Level."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifodvl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO Data Valid Level."]
        #[inline(always)]
        pub const fn set_scanfifodvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single Result Window Compare."]
        #[must_use]
        #[inline(always)]
        pub const fn singlecmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Single Result Window Compare."]
        #[inline(always)]
        pub const fn set_singlecmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Scan Result Window Compare."]
        #[must_use]
        #[inline(always)]
        pub const fn scancmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Result Window Compare."]
        #[inline(always)]
        pub const fn set_scancmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scan Entry Done."]
        #[must_use]
        #[inline(always)]
        pub const fn scanentrydone(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Entry Done."]
        #[inline(always)]
        pub const fn set_scanentrydone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Scan Table Done."]
        #[must_use]
        #[inline(always)]
        pub const fn scantabledone(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Table Done."]
        #[inline(always)]
        pub const fn set_scantabledone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Single Conversion Done."]
        #[must_use]
        #[inline(always)]
        pub const fn singledone(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Single Conversion Done."]
        #[inline(always)]
        pub const fn set_singledone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Polarity Error."]
        #[must_use]
        #[inline(always)]
        pub const fn polarityerr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity Error."]
        #[inline(always)]
        pub const fn set_polarityerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port Allocation Error."]
        #[must_use]
        #[inline(always)]
        pub const fn portallocerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port Allocation Error."]
        #[inline(always)]
        pub const fn set_portallocerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Single FIFO Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifoof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO Overflow."]
        #[inline(always)]
        pub const fn set_singlefifoof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Scan FIFO Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifoof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO Overflow."]
        #[inline(always)]
        pub const fn set_scanfifoof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Single FIFO Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifouf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO Underflow."]
        #[inline(always)]
        pub const fn set_singlefifouf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Scan FIFO Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifouf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO Underflow."]
        #[inline(always)]
        pub const fn set_scanfifouf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "EM2/3 Abort Error."]
        #[must_use]
        #[inline(always)]
        pub const fn em23aborterror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EM2/3 Abort Error."]
        #[inline(always)]
        pub const fn set_em23aborterror(&mut self, val: bool) {
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
                .field("singlefifodvl", &self.singlefifodvl())
                .field("scanfifodvl", &self.scanfifodvl())
                .field("singlecmp", &self.singlecmp())
                .field("scancmp", &self.scancmp())
                .field("scanentrydone", &self.scanentrydone())
                .field("scantabledone", &self.scantabledone())
                .field("singledone", &self.singledone())
                .field("polarityerr", &self.polarityerr())
                .field("portallocerr", &self.portallocerr())
                .field("singlefifoof", &self.singlefifoof())
                .field("scanfifoof", &self.scanfifoof())
                .field("singlefifouf", &self.singlefifouf())
                .field("scanfifouf", &self.scanfifouf())
                .field("em23aborterror", &self.em23aborterror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ singlefifodvl: {=bool:?}, scanfifodvl: {=bool:?}, singlecmp: {=bool:?}, scancmp: {=bool:?}, scanentrydone: {=bool:?}, scantabledone: {=bool:?}, singledone: {=bool:?}, polarityerr: {=bool:?}, portallocerr: {=bool:?}, singlefifoof: {=bool:?}, scanfifoof: {=bool:?}, singlefifouf: {=bool:?}, scanfifouf: {=bool:?}, em23aborterror: {=bool:?} }}",
                self.singlefifodvl(),
                self.scanfifodvl(),
                self.singlecmp(),
                self.scancmp(),
                self.scanentrydone(),
                self.scantabledone(),
                self.singledone(),
                self.polarityerr(),
                self.portallocerr(),
                self.singlefifoof(),
                self.scanfifoof(),
                self.singlefifouf(),
                self.scanfifouf(),
                self.em23aborterror()
            )
        }
    }
    #[doc = "IPVERSION."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "IP version ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP version ID."]
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
    #[doc = "Mask Request."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maskreq(pub u32);
    impl Maskreq {
        #[doc = "Scan Queue Mask Request."]
        #[must_use]
        #[inline(always)]
        pub const fn maskreq(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Scan Queue Mask Request."]
        #[inline(always)]
        pub const fn set_maskreq(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Maskreq {
        #[inline(always)]
        fn default() -> Maskreq {
            Maskreq(0)
        }
    }
    impl core::fmt::Debug for Maskreq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maskreq").field("maskreq", &self.maskreq()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maskreq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maskreq {{ maskreq: {=u16:?} }}", self.maskreq())
        }
    }
    #[doc = "Scale."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scale0(pub u32);
    impl Scale0 {
        #[doc = "Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Offset."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "Gain 13 LSBs."]
        #[must_use]
        #[inline(always)]
        pub const fn gain13lsb(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x1fff;
            val as u16
        }
        #[doc = "Gain 13 LSBs."]
        #[inline(always)]
        pub const fn set_gain13lsb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 18usize)) | (((val as u32) & 0x1fff) << 18usize);
        }
        #[doc = "Gain 3 MSBs."]
        #[must_use]
        #[inline(always)]
        pub const fn gain3msb(&self) -> super::vals::Scale0Gain3msb {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Scale0Gain3msb::from_bits(val as u8)
        }
        #[doc = "Gain 3 MSBs."]
        #[inline(always)]
        pub const fn set_gain3msb(&mut self, val: super::vals::Scale0Gain3msb) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Scale0 {
        #[inline(always)]
        fn default() -> Scale0 {
            Scale0(0)
        }
    }
    impl core::fmt::Debug for Scale0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scale0")
                .field("offset", &self.offset())
                .field("gain13lsb", &self.gain13lsb())
                .field("gain3msb", &self.gain3msb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scale0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scale0 {{ offset: {=u32:?}, gain13lsb: {=u16:?}, gain3msb: {:?} }}",
                self.offset(),
                self.gain13lsb(),
                self.gain3msb()
            )
        }
    }
    #[doc = "Scale."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scale1(pub u32);
    impl Scale1 {
        #[doc = "Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Offset."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "Gain 13 LSBs."]
        #[must_use]
        #[inline(always)]
        pub const fn gain13lsb(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x1fff;
            val as u16
        }
        #[doc = "Gain 13 LSBs."]
        #[inline(always)]
        pub const fn set_gain13lsb(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 18usize)) | (((val as u32) & 0x1fff) << 18usize);
        }
        #[doc = "Gain 3 MSBs."]
        #[must_use]
        #[inline(always)]
        pub const fn gain3msb(&self) -> super::vals::Scale1Gain3msb {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Scale1Gain3msb::from_bits(val as u8)
        }
        #[doc = "Gain 3 MSBs."]
        #[inline(always)]
        pub const fn set_gain3msb(&mut self, val: super::vals::Scale1Gain3msb) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Scale1 {
        #[inline(always)]
        fn default() -> Scale1 {
            Scale1(0)
        }
    }
    impl core::fmt::Debug for Scale1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scale1")
                .field("offset", &self.offset())
                .field("gain13lsb", &self.gain13lsb())
                .field("gain3msb", &self.gain3msb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scale1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scale1 {{ offset: {=u32:?}, gain13lsb: {=u16:?}, gain3msb: {:?} }}",
                self.offset(),
                self.gain13lsb(),
                self.gain3msb()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan0(pub u32);
    impl Scan0 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan0Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan0Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan0Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan0Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan0Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan0Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan0Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan0Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan0Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan0 {
        #[inline(always)]
        fn default() -> Scan0 {
            Scan0(0)
        }
    }
    impl core::fmt::Debug for Scan0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan0")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan0 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan1(pub u32);
    impl Scan1 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan1Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan1Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan1Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan1Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan1Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan1Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan1Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan1Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan1Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan1 {
        #[inline(always)]
        fn default() -> Scan1 {
            Scan1(0)
        }
    }
    impl core::fmt::Debug for Scan1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan1")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan1 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan10(pub u32);
    impl Scan10 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan10Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan10Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan10Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan10Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan10Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan10Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan10Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan10Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan10Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan10 {
        #[inline(always)]
        fn default() -> Scan10 {
            Scan10(0)
        }
    }
    impl core::fmt::Debug for Scan10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan10")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan10 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan11(pub u32);
    impl Scan11 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan11Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan11Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan11Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan11Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan11Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan11Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan11Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan11Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan11Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan11 {
        #[inline(always)]
        fn default() -> Scan11 {
            Scan11(0)
        }
    }
    impl core::fmt::Debug for Scan11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan11")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan11 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan12(pub u32);
    impl Scan12 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan12Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan12Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan12Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan12Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan12Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan12Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan12Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan12Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan12Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan12 {
        #[inline(always)]
        fn default() -> Scan12 {
            Scan12(0)
        }
    }
    impl core::fmt::Debug for Scan12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan12")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan12 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan13(pub u32);
    impl Scan13 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan13Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan13Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan13Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan13Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan13Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan13Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan13Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan13Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan13Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan13 {
        #[inline(always)]
        fn default() -> Scan13 {
            Scan13(0)
        }
    }
    impl core::fmt::Debug for Scan13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan13")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan13 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan14(pub u32);
    impl Scan14 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan14Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan14Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan14Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan14Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan14Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan14Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan14Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan14Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan14Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan14 {
        #[inline(always)]
        fn default() -> Scan14 {
            Scan14(0)
        }
    }
    impl core::fmt::Debug for Scan14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan14")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan14 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan15(pub u32);
    impl Scan15 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan15Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan15Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan15Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan15Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan15Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan15Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan15Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan15Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan15Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan15 {
        #[inline(always)]
        fn default() -> Scan15 {
            Scan15(0)
        }
    }
    impl core::fmt::Debug for Scan15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan15")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan15 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan2(pub u32);
    impl Scan2 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan2Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan2Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan2Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan2Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan2Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan2Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan2Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan2Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan2Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan2 {
        #[inline(always)]
        fn default() -> Scan2 {
            Scan2(0)
        }
    }
    impl core::fmt::Debug for Scan2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan2")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan2 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan3(pub u32);
    impl Scan3 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan3Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan3Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan3Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan3Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan3Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan3Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan3Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan3Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan3Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan3 {
        #[inline(always)]
        fn default() -> Scan3 {
            Scan3(0)
        }
    }
    impl core::fmt::Debug for Scan3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan3")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan3 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan4(pub u32);
    impl Scan4 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan4Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan4Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan4Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan4Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan4Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan4Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan4Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan4Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan4Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan4 {
        #[inline(always)]
        fn default() -> Scan4 {
            Scan4(0)
        }
    }
    impl core::fmt::Debug for Scan4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan4")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan4 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan5(pub u32);
    impl Scan5 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan5Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan5Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan5Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan5Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan5Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan5Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan5Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan5Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan5Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan5 {
        #[inline(always)]
        fn default() -> Scan5 {
            Scan5(0)
        }
    }
    impl core::fmt::Debug for Scan5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan5")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan5 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan6(pub u32);
    impl Scan6 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan6Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan6Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan6Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan6Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan6Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan6Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan6Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan6Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan6Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan6 {
        #[inline(always)]
        fn default() -> Scan6 {
            Scan6(0)
        }
    }
    impl core::fmt::Debug for Scan6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan6")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan6 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan7(pub u32);
    impl Scan7 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan7Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan7Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan7Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan7Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan7Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan7Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan7Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan7Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan7Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan7 {
        #[inline(always)]
        fn default() -> Scan7 {
            Scan7(0)
        }
    }
    impl core::fmt::Debug for Scan7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan7")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan7 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan8(pub u32);
    impl Scan8 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan8Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan8Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan8Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan8Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan8Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan8Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan8Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan8Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan8Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan8 {
        #[inline(always)]
        fn default() -> Scan8 {
            Scan8(0)
        }
    }
    impl core::fmt::Debug for Scan8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan8")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan8 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scan9(pub u32);
    impl Scan9 {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::Scan9Portneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Scan9Portneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::Scan9Portneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::Scan9Portpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Scan9Portpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::Scan9Portpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::Scan9Cfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Scan9Cfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::Scan9Cfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Scan9 {
        #[inline(always)]
        fn default() -> Scan9 {
            Scan9(0)
        }
    }
    impl core::fmt::Debug for Scan9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scan9")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scan9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scan9 {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "Most recent data data from scan queue conversion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scandata(pub u32);
    impl Scandata {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Scandata {
        #[inline(always)]
        fn default() -> Scandata {
            Scandata(0)
        }
    }
    impl core::fmt::Debug for Scandata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scandata").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scandata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Scandata {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Scan FIFO Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scanfifocfg(pub u32);
    impl Scanfifocfg {
        #[doc = "Alignment."]
        #[must_use]
        #[inline(always)]
        pub const fn alignment(&self) -> super::vals::ScanfifocfgAlignment {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::ScanfifocfgAlignment::from_bits(val as u8)
        }
        #[doc = "Alignment."]
        #[inline(always)]
        pub const fn set_alignment(&mut self, val: super::vals::ScanfifocfgAlignment) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Show ID."]
        #[must_use]
        #[inline(always)]
        pub const fn showid(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Show ID."]
        #[inline(always)]
        pub const fn set_showid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data Valid Level."]
        #[must_use]
        #[inline(always)]
        pub const fn dvl(&self) -> super::vals::ScanfifocfgDvl {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::ScanfifocfgDvl::from_bits(val as u8)
        }
        #[doc = "Data Valid Level."]
        #[inline(always)]
        pub const fn set_dvl(&mut self, val: super::vals::ScanfifocfgDvl) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Scan FIFO DMA Wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn dmawufifoscan(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Scan FIFO DMA Wakeup."]
        #[inline(always)]
        pub const fn set_dmawufifoscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Scanfifocfg {
        #[inline(always)]
        fn default() -> Scanfifocfg {
            Scanfifocfg(0)
        }
    }
    impl core::fmt::Debug for Scanfifocfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scanfifocfg")
                .field("alignment", &self.alignment())
                .field("showid", &self.showid())
                .field("dvl", &self.dvl())
                .field("dmawufifoscan", &self.dmawufifoscan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scanfifocfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scanfifocfg {{ alignment: {:?}, showid: {=bool:?}, dvl: {:?}, dmawufifoscan: {=bool:?} }}",
                self.alignment(),
                self.showid(),
                self.dvl(),
                self.dmawufifoscan()
            )
        }
    }
    #[doc = "Read the oldest valid data from the scan FIFO and pop the FIFO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scanfifodata(pub u32);
    impl Scanfifodata {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Scanfifodata {
        #[inline(always)]
        fn default() -> Scanfifodata {
            Scanfifodata(0)
        }
    }
    impl core::fmt::Debug for Scanfifodata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scanfifodata").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scanfifodata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Scanfifodata {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Scan FIFO status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scanfifostat(pub u32);
    impl Scanfifostat {
        #[doc = "FIFO Read Count."]
        #[must_use]
        #[inline(always)]
        pub const fn fiforeadcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "FIFO Read Count."]
        #[inline(always)]
        pub const fn set_fiforeadcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Scanfifostat {
        #[inline(always)]
        fn default() -> Scanfifostat {
            Scanfifostat(0)
        }
    }
    impl core::fmt::Debug for Scanfifostat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scanfifostat")
                .field("fiforeadcnt", &self.fiforeadcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scanfifostat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Scanfifostat {{ fiforeadcnt: {=u8:?} }}", self.fiforeadcnt())
        }
    }
    #[doc = "Scheduling."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sched0(pub u32);
    impl Sched0 {
        #[doc = "Prescale."]
        #[must_use]
        #[inline(always)]
        pub const fn prescale(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Prescale."]
        #[inline(always)]
        pub const fn set_prescale(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Sched0 {
        #[inline(always)]
        fn default() -> Sched0 {
            Sched0(0)
        }
    }
    impl core::fmt::Debug for Sched0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sched0").field("prescale", &self.prescale()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sched0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sched0 {{ prescale: {=u16:?} }}", self.prescale())
        }
    }
    #[doc = "Scheduling."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sched1(pub u32);
    impl Sched1 {
        #[doc = "Prescale."]
        #[must_use]
        #[inline(always)]
        pub const fn prescale(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Prescale."]
        #[inline(always)]
        pub const fn set_prescale(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Sched1 {
        #[inline(always)]
        fn default() -> Sched1 {
            Sched1(0)
        }
    }
    impl core::fmt::Debug for Sched1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sched1").field("prescale", &self.prescale()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sched1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sched1 {{ prescale: {=u16:?} }}", self.prescale())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Single(pub u32);
    impl Single {
        #[doc = "Negative Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinneg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Negative Pin Select."]
        #[inline(always)]
        pub const fn set_pinneg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Negative Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portneg(&self) -> super::vals::SinglePortneg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::SinglePortneg::from_bits(val as u8)
        }
        #[doc = "Negative Port Select."]
        #[inline(always)]
        pub const fn set_portneg(&mut self, val: super::vals::SinglePortneg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Positive Pin Select."]
        #[must_use]
        #[inline(always)]
        pub const fn pinpos(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Positive Pin Select."]
        #[inline(always)]
        pub const fn set_pinpos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Positive Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn portpos(&self) -> super::vals::SinglePortpos {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::SinglePortpos::from_bits(val as u8)
        }
        #[doc = "Positive Port Select."]
        #[inline(always)]
        pub const fn set_portpos(&mut self, val: super::vals::SinglePortpos) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Configuration Group Select."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg(&self) -> super::vals::SingleCfg {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::SingleCfg::from_bits(val as u8)
        }
        #[doc = "Configuration Group Select."]
        #[inline(always)]
        pub const fn set_cfg(&mut self, val: super::vals::SingleCfg) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Comparison Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Comparison Enable."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Single {
        #[inline(always)]
        fn default() -> Single {
            Single(0)
        }
    }
    impl core::fmt::Debug for Single {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Single")
                .field("pinneg", &self.pinneg())
                .field("portneg", &self.portneg())
                .field("pinpos", &self.pinpos())
                .field("portpos", &self.portpos())
                .field("cfg", &self.cfg())
                .field("cmp", &self.cmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Single {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Single {{ pinneg: {=u8:?}, portneg: {:?}, pinpos: {=u8:?}, portpos: {:?}, cfg: {:?}, cmp: {=bool:?} }}",
                self.pinneg(),
                self.portneg(),
                self.pinpos(),
                self.portpos(),
                self.cfg(),
                self.cmp()
            )
        }
    }
    #[doc = "latest single queue conversion data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Singledata(pub u32);
    impl Singledata {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Singledata {
        #[inline(always)]
        fn default() -> Singledata {
            Singledata(0)
        }
    }
    impl core::fmt::Debug for Singledata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Singledata").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Singledata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Singledata {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Single FIFO Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Singlefifocfg(pub u32);
    impl Singlefifocfg {
        #[doc = "Alignment."]
        #[must_use]
        #[inline(always)]
        pub const fn alignment(&self) -> super::vals::SinglefifocfgAlignment {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SinglefifocfgAlignment::from_bits(val as u8)
        }
        #[doc = "Alignment."]
        #[inline(always)]
        pub const fn set_alignment(&mut self, val: super::vals::SinglefifocfgAlignment) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Show ID."]
        #[must_use]
        #[inline(always)]
        pub const fn showid(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Show ID."]
        #[inline(always)]
        pub const fn set_showid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data Valid Level."]
        #[must_use]
        #[inline(always)]
        pub const fn dvl(&self) -> super::vals::SinglefifocfgDvl {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::SinglefifocfgDvl::from_bits(val as u8)
        }
        #[doc = "Data Valid Level."]
        #[inline(always)]
        pub const fn set_dvl(&mut self, val: super::vals::SinglefifocfgDvl) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Single FIFO DMA wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn dmawufifosingle(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Single FIFO DMA wakeup."]
        #[inline(always)]
        pub const fn set_dmawufifosingle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Singlefifocfg {
        #[inline(always)]
        fn default() -> Singlefifocfg {
            Singlefifocfg(0)
        }
    }
    impl core::fmt::Debug for Singlefifocfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Singlefifocfg")
                .field("alignment", &self.alignment())
                .field("showid", &self.showid())
                .field("dvl", &self.dvl())
                .field("dmawufifosingle", &self.dmawufifosingle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Singlefifocfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Singlefifocfg {{ alignment: {:?}, showid: {=bool:?}, dvl: {:?}, dmawufifosingle: {=bool:?} }}",
                self.alignment(),
                self.showid(),
                self.dvl(),
                self.dmawufifosingle()
            )
        }
    }
    #[doc = "Read the oldest valid data from the single FIFO and pop the FIFO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Singlefifodata(pub u32);
    impl Singlefifodata {
        #[doc = "Single FIFO Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single FIFO Read Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Singlefifodata {
        #[inline(always)]
        fn default() -> Singlefifodata {
            Singlefifodata(0)
        }
    }
    impl core::fmt::Debug for Singlefifodata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Singlefifodata").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Singlefifodata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Singlefifodata {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Single FIFO status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Singlefifostat(pub u32);
    impl Singlefifostat {
        #[doc = "FIFO Read Count."]
        #[must_use]
        #[inline(always)]
        pub const fn fiforeadcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "FIFO Read Count."]
        #[inline(always)]
        pub const fn set_fiforeadcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Singlefifostat {
        #[inline(always)]
        fn default() -> Singlefifostat {
            Singlefifostat(0)
        }
    }
    impl core::fmt::Debug for Singlefifostat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Singlefifostat")
                .field("fiforeadcnt", &self.fiforeadcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Singlefifostat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Singlefifostat {{ fiforeadcnt: {=u8:?} }}", self.fiforeadcnt())
        }
    }
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Single Queue Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn singleqen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Single Queue Enabled."]
        #[inline(always)]
        pub const fn set_singleqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Single Queue Pending."]
        #[must_use]
        #[inline(always)]
        pub const fn singlequeuepending(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Single Queue Pending."]
        #[inline(always)]
        pub const fn set_singlequeuepending(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Scan Queued Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn scanqen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Queued Enabled."]
        #[inline(always)]
        pub const fn set_scanqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scan Queue Pending."]
        #[must_use]
        #[inline(always)]
        pub const fn scanqueuepending(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Scan Queue Pending."]
        #[inline(always)]
        pub const fn set_scanqueuepending(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Converting."]
        #[must_use]
        #[inline(always)]
        pub const fn converting(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Converting."]
        #[inline(always)]
        pub const fn set_converting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SINGLEFIFO Data Valid."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifodv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SINGLEFIFO Data Valid."]
        #[inline(always)]
        pub const fn set_singlefifodv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SCANFIFO Data Valid."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifodv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SCANFIFO Data Valid."]
        #[inline(always)]
        pub const fn set_scanfifodv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "The Single FIFO is flushing."]
        #[must_use]
        #[inline(always)]
        pub const fn singlefifoflushing(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "The Single FIFO is flushing."]
        #[inline(always)]
        pub const fn set_singlefifoflushing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "The Scan FIFO is flushing."]
        #[must_use]
        #[inline(always)]
        pub const fn scanfifoflushing(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "The Scan FIFO is flushing."]
        #[inline(always)]
        pub const fn set_scanfifoflushing(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timer Active."]
        #[must_use]
        #[inline(always)]
        pub const fn timeractive(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Active."]
        #[inline(always)]
        pub const fn set_timeractive(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SINGLE write pending."]
        #[must_use]
        #[inline(always)]
        pub const fn singlewritepending(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SINGLE write pending."]
        #[inline(always)]
        pub const fn set_singlewritepending(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MASKREQ write pending."]
        #[must_use]
        #[inline(always)]
        pub const fn maskreqwritepending(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "MASKREQ write pending."]
        #[inline(always)]
        pub const fn set_maskreqwritepending(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SYNCBUSY."]
        #[must_use]
        #[inline(always)]
        pub const fn syncbusy(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCBUSY."]
        #[inline(always)]
        pub const fn set_syncbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADCWARM."]
        #[must_use]
        #[inline(always)]
        pub const fn adcwarm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ADCWARM."]
        #[inline(always)]
        pub const fn set_adcwarm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("singleqen", &self.singleqen())
                .field("singlequeuepending", &self.singlequeuepending())
                .field("scanqen", &self.scanqen())
                .field("scanqueuepending", &self.scanqueuepending())
                .field("converting", &self.converting())
                .field("singlefifodv", &self.singlefifodv())
                .field("scanfifodv", &self.scanfifodv())
                .field("singlefifoflushing", &self.singlefifoflushing())
                .field("scanfifoflushing", &self.scanfifoflushing())
                .field("timeractive", &self.timeractive())
                .field("singlewritepending", &self.singlewritepending())
                .field("maskreqwritepending", &self.maskreqwritepending())
                .field("syncbusy", &self.syncbusy())
                .field("adcwarm", &self.adcwarm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ singleqen: {=bool:?}, singlequeuepending: {=bool:?}, scanqen: {=bool:?}, scanqueuepending: {=bool:?}, converting: {=bool:?}, singlefifodv: {=bool:?}, scanfifodv: {=bool:?}, singlefifoflushing: {=bool:?}, scanfifoflushing: {=bool:?}, timeractive: {=bool:?}, singlewritepending: {=bool:?}, maskreqwritepending: {=bool:?}, syncbusy: {=bool:?}, adcwarm: {=bool:?} }}",
                self.singleqen(),
                self.singlequeuepending(),
                self.scanqen(),
                self.scanqueuepending(),
                self.converting(),
                self.singlefifodv(),
                self.scanfifodv(),
                self.singlefifoflushing(),
                self.scanfifoflushing(),
                self.timeractive(),
                self.singlewritepending(),
                self.maskreqwritepending(),
                self.syncbusy(),
                self.adcwarm()
            )
        }
    }
    #[doc = "Scan Table Mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stmask(pub u32);
    impl Stmask {
        #[doc = "Scan Table Mask."]
        #[must_use]
        #[inline(always)]
        pub const fn stmask(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Scan Table Mask."]
        #[inline(always)]
        pub const fn set_stmask(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Stmask {
        #[inline(always)]
        fn default() -> Stmask {
            Stmask(0)
        }
    }
    impl core::fmt::Debug for Stmask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stmask").field("stmask", &self.stmask()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stmask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Stmask {{ stmask: {=u16:?} }}", self.stmask())
        }
    }
    #[doc = "Timer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer(pub u32);
    impl Timer {
        #[doc = "Timer Period."]
        #[must_use]
        #[inline(always)]
        pub const fn timer(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Timer Period."]
        #[inline(always)]
        pub const fn set_timer(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Timer {
        #[inline(always)]
        fn default() -> Timer {
            Timer(0)
        }
    }
    impl core::fmt::Debug for Timer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer").field("timer", &self.timer()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Timer {{ timer: {=u16:?} }}", self.timer())
        }
    }
    #[doc = "Trigger."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trigger(pub u32);
    impl Trigger {
        #[doc = "Scan Trigger Select."]
        #[must_use]
        #[inline(always)]
        pub const fn scantrigsel(&self) -> super::vals::Scantrigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Scantrigsel::from_bits(val as u8)
        }
        #[doc = "Scan Trigger Select."]
        #[inline(always)]
        pub const fn set_scantrigsel(&mut self, val: super::vals::Scantrigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Scan Trigger Action."]
        #[must_use]
        #[inline(always)]
        pub const fn scantrigaction(&self) -> super::vals::Scantrigaction {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Scantrigaction::from_bits(val as u8)
        }
        #[doc = "Scan Trigger Action."]
        #[inline(always)]
        pub const fn set_scantrigaction(&mut self, val: super::vals::Scantrigaction) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Single Trigger Select."]
        #[must_use]
        #[inline(always)]
        pub const fn singletrigsel(&self) -> super::vals::Singletrigsel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Singletrigsel::from_bits(val as u8)
        }
        #[doc = "Single Trigger Select."]
        #[inline(always)]
        pub const fn set_singletrigsel(&mut self, val: super::vals::Singletrigsel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Single Trigger Action."]
        #[must_use]
        #[inline(always)]
        pub const fn singletrigaction(&self) -> super::vals::Singletrigaction {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Singletrigaction::from_bits(val as u8)
        }
        #[doc = "Single Trigger Action."]
        #[inline(always)]
        pub const fn set_singletrigaction(&mut self, val: super::vals::Singletrigaction) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Single Tailgate Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn singletailgate(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Single Tailgate Enable."]
        #[inline(always)]
        pub const fn set_singletailgate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Trigger {
        #[inline(always)]
        fn default() -> Trigger {
            Trigger(0)
        }
    }
    impl core::fmt::Debug for Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trigger")
                .field("scantrigsel", &self.scantrigsel())
                .field("scantrigaction", &self.scantrigaction())
                .field("singletrigsel", &self.singletrigsel())
                .field("singletrigaction", &self.singletrigaction())
                .field("singletailgate", &self.singletailgate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trigger {{ scantrigsel: {:?}, scantrigaction: {:?}, singletrigsel: {:?}, singletrigaction: {:?}, singletailgate: {=bool:?} }}",
                self.scantrigsel(),
                self.scantrigaction(),
                self.singletrigsel(),
                self.singletrigaction(),
                self.singletailgate()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg0Adcmode {
        #[doc = "High speed mode with a maximum ADC_CLK of 10 MHz."]
        Normal = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cfg0Adcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg0Adcmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg0Adcmode {
        #[inline(always)]
        fn from(val: u8) -> Cfg0Adcmode {
            Cfg0Adcmode::from_bits(val)
        }
    }
    impl From<Cfg0Adcmode> for u8 {
        #[inline(always)]
        fn from(val: Cfg0Adcmode) -> u8 {
            Cfg0Adcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg0Analoggain {
        _RESERVED_0 = 0x0,
        #[doc = "Analog gain of 0.5x."]
        Anagain0p5 = 0x01,
        #[doc = "Analog gain of 1x."]
        Anagain1 = 0x02,
        #[doc = "Analog gain of 2x."]
        Anagain2 = 0x03,
        #[doc = "Analog gain of 3x."]
        Anagain3 = 0x04,
        #[doc = "Analog gain of 4x."]
        Anagain4 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg0Analoggain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg0Analoggain {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg0Analoggain {
        #[inline(always)]
        fn from(val: u8) -> Cfg0Analoggain {
            Cfg0Analoggain::from_bits(val)
        }
    }
    impl From<Cfg0Analoggain> for u8 {
        #[inline(always)]
        fn from(val: Cfg0Analoggain) -> u8 {
            Cfg0Analoggain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg0Digavg {
        #[doc = "Collect one output word (no digital averaging)."]
        Avg1 = 0x0,
        #[doc = "Collect and average 2 digital output words."]
        Avg2 = 0x01,
        #[doc = "Collect and average 4 digital output words."]
        Avg4 = 0x02,
        #[doc = "Collect and average 8 digital output words."]
        Avg8 = 0x03,
        #[doc = "Collect and average 16 digital output words."]
        Avg16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg0Digavg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg0Digavg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg0Digavg {
        #[inline(always)]
        fn from(val: u8) -> Cfg0Digavg {
            Cfg0Digavg::from_bits(val)
        }
    }
    impl From<Cfg0Digavg> for u8 {
        #[inline(always)]
        fn from(val: Cfg0Digavg) -> u8 {
            Cfg0Digavg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg0Osrhs {
        #[doc = "High speed over sampling of 2x."]
        Hispd2 = 0x0,
        #[doc = "High speed over sampling of 4x."]
        Hispd4 = 0x01,
        #[doc = "High speed over sampling of 8x."]
        Hispd8 = 0x02,
        #[doc = "High speed over sampling of 16x."]
        Hispd16 = 0x03,
        #[doc = "HIgh speed over sampling of 32x."]
        Hispd32 = 0x04,
        #[doc = "High speed over sampling of 64x."]
        Hispd64 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg0Osrhs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg0Osrhs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg0Osrhs {
        #[inline(always)]
        fn from(val: u8) -> Cfg0Osrhs {
            Cfg0Osrhs::from_bits(val)
        }
    }
    impl From<Cfg0Osrhs> for u8 {
        #[inline(always)]
        fn from(val: Cfg0Osrhs) -> u8 {
            Cfg0Osrhs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg0Refsel {
        #[doc = "Internal 1.21 V reference."]
        Vbgr = 0x0,
        #[doc = "External Reference. (Calibrated for 1.25V nominal.)."]
        Vref = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "AVDD (unbuffered)."]
        Vddx = 0x03,
        #[doc = "AVDD (buffered) * 0.8."]
        Vddx0p8buf = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg0Refsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg0Refsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg0Refsel {
        #[inline(always)]
        fn from(val: u8) -> Cfg0Refsel {
            Cfg0Refsel::from_bits(val)
        }
    }
    impl From<Cfg0Refsel> for u8 {
        #[inline(always)]
        fn from(val: Cfg0Refsel) -> u8 {
            Cfg0Refsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg0Twoscompl {
        #[doc = "Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
        Auto = 0x0,
        #[doc = "Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
        Forceunipolar = 0x01,
        #[doc = "Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
        Forcebipolar = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cfg0Twoscompl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg0Twoscompl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg0Twoscompl {
        #[inline(always)]
        fn from(val: u8) -> Cfg0Twoscompl {
            Cfg0Twoscompl::from_bits(val)
        }
    }
    impl From<Cfg0Twoscompl> for u8 {
        #[inline(always)]
        fn from(val: Cfg0Twoscompl) -> u8 {
            Cfg0Twoscompl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg1Adcmode {
        #[doc = "High speed mode with a maximum ADC_CLK of 10 MHz."]
        Normal = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cfg1Adcmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg1Adcmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg1Adcmode {
        #[inline(always)]
        fn from(val: u8) -> Cfg1Adcmode {
            Cfg1Adcmode::from_bits(val)
        }
    }
    impl From<Cfg1Adcmode> for u8 {
        #[inline(always)]
        fn from(val: Cfg1Adcmode) -> u8 {
            Cfg1Adcmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg1Analoggain {
        _RESERVED_0 = 0x0,
        #[doc = "Analog gain of 0.5x."]
        Anagain0p5 = 0x01,
        #[doc = "Analog gain of 1x."]
        Anagain1 = 0x02,
        #[doc = "Analog gain of 2x."]
        Anagain2 = 0x03,
        #[doc = "Analog gain of 3x."]
        Anagain3 = 0x04,
        #[doc = "Analog gain of 4x."]
        Anagain4 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg1Analoggain {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg1Analoggain {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg1Analoggain {
        #[inline(always)]
        fn from(val: u8) -> Cfg1Analoggain {
            Cfg1Analoggain::from_bits(val)
        }
    }
    impl From<Cfg1Analoggain> for u8 {
        #[inline(always)]
        fn from(val: Cfg1Analoggain) -> u8 {
            Cfg1Analoggain::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg1Digavg {
        #[doc = "Collect one output word (no digital averaging)."]
        Avg1 = 0x0,
        #[doc = "Collect and average 2 digital output words."]
        Avg2 = 0x01,
        #[doc = "Collect and average 4 digital output words."]
        Avg4 = 0x02,
        #[doc = "Collect and average 8 digital output words."]
        Avg8 = 0x03,
        #[doc = "Collect and average 16 digital output words."]
        Avg16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg1Digavg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg1Digavg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg1Digavg {
        #[inline(always)]
        fn from(val: u8) -> Cfg1Digavg {
            Cfg1Digavg::from_bits(val)
        }
    }
    impl From<Cfg1Digavg> for u8 {
        #[inline(always)]
        fn from(val: Cfg1Digavg) -> u8 {
            Cfg1Digavg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg1Osrhs {
        #[doc = "High speed over sampling of 2x."]
        Hispd2 = 0x0,
        #[doc = "High speed over sampling of 4x."]
        Hispd4 = 0x01,
        #[doc = "High speed over sampling of 8x."]
        Hispd8 = 0x02,
        #[doc = "High speed over sampling of 16x."]
        Hispd16 = 0x03,
        #[doc = "HIgh speed over sampling of 32x."]
        Hispd32 = 0x04,
        #[doc = "High speed over sampling of 64x."]
        Hispd64 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg1Osrhs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg1Osrhs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg1Osrhs {
        #[inline(always)]
        fn from(val: u8) -> Cfg1Osrhs {
            Cfg1Osrhs::from_bits(val)
        }
    }
    impl From<Cfg1Osrhs> for u8 {
        #[inline(always)]
        fn from(val: Cfg1Osrhs) -> u8 {
            Cfg1Osrhs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg1Refsel {
        #[doc = "Internal 1.21 V reference."]
        Vbgr = 0x0,
        #[doc = "External Reference. (Calibrated for 1.25V nominal.)."]
        Vref = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "AVDD (unbuffered)."]
        Vddx = 0x03,
        #[doc = "AVDD (buffered) * 0.8."]
        Vddx0p8buf = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cfg1Refsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg1Refsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg1Refsel {
        #[inline(always)]
        fn from(val: u8) -> Cfg1Refsel {
            Cfg1Refsel::from_bits(val)
        }
    }
    impl From<Cfg1Refsel> for u8 {
        #[inline(always)]
        fn from(val: Cfg1Refsel) -> u8 {
            Cfg1Refsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cfg1Twoscompl {
        #[doc = "Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
        Auto = 0x0,
        #[doc = "Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
        Forceunipolar = 0x01,
        #[doc = "Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
        Forcebipolar = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cfg1Twoscompl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfg1Twoscompl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfg1Twoscompl {
        #[inline(always)]
        fn from(val: u8) -> Cfg1Twoscompl {
            Cfg1Twoscompl::from_bits(val)
        }
    }
    impl From<Cfg1Twoscompl> for u8 {
        #[inline(always)]
        fn from(val: Cfg1Twoscompl) -> u8 {
            Cfg1Twoscompl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dbghalt {
        #[doc = "Continue operation as normal during debug mode."]
        Normal = 0x0,
        #[doc = "Complete the current conversion and then halt during debug mode."]
        Halt = 0x01,
    }
    impl Dbghalt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbghalt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbghalt {
        #[inline(always)]
        fn from(val: u8) -> Dbghalt {
            Dbghalt::from_bits(val)
        }
    }
    impl From<Dbghalt> for u8 {
        #[inline(always)]
        fn from(val: Dbghalt) -> u8 {
            Dbghalt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Em23wuconvert {
        #[doc = "When using suspend mode, conversions performed in EM2 or EM3 should not wake up the DMA until the FIFO's DVL setting is reached. This saves more power for large OSR settings or infrequent sampling."]
        Wudvl = 0x0,
        #[doc = "When using suspend mode, conversions performed in EM2 or EM3 will wake up the DMA and keep it awake until the conversions are done, regardless of the DVL setting. This mode burns more power, but it is useful when the conversion rate is faster than the time for the DMA to cycle through wake up and going back to sleep as it converts more than 4 scan table entries. Without using the wake up on conversion mode, the FIFO may overflow while the DMA is going in and out of sleep."]
        Wuconvert = 0x01,
    }
    impl Em23wuconvert {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Em23wuconvert {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Em23wuconvert {
        #[inline(always)]
        fn from(val: u8) -> Em23wuconvert {
            Em23wuconvert::from_bits(val)
        }
    }
    impl From<Em23wuconvert> for u8 {
        #[inline(always)]
        fn from(val: Em23wuconvert) -> u8 {
            Em23wuconvert::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsclkrate {
        #[doc = "Use CMU_CLK_ADC directly. The source clock must be 40 MHz or less."]
        Div1 = 0x0,
        #[doc = "Divide CMU_CLK_ADC by 2 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
        Div2 = 0x01,
        #[doc = "Divide CMU_CLK_ADC by 3 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
        Div3 = 0x02,
        #[doc = "Divide CMU_CLK_ADC by 4 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
        Div4 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Hsclkrate {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsclkrate {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsclkrate {
        #[inline(always)]
        fn from(val: u8) -> Hsclkrate {
            Hsclkrate::from_bits(val)
        }
    }
    impl From<Hsclkrate> for u8 {
        #[inline(always)]
        fn from(val: Hsclkrate) -> u8 {
            Hsclkrate::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scale0Gain3msb {
        #[doc = "Upper 3 bits of gain = 011 (0.75x)."]
        Gain011 = 0x0,
        #[doc = "Upper 3 bits of gain = 100 (1.00x)."]
        Gain100 = 0x01,
    }
    impl Scale0Gain3msb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scale0Gain3msb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scale0Gain3msb {
        #[inline(always)]
        fn from(val: u8) -> Scale0Gain3msb {
            Scale0Gain3msb::from_bits(val)
        }
    }
    impl From<Scale0Gain3msb> for u8 {
        #[inline(always)]
        fn from(val: Scale0Gain3msb) -> u8 {
            Scale0Gain3msb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scale1Gain3msb {
        #[doc = "Upper 3 bits of gain = 011 (0.75x)."]
        Gain011 = 0x0,
        #[doc = "Upper 3 bits of gain = 100 (1.00x)."]
        Gain100 = 0x01,
    }
    impl Scale1Gain3msb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scale1Gain3msb {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scale1Gain3msb {
        #[inline(always)]
        fn from(val: u8) -> Scale1Gain3msb {
            Scale1Gain3msb::from_bits(val)
        }
    }
    impl From<Scale1Gain3msb> for u8 {
        #[inline(always)]
        fn from(val: Scale1Gain3msb) -> u8 {
            Scale1Gain3msb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan0Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan0Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan0Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan0Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan0Cfg {
            Scan0Cfg::from_bits(val)
        }
    }
    impl From<Scan0Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan0Cfg) -> u8 {
            Scan0Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan0Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan0Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan0Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan0Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan0Portneg {
            Scan0Portneg::from_bits(val)
        }
    }
    impl From<Scan0Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan0Portneg) -> u8 {
            Scan0Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan0Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan0Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan0Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan0Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan0Portpos {
            Scan0Portpos::from_bits(val)
        }
    }
    impl From<Scan0Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan0Portpos) -> u8 {
            Scan0Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan10Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan10Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan10Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan10Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan10Cfg {
            Scan10Cfg::from_bits(val)
        }
    }
    impl From<Scan10Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan10Cfg) -> u8 {
            Scan10Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan10Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan10Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan10Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan10Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan10Portneg {
            Scan10Portneg::from_bits(val)
        }
    }
    impl From<Scan10Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan10Portneg) -> u8 {
            Scan10Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan10Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan10Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan10Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan10Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan10Portpos {
            Scan10Portpos::from_bits(val)
        }
    }
    impl From<Scan10Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan10Portpos) -> u8 {
            Scan10Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan11Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan11Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan11Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan11Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan11Cfg {
            Scan11Cfg::from_bits(val)
        }
    }
    impl From<Scan11Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan11Cfg) -> u8 {
            Scan11Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan11Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan11Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan11Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan11Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan11Portneg {
            Scan11Portneg::from_bits(val)
        }
    }
    impl From<Scan11Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan11Portneg) -> u8 {
            Scan11Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan11Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan11Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan11Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan11Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan11Portpos {
            Scan11Portpos::from_bits(val)
        }
    }
    impl From<Scan11Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan11Portpos) -> u8 {
            Scan11Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan12Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan12Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan12Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan12Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan12Cfg {
            Scan12Cfg::from_bits(val)
        }
    }
    impl From<Scan12Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan12Cfg) -> u8 {
            Scan12Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan12Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan12Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan12Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan12Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan12Portneg {
            Scan12Portneg::from_bits(val)
        }
    }
    impl From<Scan12Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan12Portneg) -> u8 {
            Scan12Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan12Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan12Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan12Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan12Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan12Portpos {
            Scan12Portpos::from_bits(val)
        }
    }
    impl From<Scan12Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan12Portpos) -> u8 {
            Scan12Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan13Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan13Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan13Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan13Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan13Cfg {
            Scan13Cfg::from_bits(val)
        }
    }
    impl From<Scan13Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan13Cfg) -> u8 {
            Scan13Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan13Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan13Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan13Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan13Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan13Portneg {
            Scan13Portneg::from_bits(val)
        }
    }
    impl From<Scan13Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan13Portneg) -> u8 {
            Scan13Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan13Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan13Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan13Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan13Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan13Portpos {
            Scan13Portpos::from_bits(val)
        }
    }
    impl From<Scan13Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan13Portpos) -> u8 {
            Scan13Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan14Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan14Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan14Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan14Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan14Cfg {
            Scan14Cfg::from_bits(val)
        }
    }
    impl From<Scan14Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan14Cfg) -> u8 {
            Scan14Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan14Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan14Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan14Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan14Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan14Portneg {
            Scan14Portneg::from_bits(val)
        }
    }
    impl From<Scan14Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan14Portneg) -> u8 {
            Scan14Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan14Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan14Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan14Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan14Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan14Portpos {
            Scan14Portpos::from_bits(val)
        }
    }
    impl From<Scan14Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan14Portpos) -> u8 {
            Scan14Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan15Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan15Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan15Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan15Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan15Cfg {
            Scan15Cfg::from_bits(val)
        }
    }
    impl From<Scan15Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan15Cfg) -> u8 {
            Scan15Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan15Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan15Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan15Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan15Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan15Portneg {
            Scan15Portneg::from_bits(val)
        }
    }
    impl From<Scan15Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan15Portneg) -> u8 {
            Scan15Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan15Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan15Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan15Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan15Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan15Portpos {
            Scan15Portpos::from_bits(val)
        }
    }
    impl From<Scan15Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan15Portpos) -> u8 {
            Scan15Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan1Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan1Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan1Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan1Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan1Cfg {
            Scan1Cfg::from_bits(val)
        }
    }
    impl From<Scan1Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan1Cfg) -> u8 {
            Scan1Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan1Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan1Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan1Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan1Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan1Portneg {
            Scan1Portneg::from_bits(val)
        }
    }
    impl From<Scan1Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan1Portneg) -> u8 {
            Scan1Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan1Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan1Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan1Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan1Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan1Portpos {
            Scan1Portpos::from_bits(val)
        }
    }
    impl From<Scan1Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan1Portpos) -> u8 {
            Scan1Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan2Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan2Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan2Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan2Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan2Cfg {
            Scan2Cfg::from_bits(val)
        }
    }
    impl From<Scan2Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan2Cfg) -> u8 {
            Scan2Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan2Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan2Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan2Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan2Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan2Portneg {
            Scan2Portneg::from_bits(val)
        }
    }
    impl From<Scan2Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan2Portneg) -> u8 {
            Scan2Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan2Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan2Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan2Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan2Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan2Portpos {
            Scan2Portpos::from_bits(val)
        }
    }
    impl From<Scan2Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan2Portpos) -> u8 {
            Scan2Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan3Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan3Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan3Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan3Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan3Cfg {
            Scan3Cfg::from_bits(val)
        }
    }
    impl From<Scan3Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan3Cfg) -> u8 {
            Scan3Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan3Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan3Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan3Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan3Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan3Portneg {
            Scan3Portneg::from_bits(val)
        }
    }
    impl From<Scan3Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan3Portneg) -> u8 {
            Scan3Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan3Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan3Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan3Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan3Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan3Portpos {
            Scan3Portpos::from_bits(val)
        }
    }
    impl From<Scan3Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan3Portpos) -> u8 {
            Scan3Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan4Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan4Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan4Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan4Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan4Cfg {
            Scan4Cfg::from_bits(val)
        }
    }
    impl From<Scan4Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan4Cfg) -> u8 {
            Scan4Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan4Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan4Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan4Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan4Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan4Portneg {
            Scan4Portneg::from_bits(val)
        }
    }
    impl From<Scan4Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan4Portneg) -> u8 {
            Scan4Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan4Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan4Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan4Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan4Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan4Portpos {
            Scan4Portpos::from_bits(val)
        }
    }
    impl From<Scan4Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan4Portpos) -> u8 {
            Scan4Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan5Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan5Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan5Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan5Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan5Cfg {
            Scan5Cfg::from_bits(val)
        }
    }
    impl From<Scan5Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan5Cfg) -> u8 {
            Scan5Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan5Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan5Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan5Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan5Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan5Portneg {
            Scan5Portneg::from_bits(val)
        }
    }
    impl From<Scan5Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan5Portneg) -> u8 {
            Scan5Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan5Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan5Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan5Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan5Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan5Portpos {
            Scan5Portpos::from_bits(val)
        }
    }
    impl From<Scan5Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan5Portpos) -> u8 {
            Scan5Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan6Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan6Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan6Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan6Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan6Cfg {
            Scan6Cfg::from_bits(val)
        }
    }
    impl From<Scan6Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan6Cfg) -> u8 {
            Scan6Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan6Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan6Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan6Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan6Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan6Portneg {
            Scan6Portneg::from_bits(val)
        }
    }
    impl From<Scan6Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan6Portneg) -> u8 {
            Scan6Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan6Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan6Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan6Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan6Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan6Portpos {
            Scan6Portpos::from_bits(val)
        }
    }
    impl From<Scan6Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan6Portpos) -> u8 {
            Scan6Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan7Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan7Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan7Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan7Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan7Cfg {
            Scan7Cfg::from_bits(val)
        }
    }
    impl From<Scan7Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan7Cfg) -> u8 {
            Scan7Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan7Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan7Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan7Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan7Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan7Portneg {
            Scan7Portneg::from_bits(val)
        }
    }
    impl From<Scan7Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan7Portneg) -> u8 {
            Scan7Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan7Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan7Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan7Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan7Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan7Portpos {
            Scan7Portpos::from_bits(val)
        }
    }
    impl From<Scan7Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan7Portpos) -> u8 {
            Scan7Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan8Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan8Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan8Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan8Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan8Cfg {
            Scan8Cfg::from_bits(val)
        }
    }
    impl From<Scan8Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan8Cfg) -> u8 {
            Scan8Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan8Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan8Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan8Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan8Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan8Portneg {
            Scan8Portneg::from_bits(val)
        }
    }
    impl From<Scan8Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan8Portneg) -> u8 {
            Scan8Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan8Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan8Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan8Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan8Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan8Portpos {
            Scan8Portpos::from_bits(val)
        }
    }
    impl From<Scan8Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan8Portpos) -> u8 {
            Scan8Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan9Cfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl Scan9Cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan9Cfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan9Cfg {
        #[inline(always)]
        fn from(val: u8) -> Scan9Cfg {
            Scan9Cfg::from_bits(val)
        }
    }
    impl From<Scan9Cfg> for u8 {
        #[inline(always)]
        fn from(val: Scan9Cfg) -> u8 {
            Scan9Cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan9Portneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan9Portneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan9Portneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan9Portneg {
        #[inline(always)]
        fn from(val: u8) -> Scan9Portneg {
            Scan9Portneg::from_bits(val)
        }
    }
    impl From<Scan9Portneg> for u8 {
        #[inline(always)]
        fn from(val: Scan9Portneg) -> u8 {
            Scan9Portneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scan9Portpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Scan9Portpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scan9Portpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scan9Portpos {
        #[inline(always)]
        fn from(val: u8) -> Scan9Portpos {
            Scan9Portpos::from_bits(val)
        }
    }
    impl From<Scan9Portpos> for u8 {
        #[inline(always)]
        fn from(val: Scan9Portpos) -> u8 {
            Scan9Portpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ScanfifocfgAlignment {
        #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]."]
        Right12 = 0x0,
        #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]."]
        Right16 = 0x01,
        #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]."]
        Right20 = 0x02,
        #[doc = "DATA\\[11:0\\], 000000000000, ID\\[7:0\\]."]
        Left12 = 0x03,
        #[doc = "DATA\\[15:0\\], 00000000, ID\\[7:0\\]."]
        Left16 = 0x04,
        #[doc = "DATA\\[19:0\\], 0000, ID\\[7:0\\]."]
        Left20 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl ScanfifocfgAlignment {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ScanfifocfgAlignment {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ScanfifocfgAlignment {
        #[inline(always)]
        fn from(val: u8) -> ScanfifocfgAlignment {
            ScanfifocfgAlignment::from_bits(val)
        }
    }
    impl From<ScanfifocfgAlignment> for u8 {
        #[inline(always)]
        fn from(val: ScanfifocfgAlignment) -> u8 {
            ScanfifocfgAlignment::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ScanfifocfgDvl {
        #[doc = "When 1 entry in the scan FIFO is valid, set the SCANFIFODVL interrupt and request DMA."]
        Valid1 = 0x0,
        #[doc = "When 2 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
        Valid2 = 0x01,
        #[doc = "When 3 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
        Valid3 = 0x02,
        #[doc = "When 4 entries in the scan FIFO are valid, set the SCANFIFODVL interrupt and request DMA."]
        Valid4 = 0x03,
    }
    impl ScanfifocfgDvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ScanfifocfgDvl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ScanfifocfgDvl {
        #[inline(always)]
        fn from(val: u8) -> ScanfifocfgDvl {
            ScanfifocfgDvl::from_bits(val)
        }
    }
    impl From<ScanfifocfgDvl> for u8 {
        #[inline(always)]
        fn from(val: ScanfifocfgDvl) -> u8 {
            ScanfifocfgDvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scantrigaction {
        #[doc = "For TRIGSEL=IMMEDIATE, goes through the scan table once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, goes through the scan table once per trigger."]
        Once = 0x0,
        #[doc = "Goes through the scan table, converts each entry with a mask bit set, and puts it back into the scan queue to repeat again continuously. The queues are first come first serve. If both queues are triggered, the single queue will get to convert after each scan table completes. The scan queue will get to convert after each single conversion completes."]
        Continuous = 0x01,
    }
    impl Scantrigaction {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scantrigaction {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scantrigaction {
        #[inline(always)]
        fn from(val: u8) -> Scantrigaction {
            Scantrigaction::from_bits(val)
        }
    }
    impl From<Scantrigaction> for u8 {
        #[inline(always)]
        fn from(val: Scantrigaction) -> u8 {
            Scantrigaction::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Scantrigsel {
        #[doc = "Immediate triggering. The scan queue will be disabled once all conversions in the scan table are complete, unless TRIGGERACTION is set to continuous."]
        Immediate = 0x0,
        #[doc = "Triggers when the local timer count reaches zero."]
        Timer = 0x01,
        #[doc = "Triggers on PRS0 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
        Prsclkgrp = 0x02,
        #[doc = "Triggers on asynchronous PRS0 positive edge. Requires PRS0 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
        Prspos = 0x03,
        #[doc = "Triggers on asynchronous PRS0 negative edge. Requires PRS0 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
        Prsneg = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Scantrigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Scantrigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Scantrigsel {
        #[inline(always)]
        fn from(val: u8) -> Scantrigsel {
            Scantrigsel::from_bits(val)
        }
    }
    impl From<Scantrigsel> for u8 {
        #[inline(always)]
        fn from(val: Scantrigsel) -> u8 {
            Scantrigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SingleCfg {
        #[doc = "Use configuration group 0."]
        Config0 = 0x0,
        #[doc = "Use configuration group 1."]
        Config1 = 0x01,
    }
    impl SingleCfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SingleCfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SingleCfg {
        #[inline(always)]
        fn from(val: u8) -> SingleCfg {
            SingleCfg::from_bits(val)
        }
    }
    impl From<SingleCfg> for u8 {
        #[inline(always)]
        fn from(val: SingleCfg) -> u8 {
            SingleCfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SinglePortneg {
        #[doc = "Ground (single-ended)."]
        Gnd = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINNEG."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINNEG."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINNEG."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINNEG."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl SinglePortneg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SinglePortneg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SinglePortneg {
        #[inline(always)]
        fn from(val: u8) -> SinglePortneg {
            SinglePortneg::from_bits(val)
        }
    }
    impl From<SinglePortneg> for u8 {
        #[inline(always)]
        fn from(val: SinglePortneg) -> u8 {
            SinglePortneg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SinglePortpos {
        #[doc = "Ground."]
        Gnd = 0x0,
        #[doc = "Supply Pin - Select specific supply using PINPOS."]
        Supply = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "Port A - Select pin number using PINPOS."]
        Porta = 0x08,
        #[doc = "Port B - Select pin number using PINPOS."]
        Portb = 0x09,
        #[doc = "Port C - Select pin number using PINPOS."]
        Portc = 0x0a,
        #[doc = "Port D - Select pin number using PINPOS."]
        Portd = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl SinglePortpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SinglePortpos {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SinglePortpos {
        #[inline(always)]
        fn from(val: u8) -> SinglePortpos {
            SinglePortpos::from_bits(val)
        }
    }
    impl From<SinglePortpos> for u8 {
        #[inline(always)]
        fn from(val: SinglePortpos) -> u8 {
            SinglePortpos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SinglefifocfgAlignment {
        #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]."]
        Right12 = 0x0,
        #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]."]
        Right16 = 0x01,
        #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]."]
        Right20 = 0x02,
        #[doc = "DATA\\[11:0\\], 000000000000, ID\\[7:0\\]."]
        Left12 = 0x03,
        #[doc = "DATA\\[15:0\\], 00000000, ID\\[7:0\\]."]
        Left16 = 0x04,
        #[doc = "DATA\\[19:0\\], 0000, ID\\[7:0\\]."]
        Left20 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl SinglefifocfgAlignment {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SinglefifocfgAlignment {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SinglefifocfgAlignment {
        #[inline(always)]
        fn from(val: u8) -> SinglefifocfgAlignment {
            SinglefifocfgAlignment::from_bits(val)
        }
    }
    impl From<SinglefifocfgAlignment> for u8 {
        #[inline(always)]
        fn from(val: SinglefifocfgAlignment) -> u8 {
            SinglefifocfgAlignment::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SinglefifocfgDvl {
        #[doc = "When 1 entry in the single FIFO is valid, set the SINGLEFIFODVL interrupt and request DMA."]
        Valid1 = 0x0,
        #[doc = "When 2 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
        Valid2 = 0x01,
        #[doc = "When 3 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
        Valid3 = 0x02,
        #[doc = "When 4 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
        Valid4 = 0x03,
    }
    impl SinglefifocfgDvl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SinglefifocfgDvl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SinglefifocfgDvl {
        #[inline(always)]
        fn from(val: u8) -> SinglefifocfgDvl {
            SinglefifocfgDvl::from_bits(val)
        }
    }
    impl From<SinglefifocfgDvl> for u8 {
        #[inline(always)]
        fn from(val: SinglefifocfgDvl) -> u8 {
            SinglefifocfgDvl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Singletrigaction {
        #[doc = "For TRIGSEL=IMMEDIATE, converts the single queue once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, converts the single queue once per trigger.ask."]
        Once = 0x0,
        #[doc = "Converts the single queue, then checks for a pending scan queue before converting the single queue again continuously. The queues are first come first serve. If both queues are continuous, the IADC alternates between them."]
        Continuous = 0x01,
    }
    impl Singletrigaction {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Singletrigaction {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Singletrigaction {
        #[inline(always)]
        fn from(val: u8) -> Singletrigaction {
            Singletrigaction::from_bits(val)
        }
    }
    impl From<Singletrigaction> for u8 {
        #[inline(always)]
        fn from(val: Singletrigaction) -> u8 {
            Singletrigaction::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Singletrigsel {
        #[doc = "Immediate triggering. The single queue will be disabled once the conversion is complete, unless TRIGGERACTION is set to continuous."]
        Immediate = 0x0,
        #[doc = "Triggers when the local timer count reaches zero."]
        Timer = 0x01,
        #[doc = "Triggers on PRS1 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
        Prsclkgrp = 0x02,
        #[doc = "Triggers on asynchronous PRS1 positive edge. Requires PRS1 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
        Prspos = 0x03,
        #[doc = "Triggers on asynchronous PRS1 negative edge. Requires PRS1 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
        Prsneg = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Singletrigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Singletrigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Singletrigsel {
        #[inline(always)]
        fn from(val: u8) -> Singletrigsel {
            Singletrigsel::from_bits(val)
        }
    }
    impl From<Singletrigsel> for u8 {
        #[inline(always)]
        fn from(val: Singletrigsel) -> u8 {
            Singletrigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Warmupmode {
        #[doc = "Shut down the IADC after conversions have completed."]
        Normal = 0x0,
        #[doc = "Switch to standby mode after conversions have completed. The next warmup time will require 1us."]
        Keepinstandby = 0x01,
        #[doc = "Keep IADC fully powered after conversions have completed."]
        Keepwarm = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Warmupmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Warmupmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Warmupmode {
        #[inline(always)]
        fn from(val: u8) -> Warmupmode {
            Warmupmode::from_bits(val)
        }
    }
    impl From<Warmupmode> for u8 {
        #[inline(always)]
        fn from(val: Warmupmode) -> u8 {
            Warmupmode::to_bits(val)
        }
    }
}
