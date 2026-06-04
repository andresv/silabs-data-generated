#[doc = "BUFC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bufc {
    ptr: *mut u8,
}
unsafe impl Send for Bufc {}
unsafe impl Sync for Bufc {}
impl Bufc {
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
    pub const fn lpmode(self) -> crate::common::Reg<regs::Lpmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_ctrl(self) -> crate::common::Reg<regs::Buf0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_addr(self) -> crate::common::Reg<regs::Buf0Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_writeoffset(self) -> crate::common::Reg<regs::Buf0Writeoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_readoffset(self) -> crate::common::Reg<regs::Buf0Readoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_readdata(self) -> crate::common::Reg<regs::Buf0Readdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_writedata(self) -> crate::common::Reg<regs::Buf0Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_xwrite(self) -> crate::common::Reg<regs::Buf0Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_status(self) -> crate::common::Reg<regs::Buf0Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_thresholdctrl(self) -> crate::common::Reg<regs::Buf0Thresholdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_cmd(self) -> crate::common::Reg<regs::Buf0Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_readdata32(self) -> crate::common::Reg<regs::Buf0Readdata32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_writedata32(self) -> crate::common::Reg<regs::Buf0Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf0_xwrite32(self) -> crate::common::Reg<regs::Buf0Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_ctrl(self) -> crate::common::Reg<regs::Buf1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_addr(self) -> crate::common::Reg<regs::Buf1Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_writeoffset(self) -> crate::common::Reg<regs::Buf1Writeoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_readoffset(self) -> crate::common::Reg<regs::Buf1Readoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_readdata(self) -> crate::common::Reg<regs::Buf1Readdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_writedata(self) -> crate::common::Reg<regs::Buf1Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_xwrite(self) -> crate::common::Reg<regs::Buf1Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_status(self) -> crate::common::Reg<regs::Buf1Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_thresholdctrl(self) -> crate::common::Reg<regs::Buf1Thresholdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_cmd(self) -> crate::common::Reg<regs::Buf1Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_readdata32(self) -> crate::common::Reg<regs::Buf1Readdata32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_writedata32(self) -> crate::common::Reg<regs::Buf1Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf1_xwrite32(self) -> crate::common::Reg<regs::Buf1Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_ctrl(self) -> crate::common::Reg<regs::Buf2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_addr(self) -> crate::common::Reg<regs::Buf2Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_writeoffset(self) -> crate::common::Reg<regs::Buf2Writeoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_readoffset(self) -> crate::common::Reg<regs::Buf2Readoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_readdata(self) -> crate::common::Reg<regs::Buf2Readdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_writedata(self) -> crate::common::Reg<regs::Buf2Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_xwrite(self) -> crate::common::Reg<regs::Buf2Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_status(self) -> crate::common::Reg<regs::Buf2Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_thresholdctrl(self) -> crate::common::Reg<regs::Buf2Thresholdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_cmd(self) -> crate::common::Reg<regs::Buf2Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_readdata32(self) -> crate::common::Reg<regs::Buf2Readdata32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_writedata32(self) -> crate::common::Reg<regs::Buf2Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf2_xwrite32(self) -> crate::common::Reg<regs::Buf2Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_ctrl(self) -> crate::common::Reg<regs::Buf3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_addr(self) -> crate::common::Reg<regs::Buf3Addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_writeoffset(self) -> crate::common::Reg<regs::Buf3Writeoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_readoffset(self) -> crate::common::Reg<regs::Buf3Readoffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_readdata(self) -> crate::common::Reg<regs::Buf3Readdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_writedata(self) -> crate::common::Reg<regs::Buf3Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_xwrite(self) -> crate::common::Reg<regs::Buf3Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_status(self) -> crate::common::Reg<regs::Buf3Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_thresholdctrl(self) -> crate::common::Reg<regs::Buf3Thresholdctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_cmd(self) -> crate::common::Reg<regs::Buf3Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_readdata32(self) -> crate::common::Reg<regs::Buf3Readdata32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_writedata32(self) -> crate::common::Reg<regs::Buf3Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn buf3_xwrite32(self) -> crate::common::Reg<regs::Buf3Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn seqif(self) -> crate::common::Reg<regs::Seqif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn seqien(self) -> crate::common::Reg<regs::Seqien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sfmif(self) -> crate::common::Reg<regs::Sfmif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sfmien(self) -> crate::common::Reg<regs::Sfmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lpmode_set(self) -> crate::common::Reg<regs::Lpmode, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_ctrl_set(self) -> crate::common::Reg<regs::Buf0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_addr_set(self) -> crate::common::Reg<regs::Buf0Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_writeoffset_set(self) -> crate::common::Reg<regs::Buf0Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_readoffset_set(self) -> crate::common::Reg<regs::Buf0Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_writedata_set(self) -> crate::common::Reg<regs::Buf0Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_xwrite_set(self) -> crate::common::Reg<regs::Buf0Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_thresholdctrl_set(self) -> crate::common::Reg<regs::Buf0Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_cmd_set(self) -> crate::common::Reg<regs::Buf0Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_writedata32_set(self) -> crate::common::Reg<regs::Buf0Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf0_xwrite32_set(self) -> crate::common::Reg<regs::Buf0Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_ctrl_set(self) -> crate::common::Reg<regs::Buf1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_addr_set(self) -> crate::common::Reg<regs::Buf1Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_writeoffset_set(self) -> crate::common::Reg<regs::Buf1Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_readoffset_set(self) -> crate::common::Reg<regs::Buf1Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_writedata_set(self) -> crate::common::Reg<regs::Buf1Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_xwrite_set(self) -> crate::common::Reg<regs::Buf1Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_thresholdctrl_set(self) -> crate::common::Reg<regs::Buf1Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_cmd_set(self) -> crate::common::Reg<regs::Buf1Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_writedata32_set(self) -> crate::common::Reg<regs::Buf1Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf1_xwrite32_set(self) -> crate::common::Reg<regs::Buf1Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1084usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_ctrl_set(self) -> crate::common::Reg<regs::Buf2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x108cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_addr_set(self) -> crate::common::Reg<regs::Buf2Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_writeoffset_set(self) -> crate::common::Reg<regs::Buf2Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_readoffset_set(self) -> crate::common::Reg<regs::Buf2Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_writedata_set(self) -> crate::common::Reg<regs::Buf2Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_xwrite_set(self) -> crate::common::Reg<regs::Buf2Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_thresholdctrl_set(self) -> crate::common::Reg<regs::Buf2Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_cmd_set(self) -> crate::common::Reg<regs::Buf2Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_writedata32_set(self) -> crate::common::Reg<regs::Buf2Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf2_xwrite32_set(self) -> crate::common::Reg<regs::Buf2Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_ctrl_set(self) -> crate::common::Reg<regs::Buf3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_addr_set(self) -> crate::common::Reg<regs::Buf3Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_writeoffset_set(self) -> crate::common::Reg<regs::Buf3Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_readoffset_set(self) -> crate::common::Reg<regs::Buf3Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_writedata_set(self) -> crate::common::Reg<regs::Buf3Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_xwrite_set(self) -> crate::common::Reg<regs::Buf3Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_thresholdctrl_set(self) -> crate::common::Reg<regs::Buf3Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_cmd_set(self) -> crate::common::Reg<regs::Buf3Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_writedata32_set(self) -> crate::common::Reg<regs::Buf3Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn buf3_xwrite32_set(self) -> crate::common::Reg<regs::Buf3Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn seqif_set(self) -> crate::common::Reg<regs::Seqif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn seqien_set(self) -> crate::common::Reg<regs::Seqien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sfmif_set(self) -> crate::common::Reg<regs::Sfmif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sfmien_set(self) -> crate::common::Reg<regs::Sfmien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lpmode_clr(self) -> crate::common::Reg<regs::Lpmode, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_ctrl_clr(self) -> crate::common::Reg<regs::Buf0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_addr_clr(self) -> crate::common::Reg<regs::Buf0Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_writeoffset_clr(self) -> crate::common::Reg<regs::Buf0Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_readoffset_clr(self) -> crate::common::Reg<regs::Buf0Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_writedata_clr(self) -> crate::common::Reg<regs::Buf0Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_xwrite_clr(self) -> crate::common::Reg<regs::Buf0Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_thresholdctrl_clr(self) -> crate::common::Reg<regs::Buf0Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_cmd_clr(self) -> crate::common::Reg<regs::Buf0Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_writedata32_clr(self) -> crate::common::Reg<regs::Buf0Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf0_xwrite32_clr(self) -> crate::common::Reg<regs::Buf0Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_ctrl_clr(self) -> crate::common::Reg<regs::Buf1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x204cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_addr_clr(self) -> crate::common::Reg<regs::Buf1Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_writeoffset_clr(self) -> crate::common::Reg<regs::Buf1Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_readoffset_clr(self) -> crate::common::Reg<regs::Buf1Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_writedata_clr(self) -> crate::common::Reg<regs::Buf1Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_xwrite_clr(self) -> crate::common::Reg<regs::Buf1Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_thresholdctrl_clr(self) -> crate::common::Reg<regs::Buf1Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_cmd_clr(self) -> crate::common::Reg<regs::Buf1Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_writedata32_clr(self) -> crate::common::Reg<regs::Buf1Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf1_xwrite32_clr(self) -> crate::common::Reg<regs::Buf1Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2084usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_ctrl_clr(self) -> crate::common::Reg<regs::Buf2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x208cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_addr_clr(self) -> crate::common::Reg<regs::Buf2Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_writeoffset_clr(self) -> crate::common::Reg<regs::Buf2Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2094usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_readoffset_clr(self) -> crate::common::Reg<regs::Buf2Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2098usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_writedata_clr(self) -> crate::common::Reg<regs::Buf2Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_xwrite_clr(self) -> crate::common::Reg<regs::Buf2Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_thresholdctrl_clr(self) -> crate::common::Reg<regs::Buf2Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_cmd_clr(self) -> crate::common::Reg<regs::Buf2Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_writedata32_clr(self) -> crate::common::Reg<regs::Buf2Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf2_xwrite32_clr(self) -> crate::common::Reg<regs::Buf2Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_ctrl_clr(self) -> crate::common::Reg<regs::Buf3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_addr_clr(self) -> crate::common::Reg<regs::Buf3Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_writeoffset_clr(self) -> crate::common::Reg<regs::Buf3Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_readoffset_clr(self) -> crate::common::Reg<regs::Buf3Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_writedata_clr(self) -> crate::common::Reg<regs::Buf3Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20e4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_xwrite_clr(self) -> crate::common::Reg<regs::Buf3Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20e8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_thresholdctrl_clr(self) -> crate::common::Reg<regs::Buf3Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_cmd_clr(self) -> crate::common::Reg<regs::Buf3Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_writedata32_clr(self) -> crate::common::Reg<regs::Buf3Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn buf3_xwrite32_clr(self) -> crate::common::Reg<regs::Buf3Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2114usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2118usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn seqif_clr(self) -> crate::common::Reg<regs::Seqif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x211cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn seqien_clr(self) -> crate::common::Reg<regs::Seqien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sfmif_clr(self) -> crate::common::Reg<regs::Sfmif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sfmien_clr(self) -> crate::common::Reg<regs::Sfmien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lpmode_tgl(self) -> crate::common::Reg<regs::Lpmode, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_ctrl_tgl(self) -> crate::common::Reg<regs::Buf0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_addr_tgl(self) -> crate::common::Reg<regs::Buf0Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_writeoffset_tgl(self) -> crate::common::Reg<regs::Buf0Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_readoffset_tgl(self) -> crate::common::Reg<regs::Buf0Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_writedata_tgl(self) -> crate::common::Reg<regs::Buf0Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_xwrite_tgl(self) -> crate::common::Reg<regs::Buf0Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_thresholdctrl_tgl(self) -> crate::common::Reg<regs::Buf0Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_cmd_tgl(self) -> crate::common::Reg<regs::Buf0Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_writedata32_tgl(self) -> crate::common::Reg<regs::Buf0Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf0_xwrite32_tgl(self) -> crate::common::Reg<regs::Buf0Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_ctrl_tgl(self) -> crate::common::Reg<regs::Buf1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_addr_tgl(self) -> crate::common::Reg<regs::Buf1Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_writeoffset_tgl(self) -> crate::common::Reg<regs::Buf1Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_readoffset_tgl(self) -> crate::common::Reg<regs::Buf1Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_writedata_tgl(self) -> crate::common::Reg<regs::Buf1Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_xwrite_tgl(self) -> crate::common::Reg<regs::Buf1Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_thresholdctrl_tgl(self) -> crate::common::Reg<regs::Buf1Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_cmd_tgl(self) -> crate::common::Reg<regs::Buf1Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_writedata32_tgl(self) -> crate::common::Reg<regs::Buf1Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf1_xwrite32_tgl(self) -> crate::common::Reg<regs::Buf1Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3084usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_ctrl_tgl(self) -> crate::common::Reg<regs::Buf2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x308cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_addr_tgl(self) -> crate::common::Reg<regs::Buf2Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3090usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_writeoffset_tgl(self) -> crate::common::Reg<regs::Buf2Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3094usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_readoffset_tgl(self) -> crate::common::Reg<regs::Buf2Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3098usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_writedata_tgl(self) -> crate::common::Reg<regs::Buf2Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_xwrite_tgl(self) -> crate::common::Reg<regs::Buf2Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_thresholdctrl_tgl(self) -> crate::common::Reg<regs::Buf2Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_cmd_tgl(self) -> crate::common::Reg<regs::Buf2Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_writedata32_tgl(self) -> crate::common::Reg<regs::Buf2Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf2_xwrite32_tgl(self) -> crate::common::Reg<regs::Buf2Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_ctrl_tgl(self) -> crate::common::Reg<regs::Buf3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30ccusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_addr_tgl(self) -> crate::common::Reg<regs::Buf3Addr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_writeoffset_tgl(self) -> crate::common::Reg<regs::Buf3Writeoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_readoffset_tgl(self) -> crate::common::Reg<regs::Buf3Readoffset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_writedata_tgl(self) -> crate::common::Reg<regs::Buf3Writedata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30e4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_xwrite_tgl(self) -> crate::common::Reg<regs::Buf3Xwrite, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30e8usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_thresholdctrl_tgl(self) -> crate::common::Reg<regs::Buf3Thresholdctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f0usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_cmd_tgl(self) -> crate::common::Reg<regs::Buf3Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f4usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_writedata32_tgl(self) -> crate::common::Reg<regs::Buf3Writedata32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3100usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn buf3_xwrite32_tgl(self) -> crate::common::Reg<regs::Buf3Xwrite32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3104usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3114usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3118usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn seqif_tgl(self) -> crate::common::Reg<regs::Seqif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x311cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn seqien_tgl(self) -> crate::common::Reg<regs::Seqien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sfmif_tgl(self) -> crate::common::Reg<regs::Sfmif, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sfmien_tgl(self) -> crate::common::Reg<regs::Sfmien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Addr(pub u32);
    impl Buf0Addr {
        #[doc = "Buffer Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Buffer Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Buf0Addr {
        #[inline(always)]
        fn default() -> Buf0Addr {
            Buf0Addr(0)
        }
    }
    impl core::fmt::Debug for Buf0Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Addr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Cmd(pub u32);
    impl Buf0Cmd {
        #[doc = "Buffer Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Clear."]
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Prefetch."]
        #[must_use]
        #[inline(always)]
        pub const fn prefetch(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch."]
        #[inline(always)]
        pub const fn set_prefetch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Buf0Cmd {
        #[inline(always)]
        fn default() -> Buf0Cmd {
            Buf0Cmd(0)
        }
    }
    impl core::fmt::Debug for Buf0Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Cmd")
                .field("clear", &self.clear())
                .field("prefetch", &self.prefetch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf0Cmd {{ clear: {=bool:?}, prefetch: {=bool:?} }}",
                self.clear(),
                self.prefetch()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Ctrl(pub u32);
    impl Buf0Ctrl {
        #[doc = "Buffer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Buf0CtrlSize {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Buf0CtrlSize::from_bits(val as u8)
        }
        #[doc = "Buffer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Buf0CtrlSize) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Buf0Ctrl {
        #[inline(always)]
        fn default() -> Buf0Ctrl {
            Buf0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Buf0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Ctrl").field("size", &self.size()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Ctrl {{ size: {:?} }}", self.size())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Readdata(pub u32);
    impl Buf0Readdata {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf0Readdata {
        #[inline(always)]
        fn default() -> Buf0Readdata {
            Buf0Readdata(0)
        }
    }
    impl core::fmt::Debug for Buf0Readdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Readdata")
                .field("readdata", &self.readdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Readdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Readdata {{ readdata: {=u8:?} }}", self.readdata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Readdata32(pub u32);
    impl Buf0Readdata32 {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf0Readdata32 {
        #[inline(always)]
        fn default() -> Buf0Readdata32 {
            Buf0Readdata32(0)
        }
    }
    impl core::fmt::Debug for Buf0Readdata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Readdata32")
                .field("readdata32", &self.readdata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Readdata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Readdata32 {{ readdata32: {=u32:?} }}", self.readdata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Readoffset(pub u32);
    impl Buf0Readoffset {
        #[doc = "Read Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn readoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Read Offset."]
        #[inline(always)]
        pub const fn set_readoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf0Readoffset {
        #[inline(always)]
        fn default() -> Buf0Readoffset {
            Buf0Readoffset(0)
        }
    }
    impl core::fmt::Debug for Buf0Readoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Readoffset")
                .field("readoffset", &self.readoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Readoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Readoffset {{ readoffset: {=u16:?} }}", self.readoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Status(pub u32);
    impl Buf0Status {
        #[doc = "Number of Bytes in the Buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn bytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of Bytes in the Buffer."]
        #[inline(always)]
        pub const fn set_bytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdflag(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Threshold Flag."]
        #[inline(always)]
        pub const fn set_thresholdflag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Buf0Status {
        #[inline(always)]
        fn default() -> Buf0Status {
            Buf0Status(0)
        }
    }
    impl core::fmt::Debug for Buf0Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Status")
                .field("bytes", &self.bytes())
                .field("thresholdflag", &self.thresholdflag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf0Status {{ bytes: {=u16:?}, thresholdflag: {=bool:?} }}",
                self.bytes(),
                self.thresholdflag()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Thresholdctrl(pub u32);
    impl Buf0Thresholdctrl {
        #[doc = "Buffer Threshold Value."]
        #[must_use]
        #[inline(always)]
        pub const fn threshold(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Buffer Threshold Value."]
        #[inline(always)]
        pub const fn set_threshold(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdmode(&self) -> super::vals::Buf0ThresholdctrlThresholdmode {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Buf0ThresholdctrlThresholdmode::from_bits(val as u8)
        }
        #[doc = "Buffer Threshold Mode."]
        #[inline(always)]
        pub const fn set_thresholdmode(&mut self, val: super::vals::Buf0ThresholdctrlThresholdmode) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Buf0Thresholdctrl {
        #[inline(always)]
        fn default() -> Buf0Thresholdctrl {
            Buf0Thresholdctrl(0)
        }
    }
    impl core::fmt::Debug for Buf0Thresholdctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Thresholdctrl")
                .field("threshold", &self.threshold())
                .field("thresholdmode", &self.thresholdmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Thresholdctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf0Thresholdctrl {{ threshold: {=u16:?}, thresholdmode: {:?} }}",
                self.threshold(),
                self.thresholdmode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Writedata(pub u32);
    impl Buf0Writedata {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf0Writedata {
        #[inline(always)]
        fn default() -> Buf0Writedata {
            Buf0Writedata(0)
        }
    }
    impl core::fmt::Debug for Buf0Writedata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Writedata")
                .field("writedata", &self.writedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Writedata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Writedata {{ writedata: {=u8:?} }}", self.writedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Writedata32(pub u32);
    impl Buf0Writedata32 {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf0Writedata32 {
        #[inline(always)]
        fn default() -> Buf0Writedata32 {
            Buf0Writedata32(0)
        }
    }
    impl core::fmt::Debug for Buf0Writedata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Writedata32")
                .field("writedata32", &self.writedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Writedata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Writedata32 {{ writedata32: {=u32:?} }}", self.writedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Writeoffset(pub u32);
    impl Buf0Writeoffset {
        #[doc = "Write Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn writeoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Write Offset."]
        #[inline(always)]
        pub const fn set_writeoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf0Writeoffset {
        #[inline(always)]
        fn default() -> Buf0Writeoffset {
            Buf0Writeoffset(0)
        }
    }
    impl core::fmt::Debug for Buf0Writeoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Writeoffset")
                .field("writeoffset", &self.writeoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Writeoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Writeoffset {{ writeoffset: {=u16:?} }}", self.writeoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Xwrite(pub u32);
    impl Buf0Xwrite {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf0Xwrite {
        #[inline(always)]
        fn default() -> Buf0Xwrite {
            Buf0Xwrite(0)
        }
    }
    impl core::fmt::Debug for Buf0Xwrite {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Xwrite")
                .field("xorwritedata", &self.xorwritedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Xwrite {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Xwrite {{ xorwritedata: {=u8:?} }}", self.xorwritedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf0Xwrite32(pub u32);
    impl Buf0Xwrite32 {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf0Xwrite32 {
        #[inline(always)]
        fn default() -> Buf0Xwrite32 {
            Buf0Xwrite32(0)
        }
    }
    impl core::fmt::Debug for Buf0Xwrite32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf0Xwrite32")
                .field("xorwritedata32", &self.xorwritedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf0Xwrite32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf0Xwrite32 {{ xorwritedata32: {=u32:?} }}", self.xorwritedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Addr(pub u32);
    impl Buf1Addr {
        #[doc = "Buffer Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Buffer Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Buf1Addr {
        #[inline(always)]
        fn default() -> Buf1Addr {
            Buf1Addr(0)
        }
    }
    impl core::fmt::Debug for Buf1Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Addr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Cmd(pub u32);
    impl Buf1Cmd {
        #[doc = "Buffer Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Clear."]
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Prefetch."]
        #[must_use]
        #[inline(always)]
        pub const fn prefetch(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch."]
        #[inline(always)]
        pub const fn set_prefetch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Buf1Cmd {
        #[inline(always)]
        fn default() -> Buf1Cmd {
            Buf1Cmd(0)
        }
    }
    impl core::fmt::Debug for Buf1Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Cmd")
                .field("clear", &self.clear())
                .field("prefetch", &self.prefetch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf1Cmd {{ clear: {=bool:?}, prefetch: {=bool:?} }}",
                self.clear(),
                self.prefetch()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Ctrl(pub u32);
    impl Buf1Ctrl {
        #[doc = "Buffer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Buf1CtrlSize {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Buf1CtrlSize::from_bits(val as u8)
        }
        #[doc = "Buffer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Buf1CtrlSize) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Buf1Ctrl {
        #[inline(always)]
        fn default() -> Buf1Ctrl {
            Buf1Ctrl(0)
        }
    }
    impl core::fmt::Debug for Buf1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Ctrl").field("size", &self.size()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Ctrl {{ size: {:?} }}", self.size())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Readdata(pub u32);
    impl Buf1Readdata {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf1Readdata {
        #[inline(always)]
        fn default() -> Buf1Readdata {
            Buf1Readdata(0)
        }
    }
    impl core::fmt::Debug for Buf1Readdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Readdata")
                .field("readdata", &self.readdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Readdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Readdata {{ readdata: {=u8:?} }}", self.readdata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Readdata32(pub u32);
    impl Buf1Readdata32 {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf1Readdata32 {
        #[inline(always)]
        fn default() -> Buf1Readdata32 {
            Buf1Readdata32(0)
        }
    }
    impl core::fmt::Debug for Buf1Readdata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Readdata32")
                .field("readdata32", &self.readdata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Readdata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Readdata32 {{ readdata32: {=u32:?} }}", self.readdata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Readoffset(pub u32);
    impl Buf1Readoffset {
        #[doc = "Read Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn readoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Read Offset."]
        #[inline(always)]
        pub const fn set_readoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf1Readoffset {
        #[inline(always)]
        fn default() -> Buf1Readoffset {
            Buf1Readoffset(0)
        }
    }
    impl core::fmt::Debug for Buf1Readoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Readoffset")
                .field("readoffset", &self.readoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Readoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Readoffset {{ readoffset: {=u16:?} }}", self.readoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Status(pub u32);
    impl Buf1Status {
        #[doc = "Number of Bytes in the Buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn bytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of Bytes in the Buffer."]
        #[inline(always)]
        pub const fn set_bytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdflag(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Threshold Flag."]
        #[inline(always)]
        pub const fn set_thresholdflag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Buf1Status {
        #[inline(always)]
        fn default() -> Buf1Status {
            Buf1Status(0)
        }
    }
    impl core::fmt::Debug for Buf1Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Status")
                .field("bytes", &self.bytes())
                .field("thresholdflag", &self.thresholdflag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf1Status {{ bytes: {=u16:?}, thresholdflag: {=bool:?} }}",
                self.bytes(),
                self.thresholdflag()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Thresholdctrl(pub u32);
    impl Buf1Thresholdctrl {
        #[doc = "Buffer Threshold Value."]
        #[must_use]
        #[inline(always)]
        pub const fn threshold(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Buffer Threshold Value."]
        #[inline(always)]
        pub const fn set_threshold(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdmode(&self) -> super::vals::Buf1ThresholdctrlThresholdmode {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Buf1ThresholdctrlThresholdmode::from_bits(val as u8)
        }
        #[doc = "Buffer Threshold Mode."]
        #[inline(always)]
        pub const fn set_thresholdmode(&mut self, val: super::vals::Buf1ThresholdctrlThresholdmode) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Buf1Thresholdctrl {
        #[inline(always)]
        fn default() -> Buf1Thresholdctrl {
            Buf1Thresholdctrl(0)
        }
    }
    impl core::fmt::Debug for Buf1Thresholdctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Thresholdctrl")
                .field("threshold", &self.threshold())
                .field("thresholdmode", &self.thresholdmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Thresholdctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf1Thresholdctrl {{ threshold: {=u16:?}, thresholdmode: {:?} }}",
                self.threshold(),
                self.thresholdmode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Writedata(pub u32);
    impl Buf1Writedata {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf1Writedata {
        #[inline(always)]
        fn default() -> Buf1Writedata {
            Buf1Writedata(0)
        }
    }
    impl core::fmt::Debug for Buf1Writedata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Writedata")
                .field("writedata", &self.writedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Writedata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Writedata {{ writedata: {=u8:?} }}", self.writedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Writedata32(pub u32);
    impl Buf1Writedata32 {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf1Writedata32 {
        #[inline(always)]
        fn default() -> Buf1Writedata32 {
            Buf1Writedata32(0)
        }
    }
    impl core::fmt::Debug for Buf1Writedata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Writedata32")
                .field("writedata32", &self.writedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Writedata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Writedata32 {{ writedata32: {=u32:?} }}", self.writedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Writeoffset(pub u32);
    impl Buf1Writeoffset {
        #[doc = "Write Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn writeoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Write Offset."]
        #[inline(always)]
        pub const fn set_writeoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf1Writeoffset {
        #[inline(always)]
        fn default() -> Buf1Writeoffset {
            Buf1Writeoffset(0)
        }
    }
    impl core::fmt::Debug for Buf1Writeoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Writeoffset")
                .field("writeoffset", &self.writeoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Writeoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Writeoffset {{ writeoffset: {=u16:?} }}", self.writeoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Xwrite(pub u32);
    impl Buf1Xwrite {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf1Xwrite {
        #[inline(always)]
        fn default() -> Buf1Xwrite {
            Buf1Xwrite(0)
        }
    }
    impl core::fmt::Debug for Buf1Xwrite {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Xwrite")
                .field("xorwritedata", &self.xorwritedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Xwrite {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Xwrite {{ xorwritedata: {=u8:?} }}", self.xorwritedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf1Xwrite32(pub u32);
    impl Buf1Xwrite32 {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf1Xwrite32 {
        #[inline(always)]
        fn default() -> Buf1Xwrite32 {
            Buf1Xwrite32(0)
        }
    }
    impl core::fmt::Debug for Buf1Xwrite32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf1Xwrite32")
                .field("xorwritedata32", &self.xorwritedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf1Xwrite32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf1Xwrite32 {{ xorwritedata32: {=u32:?} }}", self.xorwritedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Addr(pub u32);
    impl Buf2Addr {
        #[doc = "Buffer Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Buffer Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Buf2Addr {
        #[inline(always)]
        fn default() -> Buf2Addr {
            Buf2Addr(0)
        }
    }
    impl core::fmt::Debug for Buf2Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Addr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Cmd(pub u32);
    impl Buf2Cmd {
        #[doc = "Buffer Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Clear."]
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Prefetch."]
        #[must_use]
        #[inline(always)]
        pub const fn prefetch(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch."]
        #[inline(always)]
        pub const fn set_prefetch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Buf2Cmd {
        #[inline(always)]
        fn default() -> Buf2Cmd {
            Buf2Cmd(0)
        }
    }
    impl core::fmt::Debug for Buf2Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Cmd")
                .field("clear", &self.clear())
                .field("prefetch", &self.prefetch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf2Cmd {{ clear: {=bool:?}, prefetch: {=bool:?} }}",
                self.clear(),
                self.prefetch()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Ctrl(pub u32);
    impl Buf2Ctrl {
        #[doc = "Buffer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Buf2CtrlSize {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Buf2CtrlSize::from_bits(val as u8)
        }
        #[doc = "Buffer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Buf2CtrlSize) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Buf2Ctrl {
        #[inline(always)]
        fn default() -> Buf2Ctrl {
            Buf2Ctrl(0)
        }
    }
    impl core::fmt::Debug for Buf2Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Ctrl").field("size", &self.size()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Ctrl {{ size: {:?} }}", self.size())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Readdata(pub u32);
    impl Buf2Readdata {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf2Readdata {
        #[inline(always)]
        fn default() -> Buf2Readdata {
            Buf2Readdata(0)
        }
    }
    impl core::fmt::Debug for Buf2Readdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Readdata")
                .field("readdata", &self.readdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Readdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Readdata {{ readdata: {=u8:?} }}", self.readdata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Readdata32(pub u32);
    impl Buf2Readdata32 {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf2Readdata32 {
        #[inline(always)]
        fn default() -> Buf2Readdata32 {
            Buf2Readdata32(0)
        }
    }
    impl core::fmt::Debug for Buf2Readdata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Readdata32")
                .field("readdata32", &self.readdata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Readdata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Readdata32 {{ readdata32: {=u32:?} }}", self.readdata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Readoffset(pub u32);
    impl Buf2Readoffset {
        #[doc = "Read Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn readoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Read Offset."]
        #[inline(always)]
        pub const fn set_readoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf2Readoffset {
        #[inline(always)]
        fn default() -> Buf2Readoffset {
            Buf2Readoffset(0)
        }
    }
    impl core::fmt::Debug for Buf2Readoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Readoffset")
                .field("readoffset", &self.readoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Readoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Readoffset {{ readoffset: {=u16:?} }}", self.readoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Status(pub u32);
    impl Buf2Status {
        #[doc = "Number of Bytes in the Buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn bytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of Bytes in the Buffer."]
        #[inline(always)]
        pub const fn set_bytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdflag(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Threshold Flag."]
        #[inline(always)]
        pub const fn set_thresholdflag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Buf2Status {
        #[inline(always)]
        fn default() -> Buf2Status {
            Buf2Status(0)
        }
    }
    impl core::fmt::Debug for Buf2Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Status")
                .field("bytes", &self.bytes())
                .field("thresholdflag", &self.thresholdflag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf2Status {{ bytes: {=u16:?}, thresholdflag: {=bool:?} }}",
                self.bytes(),
                self.thresholdflag()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Thresholdctrl(pub u32);
    impl Buf2Thresholdctrl {
        #[doc = "Buffer Threshold Value."]
        #[must_use]
        #[inline(always)]
        pub const fn threshold(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Buffer Threshold Value."]
        #[inline(always)]
        pub const fn set_threshold(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdmode(&self) -> super::vals::Buf2ThresholdctrlThresholdmode {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Buf2ThresholdctrlThresholdmode::from_bits(val as u8)
        }
        #[doc = "Buffer Threshold Mode."]
        #[inline(always)]
        pub const fn set_thresholdmode(&mut self, val: super::vals::Buf2ThresholdctrlThresholdmode) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Buf2Thresholdctrl {
        #[inline(always)]
        fn default() -> Buf2Thresholdctrl {
            Buf2Thresholdctrl(0)
        }
    }
    impl core::fmt::Debug for Buf2Thresholdctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Thresholdctrl")
                .field("threshold", &self.threshold())
                .field("thresholdmode", &self.thresholdmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Thresholdctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf2Thresholdctrl {{ threshold: {=u16:?}, thresholdmode: {:?} }}",
                self.threshold(),
                self.thresholdmode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Writedata(pub u32);
    impl Buf2Writedata {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf2Writedata {
        #[inline(always)]
        fn default() -> Buf2Writedata {
            Buf2Writedata(0)
        }
    }
    impl core::fmt::Debug for Buf2Writedata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Writedata")
                .field("writedata", &self.writedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Writedata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Writedata {{ writedata: {=u8:?} }}", self.writedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Writedata32(pub u32);
    impl Buf2Writedata32 {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf2Writedata32 {
        #[inline(always)]
        fn default() -> Buf2Writedata32 {
            Buf2Writedata32(0)
        }
    }
    impl core::fmt::Debug for Buf2Writedata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Writedata32")
                .field("writedata32", &self.writedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Writedata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Writedata32 {{ writedata32: {=u32:?} }}", self.writedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Writeoffset(pub u32);
    impl Buf2Writeoffset {
        #[doc = "Write Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn writeoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Write Offset."]
        #[inline(always)]
        pub const fn set_writeoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf2Writeoffset {
        #[inline(always)]
        fn default() -> Buf2Writeoffset {
            Buf2Writeoffset(0)
        }
    }
    impl core::fmt::Debug for Buf2Writeoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Writeoffset")
                .field("writeoffset", &self.writeoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Writeoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Writeoffset {{ writeoffset: {=u16:?} }}", self.writeoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Xwrite(pub u32);
    impl Buf2Xwrite {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf2Xwrite {
        #[inline(always)]
        fn default() -> Buf2Xwrite {
            Buf2Xwrite(0)
        }
    }
    impl core::fmt::Debug for Buf2Xwrite {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Xwrite")
                .field("xorwritedata", &self.xorwritedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Xwrite {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Xwrite {{ xorwritedata: {=u8:?} }}", self.xorwritedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf2Xwrite32(pub u32);
    impl Buf2Xwrite32 {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf2Xwrite32 {
        #[inline(always)]
        fn default() -> Buf2Xwrite32 {
            Buf2Xwrite32(0)
        }
    }
    impl core::fmt::Debug for Buf2Xwrite32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf2Xwrite32")
                .field("xorwritedata32", &self.xorwritedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf2Xwrite32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf2Xwrite32 {{ xorwritedata32: {=u32:?} }}", self.xorwritedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Addr(pub u32);
    impl Buf3Addr {
        #[doc = "Buffer Address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Buffer Address."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Buf3Addr {
        #[inline(always)]
        fn default() -> Buf3Addr {
            Buf3Addr(0)
        }
    }
    impl core::fmt::Debug for Buf3Addr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Addr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Addr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Addr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Cmd(pub u32);
    impl Buf3Cmd {
        #[doc = "Buffer Clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Clear."]
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Prefetch."]
        #[must_use]
        #[inline(always)]
        pub const fn prefetch(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch."]
        #[inline(always)]
        pub const fn set_prefetch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Buf3Cmd {
        #[inline(always)]
        fn default() -> Buf3Cmd {
            Buf3Cmd(0)
        }
    }
    impl core::fmt::Debug for Buf3Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Cmd")
                .field("clear", &self.clear())
                .field("prefetch", &self.prefetch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf3Cmd {{ clear: {=bool:?}, prefetch: {=bool:?} }}",
                self.clear(),
                self.prefetch()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Ctrl(pub u32);
    impl Buf3Ctrl {
        #[doc = "Buffer Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> super::vals::Buf3CtrlSize {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Buf3CtrlSize::from_bits(val as u8)
        }
        #[doc = "Buffer Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Buf3CtrlSize) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Buf3Ctrl {
        #[inline(always)]
        fn default() -> Buf3Ctrl {
            Buf3Ctrl(0)
        }
    }
    impl core::fmt::Debug for Buf3Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Ctrl").field("size", &self.size()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Ctrl {{ size: {:?} }}", self.size())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Readdata(pub u32);
    impl Buf3Readdata {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf3Readdata {
        #[inline(always)]
        fn default() -> Buf3Readdata {
            Buf3Readdata(0)
        }
    }
    impl core::fmt::Debug for Buf3Readdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Readdata")
                .field("readdata", &self.readdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Readdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Readdata {{ readdata: {=u8:?} }}", self.readdata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Readdata32(pub u32);
    impl Buf3Readdata32 {
        #[doc = "Buffer Read Data."]
        #[must_use]
        #[inline(always)]
        pub const fn readdata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Read Data."]
        #[inline(always)]
        pub const fn set_readdata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf3Readdata32 {
        #[inline(always)]
        fn default() -> Buf3Readdata32 {
            Buf3Readdata32(0)
        }
    }
    impl core::fmt::Debug for Buf3Readdata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Readdata32")
                .field("readdata32", &self.readdata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Readdata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Readdata32 {{ readdata32: {=u32:?} }}", self.readdata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Readoffset(pub u32);
    impl Buf3Readoffset {
        #[doc = "Read Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn readoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Read Offset."]
        #[inline(always)]
        pub const fn set_readoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf3Readoffset {
        #[inline(always)]
        fn default() -> Buf3Readoffset {
            Buf3Readoffset(0)
        }
    }
    impl core::fmt::Debug for Buf3Readoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Readoffset")
                .field("readoffset", &self.readoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Readoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Readoffset {{ readoffset: {=u16:?} }}", self.readoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Status(pub u32);
    impl Buf3Status {
        #[doc = "Number of Bytes in the Buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn bytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Number of Bytes in the Buffer."]
        #[inline(always)]
        pub const fn set_bytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdflag(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer Threshold Flag."]
        #[inline(always)]
        pub const fn set_thresholdflag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Buf3Status {
        #[inline(always)]
        fn default() -> Buf3Status {
            Buf3Status(0)
        }
    }
    impl core::fmt::Debug for Buf3Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Status")
                .field("bytes", &self.bytes())
                .field("thresholdflag", &self.thresholdflag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf3Status {{ bytes: {=u16:?}, thresholdflag: {=bool:?} }}",
                self.bytes(),
                self.thresholdflag()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Thresholdctrl(pub u32);
    impl Buf3Thresholdctrl {
        #[doc = "Buffer Threshold Value."]
        #[must_use]
        #[inline(always)]
        pub const fn threshold(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Buffer Threshold Value."]
        #[inline(always)]
        pub const fn set_threshold(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Buffer Threshold Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn thresholdmode(&self) -> super::vals::Buf3ThresholdctrlThresholdmode {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Buf3ThresholdctrlThresholdmode::from_bits(val as u8)
        }
        #[doc = "Buffer Threshold Mode."]
        #[inline(always)]
        pub const fn set_thresholdmode(&mut self, val: super::vals::Buf3ThresholdctrlThresholdmode) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Buf3Thresholdctrl {
        #[inline(always)]
        fn default() -> Buf3Thresholdctrl {
            Buf3Thresholdctrl(0)
        }
    }
    impl core::fmt::Debug for Buf3Thresholdctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Thresholdctrl")
                .field("threshold", &self.threshold())
                .field("thresholdmode", &self.thresholdmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Thresholdctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Buf3Thresholdctrl {{ threshold: {=u16:?}, thresholdmode: {:?} }}",
                self.threshold(),
                self.thresholdmode()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Writedata(pub u32);
    impl Buf3Writedata {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf3Writedata {
        #[inline(always)]
        fn default() -> Buf3Writedata {
            Buf3Writedata(0)
        }
    }
    impl core::fmt::Debug for Buf3Writedata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Writedata")
                .field("writedata", &self.writedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Writedata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Writedata {{ writedata: {=u8:?} }}", self.writedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Writedata32(pub u32);
    impl Buf3Writedata32 {
        #[doc = "Buffer Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn writedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer Write Data."]
        #[inline(always)]
        pub const fn set_writedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf3Writedata32 {
        #[inline(always)]
        fn default() -> Buf3Writedata32 {
            Buf3Writedata32(0)
        }
    }
    impl core::fmt::Debug for Buf3Writedata32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Writedata32")
                .field("writedata32", &self.writedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Writedata32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Writedata32 {{ writedata32: {=u32:?} }}", self.writedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Writeoffset(pub u32);
    impl Buf3Writeoffset {
        #[doc = "Write Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn writeoffset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Write Offset."]
        #[inline(always)]
        pub const fn set_writeoffset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for Buf3Writeoffset {
        #[inline(always)]
        fn default() -> Buf3Writeoffset {
            Buf3Writeoffset(0)
        }
    }
    impl core::fmt::Debug for Buf3Writeoffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Writeoffset")
                .field("writeoffset", &self.writeoffset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Writeoffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Writeoffset {{ writeoffset: {=u16:?} }}", self.writeoffset())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Xwrite(pub u32);
    impl Buf3Xwrite {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Buf3Xwrite {
        #[inline(always)]
        fn default() -> Buf3Xwrite {
            Buf3Xwrite(0)
        }
    }
    impl core::fmt::Debug for Buf3Xwrite {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Xwrite")
                .field("xorwritedata", &self.xorwritedata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Xwrite {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Xwrite {{ xorwritedata: {=u8:?} }}", self.xorwritedata())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf3Xwrite32(pub u32);
    impl Buf3Xwrite32 {
        #[doc = "Buffer XOR Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn xorwritedata32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer XOR Write Data."]
        #[inline(always)]
        pub const fn set_xorwritedata32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf3Xwrite32 {
        #[inline(always)]
        fn default() -> Buf3Xwrite32 {
            Buf3Xwrite32(0)
        }
    }
    impl core::fmt::Debug for Buf3Xwrite32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buf3Xwrite32")
                .field("xorwritedata32", &self.xorwritedata32())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Buf3Xwrite32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Buf3Xwrite32 {{ xorwritedata32: {=u32:?} }}", self.xorwritedata32())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct En(pub u32);
    impl En {
        #[doc = "Enable peripheral clock to this module."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable peripheral clock to this module."]
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
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "BUF0OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0of(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BUF0UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0uf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BUF0THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0thr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BUF0CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0corr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BUF0NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0nwa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "BUF1OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1of(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BUF1UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1uf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BUF1THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1thr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "BUF1CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1corr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BUF1NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1nwa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BUF2OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2of(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BUF2UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2uf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BUF2THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2thr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUF2CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2corr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BUF2NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2nwa(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BUF3OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3of(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BUF3UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3uf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "BUF3THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3thr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BUF3CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3corr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "BUF3NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3nwa(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "BUSERROR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buserror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "BUSERROR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buserror(&mut self, val: bool) {
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
                .field("buf0of", &self.buf0of())
                .field("buf0uf", &self.buf0uf())
                .field("buf0thr", &self.buf0thr())
                .field("buf0corr", &self.buf0corr())
                .field("buf0nwa", &self.buf0nwa())
                .field("buf1of", &self.buf1of())
                .field("buf1uf", &self.buf1uf())
                .field("buf1thr", &self.buf1thr())
                .field("buf1corr", &self.buf1corr())
                .field("buf1nwa", &self.buf1nwa())
                .field("buf2of", &self.buf2of())
                .field("buf2uf", &self.buf2uf())
                .field("buf2thr", &self.buf2thr())
                .field("buf2corr", &self.buf2corr())
                .field("buf2nwa", &self.buf2nwa())
                .field("buf3of", &self.buf3of())
                .field("buf3uf", &self.buf3uf())
                .field("buf3thr", &self.buf3thr())
                .field("buf3corr", &self.buf3corr())
                .field("buf3nwa", &self.buf3nwa())
                .field("buserror", &self.buserror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ buf0of: {=bool:?}, buf0uf: {=bool:?}, buf0thr: {=bool:?}, buf0corr: {=bool:?}, buf0nwa: {=bool:?}, buf1of: {=bool:?}, buf1uf: {=bool:?}, buf1thr: {=bool:?}, buf1corr: {=bool:?}, buf1nwa: {=bool:?}, buf2of: {=bool:?}, buf2uf: {=bool:?}, buf2thr: {=bool:?}, buf2corr: {=bool:?}, buf2nwa: {=bool:?}, buf3of: {=bool:?}, buf3uf: {=bool:?}, buf3thr: {=bool:?}, buf3corr: {=bool:?}, buf3nwa: {=bool:?}, buserror: {=bool:?} }}",
                self.buf0of(),
                self.buf0uf(),
                self.buf0thr(),
                self.buf0corr(),
                self.buf0nwa(),
                self.buf1of(),
                self.buf1uf(),
                self.buf1thr(),
                self.buf1corr(),
                self.buf1nwa(),
                self.buf2of(),
                self.buf2uf(),
                self.buf2thr(),
                self.buf2corr(),
                self.buf2nwa(),
                self.buf3of(),
                self.buf3uf(),
                self.buf3thr(),
                self.buf3corr(),
                self.buf3nwa(),
                self.buserror()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Buffer 0 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0of(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Overflow."]
        #[inline(always)]
        pub const fn set_buf0of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Buffer 0 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0uf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Underflow."]
        #[inline(always)]
        pub const fn set_buf0uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Buffer 0 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0thr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf0thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Buffer 0 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0corr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Corrupt."]
        #[inline(always)]
        pub const fn set_buf0corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Buffer 0 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0nwa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf0nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Buffer 1 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1of(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Overflow."]
        #[inline(always)]
        pub const fn set_buf1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Buffer 1 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1uf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Underflow."]
        #[inline(always)]
        pub const fn set_buf1uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Buffer 1 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1thr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf1thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Buffer 1 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1corr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Corrupt."]
        #[inline(always)]
        pub const fn set_buf1corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Buffer 1 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1nwa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf1nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Buffer 2 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2of(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Overflow."]
        #[inline(always)]
        pub const fn set_buf2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Buffer 2 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2uf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Underflow."]
        #[inline(always)]
        pub const fn set_buf2uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Buffer 2 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2thr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf2thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Buffer 2 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2corr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Corrupt."]
        #[inline(always)]
        pub const fn set_buf2corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Buffer 2 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2nwa(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf2nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Buffer 3 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3of(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Overflow."]
        #[inline(always)]
        pub const fn set_buf3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Buffer 3 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3uf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Underflow."]
        #[inline(always)]
        pub const fn set_buf3uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Buffer 3 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3thr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf3thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Buffer 3 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3corr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Corrupt."]
        #[inline(always)]
        pub const fn set_buf3corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Buffer 3 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3nwa(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf3nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Bus Error."]
        #[must_use]
        #[inline(always)]
        pub const fn buserror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error."]
        #[inline(always)]
        pub const fn set_buserror(&mut self, val: bool) {
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
                .field("buf0of", &self.buf0of())
                .field("buf0uf", &self.buf0uf())
                .field("buf0thr", &self.buf0thr())
                .field("buf0corr", &self.buf0corr())
                .field("buf0nwa", &self.buf0nwa())
                .field("buf1of", &self.buf1of())
                .field("buf1uf", &self.buf1uf())
                .field("buf1thr", &self.buf1thr())
                .field("buf1corr", &self.buf1corr())
                .field("buf1nwa", &self.buf1nwa())
                .field("buf2of", &self.buf2of())
                .field("buf2uf", &self.buf2uf())
                .field("buf2thr", &self.buf2thr())
                .field("buf2corr", &self.buf2corr())
                .field("buf2nwa", &self.buf2nwa())
                .field("buf3of", &self.buf3of())
                .field("buf3uf", &self.buf3uf())
                .field("buf3thr", &self.buf3thr())
                .field("buf3corr", &self.buf3corr())
                .field("buf3nwa", &self.buf3nwa())
                .field("buserror", &self.buserror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ buf0of: {=bool:?}, buf0uf: {=bool:?}, buf0thr: {=bool:?}, buf0corr: {=bool:?}, buf0nwa: {=bool:?}, buf1of: {=bool:?}, buf1uf: {=bool:?}, buf1thr: {=bool:?}, buf1corr: {=bool:?}, buf1nwa: {=bool:?}, buf2of: {=bool:?}, buf2uf: {=bool:?}, buf2thr: {=bool:?}, buf2corr: {=bool:?}, buf2nwa: {=bool:?}, buf3of: {=bool:?}, buf3uf: {=bool:?}, buf3thr: {=bool:?}, buf3corr: {=bool:?}, buf3nwa: {=bool:?}, buserror: {=bool:?} }}",
                self.buf0of(),
                self.buf0uf(),
                self.buf0thr(),
                self.buf0corr(),
                self.buf0nwa(),
                self.buf1of(),
                self.buf1uf(),
                self.buf1thr(),
                self.buf1corr(),
                self.buf1nwa(),
                self.buf2of(),
                self.buf2uf(),
                self.buf2thr(),
                self.buf2corr(),
                self.buf2nwa(),
                self.buf3of(),
                self.buf3uf(),
                self.buf3thr(),
                self.buf3corr(),
                self.buf3nwa(),
                self.buserror()
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
    pub struct Lpmode(pub u32);
    impl Lpmode {
        #[doc = "Low power mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Low power mode enable."]
        #[inline(always)]
        pub const fn set_lpen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Lpmode {
        #[inline(always)]
        fn default() -> Lpmode {
            Lpmode(0)
        }
    }
    impl core::fmt::Debug for Lpmode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpmode").field("lpen", &self.lpen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpmode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lpmode {{ lpen: {=u8:?} }}", self.lpen())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seqien(pub u32);
    impl Seqien {
        #[doc = "BUF0OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0of(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BUF0UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0uf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BUF0THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0thr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BUF0CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0corr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BUF0NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0nwa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "BUF1OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1of(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BUF1UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1uf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BUF1THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1thr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "BUF1CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1corr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BUF1NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1nwa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BUF2OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2of(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BUF2UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2uf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BUF2THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2thr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUF2CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2corr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BUF2NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2nwa(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BUF3OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3of(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BUF3UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3uf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "BUF3THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3thr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BUF3CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3corr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "BUF3NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3nwa(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "BUSERROR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buserror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "BUSERROR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buserror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seqien {
        #[inline(always)]
        fn default() -> Seqien {
            Seqien(0)
        }
    }
    impl core::fmt::Debug for Seqien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seqien")
                .field("buf0of", &self.buf0of())
                .field("buf0uf", &self.buf0uf())
                .field("buf0thr", &self.buf0thr())
                .field("buf0corr", &self.buf0corr())
                .field("buf0nwa", &self.buf0nwa())
                .field("buf1of", &self.buf1of())
                .field("buf1uf", &self.buf1uf())
                .field("buf1thr", &self.buf1thr())
                .field("buf1corr", &self.buf1corr())
                .field("buf1nwa", &self.buf1nwa())
                .field("buf2of", &self.buf2of())
                .field("buf2uf", &self.buf2uf())
                .field("buf2thr", &self.buf2thr())
                .field("buf2corr", &self.buf2corr())
                .field("buf2nwa", &self.buf2nwa())
                .field("buf3of", &self.buf3of())
                .field("buf3uf", &self.buf3uf())
                .field("buf3thr", &self.buf3thr())
                .field("buf3corr", &self.buf3corr())
                .field("buf3nwa", &self.buf3nwa())
                .field("buserror", &self.buserror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seqien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seqien {{ buf0of: {=bool:?}, buf0uf: {=bool:?}, buf0thr: {=bool:?}, buf0corr: {=bool:?}, buf0nwa: {=bool:?}, buf1of: {=bool:?}, buf1uf: {=bool:?}, buf1thr: {=bool:?}, buf1corr: {=bool:?}, buf1nwa: {=bool:?}, buf2of: {=bool:?}, buf2uf: {=bool:?}, buf2thr: {=bool:?}, buf2corr: {=bool:?}, buf2nwa: {=bool:?}, buf3of: {=bool:?}, buf3uf: {=bool:?}, buf3thr: {=bool:?}, buf3corr: {=bool:?}, buf3nwa: {=bool:?}, buserror: {=bool:?} }}",
                self.buf0of(),
                self.buf0uf(),
                self.buf0thr(),
                self.buf0corr(),
                self.buf0nwa(),
                self.buf1of(),
                self.buf1uf(),
                self.buf1thr(),
                self.buf1corr(),
                self.buf1nwa(),
                self.buf2of(),
                self.buf2uf(),
                self.buf2thr(),
                self.buf2corr(),
                self.buf2nwa(),
                self.buf3of(),
                self.buf3uf(),
                self.buf3thr(),
                self.buf3corr(),
                self.buf3nwa(),
                self.buserror()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seqif(pub u32);
    impl Seqif {
        #[doc = "Buffer 0 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0of(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Overflow."]
        #[inline(always)]
        pub const fn set_buf0of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Buffer 0 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0uf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Underflow."]
        #[inline(always)]
        pub const fn set_buf0uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Buffer 0 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0thr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf0thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Buffer 0 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0corr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Corrupt."]
        #[inline(always)]
        pub const fn set_buf0corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Buffer 0 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0nwa(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf0nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Buffer 1 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1of(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Overflow."]
        #[inline(always)]
        pub const fn set_buf1of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Buffer 1 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1uf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Underflow."]
        #[inline(always)]
        pub const fn set_buf1uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Buffer 1 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1thr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf1thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Buffer 1 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1corr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Corrupt."]
        #[inline(always)]
        pub const fn set_buf1corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Buffer 1 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1nwa(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf1nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Buffer 2 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2of(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Overflow."]
        #[inline(always)]
        pub const fn set_buf2of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Buffer 2 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2uf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Underflow."]
        #[inline(always)]
        pub const fn set_buf2uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Buffer 2 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2thr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf2thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Buffer 2 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2corr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Corrupt."]
        #[inline(always)]
        pub const fn set_buf2corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Buffer 2 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2nwa(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf2nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Buffer 3 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3of(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Overflow."]
        #[inline(always)]
        pub const fn set_buf3of(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Buffer 3 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3uf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Underflow."]
        #[inline(always)]
        pub const fn set_buf3uf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Buffer 3 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3thr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf3thr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Buffer 3 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3corr(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Corrupt."]
        #[inline(always)]
        pub const fn set_buf3corr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Buffer 3 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3nwa(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf3nwa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Bus Error."]
        #[must_use]
        #[inline(always)]
        pub const fn buserror(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error."]
        #[inline(always)]
        pub const fn set_buserror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seqif {
        #[inline(always)]
        fn default() -> Seqif {
            Seqif(0)
        }
    }
    impl core::fmt::Debug for Seqif {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seqif")
                .field("buf0of", &self.buf0of())
                .field("buf0uf", &self.buf0uf())
                .field("buf0thr", &self.buf0thr())
                .field("buf0corr", &self.buf0corr())
                .field("buf0nwa", &self.buf0nwa())
                .field("buf1of", &self.buf1of())
                .field("buf1uf", &self.buf1uf())
                .field("buf1thr", &self.buf1thr())
                .field("buf1corr", &self.buf1corr())
                .field("buf1nwa", &self.buf1nwa())
                .field("buf2of", &self.buf2of())
                .field("buf2uf", &self.buf2uf())
                .field("buf2thr", &self.buf2thr())
                .field("buf2corr", &self.buf2corr())
                .field("buf2nwa", &self.buf2nwa())
                .field("buf3of", &self.buf3of())
                .field("buf3uf", &self.buf3uf())
                .field("buf3thr", &self.buf3thr())
                .field("buf3corr", &self.buf3corr())
                .field("buf3nwa", &self.buf3nwa())
                .field("buserror", &self.buserror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seqif {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seqif {{ buf0of: {=bool:?}, buf0uf: {=bool:?}, buf0thr: {=bool:?}, buf0corr: {=bool:?}, buf0nwa: {=bool:?}, buf1of: {=bool:?}, buf1uf: {=bool:?}, buf1thr: {=bool:?}, buf1corr: {=bool:?}, buf1nwa: {=bool:?}, buf2of: {=bool:?}, buf2uf: {=bool:?}, buf2thr: {=bool:?}, buf2corr: {=bool:?}, buf2nwa: {=bool:?}, buf3of: {=bool:?}, buf3uf: {=bool:?}, buf3thr: {=bool:?}, buf3corr: {=bool:?}, buf3nwa: {=bool:?}, buserror: {=bool:?} }}",
                self.buf0of(),
                self.buf0uf(),
                self.buf0thr(),
                self.buf0corr(),
                self.buf0nwa(),
                self.buf1of(),
                self.buf1uf(),
                self.buf1thr(),
                self.buf1corr(),
                self.buf1nwa(),
                self.buf2of(),
                self.buf2uf(),
                self.buf2thr(),
                self.buf2corr(),
                self.buf2nwa(),
                self.buf3of(),
                self.buf3uf(),
                self.buf3thr(),
                self.buf3corr(),
                self.buf3nwa(),
                self.buserror()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sfmien(pub u32);
    impl Sfmien {
        #[doc = "BUF0OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0ofsfmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0ofsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BUF0UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0ufsfmien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0ufsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BUF0THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0thrsfmien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0thrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BUF0CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0corrsfmien(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0corrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BUF0NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0nwasfmien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BUF0NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf0nwasfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "BUF1OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1ofsfmien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1ofsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BUF1UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1ufsfmien(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1ufsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BUF1THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1thrsfmien(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1thrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "BUF1CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1corrsfmien(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1corrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BUF1NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1nwasfmien(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BUF1NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf1nwasfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BUF2OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2ofsfmien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2ofsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BUF2UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2ufsfmien(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2ufsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BUF2THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2thrsfmien(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2thrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUF2CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2corrsfmien(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2corrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BUF2NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2nwasfmien(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "BUF2NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf2nwasfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BUF3OF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3ofsfmien(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3OF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3ofsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BUF3UF Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3ufsfmien(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3UF Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3ufsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "BUF3THR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3thrsfmien(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3THR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3thrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BUF3CORR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3corrsfmien(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3CORR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3corrsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "BUF3NWA Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3nwasfmien(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BUF3NWA Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buf3nwasfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "BUSERROR Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buserrorsfmien(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "BUSERROR Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buserrorsfmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sfmien {
        #[inline(always)]
        fn default() -> Sfmien {
            Sfmien(0)
        }
    }
    impl core::fmt::Debug for Sfmien {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sfmien")
                .field("buf0ofsfmien", &self.buf0ofsfmien())
                .field("buf0ufsfmien", &self.buf0ufsfmien())
                .field("buf0thrsfmien", &self.buf0thrsfmien())
                .field("buf0corrsfmien", &self.buf0corrsfmien())
                .field("buf0nwasfmien", &self.buf0nwasfmien())
                .field("buf1ofsfmien", &self.buf1ofsfmien())
                .field("buf1ufsfmien", &self.buf1ufsfmien())
                .field("buf1thrsfmien", &self.buf1thrsfmien())
                .field("buf1corrsfmien", &self.buf1corrsfmien())
                .field("buf1nwasfmien", &self.buf1nwasfmien())
                .field("buf2ofsfmien", &self.buf2ofsfmien())
                .field("buf2ufsfmien", &self.buf2ufsfmien())
                .field("buf2thrsfmien", &self.buf2thrsfmien())
                .field("buf2corrsfmien", &self.buf2corrsfmien())
                .field("buf2nwasfmien", &self.buf2nwasfmien())
                .field("buf3ofsfmien", &self.buf3ofsfmien())
                .field("buf3ufsfmien", &self.buf3ufsfmien())
                .field("buf3thrsfmien", &self.buf3thrsfmien())
                .field("buf3corrsfmien", &self.buf3corrsfmien())
                .field("buf3nwasfmien", &self.buf3nwasfmien())
                .field("buserrorsfmien", &self.buserrorsfmien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sfmien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sfmien {{ buf0ofsfmien: {=bool:?}, buf0ufsfmien: {=bool:?}, buf0thrsfmien: {=bool:?}, buf0corrsfmien: {=bool:?}, buf0nwasfmien: {=bool:?}, buf1ofsfmien: {=bool:?}, buf1ufsfmien: {=bool:?}, buf1thrsfmien: {=bool:?}, buf1corrsfmien: {=bool:?}, buf1nwasfmien: {=bool:?}, buf2ofsfmien: {=bool:?}, buf2ufsfmien: {=bool:?}, buf2thrsfmien: {=bool:?}, buf2corrsfmien: {=bool:?}, buf2nwasfmien: {=bool:?}, buf3ofsfmien: {=bool:?}, buf3ufsfmien: {=bool:?}, buf3thrsfmien: {=bool:?}, buf3corrsfmien: {=bool:?}, buf3nwasfmien: {=bool:?}, buserrorsfmien: {=bool:?} }}",
                self.buf0ofsfmien(),
                self.buf0ufsfmien(),
                self.buf0thrsfmien(),
                self.buf0corrsfmien(),
                self.buf0nwasfmien(),
                self.buf1ofsfmien(),
                self.buf1ufsfmien(),
                self.buf1thrsfmien(),
                self.buf1corrsfmien(),
                self.buf1nwasfmien(),
                self.buf2ofsfmien(),
                self.buf2ufsfmien(),
                self.buf2thrsfmien(),
                self.buf2corrsfmien(),
                self.buf2nwasfmien(),
                self.buf3ofsfmien(),
                self.buf3ufsfmien(),
                self.buf3thrsfmien(),
                self.buf3corrsfmien(),
                self.buf3nwasfmien(),
                self.buserrorsfmien()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sfmif(pub u32);
    impl Sfmif {
        #[doc = "Buffer 0 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0ofsfmif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Overflow."]
        #[inline(always)]
        pub const fn set_buf0ofsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Buffer 0 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0ufsfmif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Underflow."]
        #[inline(always)]
        pub const fn set_buf0ufsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Buffer 0 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0thrsfmif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf0thrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Buffer 0 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0corrsfmif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Corrupt."]
        #[inline(always)]
        pub const fn set_buf0corrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Buffer 0 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf0nwasfmif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf0nwasfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Buffer 1 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1ofsfmif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Overflow."]
        #[inline(always)]
        pub const fn set_buf1ofsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Buffer 1 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1ufsfmif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Underflow."]
        #[inline(always)]
        pub const fn set_buf1ufsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Buffer 1 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1thrsfmif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf1thrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Buffer 1 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1corrsfmif(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Corrupt."]
        #[inline(always)]
        pub const fn set_buf1corrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Buffer 1 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf1nwasfmif(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 1 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf1nwasfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Buffer 2 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2ofsfmif(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Overflow."]
        #[inline(always)]
        pub const fn set_buf2ofsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Buffer 2 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2ufsfmif(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Underflow."]
        #[inline(always)]
        pub const fn set_buf2ufsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Buffer 2 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2thrsfmif(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf2thrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Buffer 2 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2corrsfmif(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Corrupt."]
        #[inline(always)]
        pub const fn set_buf2corrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Buffer 2 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf2nwasfmif(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 2 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf2nwasfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Buffer 3 Overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3ofsfmif(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Overflow."]
        #[inline(always)]
        pub const fn set_buf3ofsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Buffer 3 Underflow."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3ufsfmif(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Underflow."]
        #[inline(always)]
        pub const fn set_buf3ufsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Buffer 3 Threshold Event."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3thrsfmif(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Threshold Event."]
        #[inline(always)]
        pub const fn set_buf3thrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Buffer 3 Corrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3corrsfmif(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Corrupt."]
        #[inline(always)]
        pub const fn set_buf3corrsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Buffer 3 Not Word-Aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn buf3nwasfmif(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer 3 Not Word-Aligned."]
        #[inline(always)]
        pub const fn set_buf3nwasfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Bus Error."]
        #[must_use]
        #[inline(always)]
        pub const fn buserrorsfmif(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error."]
        #[inline(always)]
        pub const fn set_buserrorsfmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sfmif {
        #[inline(always)]
        fn default() -> Sfmif {
            Sfmif(0)
        }
    }
    impl core::fmt::Debug for Sfmif {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sfmif")
                .field("buf0ofsfmif", &self.buf0ofsfmif())
                .field("buf0ufsfmif", &self.buf0ufsfmif())
                .field("buf0thrsfmif", &self.buf0thrsfmif())
                .field("buf0corrsfmif", &self.buf0corrsfmif())
                .field("buf0nwasfmif", &self.buf0nwasfmif())
                .field("buf1ofsfmif", &self.buf1ofsfmif())
                .field("buf1ufsfmif", &self.buf1ufsfmif())
                .field("buf1thrsfmif", &self.buf1thrsfmif())
                .field("buf1corrsfmif", &self.buf1corrsfmif())
                .field("buf1nwasfmif", &self.buf1nwasfmif())
                .field("buf2ofsfmif", &self.buf2ofsfmif())
                .field("buf2ufsfmif", &self.buf2ufsfmif())
                .field("buf2thrsfmif", &self.buf2thrsfmif())
                .field("buf2corrsfmif", &self.buf2corrsfmif())
                .field("buf2nwasfmif", &self.buf2nwasfmif())
                .field("buf3ofsfmif", &self.buf3ofsfmif())
                .field("buf3ufsfmif", &self.buf3ufsfmif())
                .field("buf3thrsfmif", &self.buf3thrsfmif())
                .field("buf3corrsfmif", &self.buf3corrsfmif())
                .field("buf3nwasfmif", &self.buf3nwasfmif())
                .field("buserrorsfmif", &self.buserrorsfmif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sfmif {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sfmif {{ buf0ofsfmif: {=bool:?}, buf0ufsfmif: {=bool:?}, buf0thrsfmif: {=bool:?}, buf0corrsfmif: {=bool:?}, buf0nwasfmif: {=bool:?}, buf1ofsfmif: {=bool:?}, buf1ufsfmif: {=bool:?}, buf1thrsfmif: {=bool:?}, buf1corrsfmif: {=bool:?}, buf1nwasfmif: {=bool:?}, buf2ofsfmif: {=bool:?}, buf2ufsfmif: {=bool:?}, buf2thrsfmif: {=bool:?}, buf2corrsfmif: {=bool:?}, buf2nwasfmif: {=bool:?}, buf3ofsfmif: {=bool:?}, buf3ufsfmif: {=bool:?}, buf3thrsfmif: {=bool:?}, buf3corrsfmif: {=bool:?}, buf3nwasfmif: {=bool:?}, buserrorsfmif: {=bool:?} }}",
                self.buf0ofsfmif(),
                self.buf0ufsfmif(),
                self.buf0thrsfmif(),
                self.buf0corrsfmif(),
                self.buf0nwasfmif(),
                self.buf1ofsfmif(),
                self.buf1ufsfmif(),
                self.buf1thrsfmif(),
                self.buf1corrsfmif(),
                self.buf1nwasfmif(),
                self.buf2ofsfmif(),
                self.buf2ufsfmif(),
                self.buf2thrsfmif(),
                self.buf2corrsfmif(),
                self.buf2nwasfmif(),
                self.buf3ofsfmif(),
                self.buf3ufsfmif(),
                self.buf3thrsfmif(),
                self.buf3corrsfmif(),
                self.buf3nwasfmif(),
                self.buserrorsfmif()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf0CtrlSize {
        #[doc = "Sets Buffer size to 64 bytes."]
        Size64 = 0x0,
        #[doc = "Sets Buffer size to 128 bytes."]
        Size128 = 0x01,
        #[doc = "Sets Buffer size to 256 bytes."]
        Size256 = 0x02,
        #[doc = "Sets Buffer size to 512 bytes."]
        Size512 = 0x03,
        #[doc = "Sets Buffer size to 1024 bytes."]
        Size1024 = 0x04,
        #[doc = "Sets Buffer size to 2048 bytes."]
        Size2048 = 0x05,
        #[doc = "Sets Buffer size to 4096 bytes."]
        Size4096 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Buf0CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf0CtrlSize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf0CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Buf0CtrlSize {
            Buf0CtrlSize::from_bits(val)
        }
    }
    impl From<Buf0CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Buf0CtrlSize) -> u8 {
            Buf0CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf0ThresholdctrlThresholdmode {
        #[doc = "THRESHOLDIF will be set if BYTES is larger than THRESHOLD."]
        Larger = 0x0,
        #[doc = "THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD."]
        Lessorequal = 0x01,
    }
    impl Buf0ThresholdctrlThresholdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf0ThresholdctrlThresholdmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf0ThresholdctrlThresholdmode {
        #[inline(always)]
        fn from(val: u8) -> Buf0ThresholdctrlThresholdmode {
            Buf0ThresholdctrlThresholdmode::from_bits(val)
        }
    }
    impl From<Buf0ThresholdctrlThresholdmode> for u8 {
        #[inline(always)]
        fn from(val: Buf0ThresholdctrlThresholdmode) -> u8 {
            Buf0ThresholdctrlThresholdmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf1CtrlSize {
        #[doc = "Sets Buffer size to 64 bytes."]
        Size64 = 0x0,
        #[doc = "Sets Buffer size to 128 bytes."]
        Size128 = 0x01,
        #[doc = "Sets Buffer size to 256 bytes."]
        Size256 = 0x02,
        #[doc = "Sets Buffer size to 512 bytes."]
        Size512 = 0x03,
        #[doc = "Sets Buffer size to 1024 bytes."]
        Size1024 = 0x04,
        #[doc = "Sets Buffer size to 2048 bytes."]
        Size2048 = 0x05,
        #[doc = "Sets Buffer size to 4096 bytes."]
        Size4096 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Buf1CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf1CtrlSize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf1CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Buf1CtrlSize {
            Buf1CtrlSize::from_bits(val)
        }
    }
    impl From<Buf1CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Buf1CtrlSize) -> u8 {
            Buf1CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf1ThresholdctrlThresholdmode {
        #[doc = "THRESHOLDIF will be set if BYTES is larger than THRESHOLD."]
        Larger = 0x0,
        #[doc = "THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD."]
        Lessorequal = 0x01,
    }
    impl Buf1ThresholdctrlThresholdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf1ThresholdctrlThresholdmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf1ThresholdctrlThresholdmode {
        #[inline(always)]
        fn from(val: u8) -> Buf1ThresholdctrlThresholdmode {
            Buf1ThresholdctrlThresholdmode::from_bits(val)
        }
    }
    impl From<Buf1ThresholdctrlThresholdmode> for u8 {
        #[inline(always)]
        fn from(val: Buf1ThresholdctrlThresholdmode) -> u8 {
            Buf1ThresholdctrlThresholdmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf2CtrlSize {
        #[doc = "Sets Buffer size to 64 bytes."]
        Size64 = 0x0,
        #[doc = "Sets Buffer size to 128 bytes."]
        Size128 = 0x01,
        #[doc = "Sets Buffer size to 256 bytes."]
        Size256 = 0x02,
        #[doc = "Sets Buffer size to 512 bytes."]
        Size512 = 0x03,
        #[doc = "Sets Buffer size to 1024 bytes."]
        Size1024 = 0x04,
        #[doc = "Sets Buffer size to 2048 bytes."]
        Size2048 = 0x05,
        #[doc = "Sets Buffer size to 4096 bytes."]
        Size4096 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Buf2CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf2CtrlSize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf2CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Buf2CtrlSize {
            Buf2CtrlSize::from_bits(val)
        }
    }
    impl From<Buf2CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Buf2CtrlSize) -> u8 {
            Buf2CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf2ThresholdctrlThresholdmode {
        #[doc = "THRESHOLDIF will be set if BYTES is larger than THRESHOLD."]
        Larger = 0x0,
        #[doc = "THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD."]
        Lessorequal = 0x01,
    }
    impl Buf2ThresholdctrlThresholdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf2ThresholdctrlThresholdmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf2ThresholdctrlThresholdmode {
        #[inline(always)]
        fn from(val: u8) -> Buf2ThresholdctrlThresholdmode {
            Buf2ThresholdctrlThresholdmode::from_bits(val)
        }
    }
    impl From<Buf2ThresholdctrlThresholdmode> for u8 {
        #[inline(always)]
        fn from(val: Buf2ThresholdctrlThresholdmode) -> u8 {
            Buf2ThresholdctrlThresholdmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf3CtrlSize {
        #[doc = "Sets Buffer size to 64 bytes."]
        Size64 = 0x0,
        #[doc = "Sets Buffer size to 128 bytes."]
        Size128 = 0x01,
        #[doc = "Sets Buffer size to 256 bytes."]
        Size256 = 0x02,
        #[doc = "Sets Buffer size to 512 bytes."]
        Size512 = 0x03,
        #[doc = "Sets Buffer size to 1024 bytes."]
        Size1024 = 0x04,
        #[doc = "Sets Buffer size to 2048 bytes."]
        Size2048 = 0x05,
        #[doc = "Sets Buffer size to 4096 bytes."]
        Size4096 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Buf3CtrlSize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf3CtrlSize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf3CtrlSize {
        #[inline(always)]
        fn from(val: u8) -> Buf3CtrlSize {
            Buf3CtrlSize::from_bits(val)
        }
    }
    impl From<Buf3CtrlSize> for u8 {
        #[inline(always)]
        fn from(val: Buf3CtrlSize) -> u8 {
            Buf3CtrlSize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Buf3ThresholdctrlThresholdmode {
        #[doc = "THRESHOLDIF will be set if BYTES is larger than THRESHOLD."]
        Larger = 0x0,
        #[doc = "THRESHOLDIF will be set if BYTES is less than or equal to THRESHOLD."]
        Lessorequal = 0x01,
    }
    impl Buf3ThresholdctrlThresholdmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Buf3ThresholdctrlThresholdmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Buf3ThresholdctrlThresholdmode {
        #[inline(always)]
        fn from(val: u8) -> Buf3ThresholdctrlThresholdmode {
            Buf3ThresholdctrlThresholdmode::from_bits(val)
        }
    }
    impl From<Buf3ThresholdctrlThresholdmode> for u8 {
        #[inline(always)]
        fn from(val: Buf3ThresholdctrlThresholdmode) -> u8 {
            Buf3ThresholdctrlThresholdmode::to_bits(val)
        }
    }
}
