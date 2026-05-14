#[doc = "MSC peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc {
    ptr: *mut u8,
}
unsafe impl Send for Msc {}
unsafe impl Sync for Msc {}
impl Msc {
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
    pub const fn readctrl(self) -> crate::common::Reg<regs::Readctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn rdatactrl(self) -> crate::common::Reg<regs::Rdatactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn writectrl(self) -> crate::common::Reg<regs::Writectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn writecmd(self) -> crate::common::Reg<regs::Writecmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn addrb(self) -> crate::common::Reg<regs::Addrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<regs::Wdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
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
    pub const fn userdatasize(self) -> crate::common::Reg<regs::Userdatasize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn misclockword(self) -> crate::common::Reg<regs::Misclockword, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pwrctrl(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pagelock0(self) -> crate::common::Reg<regs::Pagelock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pagelock1(self) -> crate::common::Reg<regs::Pagelock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pagelock2(self) -> crate::common::Reg<regs::Pagelock2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pagelock3(self) -> crate::common::Reg<regs::Pagelock3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pagelock4(self) -> crate::common::Reg<regs::Pagelock4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn pagelock5(self) -> crate::common::Reg<regs::Pagelock5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn readctrl_set(self) -> crate::common::Reg<regs::Readctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn rdatactrl_set(self) -> crate::common::Reg<regs::Rdatactrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn writectrl_set(self) -> crate::common::Reg<regs::Writectrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn writecmd_set(self) -> crate::common::Reg<regs::Writecmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn addrb_set(self) -> crate::common::Reg<regs::Addrb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn wdata_set(self) -> crate::common::Reg<regs::Wdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
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
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn misclockword_set(self) -> crate::common::Reg<regs::Misclockword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pwrctrl_set(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pagelock0_set(self) -> crate::common::Reg<regs::Pagelock0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pagelock1_set(self) -> crate::common::Reg<regs::Pagelock1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pagelock2_set(self) -> crate::common::Reg<regs::Pagelock2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pagelock3_set(self) -> crate::common::Reg<regs::Pagelock3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pagelock4_set(self) -> crate::common::Reg<regs::Pagelock4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pagelock5_set(self) -> crate::common::Reg<regs::Pagelock5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn readctrl_clr(self) -> crate::common::Reg<regs::Readctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn rdatactrl_clr(self) -> crate::common::Reg<regs::Rdatactrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn writectrl_clr(self) -> crate::common::Reg<regs::Writectrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn writecmd_clr(self) -> crate::common::Reg<regs::Writecmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn addrb_clr(self) -> crate::common::Reg<regs::Addrb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn wdata_clr(self) -> crate::common::Reg<regs::Wdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
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
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn misclockword_clr(self) -> crate::common::Reg<regs::Misclockword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pwrctrl_clr(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pagelock0_clr(self) -> crate::common::Reg<regs::Pagelock0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pagelock1_clr(self) -> crate::common::Reg<regs::Pagelock1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pagelock2_clr(self) -> crate::common::Reg<regs::Pagelock2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pagelock3_clr(self) -> crate::common::Reg<regs::Pagelock3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x212cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pagelock4_clr(self) -> crate::common::Reg<regs::Pagelock4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2130usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pagelock5_clr(self) -> crate::common::Reg<regs::Pagelock5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2134usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn readctrl_tgl(self) -> crate::common::Reg<regs::Readctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn rdatactrl_tgl(self) -> crate::common::Reg<regs::Rdatactrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn writectrl_tgl(self) -> crate::common::Reg<regs::Writectrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn writecmd_tgl(self) -> crate::common::Reg<regs::Writecmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn addrb_tgl(self) -> crate::common::Reg<regs::Addrb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn wdata_tgl(self) -> crate::common::Reg<regs::Wdata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
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
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn misclockword_tgl(self) -> crate::common::Reg<regs::Misclockword, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pwrctrl_tgl(self) -> crate::common::Reg<regs::Pwrctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pagelock0_tgl(self) -> crate::common::Reg<regs::Pagelock0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pagelock1_tgl(self) -> crate::common::Reg<regs::Pagelock1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3124usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pagelock2_tgl(self) -> crate::common::Reg<regs::Pagelock2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pagelock3_tgl(self) -> crate::common::Reg<regs::Pagelock3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x312cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pagelock4_tgl(self) -> crate::common::Reg<regs::Pagelock4, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3130usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pagelock5_tgl(self) -> crate::common::Reg<regs::Pagelock5, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3134usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Addrb(pub u32);
    impl Addrb {
        #[doc = "Page Erase or Write Address Buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn addrb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Page Erase or Write Address Buffer."]
        #[inline(always)]
        pub const fn set_addrb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Addrb {
        #[inline(always)]
        fn default() -> Addrb {
            Addrb(0)
        }
    }
    impl core::fmt::Debug for Addrb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Addrb").field("addrb", &self.addrb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Addrb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Addrb {{ addrb: {=u32:?} }}", self.addrb())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Flash Power Up Command."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power Up Command."]
        #[inline(always)]
        pub const fn set_pwrup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Flash power off/sleep command."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power off/sleep command."]
        #[inline(always)]
        pub const fn set_pwroff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("pwrup", &self.pwrup())
                .field("pwroff", &self.pwroff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ pwrup: {=bool:?}, pwroff: {=bool:?} }}",
                self.pwrup(),
                self.pwroff()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Erase Done Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn erase(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Erase Done Interrupt enable."]
        #[inline(always)]
        pub const fn set_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write Done Interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn write(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write Done Interrupt enable."]
        #[inline(always)]
        pub const fn set_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "write data buffer overflow irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wdataov(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "write data buffer overflow irq enable."]
        #[inline(always)]
        pub const fn set_wdataov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash Power Up Seq done irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrupf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power Up Seq done irq enable."]
        #[inline(always)]
        pub const fn set_pwrupf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Flash Power Off Seq done irq enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroff(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power Off Seq done irq enable."]
        #[inline(always)]
        pub const fn set_pwroff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("erase", &self.erase())
                .field("write", &self.write())
                .field("wdataov", &self.wdataov())
                .field("pwrupf", &self.pwrupf())
                .field("pwroff", &self.pwroff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ erase: {=bool:?}, write: {=bool:?}, wdataov: {=bool:?}, pwrupf: {=bool:?}, pwroff: {=bool:?} }}",
                self.erase(),
                self.write(),
                self.wdataov(),
                self.pwrupf(),
                self.pwroff()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Host Erase Done Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn erase(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Host Erase Done Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Host Write Done Interrupt Read Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn write(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Host Write Done Interrupt Read Flag."]
        #[inline(always)]
        pub const fn set_write(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Host write buffer overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn wdataov(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Host write buffer overflow."]
        #[inline(always)]
        pub const fn set_wdataov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash Power Up Sequence Complete Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrupf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power Up Sequence Complete Flag."]
        #[inline(always)]
        pub const fn set_pwrupf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Flash Power Off Sequence Complete Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroff(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Power Off Sequence Complete Flag."]
        #[inline(always)]
        pub const fn set_pwroff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("erase", &self.erase())
                .field("write", &self.write())
                .field("wdataov", &self.wdataov())
                .field("pwrupf", &self.pwrupf())
                .field("pwroff", &self.pwroff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ erase: {=bool:?}, write: {=bool:?}, wdataov: {=bool:?}, pwrupf: {=bool:?}, pwroff: {=bool:?} }}",
                self.erase(),
                self.write(),
                self.wdataov(),
                self.pwrupf(),
                self.pwroff()
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
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "Configuration Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockkey(&self) -> super::vals::Lockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Lockkey::from_bits(val as u16)
        }
        #[doc = "Configuration Lock."]
        #[inline(always)]
        pub const fn set_lockkey(&mut self, val: super::vals::Lockkey) {
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
    pub struct Misclockword(pub u32);
    impl Misclockword {
        #[doc = "Mass Erase Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn melockbit(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Mass Erase Lock."]
        #[inline(always)]
        pub const fn set_melockbit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "User Data Lock."]
        #[must_use]
        #[inline(always)]
        pub const fn udlockbit(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "User Data Lock."]
        #[inline(always)]
        pub const fn set_udlockbit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Misclockword {
        #[inline(always)]
        fn default() -> Misclockword {
            Misclockword(0)
        }
    }
    impl core::fmt::Debug for Misclockword {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misclockword")
                .field("melockbit", &self.melockbit())
                .field("udlockbit", &self.udlockbit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misclockword {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Misclockword {{ melockbit: {=bool:?}, udlockbit: {=bool:?} }}",
                self.melockbit(),
                self.udlockbit()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock0(pub u32);
    impl Pagelock0 {
        #[doc = "page lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "page lock bit."]
        #[inline(always)]
        pub const fn set_lockbit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pagelock0 {
        #[inline(always)]
        fn default() -> Pagelock0 {
            Pagelock0(0)
        }
    }
    impl core::fmt::Debug for Pagelock0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock0").field("lockbit", &self.lockbit()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pagelock0 {{ lockbit: {=u32:?} }}", self.lockbit())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock1(pub u32);
    impl Pagelock1 {
        #[doc = "page lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "page lock bit."]
        #[inline(always)]
        pub const fn set_lockbit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pagelock1 {
        #[inline(always)]
        fn default() -> Pagelock1 {
            Pagelock1(0)
        }
    }
    impl core::fmt::Debug for Pagelock1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock1").field("lockbit", &self.lockbit()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pagelock1 {{ lockbit: {=u32:?} }}", self.lockbit())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock2(pub u32);
    impl Pagelock2 {
        #[doc = "page lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "page lock bit."]
        #[inline(always)]
        pub const fn set_lockbit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pagelock2 {
        #[inline(always)]
        fn default() -> Pagelock2 {
            Pagelock2(0)
        }
    }
    impl core::fmt::Debug for Pagelock2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock2").field("lockbit", &self.lockbit()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pagelock2 {{ lockbit: {=u32:?} }}", self.lockbit())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock3(pub u32);
    impl Pagelock3 {
        #[doc = "page lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "page lock bit."]
        #[inline(always)]
        pub const fn set_lockbit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pagelock3 {
        #[inline(always)]
        fn default() -> Pagelock3 {
            Pagelock3(0)
        }
    }
    impl core::fmt::Debug for Pagelock3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock3").field("lockbit", &self.lockbit()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pagelock3 {{ lockbit: {=u32:?} }}", self.lockbit())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock4(pub u32);
    impl Pagelock4 {
        #[doc = "page lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "page lock bit."]
        #[inline(always)]
        pub const fn set_lockbit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pagelock4 {
        #[inline(always)]
        fn default() -> Pagelock4 {
            Pagelock4(0)
        }
    }
    impl core::fmt::Debug for Pagelock4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock4").field("lockbit", &self.lockbit()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pagelock4 {{ lockbit: {=u32:?} }}", self.lockbit())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pagelock5(pub u32);
    impl Pagelock5 {
        #[doc = "page lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "page lock bit."]
        #[inline(always)]
        pub const fn set_lockbit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pagelock5 {
        #[inline(always)]
        fn default() -> Pagelock5 {
            Pagelock5(0)
        }
    }
    impl core::fmt::Debug for Pagelock5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pagelock5").field("lockbit", &self.lockbit()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pagelock5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pagelock5 {{ lockbit: {=u32:?} }}", self.lockbit())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrctrl(pub u32);
    impl Pwrctrl {
        #[doc = "Power down Flash macro when enter EM1."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroffonem1entry(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down Flash macro when enter EM1."]
        #[inline(always)]
        pub const fn set_pwroffonem1entry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power down Flash macro when enter EM1P."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroffonem1pentry(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Power down Flash macro when enter EM1P."]
        #[inline(always)]
        pub const fn set_pwroffonem1pentry(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "POWER down flash again in EM1/EM1p."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroffentryagain(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "POWER down flash again in EM1/EM1p."]
        #[inline(always)]
        pub const fn set_pwroffentryagain(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Power down delay."]
        #[must_use]
        #[inline(always)]
        pub const fn pwroffdly(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Power down delay."]
        #[inline(always)]
        pub const fn set_pwroffdly(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Pwrctrl {
        #[inline(always)]
        fn default() -> Pwrctrl {
            Pwrctrl(0)
        }
    }
    impl core::fmt::Debug for Pwrctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwrctrl")
                .field("pwroffonem1entry", &self.pwroffonem1entry())
                .field("pwroffonem1pentry", &self.pwroffonem1pentry())
                .field("pwroffentryagain", &self.pwroffentryagain())
                .field("pwroffdly", &self.pwroffdly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pwrctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pwrctrl {{ pwroffonem1entry: {=bool:?}, pwroffonem1pentry: {=bool:?}, pwroffentryagain: {=bool:?}, pwroffdly: {=u8:?} }}",
                self.pwroffonem1entry(),
                self.pwroffonem1pentry(),
                self.pwroffentryagain(),
                self.pwroffdly()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdatactrl(pub u32);
    impl Rdatactrl {
        #[doc = "Automatic Invalidate Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn afdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic Invalidate Disable."]
        #[inline(always)]
        pub const fn set_afdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash dout pipeline buffer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn doutbufen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Flash dout pipeline buffer enable."]
        #[inline(always)]
        pub const fn set_doutbufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Rdatactrl {
        #[inline(always)]
        fn default() -> Rdatactrl {
            Rdatactrl(0)
        }
    }
    impl core::fmt::Debug for Rdatactrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdatactrl")
                .field("afdis", &self.afdis())
                .field("doutbufen", &self.doutbufen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdatactrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rdatactrl {{ afdis: {=bool:?}, doutbufen: {=bool:?} }}",
                self.afdis(),
                self.doutbufen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Readctrl(pub u32);
    impl Readctrl {
        #[doc = "Read Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Read Mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Readctrl {
        #[inline(always)]
        fn default() -> Readctrl {
            Readctrl(0)
        }
    }
    impl core::fmt::Debug for Readctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Readctrl").field("mode", &self.mode()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Readctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Readctrl {{ mode: {:?} }}", self.mode())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Erase/Write Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Erase/Write Busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Access Locked."]
        #[must_use]
        #[inline(always)]
        pub const fn locked(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Access Locked."]
        #[inline(always)]
        pub const fn set_locked(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Invalid Write Address or Erase Page."]
        #[must_use]
        #[inline(always)]
        pub const fn invaddr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Invalid Write Address or Erase Page."]
        #[inline(always)]
        pub const fn set_invaddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "WDATA Write Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn wdataready(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "WDATA Write Ready."]
        #[inline(always)]
        pub const fn set_wdataready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Erase Operation Aborted."]
        #[must_use]
        #[inline(always)]
        pub const fn eraseaborted(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Erase Operation Aborted."]
        #[inline(always)]
        pub const fn set_eraseaborted(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Write Command In Queue."]
        #[must_use]
        #[inline(always)]
        pub const fn pending(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Write Command In Queue."]
        #[inline(always)]
        pub const fn set_pending(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Write Command Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Write Command Timeout."]
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EraseRange with skipped locked pages."]
        #[must_use]
        #[inline(always)]
        pub const fn rangepartial(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EraseRange with skipped locked pages."]
        #[inline(always)]
        pub const fn set_rangepartial(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register Lock Status."]
        #[must_use]
        #[inline(always)]
        pub const fn reglock(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Register Lock Status."]
        #[inline(always)]
        pub const fn set_reglock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Flash power on status."]
        #[must_use]
        #[inline(always)]
        pub const fn pwron(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power on status."]
        #[inline(always)]
        pub const fn set_pwron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Flash Write Ready."]
        #[must_use]
        #[inline(always)]
        pub const fn wready(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Flash Write Ready."]
        #[inline(always)]
        pub const fn set_wready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Flash power up checkerboard pattern chec."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrupckbdfailcount(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Flash power up checkerboard pattern chec."]
        #[inline(always)]
        pub const fn set_pwrupckbdfailcount(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
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
                .field("busy", &self.busy())
                .field("locked", &self.locked())
                .field("invaddr", &self.invaddr())
                .field("wdataready", &self.wdataready())
                .field("eraseaborted", &self.eraseaborted())
                .field("pending", &self.pending())
                .field("timeout", &self.timeout())
                .field("rangepartial", &self.rangepartial())
                .field("reglock", &self.reglock())
                .field("pwron", &self.pwron())
                .field("wready", &self.wready())
                .field("pwrupckbdfailcount", &self.pwrupckbdfailcount())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ busy: {=bool:?}, locked: {=bool:?}, invaddr: {=bool:?}, wdataready: {=bool:?}, eraseaborted: {=bool:?}, pending: {=bool:?}, timeout: {=bool:?}, rangepartial: {=bool:?}, reglock: {=bool:?}, pwron: {=bool:?}, wready: {=bool:?}, pwrupckbdfailcount: {=u8:?} }}",
                self.busy(),
                self.locked(),
                self.invaddr(),
                self.wdataready(),
                self.eraseaborted(),
                self.pending(),
                self.timeout(),
                self.rangepartial(),
                self.reglock(),
                self.pwron(),
                self.wready(),
                self.pwrupckbdfailcount()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Userdatasize(pub u32);
    impl Userdatasize {
        #[doc = "User Data Size."]
        #[must_use]
        #[inline(always)]
        pub const fn userdatasize(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "User Data Size."]
        #[inline(always)]
        pub const fn set_userdatasize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Userdatasize {
        #[inline(always)]
        fn default() -> Userdatasize {
            Userdatasize(0)
        }
    }
    impl core::fmt::Debug for Userdatasize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Userdatasize")
                .field("userdatasize", &self.userdatasize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Userdatasize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Userdatasize {{ userdatasize: {=u8:?} }}", self.userdatasize())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdata(pub u32);
    impl Wdata {
        #[doc = "Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn dataw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write Data."]
        #[inline(always)]
        pub const fn set_dataw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wdata {
        #[inline(always)]
        fn default() -> Wdata {
            Wdata(0)
        }
    }
    impl core::fmt::Debug for Wdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdata").field("dataw", &self.dataw()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdata {{ dataw: {=u32:?} }}", self.dataw())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Writecmd(pub u32);
    impl Writecmd {
        #[doc = "Erase Page."]
        #[must_use]
        #[inline(always)]
        pub const fn erasepage(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Erase Page."]
        #[inline(always)]
        pub const fn set_erasepage(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End Write Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn writeend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End Write Mode."]
        #[inline(always)]
        pub const fn set_writeend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Erase range of pages."]
        #[must_use]
        #[inline(always)]
        pub const fn eraserange(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Erase range of pages."]
        #[inline(always)]
        pub const fn set_eraserange(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Abort erase sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn eraseabort(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Abort erase sequence."]
        #[inline(always)]
        pub const fn set_eraseabort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Mass erase region 0."]
        #[must_use]
        #[inline(always)]
        pub const fn erasemain0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase region 0."]
        #[inline(always)]
        pub const fn set_erasemain0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear WDATA state."]
        #[must_use]
        #[inline(always)]
        pub const fn clearwdata(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Clear WDATA state."]
        #[inline(always)]
        pub const fn set_clearwdata(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Writecmd {
        #[inline(always)]
        fn default() -> Writecmd {
            Writecmd(0)
        }
    }
    impl core::fmt::Debug for Writecmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Writecmd")
                .field("erasepage", &self.erasepage())
                .field("writeend", &self.writeend())
                .field("eraserange", &self.eraserange())
                .field("eraseabort", &self.eraseabort())
                .field("erasemain0", &self.erasemain0())
                .field("clearwdata", &self.clearwdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Writecmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Writecmd {{ erasepage: {=bool:?}, writeend: {=bool:?}, eraserange: {=bool:?}, eraseabort: {=bool:?}, erasemain0: {=bool:?}, clearwdata: {=bool:?} }}",
                self.erasepage(),
                self.writeend(),
                self.eraserange(),
                self.eraseabort(),
                self.erasemain0(),
                self.clearwdata()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Writectrl(pub u32);
    impl Writectrl {
        #[doc = "Enable Write/Erase Controller."]
        #[must_use]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Write/Erase Controller."]
        #[inline(always)]
        pub const fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Abort Page Erase on Interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn irqeraseabort(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Abort Page Erase on Interrupt."]
        #[inline(always)]
        pub const fn set_irqeraseabort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Low-Power Write."]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrite(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Low-Power Write."]
        #[inline(always)]
        pub const fn set_lpwrite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EraseRange Count."]
        #[must_use]
        #[inline(always)]
        pub const fn rangecount(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "EraseRange Count."]
        #[inline(always)]
        pub const fn set_rangecount(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Writectrl {
        #[inline(always)]
        fn default() -> Writectrl {
            Writectrl(0)
        }
    }
    impl core::fmt::Debug for Writectrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Writectrl")
                .field("wren", &self.wren())
                .field("irqeraseabort", &self.irqeraseabort())
                .field("lpwrite", &self.lpwrite())
                .field("rangecount", &self.rangecount())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Writectrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Writectrl {{ wren: {=bool:?}, irqeraseabort: {=bool:?}, lpwrite: {=bool:?}, rangecount: {=u16:?} }}",
                self.wren(),
                self.irqeraseabort(),
                self.lpwrite(),
                self.rangecount()
            )
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "LOCK."]
        pub const Lock: Self = Self(0x0);
        #[doc = "UNLOCK."]
        pub const Unlock: Self = Self(0x1b71);
    }
    impl Lockkey {
        pub const fn from_bits(val: u16) -> Lockkey {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Lockkey {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Lock"),
                0x1b71 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Lock"),
                0x1b71 => defmt::write!(f, "Unlock"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Lockkey {
        #[inline(always)]
        fn from(val: u16) -> Lockkey {
            Lockkey::from_bits(val)
        }
    }
    impl From<Lockkey> for u16 {
        #[inline(always)]
        fn from(val: Lockkey) -> u16 {
            Lockkey::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Zero wait-states inserted in fetch or read transfers."]
        Ws0 = 0x0,
        #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details."]
        Ws1 = 0x01,
        #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details."]
        Ws2 = 0x02,
        #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details."]
        Ws3 = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
}
