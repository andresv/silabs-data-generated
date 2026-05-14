#[doc = "BURAM peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buram {
    ptr: *mut u8,
}
unsafe impl Send for Buram {}
unsafe impl Sync for Buram {}
impl Buram {
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
    pub const fn ret0_reg(self) -> crate::common::Reg<regs::Ret0Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret1_reg(self) -> crate::common::Reg<regs::Ret1Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret2_reg(self) -> crate::common::Reg<regs::Ret2Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret3_reg(self) -> crate::common::Reg<regs::Ret3Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret4_reg(self) -> crate::common::Reg<regs::Ret4Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret5_reg(self) -> crate::common::Reg<regs::Ret5Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret6_reg(self) -> crate::common::Reg<regs::Ret6Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret7_reg(self) -> crate::common::Reg<regs::Ret7Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret8_reg(self) -> crate::common::Reg<regs::Ret8Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret9_reg(self) -> crate::common::Reg<regs::Ret9Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret10_reg(self) -> crate::common::Reg<regs::Ret10Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret11_reg(self) -> crate::common::Reg<regs::Ret11Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret12_reg(self) -> crate::common::Reg<regs::Ret12Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret13_reg(self) -> crate::common::Reg<regs::Ret13Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret14_reg(self) -> crate::common::Reg<regs::Ret14Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret15_reg(self) -> crate::common::Reg<regs::Ret15Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret16_reg(self) -> crate::common::Reg<regs::Ret16Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret17_reg(self) -> crate::common::Reg<regs::Ret17Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret18_reg(self) -> crate::common::Reg<regs::Ret18Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret19_reg(self) -> crate::common::Reg<regs::Ret19Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret20_reg(self) -> crate::common::Reg<regs::Ret20Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret21_reg(self) -> crate::common::Reg<regs::Ret21Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret22_reg(self) -> crate::common::Reg<regs::Ret22Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret23_reg(self) -> crate::common::Reg<regs::Ret23Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret24_reg(self) -> crate::common::Reg<regs::Ret24Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret25_reg(self) -> crate::common::Reg<regs::Ret25Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret26_reg(self) -> crate::common::Reg<regs::Ret26Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret27_reg(self) -> crate::common::Reg<regs::Ret27Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret28_reg(self) -> crate::common::Reg<regs::Ret28Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret29_reg(self) -> crate::common::Reg<regs::Ret29Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret30_reg(self) -> crate::common::Reg<regs::Ret30Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn ret31_reg(self) -> crate::common::Reg<regs::Ret31Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret0_reg_set(self) -> crate::common::Reg<regs::Ret0Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret1_reg_set(self) -> crate::common::Reg<regs::Ret1Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret2_reg_set(self) -> crate::common::Reg<regs::Ret2Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret3_reg_set(self) -> crate::common::Reg<regs::Ret3Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret4_reg_set(self) -> crate::common::Reg<regs::Ret4Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret5_reg_set(self) -> crate::common::Reg<regs::Ret5Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret6_reg_set(self) -> crate::common::Reg<regs::Ret6Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret7_reg_set(self) -> crate::common::Reg<regs::Ret7Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret8_reg_set(self) -> crate::common::Reg<regs::Ret8Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret9_reg_set(self) -> crate::common::Reg<regs::Ret9Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret10_reg_set(self) -> crate::common::Reg<regs::Ret10Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret11_reg_set(self) -> crate::common::Reg<regs::Ret11Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret12_reg_set(self) -> crate::common::Reg<regs::Ret12Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret13_reg_set(self) -> crate::common::Reg<regs::Ret13Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret14_reg_set(self) -> crate::common::Reg<regs::Ret14Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret15_reg_set(self) -> crate::common::Reg<regs::Ret15Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret16_reg_set(self) -> crate::common::Reg<regs::Ret16Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret17_reg_set(self) -> crate::common::Reg<regs::Ret17Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret18_reg_set(self) -> crate::common::Reg<regs::Ret18Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret19_reg_set(self) -> crate::common::Reg<regs::Ret19Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret20_reg_set(self) -> crate::common::Reg<regs::Ret20Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret21_reg_set(self) -> crate::common::Reg<regs::Ret21Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret22_reg_set(self) -> crate::common::Reg<regs::Ret22Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret23_reg_set(self) -> crate::common::Reg<regs::Ret23Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret24_reg_set(self) -> crate::common::Reg<regs::Ret24Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret25_reg_set(self) -> crate::common::Reg<regs::Ret25Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret26_reg_set(self) -> crate::common::Reg<regs::Ret26Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret27_reg_set(self) -> crate::common::Reg<regs::Ret27Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret28_reg_set(self) -> crate::common::Reg<regs::Ret28Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret29_reg_set(self) -> crate::common::Reg<regs::Ret29Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret30_reg_set(self) -> crate::common::Reg<regs::Ret30Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ret31_reg_set(self) -> crate::common::Reg<regs::Ret31Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret0_reg_clr(self) -> crate::common::Reg<regs::Ret0Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret1_reg_clr(self) -> crate::common::Reg<regs::Ret1Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret2_reg_clr(self) -> crate::common::Reg<regs::Ret2Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret3_reg_clr(self) -> crate::common::Reg<regs::Ret3Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret4_reg_clr(self) -> crate::common::Reg<regs::Ret4Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret5_reg_clr(self) -> crate::common::Reg<regs::Ret5Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret6_reg_clr(self) -> crate::common::Reg<regs::Ret6Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret7_reg_clr(self) -> crate::common::Reg<regs::Ret7Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret8_reg_clr(self) -> crate::common::Reg<regs::Ret8Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret9_reg_clr(self) -> crate::common::Reg<regs::Ret9Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret10_reg_clr(self) -> crate::common::Reg<regs::Ret10Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret11_reg_clr(self) -> crate::common::Reg<regs::Ret11Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret12_reg_clr(self) -> crate::common::Reg<regs::Ret12Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret13_reg_clr(self) -> crate::common::Reg<regs::Ret13Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret14_reg_clr(self) -> crate::common::Reg<regs::Ret14Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret15_reg_clr(self) -> crate::common::Reg<regs::Ret15Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret16_reg_clr(self) -> crate::common::Reg<regs::Ret16Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret17_reg_clr(self) -> crate::common::Reg<regs::Ret17Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret18_reg_clr(self) -> crate::common::Reg<regs::Ret18Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret19_reg_clr(self) -> crate::common::Reg<regs::Ret19Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x204cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret20_reg_clr(self) -> crate::common::Reg<regs::Ret20Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret21_reg_clr(self) -> crate::common::Reg<regs::Ret21Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret22_reg_clr(self) -> crate::common::Reg<regs::Ret22Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret23_reg_clr(self) -> crate::common::Reg<regs::Ret23Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x205cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret24_reg_clr(self) -> crate::common::Reg<regs::Ret24Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret25_reg_clr(self) -> crate::common::Reg<regs::Ret25Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret26_reg_clr(self) -> crate::common::Reg<regs::Ret26Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret27_reg_clr(self) -> crate::common::Reg<regs::Ret27Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x206cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret28_reg_clr(self) -> crate::common::Reg<regs::Ret28Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret29_reg_clr(self) -> crate::common::Reg<regs::Ret29Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret30_reg_clr(self) -> crate::common::Reg<regs::Ret30Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2078usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ret31_reg_clr(self) -> crate::common::Reg<regs::Ret31Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x207cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret0_reg_tgl(self) -> crate::common::Reg<regs::Ret0Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret1_reg_tgl(self) -> crate::common::Reg<regs::Ret1Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret2_reg_tgl(self) -> crate::common::Reg<regs::Ret2Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret3_reg_tgl(self) -> crate::common::Reg<regs::Ret3Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret4_reg_tgl(self) -> crate::common::Reg<regs::Ret4Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret5_reg_tgl(self) -> crate::common::Reg<regs::Ret5Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret6_reg_tgl(self) -> crate::common::Reg<regs::Ret6Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret7_reg_tgl(self) -> crate::common::Reg<regs::Ret7Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret8_reg_tgl(self) -> crate::common::Reg<regs::Ret8Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret9_reg_tgl(self) -> crate::common::Reg<regs::Ret9Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret10_reg_tgl(self) -> crate::common::Reg<regs::Ret10Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret11_reg_tgl(self) -> crate::common::Reg<regs::Ret11Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret12_reg_tgl(self) -> crate::common::Reg<regs::Ret12Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret13_reg_tgl(self) -> crate::common::Reg<regs::Ret13Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret14_reg_tgl(self) -> crate::common::Reg<regs::Ret14Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret15_reg_tgl(self) -> crate::common::Reg<regs::Ret15Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret16_reg_tgl(self) -> crate::common::Reg<regs::Ret16Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret17_reg_tgl(self) -> crate::common::Reg<regs::Ret17Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret18_reg_tgl(self) -> crate::common::Reg<regs::Ret18Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret19_reg_tgl(self) -> crate::common::Reg<regs::Ret19Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret20_reg_tgl(self) -> crate::common::Reg<regs::Ret20Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret21_reg_tgl(self) -> crate::common::Reg<regs::Ret21Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret22_reg_tgl(self) -> crate::common::Reg<regs::Ret22Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret23_reg_tgl(self) -> crate::common::Reg<regs::Ret23Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x305cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret24_reg_tgl(self) -> crate::common::Reg<regs::Ret24Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret25_reg_tgl(self) -> crate::common::Reg<regs::Ret25Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret26_reg_tgl(self) -> crate::common::Reg<regs::Ret26Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret27_reg_tgl(self) -> crate::common::Reg<regs::Ret27Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret28_reg_tgl(self) -> crate::common::Reg<regs::Ret28Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret29_reg_tgl(self) -> crate::common::Reg<regs::Ret29Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret30_reg_tgl(self) -> crate::common::Reg<regs::Ret30Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3078usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ret31_reg_tgl(self) -> crate::common::Reg<regs::Ret31Reg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x307cusize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret0Reg(pub u32);
    impl Ret0Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret0Reg {
        #[inline(always)]
        fn default() -> Ret0Reg {
            Ret0Reg(0)
        }
    }
    impl core::fmt::Debug for Ret0Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret0Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret0Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret0Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret10Reg(pub u32);
    impl Ret10Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret10Reg {
        #[inline(always)]
        fn default() -> Ret10Reg {
            Ret10Reg(0)
        }
    }
    impl core::fmt::Debug for Ret10Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret10Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret10Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret10Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret11Reg(pub u32);
    impl Ret11Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret11Reg {
        #[inline(always)]
        fn default() -> Ret11Reg {
            Ret11Reg(0)
        }
    }
    impl core::fmt::Debug for Ret11Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret11Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret11Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret11Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret12Reg(pub u32);
    impl Ret12Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret12Reg {
        #[inline(always)]
        fn default() -> Ret12Reg {
            Ret12Reg(0)
        }
    }
    impl core::fmt::Debug for Ret12Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret12Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret12Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret12Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret13Reg(pub u32);
    impl Ret13Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret13Reg {
        #[inline(always)]
        fn default() -> Ret13Reg {
            Ret13Reg(0)
        }
    }
    impl core::fmt::Debug for Ret13Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret13Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret13Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret13Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret14Reg(pub u32);
    impl Ret14Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret14Reg {
        #[inline(always)]
        fn default() -> Ret14Reg {
            Ret14Reg(0)
        }
    }
    impl core::fmt::Debug for Ret14Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret14Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret14Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret14Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret15Reg(pub u32);
    impl Ret15Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret15Reg {
        #[inline(always)]
        fn default() -> Ret15Reg {
            Ret15Reg(0)
        }
    }
    impl core::fmt::Debug for Ret15Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret15Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret15Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret15Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret16Reg(pub u32);
    impl Ret16Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret16Reg {
        #[inline(always)]
        fn default() -> Ret16Reg {
            Ret16Reg(0)
        }
    }
    impl core::fmt::Debug for Ret16Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret16Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret16Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret16Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret17Reg(pub u32);
    impl Ret17Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret17Reg {
        #[inline(always)]
        fn default() -> Ret17Reg {
            Ret17Reg(0)
        }
    }
    impl core::fmt::Debug for Ret17Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret17Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret17Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret17Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret18Reg(pub u32);
    impl Ret18Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret18Reg {
        #[inline(always)]
        fn default() -> Ret18Reg {
            Ret18Reg(0)
        }
    }
    impl core::fmt::Debug for Ret18Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret18Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret18Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret18Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret19Reg(pub u32);
    impl Ret19Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret19Reg {
        #[inline(always)]
        fn default() -> Ret19Reg {
            Ret19Reg(0)
        }
    }
    impl core::fmt::Debug for Ret19Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret19Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret19Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret19Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret1Reg(pub u32);
    impl Ret1Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret1Reg {
        #[inline(always)]
        fn default() -> Ret1Reg {
            Ret1Reg(0)
        }
    }
    impl core::fmt::Debug for Ret1Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret1Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret1Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret1Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret20Reg(pub u32);
    impl Ret20Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret20Reg {
        #[inline(always)]
        fn default() -> Ret20Reg {
            Ret20Reg(0)
        }
    }
    impl core::fmt::Debug for Ret20Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret20Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret20Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret20Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret21Reg(pub u32);
    impl Ret21Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret21Reg {
        #[inline(always)]
        fn default() -> Ret21Reg {
            Ret21Reg(0)
        }
    }
    impl core::fmt::Debug for Ret21Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret21Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret21Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret21Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret22Reg(pub u32);
    impl Ret22Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret22Reg {
        #[inline(always)]
        fn default() -> Ret22Reg {
            Ret22Reg(0)
        }
    }
    impl core::fmt::Debug for Ret22Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret22Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret22Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret22Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret23Reg(pub u32);
    impl Ret23Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret23Reg {
        #[inline(always)]
        fn default() -> Ret23Reg {
            Ret23Reg(0)
        }
    }
    impl core::fmt::Debug for Ret23Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret23Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret23Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret23Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret24Reg(pub u32);
    impl Ret24Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret24Reg {
        #[inline(always)]
        fn default() -> Ret24Reg {
            Ret24Reg(0)
        }
    }
    impl core::fmt::Debug for Ret24Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret24Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret24Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret24Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret25Reg(pub u32);
    impl Ret25Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret25Reg {
        #[inline(always)]
        fn default() -> Ret25Reg {
            Ret25Reg(0)
        }
    }
    impl core::fmt::Debug for Ret25Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret25Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret25Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret25Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret26Reg(pub u32);
    impl Ret26Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret26Reg {
        #[inline(always)]
        fn default() -> Ret26Reg {
            Ret26Reg(0)
        }
    }
    impl core::fmt::Debug for Ret26Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret26Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret26Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret26Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret27Reg(pub u32);
    impl Ret27Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret27Reg {
        #[inline(always)]
        fn default() -> Ret27Reg {
            Ret27Reg(0)
        }
    }
    impl core::fmt::Debug for Ret27Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret27Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret27Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret27Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret28Reg(pub u32);
    impl Ret28Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret28Reg {
        #[inline(always)]
        fn default() -> Ret28Reg {
            Ret28Reg(0)
        }
    }
    impl core::fmt::Debug for Ret28Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret28Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret28Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret28Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret29Reg(pub u32);
    impl Ret29Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret29Reg {
        #[inline(always)]
        fn default() -> Ret29Reg {
            Ret29Reg(0)
        }
    }
    impl core::fmt::Debug for Ret29Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret29Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret29Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret29Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret2Reg(pub u32);
    impl Ret2Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret2Reg {
        #[inline(always)]
        fn default() -> Ret2Reg {
            Ret2Reg(0)
        }
    }
    impl core::fmt::Debug for Ret2Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret2Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret2Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret2Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret30Reg(pub u32);
    impl Ret30Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret30Reg {
        #[inline(always)]
        fn default() -> Ret30Reg {
            Ret30Reg(0)
        }
    }
    impl core::fmt::Debug for Ret30Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret30Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret30Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret30Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret31Reg(pub u32);
    impl Ret31Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret31Reg {
        #[inline(always)]
        fn default() -> Ret31Reg {
            Ret31Reg(0)
        }
    }
    impl core::fmt::Debug for Ret31Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret31Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret31Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret31Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret3Reg(pub u32);
    impl Ret3Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret3Reg {
        #[inline(always)]
        fn default() -> Ret3Reg {
            Ret3Reg(0)
        }
    }
    impl core::fmt::Debug for Ret3Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret3Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret3Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret3Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret4Reg(pub u32);
    impl Ret4Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret4Reg {
        #[inline(always)]
        fn default() -> Ret4Reg {
            Ret4Reg(0)
        }
    }
    impl core::fmt::Debug for Ret4Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret4Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret4Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret4Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret5Reg(pub u32);
    impl Ret5Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret5Reg {
        #[inline(always)]
        fn default() -> Ret5Reg {
            Ret5Reg(0)
        }
    }
    impl core::fmt::Debug for Ret5Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret5Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret5Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret5Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret6Reg(pub u32);
    impl Ret6Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret6Reg {
        #[inline(always)]
        fn default() -> Ret6Reg {
            Ret6Reg(0)
        }
    }
    impl core::fmt::Debug for Ret6Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret6Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret6Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret6Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret7Reg(pub u32);
    impl Ret7Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret7Reg {
        #[inline(always)]
        fn default() -> Ret7Reg {
            Ret7Reg(0)
        }
    }
    impl core::fmt::Debug for Ret7Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret7Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret7Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret7Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret8Reg(pub u32);
    impl Ret8Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret8Reg {
        #[inline(always)]
        fn default() -> Ret8Reg {
            Ret8Reg(0)
        }
    }
    impl core::fmt::Debug for Ret8Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret8Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret8Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret8Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ret9Reg(pub u32);
    impl Ret9Reg {
        #[doc = "Latch based Retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn retreg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch based Retention register."]
        #[inline(always)]
        pub const fn set_retreg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ret9Reg {
        #[inline(always)]
        fn default() -> Ret9Reg {
            Ret9Reg(0)
        }
    }
    impl core::fmt::Debug for Ret9Reg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ret9Reg").field("retreg", &self.retreg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ret9Reg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ret9Reg {{ retreg: {=u32:?} }}", self.retreg())
        }
    }
}
