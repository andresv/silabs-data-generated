#[doc = "MVP peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mvp {
    ptr: *mut u8,
}
unsafe impl Send for Mvp {}
unsafe impl Sync for Mvp {}
impl Mvp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IP Version Register."]
    #[inline(always)]
    pub const fn ipversion(self) -> crate::common::Reg<regs::Ipversion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Block Enable Register."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Software Reset Register."]
    #[inline(always)]
    pub const fn swrst(self) -> crate::common::Reg<regs::Swrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Run Counter."]
    #[inline(always)]
    pub const fn perf0cnt(self) -> crate::common::Reg<regs::Perf0cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Run Counter."]
    #[inline(always)]
    pub const fn perf1cnt(self) -> crate::common::Reg<regs::Perf1cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Flags."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Fault Status Register."]
    #[inline(always)]
    pub const fn faultstatus(self) -> crate::common::Reg<regs::Faultstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Fault Address Register."]
    #[inline(always)]
    pub const fn faultaddr(self) -> crate::common::Reg<regs::Faultaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Program State Register."]
    #[inline(always)]
    pub const fn programstate(self) -> crate::common::Reg<regs::Programstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Array N Index State Register."]
    #[inline(always)]
    pub const fn array0indexstate(self) -> crate::common::Reg<regs::Array0indexstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Array N Index State Register."]
    #[inline(always)]
    pub const fn array1indexstate(self) -> crate::common::Reg<regs::Array1indexstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Array N Index State Register."]
    #[inline(always)]
    pub const fn array2indexstate(self) -> crate::common::Reg<regs::Array2indexstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Array N Index State Register."]
    #[inline(always)]
    pub const fn array3indexstate(self) -> crate::common::Reg<regs::Array3indexstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Array N Index State Register."]
    #[inline(always)]
    pub const fn array4indexstate(self) -> crate::common::Reg<regs::Array4indexstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop0state(self) -> crate::common::Reg<regs::Loop0state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop1state(self) -> crate::common::Reg<regs::Loop1state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop2state(self) -> crate::common::Reg<regs::Loop2state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop3state(self) -> crate::common::Reg<regs::Loop3state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop4state(self) -> crate::common::Reg<regs::Loop4state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop5state(self) -> crate::common::Reg<regs::Loop5state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop6state(self) -> crate::common::Reg<regs::Loop6state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Loop N State Register."]
    #[inline(always)]
    pub const fn loop7state(self) -> crate::common::Reg<regs::Loop7state, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu0regstate(self) -> crate::common::Reg<regs::Alu0regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu1regstate(self) -> crate::common::Reg<regs::Alu1regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu2regstate(self) -> crate::common::Reg<regs::Alu2regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu3regstate(self) -> crate::common::Reg<regs::Alu3regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu4regstate(self) -> crate::common::Reg<regs::Alu4regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu5regstate(self) -> crate::common::Reg<regs::Alu5regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu6regstate(self) -> crate::common::Reg<regs::Alu6regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "ALU Register."]
    #[inline(always)]
    pub const fn alu7regstate(self) -> crate::common::Reg<regs::Alu7regstate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Array N Base Address Register."]
    #[inline(always)]
    pub const fn array0addrcfg(self) -> crate::common::Reg<regs::Array0addrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[inline(always)]
    pub const fn array0dim0cfg(self) -> crate::common::Reg<regs::Array0dim0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[inline(always)]
    pub const fn array0dim1cfg(self) -> crate::common::Reg<regs::Array0dim1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[inline(always)]
    pub const fn array0dim2cfg(self) -> crate::common::Reg<regs::Array0dim2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Array N Base Address Register."]
    #[inline(always)]
    pub const fn array1addrcfg(self) -> crate::common::Reg<regs::Array1addrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[inline(always)]
    pub const fn array1dim0cfg(self) -> crate::common::Reg<regs::Array1dim0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[inline(always)]
    pub const fn array1dim1cfg(self) -> crate::common::Reg<regs::Array1dim1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[inline(always)]
    pub const fn array1dim2cfg(self) -> crate::common::Reg<regs::Array1dim2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Array N Base Address Register."]
    #[inline(always)]
    pub const fn array2addrcfg(self) -> crate::common::Reg<regs::Array2addrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[inline(always)]
    pub const fn array2dim0cfg(self) -> crate::common::Reg<regs::Array2dim0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[inline(always)]
    pub const fn array2dim1cfg(self) -> crate::common::Reg<regs::Array2dim1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[inline(always)]
    pub const fn array2dim2cfg(self) -> crate::common::Reg<regs::Array2dim2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Array N Base Address Register."]
    #[inline(always)]
    pub const fn array3addrcfg(self) -> crate::common::Reg<regs::Array3addrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[inline(always)]
    pub const fn array3dim0cfg(self) -> crate::common::Reg<regs::Array3dim0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[inline(always)]
    pub const fn array3dim1cfg(self) -> crate::common::Reg<regs::Array3dim1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[inline(always)]
    pub const fn array3dim2cfg(self) -> crate::common::Reg<regs::Array3dim2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Array N Base Address Register."]
    #[inline(always)]
    pub const fn array4addrcfg(self) -> crate::common::Reg<regs::Array4addrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[inline(always)]
    pub const fn array4dim0cfg(self) -> crate::common::Reg<regs::Array4dim0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[inline(always)]
    pub const fn array4dim1cfg(self) -> crate::common::Reg<regs::Array4dim1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[inline(always)]
    pub const fn array4dim2cfg(self) -> crate::common::Reg<regs::Array4dim2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop0cfg(self) -> crate::common::Reg<regs::Loop0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop0rst(self) -> crate::common::Reg<regs::Loop0rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop1cfg(self) -> crate::common::Reg<regs::Loop1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop1rst(self) -> crate::common::Reg<regs::Loop1rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop2cfg(self) -> crate::common::Reg<regs::Loop2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop2rst(self) -> crate::common::Reg<regs::Loop2rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop3cfg(self) -> crate::common::Reg<regs::Loop3cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop3rst(self) -> crate::common::Reg<regs::Loop3rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop4cfg(self) -> crate::common::Reg<regs::Loop4cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop4rst(self) -> crate::common::Reg<regs::Loop4rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop5cfg(self) -> crate::common::Reg<regs::Loop5cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop5rst(self) -> crate::common::Reg<regs::Loop5rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop6cfg(self) -> crate::common::Reg<regs::Loop6cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop6rst(self) -> crate::common::Reg<regs::Loop6rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Loop N Configuration Register."]
    #[inline(always)]
    pub const fn loop7cfg(self) -> crate::common::Reg<regs::Loop7cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[inline(always)]
    pub const fn loop7rst(self) -> crate::common::Reg<regs::Loop7rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr0cfg0(self) -> crate::common::Reg<regs::Instr0cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr0cfg1(self) -> crate::common::Reg<regs::Instr0cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr0cfg2(self) -> crate::common::Reg<regs::Instr0cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr1cfg0(self) -> crate::common::Reg<regs::Instr1cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr1cfg1(self) -> crate::common::Reg<regs::Instr1cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr1cfg2(self) -> crate::common::Reg<regs::Instr1cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr2cfg0(self) -> crate::common::Reg<regs::Instr2cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr2cfg1(self) -> crate::common::Reg<regs::Instr2cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr2cfg2(self) -> crate::common::Reg<regs::Instr2cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr3cfg0(self) -> crate::common::Reg<regs::Instr3cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr3cfg1(self) -> crate::common::Reg<regs::Instr3cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr3cfg2(self) -> crate::common::Reg<regs::Instr3cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr4cfg0(self) -> crate::common::Reg<regs::Instr4cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr4cfg1(self) -> crate::common::Reg<regs::Instr4cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr4cfg2(self) -> crate::common::Reg<regs::Instr4cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr5cfg0(self) -> crate::common::Reg<regs::Instr5cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr5cfg1(self) -> crate::common::Reg<regs::Instr5cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr5cfg2(self) -> crate::common::Reg<regs::Instr5cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr6cfg0(self) -> crate::common::Reg<regs::Instr6cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr6cfg1(self) -> crate::common::Reg<regs::Instr6cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr6cfg2(self) -> crate::common::Reg<regs::Instr6cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Instruction N Word 0."]
    #[inline(always)]
    pub const fn instr7cfg0(self) -> crate::common::Reg<regs::Instr7cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Instruction N word 1."]
    #[inline(always)]
    pub const fn instr7cfg1(self) -> crate::common::Reg<regs::Instr7cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Instruction N word 2."]
    #[inline(always)]
    pub const fn instr7cfg2(self) -> crate::common::Reg<regs::Instr7cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Debug Control Register."]
    #[inline(always)]
    pub const fn debugen(self) -> crate::common::Reg<regs::Debugen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn debugstepcnt(self) -> crate::common::Reg<regs::Debugstepcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Block Enable Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn en_set(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "Software Reset Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn swrst_set(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cfg_set(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "Program State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn programstate_set(self) -> crate::common::Reg<regs::Programstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array0indexstate_set(self) -> crate::common::Reg<regs::Array0indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array1indexstate_set(self) -> crate::common::Reg<regs::Array1indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array2indexstate_set(self) -> crate::common::Reg<regs::Array2indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array3indexstate_set(self) -> crate::common::Reg<regs::Array3indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array4indexstate_set(self) -> crate::common::Reg<regs::Array4indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop0state_set(self) -> crate::common::Reg<regs::Loop0state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop1state_set(self) -> crate::common::Reg<regs::Loop1state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop2state_set(self) -> crate::common::Reg<regs::Loop2state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop3state_set(self) -> crate::common::Reg<regs::Loop3state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop4state_set(self) -> crate::common::Reg<regs::Loop4state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop5state_set(self) -> crate::common::Reg<regs::Loop5state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop6state_set(self) -> crate::common::Reg<regs::Loop6state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop7state_set(self) -> crate::common::Reg<regs::Loop7state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu0regstate_set(self) -> crate::common::Reg<regs::Alu0regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu1regstate_set(self) -> crate::common::Reg<regs::Alu1regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu2regstate_set(self) -> crate::common::Reg<regs::Alu2regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu3regstate_set(self) -> crate::common::Reg<regs::Alu3regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu4regstate_set(self) -> crate::common::Reg<regs::Alu4regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu5regstate_set(self) -> crate::common::Reg<regs::Alu5regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu6regstate_set(self) -> crate::common::Reg<regs::Alu6regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn alu7regstate_set(self) -> crate::common::Reg<regs::Alu7regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array0addrcfg_set(self) -> crate::common::Reg<regs::Array0addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1084usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array0dim0cfg_set(self) -> crate::common::Reg<regs::Array0dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array0dim1cfg_set(self) -> crate::common::Reg<regs::Array0dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x108cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array0dim2cfg_set(self) -> crate::common::Reg<regs::Array0dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array1addrcfg_set(self) -> crate::common::Reg<regs::Array1addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array1dim0cfg_set(self) -> crate::common::Reg<regs::Array1dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array1dim1cfg_set(self) -> crate::common::Reg<regs::Array1dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array1dim2cfg_set(self) -> crate::common::Reg<regs::Array1dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array2addrcfg_set(self) -> crate::common::Reg<regs::Array2addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array2dim0cfg_set(self) -> crate::common::Reg<regs::Array2dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array2dim1cfg_set(self) -> crate::common::Reg<regs::Array2dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10acusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array2dim2cfg_set(self) -> crate::common::Reg<regs::Array2dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array3addrcfg_set(self) -> crate::common::Reg<regs::Array3addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array3dim0cfg_set(self) -> crate::common::Reg<regs::Array3dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array3dim1cfg_set(self) -> crate::common::Reg<regs::Array3dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10bcusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array3dim2cfg_set(self) -> crate::common::Reg<regs::Array3dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array4addrcfg_set(self) -> crate::common::Reg<regs::Array4addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array4dim0cfg_set(self) -> crate::common::Reg<regs::Array4dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array4dim1cfg_set(self) -> crate::common::Reg<regs::Array4dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn array4dim2cfg_set(self) -> crate::common::Reg<regs::Array4dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop0cfg_set(self) -> crate::common::Reg<regs::Loop0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop0rst_set(self) -> crate::common::Reg<regs::Loop0rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop1cfg_set(self) -> crate::common::Reg<regs::Loop1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10dcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop1rst_set(self) -> crate::common::Reg<regs::Loop1rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop2cfg_set(self) -> crate::common::Reg<regs::Loop2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop2rst_set(self) -> crate::common::Reg<regs::Loop2rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop3cfg_set(self) -> crate::common::Reg<regs::Loop3cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ecusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop3rst_set(self) -> crate::common::Reg<regs::Loop3rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop4cfg_set(self) -> crate::common::Reg<regs::Loop4cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop4rst_set(self) -> crate::common::Reg<regs::Loop4rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop5cfg_set(self) -> crate::common::Reg<regs::Loop5cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10fcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop5rst_set(self) -> crate::common::Reg<regs::Loop5rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop6cfg_set(self) -> crate::common::Reg<regs::Loop6cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop6rst_set(self) -> crate::common::Reg<regs::Loop6rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop7cfg_set(self) -> crate::common::Reg<regs::Loop7cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x110cusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn loop7rst_set(self) -> crate::common::Reg<regs::Loop7rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1110usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr0cfg0_set(self) -> crate::common::Reg<regs::Instr0cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr0cfg1_set(self) -> crate::common::Reg<regs::Instr0cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr0cfg2_set(self) -> crate::common::Reg<regs::Instr0cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr1cfg0_set(self) -> crate::common::Reg<regs::Instr1cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr1cfg1_set(self) -> crate::common::Reg<regs::Instr1cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr1cfg2_set(self) -> crate::common::Reg<regs::Instr1cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr2cfg0_set(self) -> crate::common::Reg<regs::Instr2cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr2cfg1_set(self) -> crate::common::Reg<regs::Instr2cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr2cfg2_set(self) -> crate::common::Reg<regs::Instr2cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr3cfg0_set(self) -> crate::common::Reg<regs::Instr3cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr3cfg1_set(self) -> crate::common::Reg<regs::Instr3cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr3cfg2_set(self) -> crate::common::Reg<regs::Instr3cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr4cfg0_set(self) -> crate::common::Reg<regs::Instr4cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr4cfg1_set(self) -> crate::common::Reg<regs::Instr4cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr4cfg2_set(self) -> crate::common::Reg<regs::Instr4cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr5cfg0_set(self) -> crate::common::Reg<regs::Instr5cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr5cfg1_set(self) -> crate::common::Reg<regs::Instr5cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr5cfg2_set(self) -> crate::common::Reg<regs::Instr5cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1158usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr6cfg0_set(self) -> crate::common::Reg<regs::Instr6cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr6cfg1_set(self) -> crate::common::Reg<regs::Instr6cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr6cfg2_set(self) -> crate::common::Reg<regs::Instr6cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr7cfg0_set(self) -> crate::common::Reg<regs::Instr7cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr7cfg1_set(self) -> crate::common::Reg<regs::Instr7cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn instr7cfg2_set(self) -> crate::common::Reg<regs::Instr7cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "Command Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1174usize) as _) }
    }
    #[doc = "Debug Control Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn debugen_set(self) -> crate::common::Reg<regs::Debugen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn debugstepcnt_set(self) -> crate::common::Reg<regs::Debugstepcnt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1204usize) as _) }
    }
    #[doc = "Block Enable Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn en_clr(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Software Reset Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn swrst_clr(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cfg_clr(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "Program State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn programstate_clr(self) -> crate::common::Reg<regs::Programstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array0indexstate_clr(self) -> crate::common::Reg<regs::Array0indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array1indexstate_clr(self) -> crate::common::Reg<regs::Array1indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array2indexstate_clr(self) -> crate::common::Reg<regs::Array2indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array3indexstate_clr(self) -> crate::common::Reg<regs::Array3indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array4indexstate_clr(self) -> crate::common::Reg<regs::Array4indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop0state_clr(self) -> crate::common::Reg<regs::Loop0state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop1state_clr(self) -> crate::common::Reg<regs::Loop1state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop2state_clr(self) -> crate::common::Reg<regs::Loop2state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x204cusize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop3state_clr(self) -> crate::common::Reg<regs::Loop3state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop4state_clr(self) -> crate::common::Reg<regs::Loop4state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop5state_clr(self) -> crate::common::Reg<regs::Loop5state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop6state_clr(self) -> crate::common::Reg<regs::Loop6state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x205cusize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop7state_clr(self) -> crate::common::Reg<regs::Loop7state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu0regstate_clr(self) -> crate::common::Reg<regs::Alu0regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu1regstate_clr(self) -> crate::common::Reg<regs::Alu1regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu2regstate_clr(self) -> crate::common::Reg<regs::Alu2regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x206cusize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu3regstate_clr(self) -> crate::common::Reg<regs::Alu3regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu4regstate_clr(self) -> crate::common::Reg<regs::Alu4regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu5regstate_clr(self) -> crate::common::Reg<regs::Alu5regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2078usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu6regstate_clr(self) -> crate::common::Reg<regs::Alu6regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x207cusize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn alu7regstate_clr(self) -> crate::common::Reg<regs::Alu7regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array0addrcfg_clr(self) -> crate::common::Reg<regs::Array0addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2084usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array0dim0cfg_clr(self) -> crate::common::Reg<regs::Array0dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2088usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array0dim1cfg_clr(self) -> crate::common::Reg<regs::Array0dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x208cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array0dim2cfg_clr(self) -> crate::common::Reg<regs::Array0dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array1addrcfg_clr(self) -> crate::common::Reg<regs::Array1addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2094usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array1dim0cfg_clr(self) -> crate::common::Reg<regs::Array1dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2098usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array1dim1cfg_clr(self) -> crate::common::Reg<regs::Array1dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x209cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array1dim2cfg_clr(self) -> crate::common::Reg<regs::Array1dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array2addrcfg_clr(self) -> crate::common::Reg<regs::Array2addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array2dim0cfg_clr(self) -> crate::common::Reg<regs::Array2dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array2dim1cfg_clr(self) -> crate::common::Reg<regs::Array2dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20acusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array2dim2cfg_clr(self) -> crate::common::Reg<regs::Array2dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array3addrcfg_clr(self) -> crate::common::Reg<regs::Array3addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array3dim0cfg_clr(self) -> crate::common::Reg<regs::Array3dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array3dim1cfg_clr(self) -> crate::common::Reg<regs::Array3dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20bcusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array3dim2cfg_clr(self) -> crate::common::Reg<regs::Array3dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array4addrcfg_clr(self) -> crate::common::Reg<regs::Array4addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array4dim0cfg_clr(self) -> crate::common::Reg<regs::Array4dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array4dim1cfg_clr(self) -> crate::common::Reg<regs::Array4dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20ccusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn array4dim2cfg_clr(self) -> crate::common::Reg<regs::Array4dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop0cfg_clr(self) -> crate::common::Reg<regs::Loop0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop0rst_clr(self) -> crate::common::Reg<regs::Loop0rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop1cfg_clr(self) -> crate::common::Reg<regs::Loop1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20dcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop1rst_clr(self) -> crate::common::Reg<regs::Loop1rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20e0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop2cfg_clr(self) -> crate::common::Reg<regs::Loop2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20e4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop2rst_clr(self) -> crate::common::Reg<regs::Loop2rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20e8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop3cfg_clr(self) -> crate::common::Reg<regs::Loop3cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20ecusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop3rst_clr(self) -> crate::common::Reg<regs::Loop3rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop4cfg_clr(self) -> crate::common::Reg<regs::Loop4cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop4rst_clr(self) -> crate::common::Reg<regs::Loop4rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20f8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop5cfg_clr(self) -> crate::common::Reg<regs::Loop5cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20fcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop5rst_clr(self) -> crate::common::Reg<regs::Loop5rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop6cfg_clr(self) -> crate::common::Reg<regs::Loop6cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2104usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop6rst_clr(self) -> crate::common::Reg<regs::Loop6rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2108usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop7cfg_clr(self) -> crate::common::Reg<regs::Loop7cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x210cusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn loop7rst_clr(self) -> crate::common::Reg<regs::Loop7rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2110usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr0cfg0_clr(self) -> crate::common::Reg<regs::Instr0cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2114usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr0cfg1_clr(self) -> crate::common::Reg<regs::Instr0cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2118usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr0cfg2_clr(self) -> crate::common::Reg<regs::Instr0cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x211cusize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr1cfg0_clr(self) -> crate::common::Reg<regs::Instr1cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr1cfg1_clr(self) -> crate::common::Reg<regs::Instr1cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr1cfg2_clr(self) -> crate::common::Reg<regs::Instr1cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr2cfg0_clr(self) -> crate::common::Reg<regs::Instr2cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x212cusize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr2cfg1_clr(self) -> crate::common::Reg<regs::Instr2cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2130usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr2cfg2_clr(self) -> crate::common::Reg<regs::Instr2cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2134usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr3cfg0_clr(self) -> crate::common::Reg<regs::Instr3cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2138usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr3cfg1_clr(self) -> crate::common::Reg<regs::Instr3cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x213cusize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr3cfg2_clr(self) -> crate::common::Reg<regs::Instr3cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr4cfg0_clr(self) -> crate::common::Reg<regs::Instr4cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2144usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr4cfg1_clr(self) -> crate::common::Reg<regs::Instr4cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2148usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr4cfg2_clr(self) -> crate::common::Reg<regs::Instr4cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x214cusize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr5cfg0_clr(self) -> crate::common::Reg<regs::Instr5cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2150usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr5cfg1_clr(self) -> crate::common::Reg<regs::Instr5cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2154usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr5cfg2_clr(self) -> crate::common::Reg<regs::Instr5cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2158usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr6cfg0_clr(self) -> crate::common::Reg<regs::Instr6cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x215cusize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr6cfg1_clr(self) -> crate::common::Reg<regs::Instr6cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr6cfg2_clr(self) -> crate::common::Reg<regs::Instr6cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2164usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr7cfg0_clr(self) -> crate::common::Reg<regs::Instr7cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2168usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr7cfg1_clr(self) -> crate::common::Reg<regs::Instr7cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x216cusize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn instr7cfg2_clr(self) -> crate::common::Reg<regs::Instr7cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    #[doc = "Command Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2174usize) as _) }
    }
    #[doc = "Debug Control Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn debugen_clr(self) -> crate::common::Reg<regs::Debugen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn debugstepcnt_clr(self) -> crate::common::Reg<regs::Debugstepcnt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2204usize) as _) }
    }
    #[doc = "Block Enable Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn en_tgl(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "Software Reset Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn swrst_tgl(self) -> crate::common::Reg<regs::Swrst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cfg_tgl(self) -> crate::common::Reg<regs::Cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "Interrupt Flags. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "Program State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn programstate_tgl(self) -> crate::common::Reg<regs::Programstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array0indexstate_tgl(self) -> crate::common::Reg<regs::Array0indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array1indexstate_tgl(self) -> crate::common::Reg<regs::Array1indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array2indexstate_tgl(self) -> crate::common::Reg<regs::Array2indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array3indexstate_tgl(self) -> crate::common::Reg<regs::Array3indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "Array N Index State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array4indexstate_tgl(self) -> crate::common::Reg<regs::Array4indexstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop0state_tgl(self) -> crate::common::Reg<regs::Loop0state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop1state_tgl(self) -> crate::common::Reg<regs::Loop1state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop2state_tgl(self) -> crate::common::Reg<regs::Loop2state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop3state_tgl(self) -> crate::common::Reg<regs::Loop3state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop4state_tgl(self) -> crate::common::Reg<regs::Loop4state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop5state_tgl(self) -> crate::common::Reg<regs::Loop5state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop6state_tgl(self) -> crate::common::Reg<regs::Loop6state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x305cusize) as _) }
    }
    #[doc = "Loop N State Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop7state_tgl(self) -> crate::common::Reg<regs::Loop7state, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu0regstate_tgl(self) -> crate::common::Reg<regs::Alu0regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu1regstate_tgl(self) -> crate::common::Reg<regs::Alu1regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu2regstate_tgl(self) -> crate::common::Reg<regs::Alu2regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu3regstate_tgl(self) -> crate::common::Reg<regs::Alu3regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu4regstate_tgl(self) -> crate::common::Reg<regs::Alu4regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu5regstate_tgl(self) -> crate::common::Reg<regs::Alu5regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3078usize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu6regstate_tgl(self) -> crate::common::Reg<regs::Alu6regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x307cusize) as _) }
    }
    #[doc = "ALU Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn alu7regstate_tgl(self) -> crate::common::Reg<regs::Alu7regstate, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array0addrcfg_tgl(self) -> crate::common::Reg<regs::Array0addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3084usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array0dim0cfg_tgl(self) -> crate::common::Reg<regs::Array0dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3088usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array0dim1cfg_tgl(self) -> crate::common::Reg<regs::Array0dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x308cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array0dim2cfg_tgl(self) -> crate::common::Reg<regs::Array0dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3090usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array1addrcfg_tgl(self) -> crate::common::Reg<regs::Array1addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3094usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array1dim0cfg_tgl(self) -> crate::common::Reg<regs::Array1dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3098usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array1dim1cfg_tgl(self) -> crate::common::Reg<regs::Array1dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x309cusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array1dim2cfg_tgl(self) -> crate::common::Reg<regs::Array1dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array2addrcfg_tgl(self) -> crate::common::Reg<regs::Array2addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array2dim0cfg_tgl(self) -> crate::common::Reg<regs::Array2dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array2dim1cfg_tgl(self) -> crate::common::Reg<regs::Array2dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30acusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array2dim2cfg_tgl(self) -> crate::common::Reg<regs::Array2dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array3addrcfg_tgl(self) -> crate::common::Reg<regs::Array3addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array3dim0cfg_tgl(self) -> crate::common::Reg<regs::Array3dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array3dim1cfg_tgl(self) -> crate::common::Reg<regs::Array3dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30bcusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array3dim2cfg_tgl(self) -> crate::common::Reg<regs::Array3dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c0usize) as _) }
    }
    #[doc = "Array N Base Address Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array4addrcfg_tgl(self) -> crate::common::Reg<regs::Array4addrcfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c4usize) as _) }
    }
    #[doc = "Array N Dimenion 0 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array4dim0cfg_tgl(self) -> crate::common::Reg<regs::Array4dim0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c8usize) as _) }
    }
    #[doc = "Array N Dimenion 1 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array4dim1cfg_tgl(self) -> crate::common::Reg<regs::Array4dim1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30ccusize) as _) }
    }
    #[doc = "Array N Dimenion 2 Configuration. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn array4dim2cfg_tgl(self) -> crate::common::Reg<regs::Array4dim2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop0cfg_tgl(self) -> crate::common::Reg<regs::Loop0cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop0rst_tgl(self) -> crate::common::Reg<regs::Loop0rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop1cfg_tgl(self) -> crate::common::Reg<regs::Loop1cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30dcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop1rst_tgl(self) -> crate::common::Reg<regs::Loop1rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30e0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop2cfg_tgl(self) -> crate::common::Reg<regs::Loop2cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30e4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop2rst_tgl(self) -> crate::common::Reg<regs::Loop2rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30e8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop3cfg_tgl(self) -> crate::common::Reg<regs::Loop3cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30ecusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop3rst_tgl(self) -> crate::common::Reg<regs::Loop3rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f0usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop4cfg_tgl(self) -> crate::common::Reg<regs::Loop4cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f4usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop4rst_tgl(self) -> crate::common::Reg<regs::Loop4rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30f8usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop5cfg_tgl(self) -> crate::common::Reg<regs::Loop5cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30fcusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop5rst_tgl(self) -> crate::common::Reg<regs::Loop5rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3100usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop6cfg_tgl(self) -> crate::common::Reg<regs::Loop6cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3104usize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop6rst_tgl(self) -> crate::common::Reg<regs::Loop6rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3108usize) as _) }
    }
    #[doc = "Loop N Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop7cfg_tgl(self) -> crate::common::Reg<regs::Loop7cfg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x310cusize) as _) }
    }
    #[doc = "Loop N Reset Configuration Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn loop7rst_tgl(self) -> crate::common::Reg<regs::Loop7rst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3110usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr0cfg0_tgl(self) -> crate::common::Reg<regs::Instr0cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3114usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr0cfg1_tgl(self) -> crate::common::Reg<regs::Instr0cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3118usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr0cfg2_tgl(self) -> crate::common::Reg<regs::Instr0cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x311cusize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr1cfg0_tgl(self) -> crate::common::Reg<regs::Instr1cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr1cfg1_tgl(self) -> crate::common::Reg<regs::Instr1cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3124usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr1cfg2_tgl(self) -> crate::common::Reg<regs::Instr1cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr2cfg0_tgl(self) -> crate::common::Reg<regs::Instr2cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x312cusize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr2cfg1_tgl(self) -> crate::common::Reg<regs::Instr2cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3130usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr2cfg2_tgl(self) -> crate::common::Reg<regs::Instr2cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3134usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr3cfg0_tgl(self) -> crate::common::Reg<regs::Instr3cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3138usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr3cfg1_tgl(self) -> crate::common::Reg<regs::Instr3cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x313cusize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr3cfg2_tgl(self) -> crate::common::Reg<regs::Instr3cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3140usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr4cfg0_tgl(self) -> crate::common::Reg<regs::Instr4cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3144usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr4cfg1_tgl(self) -> crate::common::Reg<regs::Instr4cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3148usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr4cfg2_tgl(self) -> crate::common::Reg<regs::Instr4cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x314cusize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr5cfg0_tgl(self) -> crate::common::Reg<regs::Instr5cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3150usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr5cfg1_tgl(self) -> crate::common::Reg<regs::Instr5cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3154usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr5cfg2_tgl(self) -> crate::common::Reg<regs::Instr5cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3158usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr6cfg0_tgl(self) -> crate::common::Reg<regs::Instr6cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x315cusize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr6cfg1_tgl(self) -> crate::common::Reg<regs::Instr6cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3160usize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr6cfg2_tgl(self) -> crate::common::Reg<regs::Instr6cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3164usize) as _) }
    }
    #[doc = "Instruction N Word 0. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr7cfg0_tgl(self) -> crate::common::Reg<regs::Instr7cfg0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3168usize) as _) }
    }
    #[doc = "Instruction N word 1. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr7cfg1_tgl(self) -> crate::common::Reg<regs::Instr7cfg1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x316cusize) as _) }
    }
    #[doc = "Instruction N word 2. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn instr7cfg2_tgl(self) -> crate::common::Reg<regs::Instr7cfg2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3170usize) as _) }
    }
    #[doc = "Command Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmd_tgl(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3174usize) as _) }
    }
    #[doc = "Debug Control Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn debugen_tgl(self) -> crate::common::Reg<regs::Debugen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn debugstepcnt_tgl(self) -> crate::common::Reg<regs::Debugstepcnt, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3204usize) as _) }
    }
}
pub mod regs {
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu0regstate(pub u32);
    impl Alu0regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu0regstate {
        #[inline(always)]
        fn default() -> Alu0regstate {
            Alu0regstate(0)
        }
    }
    impl core::fmt::Debug for Alu0regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu0regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu0regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu0regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu1regstate(pub u32);
    impl Alu1regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu1regstate {
        #[inline(always)]
        fn default() -> Alu1regstate {
            Alu1regstate(0)
        }
    }
    impl core::fmt::Debug for Alu1regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu1regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu1regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu1regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu2regstate(pub u32);
    impl Alu2regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu2regstate {
        #[inline(always)]
        fn default() -> Alu2regstate {
            Alu2regstate(0)
        }
    }
    impl core::fmt::Debug for Alu2regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu2regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu2regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu2regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu3regstate(pub u32);
    impl Alu3regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu3regstate {
        #[inline(always)]
        fn default() -> Alu3regstate {
            Alu3regstate(0)
        }
    }
    impl core::fmt::Debug for Alu3regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu3regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu3regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu3regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu4regstate(pub u32);
    impl Alu4regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu4regstate {
        #[inline(always)]
        fn default() -> Alu4regstate {
            Alu4regstate(0)
        }
    }
    impl core::fmt::Debug for Alu4regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu4regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu4regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu4regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu5regstate(pub u32);
    impl Alu5regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu5regstate {
        #[inline(always)]
        fn default() -> Alu5regstate {
            Alu5regstate(0)
        }
    }
    impl core::fmt::Debug for Alu5regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu5regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu5regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu5regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu6regstate(pub u32);
    impl Alu6regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu6regstate {
        #[inline(always)]
        fn default() -> Alu6regstate {
            Alu6regstate(0)
        }
    }
    impl core::fmt::Debug for Alu6regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu6regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu6regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu6regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "ALU Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alu7regstate(pub u32);
    impl Alu7regstate {
        #[doc = "Float Real Value."]
        #[must_use]
        #[inline(always)]
        pub const fn freal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Real Value."]
        #[inline(always)]
        pub const fn set_freal(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Float Imaginary Value."]
        #[must_use]
        #[inline(always)]
        pub const fn fimag(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Float Imaginary Value."]
        #[inline(always)]
        pub const fn set_fimag(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Alu7regstate {
        #[inline(always)]
        fn default() -> Alu7regstate {
            Alu7regstate(0)
        }
    }
    impl core::fmt::Debug for Alu7regstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alu7regstate")
                .field("freal", &self.freal())
                .field("fimag", &self.fimag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alu7regstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Alu7regstate {{ freal: {=u16:?}, fimag: {=u16:?} }}",
                self.freal(),
                self.fimag()
            )
        }
    }
    #[doc = "Array N Base Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array0addrcfg(pub u32);
    impl Array0addrcfg {
        #[doc = "Array Base Address."]
        #[must_use]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Array Base Address."]
        #[inline(always)]
        pub const fn set_base(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Array0addrcfg {
        #[inline(always)]
        fn default() -> Array0addrcfg {
            Array0addrcfg(0)
        }
    }
    impl core::fmt::Debug for Array0addrcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array0addrcfg").field("base", &self.base()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array0addrcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Array0addrcfg {{ base: {=u32:?} }}", self.base())
        }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array0dim0cfg(pub u32);
    impl Array0dim0cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Element Type."]
        #[must_use]
        #[inline(always)]
        pub const fn basetype(&self) -> super::vals::Array0dim0cfgBasetype {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Array0dim0cfgBasetype::from_bits(val as u8)
        }
        #[doc = "Element Type."]
        #[inline(always)]
        pub const fn set_basetype(&mut self, val: super::vals::Array0dim0cfgBasetype) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Complex Data Type."]
        #[must_use]
        #[inline(always)]
        pub const fn complex(&self) -> super::vals::Array0dim0cfgComplex {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Array0dim0cfgComplex::from_bits(val as u8)
        }
        #[doc = "Complex Data Type."]
        #[inline(always)]
        pub const fn set_complex(&mut self, val: super::vals::Array0dim0cfgComplex) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array0dim0cfg {
        #[inline(always)]
        fn default() -> Array0dim0cfg {
            Array0dim0cfg(0)
        }
    }
    impl core::fmt::Debug for Array0dim0cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array0dim0cfg")
                .field("size", &self.size())
                .field("basetype", &self.basetype())
                .field("complex", &self.complex())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array0dim0cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array0dim0cfg {{ size: {=u16:?}, basetype: {:?}, complex: {:?}, stride: {=u16:?} }}",
                self.size(),
                self.basetype(),
                self.complex(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array0dim1cfg(pub u32);
    impl Array0dim1cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array0dim1cfg {
        #[inline(always)]
        fn default() -> Array0dim1cfg {
            Array0dim1cfg(0)
        }
    }
    impl core::fmt::Debug for Array0dim1cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array0dim1cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array0dim1cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array0dim1cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array0dim2cfg(pub u32);
    impl Array0dim2cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array0dim2cfg {
        #[inline(always)]
        fn default() -> Array0dim2cfg {
            Array0dim2cfg(0)
        }
    }
    impl core::fmt::Debug for Array0dim2cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array0dim2cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array0dim2cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array0dim2cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Index State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array0indexstate(pub u32);
    impl Array0indexstate {
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim0index(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim0index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim1index(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim1index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim2index(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim2index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
    }
    impl Default for Array0indexstate {
        #[inline(always)]
        fn default() -> Array0indexstate {
            Array0indexstate(0)
        }
    }
    impl core::fmt::Debug for Array0indexstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array0indexstate")
                .field("dim0index", &self.dim0index())
                .field("dim1index", &self.dim1index())
                .field("dim2index", &self.dim2index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array0indexstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array0indexstate {{ dim0index: {=u16:?}, dim1index: {=u16:?}, dim2index: {=u16:?} }}",
                self.dim0index(),
                self.dim1index(),
                self.dim2index()
            )
        }
    }
    #[doc = "Array N Base Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array1addrcfg(pub u32);
    impl Array1addrcfg {
        #[doc = "Array Base Address."]
        #[must_use]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Array Base Address."]
        #[inline(always)]
        pub const fn set_base(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Array1addrcfg {
        #[inline(always)]
        fn default() -> Array1addrcfg {
            Array1addrcfg(0)
        }
    }
    impl core::fmt::Debug for Array1addrcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array1addrcfg").field("base", &self.base()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array1addrcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Array1addrcfg {{ base: {=u32:?} }}", self.base())
        }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array1dim0cfg(pub u32);
    impl Array1dim0cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Element Type."]
        #[must_use]
        #[inline(always)]
        pub const fn basetype(&self) -> super::vals::Array1dim0cfgBasetype {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Array1dim0cfgBasetype::from_bits(val as u8)
        }
        #[doc = "Element Type."]
        #[inline(always)]
        pub const fn set_basetype(&mut self, val: super::vals::Array1dim0cfgBasetype) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Complex Data Type."]
        #[must_use]
        #[inline(always)]
        pub const fn complex(&self) -> super::vals::Array1dim0cfgComplex {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Array1dim0cfgComplex::from_bits(val as u8)
        }
        #[doc = "Complex Data Type."]
        #[inline(always)]
        pub const fn set_complex(&mut self, val: super::vals::Array1dim0cfgComplex) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array1dim0cfg {
        #[inline(always)]
        fn default() -> Array1dim0cfg {
            Array1dim0cfg(0)
        }
    }
    impl core::fmt::Debug for Array1dim0cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array1dim0cfg")
                .field("size", &self.size())
                .field("basetype", &self.basetype())
                .field("complex", &self.complex())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array1dim0cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array1dim0cfg {{ size: {=u16:?}, basetype: {:?}, complex: {:?}, stride: {=u16:?} }}",
                self.size(),
                self.basetype(),
                self.complex(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array1dim1cfg(pub u32);
    impl Array1dim1cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array1dim1cfg {
        #[inline(always)]
        fn default() -> Array1dim1cfg {
            Array1dim1cfg(0)
        }
    }
    impl core::fmt::Debug for Array1dim1cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array1dim1cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array1dim1cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array1dim1cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array1dim2cfg(pub u32);
    impl Array1dim2cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array1dim2cfg {
        #[inline(always)]
        fn default() -> Array1dim2cfg {
            Array1dim2cfg(0)
        }
    }
    impl core::fmt::Debug for Array1dim2cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array1dim2cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array1dim2cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array1dim2cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Index State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array1indexstate(pub u32);
    impl Array1indexstate {
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim0index(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim0index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim1index(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim1index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim2index(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim2index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
    }
    impl Default for Array1indexstate {
        #[inline(always)]
        fn default() -> Array1indexstate {
            Array1indexstate(0)
        }
    }
    impl core::fmt::Debug for Array1indexstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array1indexstate")
                .field("dim0index", &self.dim0index())
                .field("dim1index", &self.dim1index())
                .field("dim2index", &self.dim2index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array1indexstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array1indexstate {{ dim0index: {=u16:?}, dim1index: {=u16:?}, dim2index: {=u16:?} }}",
                self.dim0index(),
                self.dim1index(),
                self.dim2index()
            )
        }
    }
    #[doc = "Array N Base Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array2addrcfg(pub u32);
    impl Array2addrcfg {
        #[doc = "Array Base Address."]
        #[must_use]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Array Base Address."]
        #[inline(always)]
        pub const fn set_base(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Array2addrcfg {
        #[inline(always)]
        fn default() -> Array2addrcfg {
            Array2addrcfg(0)
        }
    }
    impl core::fmt::Debug for Array2addrcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array2addrcfg").field("base", &self.base()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array2addrcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Array2addrcfg {{ base: {=u32:?} }}", self.base())
        }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array2dim0cfg(pub u32);
    impl Array2dim0cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Element Type."]
        #[must_use]
        #[inline(always)]
        pub const fn basetype(&self) -> super::vals::Array2dim0cfgBasetype {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Array2dim0cfgBasetype::from_bits(val as u8)
        }
        #[doc = "Element Type."]
        #[inline(always)]
        pub const fn set_basetype(&mut self, val: super::vals::Array2dim0cfgBasetype) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Complex Data Type."]
        #[must_use]
        #[inline(always)]
        pub const fn complex(&self) -> super::vals::Array2dim0cfgComplex {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Array2dim0cfgComplex::from_bits(val as u8)
        }
        #[doc = "Complex Data Type."]
        #[inline(always)]
        pub const fn set_complex(&mut self, val: super::vals::Array2dim0cfgComplex) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array2dim0cfg {
        #[inline(always)]
        fn default() -> Array2dim0cfg {
            Array2dim0cfg(0)
        }
    }
    impl core::fmt::Debug for Array2dim0cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array2dim0cfg")
                .field("size", &self.size())
                .field("basetype", &self.basetype())
                .field("complex", &self.complex())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array2dim0cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array2dim0cfg {{ size: {=u16:?}, basetype: {:?}, complex: {:?}, stride: {=u16:?} }}",
                self.size(),
                self.basetype(),
                self.complex(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array2dim1cfg(pub u32);
    impl Array2dim1cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array2dim1cfg {
        #[inline(always)]
        fn default() -> Array2dim1cfg {
            Array2dim1cfg(0)
        }
    }
    impl core::fmt::Debug for Array2dim1cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array2dim1cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array2dim1cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array2dim1cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array2dim2cfg(pub u32);
    impl Array2dim2cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array2dim2cfg {
        #[inline(always)]
        fn default() -> Array2dim2cfg {
            Array2dim2cfg(0)
        }
    }
    impl core::fmt::Debug for Array2dim2cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array2dim2cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array2dim2cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array2dim2cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Index State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array2indexstate(pub u32);
    impl Array2indexstate {
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim0index(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim0index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim1index(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim1index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim2index(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim2index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
    }
    impl Default for Array2indexstate {
        #[inline(always)]
        fn default() -> Array2indexstate {
            Array2indexstate(0)
        }
    }
    impl core::fmt::Debug for Array2indexstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array2indexstate")
                .field("dim0index", &self.dim0index())
                .field("dim1index", &self.dim1index())
                .field("dim2index", &self.dim2index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array2indexstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array2indexstate {{ dim0index: {=u16:?}, dim1index: {=u16:?}, dim2index: {=u16:?} }}",
                self.dim0index(),
                self.dim1index(),
                self.dim2index()
            )
        }
    }
    #[doc = "Array N Base Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array3addrcfg(pub u32);
    impl Array3addrcfg {
        #[doc = "Array Base Address."]
        #[must_use]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Array Base Address."]
        #[inline(always)]
        pub const fn set_base(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Array3addrcfg {
        #[inline(always)]
        fn default() -> Array3addrcfg {
            Array3addrcfg(0)
        }
    }
    impl core::fmt::Debug for Array3addrcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array3addrcfg").field("base", &self.base()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array3addrcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Array3addrcfg {{ base: {=u32:?} }}", self.base())
        }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array3dim0cfg(pub u32);
    impl Array3dim0cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Element Type."]
        #[must_use]
        #[inline(always)]
        pub const fn basetype(&self) -> super::vals::Array3dim0cfgBasetype {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Array3dim0cfgBasetype::from_bits(val as u8)
        }
        #[doc = "Element Type."]
        #[inline(always)]
        pub const fn set_basetype(&mut self, val: super::vals::Array3dim0cfgBasetype) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Complex Data Type."]
        #[must_use]
        #[inline(always)]
        pub const fn complex(&self) -> super::vals::Array3dim0cfgComplex {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Array3dim0cfgComplex::from_bits(val as u8)
        }
        #[doc = "Complex Data Type."]
        #[inline(always)]
        pub const fn set_complex(&mut self, val: super::vals::Array3dim0cfgComplex) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array3dim0cfg {
        #[inline(always)]
        fn default() -> Array3dim0cfg {
            Array3dim0cfg(0)
        }
    }
    impl core::fmt::Debug for Array3dim0cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array3dim0cfg")
                .field("size", &self.size())
                .field("basetype", &self.basetype())
                .field("complex", &self.complex())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array3dim0cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array3dim0cfg {{ size: {=u16:?}, basetype: {:?}, complex: {:?}, stride: {=u16:?} }}",
                self.size(),
                self.basetype(),
                self.complex(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array3dim1cfg(pub u32);
    impl Array3dim1cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array3dim1cfg {
        #[inline(always)]
        fn default() -> Array3dim1cfg {
            Array3dim1cfg(0)
        }
    }
    impl core::fmt::Debug for Array3dim1cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array3dim1cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array3dim1cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array3dim1cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array3dim2cfg(pub u32);
    impl Array3dim2cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array3dim2cfg {
        #[inline(always)]
        fn default() -> Array3dim2cfg {
            Array3dim2cfg(0)
        }
    }
    impl core::fmt::Debug for Array3dim2cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array3dim2cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array3dim2cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array3dim2cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Index State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array3indexstate(pub u32);
    impl Array3indexstate {
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim0index(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim0index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim1index(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim1index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim2index(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim2index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
    }
    impl Default for Array3indexstate {
        #[inline(always)]
        fn default() -> Array3indexstate {
            Array3indexstate(0)
        }
    }
    impl core::fmt::Debug for Array3indexstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array3indexstate")
                .field("dim0index", &self.dim0index())
                .field("dim1index", &self.dim1index())
                .field("dim2index", &self.dim2index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array3indexstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array3indexstate {{ dim0index: {=u16:?}, dim1index: {=u16:?}, dim2index: {=u16:?} }}",
                self.dim0index(),
                self.dim1index(),
                self.dim2index()
            )
        }
    }
    #[doc = "Array N Base Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array4addrcfg(pub u32);
    impl Array4addrcfg {
        #[doc = "Array Base Address."]
        #[must_use]
        #[inline(always)]
        pub const fn base(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Array Base Address."]
        #[inline(always)]
        pub const fn set_base(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Array4addrcfg {
        #[inline(always)]
        fn default() -> Array4addrcfg {
            Array4addrcfg(0)
        }
    }
    impl core::fmt::Debug for Array4addrcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array4addrcfg").field("base", &self.base()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array4addrcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Array4addrcfg {{ base: {=u32:?} }}", self.base())
        }
    }
    #[doc = "Array N Dimenion 0 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array4dim0cfg(pub u32);
    impl Array4dim0cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Element Type."]
        #[must_use]
        #[inline(always)]
        pub const fn basetype(&self) -> super::vals::Array4dim0cfgBasetype {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Array4dim0cfgBasetype::from_bits(val as u8)
        }
        #[doc = "Element Type."]
        #[inline(always)]
        pub const fn set_basetype(&mut self, val: super::vals::Array4dim0cfgBasetype) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Complex Data Type."]
        #[must_use]
        #[inline(always)]
        pub const fn complex(&self) -> super::vals::Array4dim0cfgComplex {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Array4dim0cfgComplex::from_bits(val as u8)
        }
        #[doc = "Complex Data Type."]
        #[inline(always)]
        pub const fn set_complex(&mut self, val: super::vals::Array4dim0cfgComplex) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array4dim0cfg {
        #[inline(always)]
        fn default() -> Array4dim0cfg {
            Array4dim0cfg(0)
        }
    }
    impl core::fmt::Debug for Array4dim0cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array4dim0cfg")
                .field("size", &self.size())
                .field("basetype", &self.basetype())
                .field("complex", &self.complex())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array4dim0cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array4dim0cfg {{ size: {=u16:?}, basetype: {:?}, complex: {:?}, stride: {=u16:?} }}",
                self.size(),
                self.basetype(),
                self.complex(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 1 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array4dim1cfg(pub u32);
    impl Array4dim1cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array4dim1cfg {
        #[inline(always)]
        fn default() -> Array4dim1cfg {
            Array4dim1cfg(0)
        }
    }
    impl core::fmt::Debug for Array4dim1cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array4dim1cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array4dim1cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array4dim1cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Dimenion 2 Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array4dim2cfg(pub u32);
    impl Array4dim2cfg {
        #[doc = "Array Dimension Size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Array Dimension Size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Dimension Stride Step."]
        #[must_use]
        #[inline(always)]
        pub const fn stride(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Dimension Stride Step."]
        #[inline(always)]
        pub const fn set_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Array4dim2cfg {
        #[inline(always)]
        fn default() -> Array4dim2cfg {
            Array4dim2cfg(0)
        }
    }
    impl core::fmt::Debug for Array4dim2cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array4dim2cfg")
                .field("size", &self.size())
                .field("stride", &self.stride())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array4dim2cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array4dim2cfg {{ size: {=u16:?}, stride: {=u16:?} }}",
                self.size(),
                self.stride()
            )
        }
    }
    #[doc = "Array N Index State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Array4indexstate(pub u32);
    impl Array4indexstate {
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim0index(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim0index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim1index(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim1index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Current Index."]
        #[must_use]
        #[inline(always)]
        pub const fn dim2index(&self) -> u16 {
            let val = (self.0 >> 20usize) & 0x03ff;
            val as u16
        }
        #[doc = "Current Index."]
        #[inline(always)]
        pub const fn set_dim2index(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
        }
    }
    impl Default for Array4indexstate {
        #[inline(always)]
        fn default() -> Array4indexstate {
            Array4indexstate(0)
        }
    }
    impl core::fmt::Debug for Array4indexstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Array4indexstate")
                .field("dim0index", &self.dim0index())
                .field("dim1index", &self.dim1index())
                .field("dim2index", &self.dim2index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Array4indexstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Array4indexstate {{ dim0index: {=u16:?}, dim1index: {=u16:?}, dim2index: {=u16:?} }}",
                self.dim0index(),
                self.dim1index(),
                self.dim2index()
            )
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Performance Counter Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn perfcnten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Performance Counter Enable."]
        #[inline(always)]
        pub const fn set_perfcnten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ALU Output Stream Compression Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn outcompressdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Output Stream Compression Disable."]
        #[inline(always)]
        pub const fn set_outcompressdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ALU Input Word Cache Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn incachedis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Input Word Cache Disable."]
        #[inline(always)]
        pub const fn set_incachedis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop Error Halt Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn looperrhaltdis(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Error Halt Disable."]
        #[inline(always)]
        pub const fn set_looperrhaltdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fence Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn infencedis(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Fence Disable."]
        #[inline(always)]
        pub const fn set_infencedis(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Performance Counter Select."]
        #[must_use]
        #[inline(always)]
        pub const fn perf0cntsel(&self) -> super::vals::Perf0cntsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Perf0cntsel::from_bits(val as u8)
        }
        #[doc = "Performance Counter Select."]
        #[inline(always)]
        pub const fn set_perf0cntsel(&mut self, val: super::vals::Perf0cntsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Performance Counter Select."]
        #[must_use]
        #[inline(always)]
        pub const fn perf1cntsel(&self) -> super::vals::Perf1cntsel {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::Perf1cntsel::from_bits(val as u8)
        }
        #[doc = "Performance Counter Select."]
        #[inline(always)]
        pub const fn set_perf1cntsel(&mut self, val: super::vals::Perf1cntsel) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
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
                .field("perfcnten", &self.perfcnten())
                .field("outcompressdis", &self.outcompressdis())
                .field("incachedis", &self.incachedis())
                .field("looperrhaltdis", &self.looperrhaltdis())
                .field("infencedis", &self.infencedis())
                .field("perf0cntsel", &self.perf0cntsel())
                .field("perf1cntsel", &self.perf1cntsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ perfcnten: {=bool:?}, outcompressdis: {=bool:?}, incachedis: {=bool:?}, looperrhaltdis: {=bool:?}, infencedis: {=u8:?}, perf0cntsel: {:?}, perf1cntsel: {:?} }}",
                self.perfcnten(),
                self.outcompressdis(),
                self.incachedis(),
                self.looperrhaltdis(),
                self.infencedis(),
                self.perf0cntsel(),
                self.perf1cntsel()
            )
        }
    }
    #[doc = "Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "Start Command."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start Command."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Halt Command."]
        #[must_use]
        #[inline(always)]
        pub const fn halt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Halt Command."]
        #[inline(always)]
        pub const fn set_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Step Command."]
        #[must_use]
        #[inline(always)]
        pub const fn step(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Step Command."]
        #[inline(always)]
        pub const fn set_step(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Initialization Command/Qualifier."]
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization Command/Qualifier."]
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("start", &self.start())
                .field("halt", &self.halt())
                .field("step", &self.step())
                .field("init", &self.init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmd {{ start: {=bool:?}, halt: {=bool:?}, step: {=bool:?}, init: {=bool:?} }}",
                self.start(),
                self.halt(),
                self.step(),
                self.init()
            )
        }
    }
    #[doc = "Debug Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Debugen(pub u32);
    impl Debugen {
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop0done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop0done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop1done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop1done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop2done(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop2done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop3done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop3done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop4done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop4done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop5done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop5done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop6done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop6done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptloop7done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on Loop Done."]
        #[inline(always)]
        pub const fn set_bkptloop7done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable Breakpoint on ALUNAN."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptalunan(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on ALUNAN."]
        #[inline(always)]
        pub const fn set_bkptalunan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Breakpoint on R0POSREAL."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptr0posreal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on R0POSREAL."]
        #[inline(always)]
        pub const fn set_bkptr0posreal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Breakpoint on ALUOF."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptaluof(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on ALUOF."]
        #[inline(always)]
        pub const fn set_bkptaluof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Breakpoint on ALUUF."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptaluuf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on ALUUF."]
        #[inline(always)]
        pub const fn set_bkptaluuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Breakpoint on STORECONVERTOF."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptstoreconvertof(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on STORECONVERTOF."]
        #[inline(always)]
        pub const fn set_bkptstoreconvertof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Breakpoint on STORECONVERTUF."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptstoreconvertuf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on STORECONVERTUF."]
        #[inline(always)]
        pub const fn set_bkptstoreconvertuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Enable Breakpoint on STORECONVERTINF."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptstoreconvertinf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on STORECONVERTINF."]
        #[inline(always)]
        pub const fn set_bkptstoreconvertinf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable Breakpoint on STORECONVERTNAN."]
        #[must_use]
        #[inline(always)]
        pub const fn bkptstoreconvertnan(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint on STORECONVERTNAN."]
        #[inline(always)]
        pub const fn set_bkptstoreconvertnan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Debug Step Count Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn debugstepcnten(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Debug Step Count Enable."]
        #[inline(always)]
        pub const fn set_debugstepcnten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Trigger Breakpoint when ALL conditions match."]
        #[must_use]
        #[inline(always)]
        pub const fn debugbkptallen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger Breakpoint when ALL conditions match."]
        #[inline(always)]
        pub const fn set_debugbkptallen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Enable Breakpoint when ANY conditions match."]
        #[must_use]
        #[inline(always)]
        pub const fn debugbkptanyen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Breakpoint when ANY conditions match."]
        #[inline(always)]
        pub const fn set_debugbkptanyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Debugen {
        #[inline(always)]
        fn default() -> Debugen {
            Debugen(0)
        }
    }
    impl core::fmt::Debug for Debugen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Debugen")
                .field("bkptloop0done", &self.bkptloop0done())
                .field("bkptloop1done", &self.bkptloop1done())
                .field("bkptloop2done", &self.bkptloop2done())
                .field("bkptloop3done", &self.bkptloop3done())
                .field("bkptloop4done", &self.bkptloop4done())
                .field("bkptloop5done", &self.bkptloop5done())
                .field("bkptloop6done", &self.bkptloop6done())
                .field("bkptloop7done", &self.bkptloop7done())
                .field("bkptalunan", &self.bkptalunan())
                .field("bkptr0posreal", &self.bkptr0posreal())
                .field("bkptaluof", &self.bkptaluof())
                .field("bkptaluuf", &self.bkptaluuf())
                .field("bkptstoreconvertof", &self.bkptstoreconvertof())
                .field("bkptstoreconvertuf", &self.bkptstoreconvertuf())
                .field("bkptstoreconvertinf", &self.bkptstoreconvertinf())
                .field("bkptstoreconvertnan", &self.bkptstoreconvertnan())
                .field("debugstepcnten", &self.debugstepcnten())
                .field("debugbkptallen", &self.debugbkptallen())
                .field("debugbkptanyen", &self.debugbkptanyen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Debugen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Debugen {{ bkptloop0done: {=bool:?}, bkptloop1done: {=bool:?}, bkptloop2done: {=bool:?}, bkptloop3done: {=bool:?}, bkptloop4done: {=bool:?}, bkptloop5done: {=bool:?}, bkptloop6done: {=bool:?}, bkptloop7done: {=bool:?}, bkptalunan: {=bool:?}, bkptr0posreal: {=bool:?}, bkptaluof: {=bool:?}, bkptaluuf: {=bool:?}, bkptstoreconvertof: {=bool:?}, bkptstoreconvertuf: {=bool:?}, bkptstoreconvertinf: {=bool:?}, bkptstoreconvertnan: {=bool:?}, debugstepcnten: {=bool:?}, debugbkptallen: {=bool:?}, debugbkptanyen: {=bool:?} }}",
                self.bkptloop0done(),
                self.bkptloop1done(),
                self.bkptloop2done(),
                self.bkptloop3done(),
                self.bkptloop4done(),
                self.bkptloop5done(),
                self.bkptloop6done(),
                self.bkptloop7done(),
                self.bkptalunan(),
                self.bkptr0posreal(),
                self.bkptaluof(),
                self.bkptaluuf(),
                self.bkptstoreconvertof(),
                self.bkptstoreconvertuf(),
                self.bkptstoreconvertinf(),
                self.bkptstoreconvertnan(),
                self.debugstepcnten(),
                self.debugbkptallen(),
                self.debugbkptanyen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Debugstepcnt(pub u32);
    impl Debugstepcnt {
        #[doc = "Debug Step Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn debugstepcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Debug Step Counter."]
        #[inline(always)]
        pub const fn set_debugstepcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Debugstepcnt {
        #[inline(always)]
        fn default() -> Debugstepcnt {
            Debugstepcnt(0)
        }
    }
    impl core::fmt::Debug for Debugstepcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Debugstepcnt")
                .field("debugstepcnt", &self.debugstepcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Debugstepcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Debugstepcnt {{ debugstepcnt: {=u32:?} }}", self.debugstepcnt())
        }
    }
    #[doc = "Block Enable Register."]
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
        #[doc = "Disablement Busy Status."]
        #[must_use]
        #[inline(always)]
        pub const fn disabling(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Disablement Busy Status."]
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
    #[doc = "Fault Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faultaddr(pub u32);
    impl Faultaddr {
        #[doc = "Bus Fault Address Register."]
        #[must_use]
        #[inline(always)]
        pub const fn faultaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bus Fault Address Register."]
        #[inline(always)]
        pub const fn set_faultaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Faultaddr {
        #[inline(always)]
        fn default() -> Faultaddr {
            Faultaddr(0)
        }
    }
    impl core::fmt::Debug for Faultaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Faultaddr")
                .field("faultaddr", &self.faultaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Faultaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Faultaddr {{ faultaddr: {=u32:?} }}", self.faultaddr())
        }
    }
    #[doc = "Fault Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faultstatus(pub u32);
    impl Faultstatus {
        #[doc = "PC when fault occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn faultpc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "PC when fault occurred."]
        #[inline(always)]
        pub const fn set_faultpc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Array access that generated a fault."]
        #[must_use]
        #[inline(always)]
        pub const fn faultarray(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Array access that generated a fault."]
        #[inline(always)]
        pub const fn set_faultarray(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Bus where fault occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn faultbus(&self) -> super::vals::Faultbus {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Faultbus::from_bits(val as u8)
        }
        #[doc = "Bus where fault occurred."]
        #[inline(always)]
        pub const fn set_faultbus(&mut self, val: super::vals::Faultbus) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Loop Fault Indicator."]
        #[must_use]
        #[inline(always)]
        pub const fn faultloop(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Loop Fault Indicator."]
        #[inline(always)]
        pub const fn set_faultloop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Faultstatus {
        #[inline(always)]
        fn default() -> Faultstatus {
            Faultstatus(0)
        }
    }
    impl core::fmt::Debug for Faultstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Faultstatus")
                .field("faultpc", &self.faultpc())
                .field("faultarray", &self.faultarray())
                .field("faultbus", &self.faultbus())
                .field("faultloop", &self.faultloop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Faultstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Faultstatus {{ faultpc: {=u8:?}, faultarray: {=u8:?}, faultbus: {:?}, faultloop: {=u8:?} }}",
                self.faultpc(),
                self.faultarray(),
                self.faultbus(),
                self.faultloop()
            )
        }
    }
    #[doc = "Interrupt Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "Program Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn progdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Program Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_progdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop0done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop1done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2done(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop2done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop3done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop4done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop5done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop6done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loop7done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Not-a-Number Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn alunan(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Not-a-Number Interrupt Enable."]
        #[inline(always)]
        pub const fn set_alunan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "R0 Non-Zero Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn r0posreal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "R0 Non-Zero Interrupt Enable."]
        #[inline(always)]
        pub const fn set_r0posreal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ALU Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aluof(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_aluof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ALU Underflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aluuf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Underflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_aluuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Store conversion Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertof(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Store conversion Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_storeconvertof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Store Conversion Underflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertuf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Store Conversion Underflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_storeconvertuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Store Conversion Infinity Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertinf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Store Conversion Infinity Interrupt Enable."]
        #[inline(always)]
        pub const fn set_storeconvertinf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Store Conversion NaN Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertnan(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Store Conversion NaN Interrupt Enable."]
        #[inline(always)]
        pub const fn set_storeconvertnan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Perf Counter 0 Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn perfcnt0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Perf Counter 0 Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_perfcnt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Perf Counter 1 Overflow Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn perfcnt1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Perf Counter 1 Overflow Interrupt Enable."]
        #[inline(always)]
        pub const fn set_perfcnt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Loop Fault Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loopfault(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Fault Interrupt Enable."]
        #[inline(always)]
        pub const fn set_loopfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus Error Fault Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buserrfault(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error Fault Interrupt Enable."]
        #[inline(always)]
        pub const fn set_buserrfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Bus Alignment Fault Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn busalignfault(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Alignment Fault Interrupt Enable."]
        #[inline(always)]
        pub const fn set_busalignfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ALU Input Fault Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn alufault(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Input Fault Interrupt Enable."]
        #[inline(always)]
        pub const fn set_alufault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Array Fault Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrayfault(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Array Fault Interrupt Enable."]
        #[inline(always)]
        pub const fn set_arrayfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("progdone", &self.progdone())
                .field("loop0done", &self.loop0done())
                .field("loop1done", &self.loop1done())
                .field("loop2done", &self.loop2done())
                .field("loop3done", &self.loop3done())
                .field("loop4done", &self.loop4done())
                .field("loop5done", &self.loop5done())
                .field("loop6done", &self.loop6done())
                .field("loop7done", &self.loop7done())
                .field("alunan", &self.alunan())
                .field("r0posreal", &self.r0posreal())
                .field("aluof", &self.aluof())
                .field("aluuf", &self.aluuf())
                .field("storeconvertof", &self.storeconvertof())
                .field("storeconvertuf", &self.storeconvertuf())
                .field("storeconvertinf", &self.storeconvertinf())
                .field("storeconvertnan", &self.storeconvertnan())
                .field("perfcnt0", &self.perfcnt0())
                .field("perfcnt1", &self.perfcnt1())
                .field("loopfault", &self.loopfault())
                .field("buserrfault", &self.buserrfault())
                .field("busalignfault", &self.busalignfault())
                .field("alufault", &self.alufault())
                .field("arrayfault", &self.arrayfault())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ progdone: {=bool:?}, loop0done: {=bool:?}, loop1done: {=bool:?}, loop2done: {=bool:?}, loop3done: {=bool:?}, loop4done: {=bool:?}, loop5done: {=bool:?}, loop6done: {=bool:?}, loop7done: {=bool:?}, alunan: {=bool:?}, r0posreal: {=bool:?}, aluof: {=bool:?}, aluuf: {=bool:?}, storeconvertof: {=bool:?}, storeconvertuf: {=bool:?}, storeconvertinf: {=bool:?}, storeconvertnan: {=bool:?}, perfcnt0: {=bool:?}, perfcnt1: {=bool:?}, loopfault: {=bool:?}, buserrfault: {=bool:?}, busalignfault: {=bool:?}, alufault: {=bool:?}, arrayfault: {=bool:?} }}",
                self.progdone(),
                self.loop0done(),
                self.loop1done(),
                self.loop2done(),
                self.loop3done(),
                self.loop4done(),
                self.loop5done(),
                self.loop6done(),
                self.loop7done(),
                self.alunan(),
                self.r0posreal(),
                self.aluof(),
                self.aluuf(),
                self.storeconvertof(),
                self.storeconvertuf(),
                self.storeconvertinf(),
                self.storeconvertnan(),
                self.perfcnt0(),
                self.perfcnt1(),
                self.loopfault(),
                self.buserrfault(),
                self.busalignfault(),
                self.alufault(),
                self.arrayfault()
            )
        }
    }
    #[doc = "Interrupt Flags."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "Program Done Interrupt Flags."]
        #[must_use]
        #[inline(always)]
        pub const fn progdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Program Done Interrupt Flags."]
        #[inline(always)]
        pub const fn set_progdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop0done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop1done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2done(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop2done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop3done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop4done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop5done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop6done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Done Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loop7done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Not-a-Number Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn alunan(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Not-a-Number Interrupt Flag."]
        #[inline(always)]
        pub const fn set_alunan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "R0 non-zero Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn r0posreal(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "R0 non-zero Interrupt Flag."]
        #[inline(always)]
        pub const fn set_r0posreal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ALU Overflow on result."]
        #[must_use]
        #[inline(always)]
        pub const fn aluof(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Overflow on result."]
        #[inline(always)]
        pub const fn set_aluof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ALU Underflow on result."]
        #[must_use]
        #[inline(always)]
        pub const fn aluuf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Underflow on result."]
        #[inline(always)]
        pub const fn set_aluuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Overflow during array store."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertof(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow during array store."]
        #[inline(always)]
        pub const fn set_storeconvertof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Underflow during array store conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertuf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow during array store conversion."]
        #[inline(always)]
        pub const fn set_storeconvertuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Infinity encountered during array store conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertinf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Infinity encountered during array store conversion."]
        #[inline(always)]
        pub const fn set_storeconvertinf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "NaN encountered during array store conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn storeconvertnan(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "NaN encountered during array store conversion."]
        #[inline(always)]
        pub const fn set_storeconvertnan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Run Count Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn perfcnt0(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Run Count Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_perfcnt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Stall Count Overflow Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn perfcnt1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Stall Count Overflow Interrupt Flag."]
        #[inline(always)]
        pub const fn set_perfcnt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Loop Fault Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn loopfault(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Fault Interrupt Flag."]
        #[inline(always)]
        pub const fn set_loopfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bus Error Fault Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn buserrfault(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Error Fault Interrupt Flag."]
        #[inline(always)]
        pub const fn set_buserrfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Bus Alignment Fault Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn busalignfault(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bus Alignment Fault Interrupt Flag."]
        #[inline(always)]
        pub const fn set_busalignfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "ALU Fault Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn alufault(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "ALU Fault Interrupt Flag."]
        #[inline(always)]
        pub const fn set_alufault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Array Fault Interrupt Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrayfault(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Array Fault Interrupt Flag."]
        #[inline(always)]
        pub const fn set_arrayfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("progdone", &self.progdone())
                .field("loop0done", &self.loop0done())
                .field("loop1done", &self.loop1done())
                .field("loop2done", &self.loop2done())
                .field("loop3done", &self.loop3done())
                .field("loop4done", &self.loop4done())
                .field("loop5done", &self.loop5done())
                .field("loop6done", &self.loop6done())
                .field("loop7done", &self.loop7done())
                .field("alunan", &self.alunan())
                .field("r0posreal", &self.r0posreal())
                .field("aluof", &self.aluof())
                .field("aluuf", &self.aluuf())
                .field("storeconvertof", &self.storeconvertof())
                .field("storeconvertuf", &self.storeconvertuf())
                .field("storeconvertinf", &self.storeconvertinf())
                .field("storeconvertnan", &self.storeconvertnan())
                .field("perfcnt0", &self.perfcnt0())
                .field("perfcnt1", &self.perfcnt1())
                .field("loopfault", &self.loopfault())
                .field("buserrfault", &self.buserrfault())
                .field("busalignfault", &self.busalignfault())
                .field("alufault", &self.alufault())
                .field("arrayfault", &self.arrayfault())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ progdone: {=bool:?}, loop0done: {=bool:?}, loop1done: {=bool:?}, loop2done: {=bool:?}, loop3done: {=bool:?}, loop4done: {=bool:?}, loop5done: {=bool:?}, loop6done: {=bool:?}, loop7done: {=bool:?}, alunan: {=bool:?}, r0posreal: {=bool:?}, aluof: {=bool:?}, aluuf: {=bool:?}, storeconvertof: {=bool:?}, storeconvertuf: {=bool:?}, storeconvertinf: {=bool:?}, storeconvertnan: {=bool:?}, perfcnt0: {=bool:?}, perfcnt1: {=bool:?}, loopfault: {=bool:?}, buserrfault: {=bool:?}, busalignfault: {=bool:?}, alufault: {=bool:?}, arrayfault: {=bool:?} }}",
                self.progdone(),
                self.loop0done(),
                self.loop1done(),
                self.loop2done(),
                self.loop3done(),
                self.loop4done(),
                self.loop5done(),
                self.loop6done(),
                self.loop7done(),
                self.alunan(),
                self.r0posreal(),
                self.aluof(),
                self.aluuf(),
                self.storeconvertof(),
                self.storeconvertuf(),
                self.storeconvertinf(),
                self.storeconvertnan(),
                self.perfcnt0(),
                self.perfcnt1(),
                self.loopfault(),
                self.buserrfault(),
                self.busalignfault(),
                self.alufault(),
                self.arrayfault()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr0cfg0(pub u32);
    impl Instr0cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr0cfg0 {
        #[inline(always)]
        fn default() -> Instr0cfg0 {
            Instr0cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr0cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr0cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr0cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr0cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr0cfg1(pub u32);
    impl Instr0cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr0cfg1 {
        #[inline(always)]
        fn default() -> Instr0cfg1 {
            Instr0cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr0cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr0cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr0cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr0cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr0cfg2(pub u32);
    impl Instr0cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr0cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr0cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr0cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr0cfg2 {
        #[inline(always)]
        fn default() -> Instr0cfg2 {
            Instr0cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr0cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr0cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr0cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr0cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr1cfg0(pub u32);
    impl Instr1cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr1cfg0 {
        #[inline(always)]
        fn default() -> Instr1cfg0 {
            Instr1cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr1cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr1cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr1cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr1cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr1cfg1(pub u32);
    impl Instr1cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr1cfg1 {
        #[inline(always)]
        fn default() -> Instr1cfg1 {
            Instr1cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr1cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr1cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr1cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr1cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr1cfg2(pub u32);
    impl Instr1cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr1cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr1cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr1cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr1cfg2 {
        #[inline(always)]
        fn default() -> Instr1cfg2 {
            Instr1cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr1cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr1cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr1cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr1cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr2cfg0(pub u32);
    impl Instr2cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr2cfg0 {
        #[inline(always)]
        fn default() -> Instr2cfg0 {
            Instr2cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr2cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr2cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr2cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr2cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr2cfg1(pub u32);
    impl Instr2cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr2cfg1 {
        #[inline(always)]
        fn default() -> Instr2cfg1 {
            Instr2cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr2cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr2cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr2cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr2cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr2cfg2(pub u32);
    impl Instr2cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr2cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr2cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr2cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr2cfg2 {
        #[inline(always)]
        fn default() -> Instr2cfg2 {
            Instr2cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr2cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr2cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr2cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr2cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr3cfg0(pub u32);
    impl Instr3cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr3cfg0 {
        #[inline(always)]
        fn default() -> Instr3cfg0 {
            Instr3cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr3cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr3cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr3cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr3cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr3cfg1(pub u32);
    impl Instr3cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr3cfg1 {
        #[inline(always)]
        fn default() -> Instr3cfg1 {
            Instr3cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr3cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr3cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr3cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr3cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr3cfg2(pub u32);
    impl Instr3cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr3cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr3cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr3cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr3cfg2 {
        #[inline(always)]
        fn default() -> Instr3cfg2 {
            Instr3cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr3cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr3cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr3cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr3cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr4cfg0(pub u32);
    impl Instr4cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr4cfg0 {
        #[inline(always)]
        fn default() -> Instr4cfg0 {
            Instr4cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr4cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr4cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr4cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr4cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr4cfg1(pub u32);
    impl Instr4cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr4cfg1 {
        #[inline(always)]
        fn default() -> Instr4cfg1 {
            Instr4cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr4cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr4cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr4cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr4cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr4cfg2(pub u32);
    impl Instr4cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr4cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr4cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr4cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr4cfg2 {
        #[inline(always)]
        fn default() -> Instr4cfg2 {
            Instr4cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr4cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr4cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr4cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr4cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr5cfg0(pub u32);
    impl Instr5cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr5cfg0 {
        #[inline(always)]
        fn default() -> Instr5cfg0 {
            Instr5cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr5cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr5cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr5cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr5cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr5cfg1(pub u32);
    impl Instr5cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr5cfg1 {
        #[inline(always)]
        fn default() -> Instr5cfg1 {
            Instr5cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr5cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr5cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr5cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr5cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr5cfg2(pub u32);
    impl Instr5cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr5cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr5cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr5cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr5cfg2 {
        #[inline(always)]
        fn default() -> Instr5cfg2 {
            Instr5cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr5cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr5cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr5cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr5cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr6cfg0(pub u32);
    impl Instr6cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr6cfg0 {
        #[inline(always)]
        fn default() -> Instr6cfg0 {
            Instr6cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr6cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr6cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr6cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr6cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr6cfg1(pub u32);
    impl Instr6cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr6cfg1 {
        #[inline(always)]
        fn default() -> Instr6cfg1 {
            Instr6cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr6cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr6cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr6cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr6cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr6cfg2(pub u32);
    impl Instr6cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr6cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr6cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr6cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr6cfg2 {
        #[inline(always)]
        fn default() -> Instr6cfg2 {
            Instr6cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr6cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr6cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr6cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr6cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "Instruction N Word 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr7cfg0(pub u32);
    impl Instr7cfg0 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realzero(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin0realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0realnegate(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin0realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagzero(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin0imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin0imagnegate(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin0imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1regid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realzero(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin1realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1realnegate(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin1realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagzero(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin1imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin1imagnegate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin1imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2regid(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluin2regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Real Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realzero(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Real Zero."]
        #[inline(always)]
        pub const fn set_aluin2realzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Real Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2realnegate(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Real Negate."]
        #[inline(always)]
        pub const fn set_aluin2realnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Imaginary Not Zero."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagzero(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Not Zero."]
        #[inline(always)]
        pub const fn set_aluin2imagzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Imaginary Negate."]
        #[must_use]
        #[inline(always)]
        pub const fn aluin2imagnegate(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Imaginary Negate."]
        #[inline(always)]
        pub const fn set_aluin2imagnegate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn aluoutregid(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_aluoutregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Instr7cfg0 {
        #[inline(always)]
        fn default() -> Instr7cfg0 {
            Instr7cfg0(0)
        }
    }
    impl core::fmt::Debug for Instr7cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr7cfg0")
                .field("aluin0regid", &self.aluin0regid())
                .field("aluin0realzero", &self.aluin0realzero())
                .field("aluin0realnegate", &self.aluin0realnegate())
                .field("aluin0imagzero", &self.aluin0imagzero())
                .field("aluin0imagnegate", &self.aluin0imagnegate())
                .field("aluin1regid", &self.aluin1regid())
                .field("aluin1realzero", &self.aluin1realzero())
                .field("aluin1realnegate", &self.aluin1realnegate())
                .field("aluin1imagzero", &self.aluin1imagzero())
                .field("aluin1imagnegate", &self.aluin1imagnegate())
                .field("aluin2regid", &self.aluin2regid())
                .field("aluin2realzero", &self.aluin2realzero())
                .field("aluin2realnegate", &self.aluin2realnegate())
                .field("aluin2imagzero", &self.aluin2imagzero())
                .field("aluin2imagnegate", &self.aluin2imagnegate())
                .field("aluoutregid", &self.aluoutregid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr7cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr7cfg0 {{ aluin0regid: {=u8:?}, aluin0realzero: {=bool:?}, aluin0realnegate: {=bool:?}, aluin0imagzero: {=bool:?}, aluin0imagnegate: {=bool:?}, aluin1regid: {=u8:?}, aluin1realzero: {=bool:?}, aluin1realnegate: {=bool:?}, aluin1imagzero: {=bool:?}, aluin1imagnegate: {=bool:?}, aluin2regid: {=u8:?}, aluin2realzero: {=bool:?}, aluin2realnegate: {=bool:?}, aluin2imagzero: {=bool:?}, aluin2imagnegate: {=bool:?}, aluoutregid: {=u8:?} }}",
                self.aluin0regid(),
                self.aluin0realzero(),
                self.aluin0realnegate(),
                self.aluin0imagzero(),
                self.aluin0imagnegate(),
                self.aluin1regid(),
                self.aluin1realzero(),
                self.aluin1realnegate(),
                self.aluin1imagzero(),
                self.aluin1imagnegate(),
                self.aluin2regid(),
                self.aluin2realzero(),
                self.aluin2realnegate(),
                self.aluin2imagzero(),
                self.aluin2imagnegate(),
                self.aluoutregid()
            )
        }
    }
    #[doc = "Instruction N word 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr7cfg1(pub u32);
    impl Instr7cfg1 {
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0regid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream0regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0load(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream0load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayid(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream0arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream0arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream0arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1regid(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_istream1regid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
        }
        #[doc = "Load register."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1load(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Load register."]
        #[inline(always)]
        pub const fn set_istream1load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayid(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_istream1arrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn istream1arrayincrdim2(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_istream1arrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Register ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamregid(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Register ID."]
        #[inline(always)]
        pub const fn set_ostreamregid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Store to Register."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamstore(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Store to Register."]
        #[inline(always)]
        pub const fn set_ostreamstore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Array ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Array ID."]
        #[inline(always)]
        pub const fn set_ostreamarrayid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Increment Array Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 0."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Increment Array Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim1(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 1."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Array Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ostreamarrayincrdim2(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Array Dimension 2."]
        #[inline(always)]
        pub const fn set_ostreamarrayincrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Instr7cfg1 {
        #[inline(always)]
        fn default() -> Instr7cfg1 {
            Instr7cfg1(0)
        }
    }
    impl core::fmt::Debug for Instr7cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr7cfg1")
                .field("istream0regid", &self.istream0regid())
                .field("istream0load", &self.istream0load())
                .field("istream0arrayid", &self.istream0arrayid())
                .field("istream0arrayincrdim0", &self.istream0arrayincrdim0())
                .field("istream0arrayincrdim1", &self.istream0arrayincrdim1())
                .field("istream0arrayincrdim2", &self.istream0arrayincrdim2())
                .field("istream1regid", &self.istream1regid())
                .field("istream1load", &self.istream1load())
                .field("istream1arrayid", &self.istream1arrayid())
                .field("istream1arrayincrdim0", &self.istream1arrayincrdim0())
                .field("istream1arrayincrdim1", &self.istream1arrayincrdim1())
                .field("istream1arrayincrdim2", &self.istream1arrayincrdim2())
                .field("ostreamregid", &self.ostreamregid())
                .field("ostreamstore", &self.ostreamstore())
                .field("ostreamarrayid", &self.ostreamarrayid())
                .field("ostreamarrayincrdim0", &self.ostreamarrayincrdim0())
                .field("ostreamarrayincrdim1", &self.ostreamarrayincrdim1())
                .field("ostreamarrayincrdim2", &self.ostreamarrayincrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr7cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr7cfg1 {{ istream0regid: {=u8:?}, istream0load: {=bool:?}, istream0arrayid: {=u8:?}, istream0arrayincrdim0: {=bool:?}, istream0arrayincrdim1: {=bool:?}, istream0arrayincrdim2: {=bool:?}, istream1regid: {=u8:?}, istream1load: {=bool:?}, istream1arrayid: {=u8:?}, istream1arrayincrdim0: {=bool:?}, istream1arrayincrdim1: {=bool:?}, istream1arrayincrdim2: {=bool:?}, ostreamregid: {=u8:?}, ostreamstore: {=bool:?}, ostreamarrayid: {=u8:?}, ostreamarrayincrdim0: {=bool:?}, ostreamarrayincrdim1: {=bool:?}, ostreamarrayincrdim2: {=bool:?} }}",
                self.istream0regid(),
                self.istream0load(),
                self.istream0arrayid(),
                self.istream0arrayincrdim0(),
                self.istream0arrayincrdim1(),
                self.istream0arrayincrdim2(),
                self.istream1regid(),
                self.istream1load(),
                self.istream1arrayid(),
                self.istream1arrayincrdim0(),
                self.istream1arrayincrdim1(),
                self.istream1arrayincrdim2(),
                self.ostreamregid(),
                self.ostreamstore(),
                self.ostreamarrayid(),
                self.ostreamarrayincrdim0(),
                self.ostreamarrayincrdim1(),
                self.ostreamarrayincrdim2()
            )
        }
    }
    #[doc = "Instruction N word 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr7cfg2(pub u32);
    impl Instr7cfg2 {
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0begin(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop0begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop0end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1begin(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop1begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop1end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop1end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2begin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop2begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop2end(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop2end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3begin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop3begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop3end(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop3end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4begin(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop4begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop4end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop4end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5begin(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop5begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop5end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop5end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6begin(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop6begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop6end(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop6end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Loop Begin."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7begin(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Begin."]
        #[inline(always)]
        pub const fn set_loop7begin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Loop End."]
        #[must_use]
        #[inline(always)]
        pub const fn loop7end(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Loop End."]
        #[inline(always)]
        pub const fn set_loop7end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ALU opcode."]
        #[must_use]
        #[inline(always)]
        pub const fn aluop(&self) -> super::vals::Instr7cfg2Aluop {
            let val = (self.0 >> 20usize) & 0x01ff;
            super::vals::Instr7cfg2Aluop::from_bits(val as u16)
        }
        #[doc = "ALU opcode."]
        #[inline(always)]
        pub const fn set_aluop(&mut self, val: super::vals::Instr7cfg2Aluop) {
            self.0 = (self.0 & !(0x01ff << 20usize)) | (((val.to_bits() as u32) & 0x01ff) << 20usize);
        }
        #[doc = "End of Program."]
        #[must_use]
        #[inline(always)]
        pub const fn endprog(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "End of Program."]
        #[inline(always)]
        pub const fn set_endprog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Instr7cfg2 {
        #[inline(always)]
        fn default() -> Instr7cfg2 {
            Instr7cfg2(0)
        }
    }
    impl core::fmt::Debug for Instr7cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr7cfg2")
                .field("loop0begin", &self.loop0begin())
                .field("loop0end", &self.loop0end())
                .field("loop1begin", &self.loop1begin())
                .field("loop1end", &self.loop1end())
                .field("loop2begin", &self.loop2begin())
                .field("loop2end", &self.loop2end())
                .field("loop3begin", &self.loop3begin())
                .field("loop3end", &self.loop3end())
                .field("loop4begin", &self.loop4begin())
                .field("loop4end", &self.loop4end())
                .field("loop5begin", &self.loop5begin())
                .field("loop5end", &self.loop5end())
                .field("loop6begin", &self.loop6begin())
                .field("loop6end", &self.loop6end())
                .field("loop7begin", &self.loop7begin())
                .field("loop7end", &self.loop7end())
                .field("aluop", &self.aluop())
                .field("endprog", &self.endprog())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr7cfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr7cfg2 {{ loop0begin: {=bool:?}, loop0end: {=bool:?}, loop1begin: {=bool:?}, loop1end: {=bool:?}, loop2begin: {=bool:?}, loop2end: {=bool:?}, loop3begin: {=bool:?}, loop3end: {=bool:?}, loop4begin: {=bool:?}, loop4end: {=bool:?}, loop5begin: {=bool:?}, loop5end: {=bool:?}, loop6begin: {=bool:?}, loop6end: {=bool:?}, loop7begin: {=bool:?}, loop7end: {=bool:?}, aluop: {:?}, endprog: {=bool:?} }}",
                self.loop0begin(),
                self.loop0end(),
                self.loop1begin(),
                self.loop1end(),
                self.loop2begin(),
                self.loop2end(),
                self.loop3begin(),
                self.loop3end(),
                self.loop4begin(),
                self.loop4end(),
                self.loop5begin(),
                self.loop5end(),
                self.loop6begin(),
                self.loop6end(),
                self.loop7begin(),
                self.loop7end(),
                self.aluop(),
                self.endprog()
            )
        }
    }
    #[doc = "IP Version Register."]
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
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop0cfg(pub u32);
    impl Loop0cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop0cfg {
        #[inline(always)]
        fn default() -> Loop0cfg {
            Loop0cfg(0)
        }
    }
    impl core::fmt::Debug for Loop0cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop0cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop0cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop0cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop0rst(pub u32);
    impl Loop0rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop0rst {
        #[inline(always)]
        fn default() -> Loop0rst {
            Loop0rst(0)
        }
    }
    impl core::fmt::Debug for Loop0rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop0rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop0rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop0rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop0state(pub u32);
    impl Loop0state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop0state {
        #[inline(always)]
        fn default() -> Loop0state {
            Loop0state(0)
        }
    }
    impl core::fmt::Debug for Loop0state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop0state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop0state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop0state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop1cfg(pub u32);
    impl Loop1cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop1cfg {
        #[inline(always)]
        fn default() -> Loop1cfg {
            Loop1cfg(0)
        }
    }
    impl core::fmt::Debug for Loop1cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop1cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop1cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop1cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop1rst(pub u32);
    impl Loop1rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop1rst {
        #[inline(always)]
        fn default() -> Loop1rst {
            Loop1rst(0)
        }
    }
    impl core::fmt::Debug for Loop1rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop1rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop1rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop1rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop1state(pub u32);
    impl Loop1state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop1state {
        #[inline(always)]
        fn default() -> Loop1state {
            Loop1state(0)
        }
    }
    impl core::fmt::Debug for Loop1state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop1state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop1state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop1state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop2cfg(pub u32);
    impl Loop2cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop2cfg {
        #[inline(always)]
        fn default() -> Loop2cfg {
            Loop2cfg(0)
        }
    }
    impl core::fmt::Debug for Loop2cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop2cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop2cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop2cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop2rst(pub u32);
    impl Loop2rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop2rst {
        #[inline(always)]
        fn default() -> Loop2rst {
            Loop2rst(0)
        }
    }
    impl core::fmt::Debug for Loop2rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop2rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop2rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop2rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop2state(pub u32);
    impl Loop2state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop2state {
        #[inline(always)]
        fn default() -> Loop2state {
            Loop2state(0)
        }
    }
    impl core::fmt::Debug for Loop2state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop2state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop2state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop2state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop3cfg(pub u32);
    impl Loop3cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop3cfg {
        #[inline(always)]
        fn default() -> Loop3cfg {
            Loop3cfg(0)
        }
    }
    impl core::fmt::Debug for Loop3cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop3cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop3cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop3cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop3rst(pub u32);
    impl Loop3rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop3rst {
        #[inline(always)]
        fn default() -> Loop3rst {
            Loop3rst(0)
        }
    }
    impl core::fmt::Debug for Loop3rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop3rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop3rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop3rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop3state(pub u32);
    impl Loop3state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop3state {
        #[inline(always)]
        fn default() -> Loop3state {
            Loop3state(0)
        }
    }
    impl core::fmt::Debug for Loop3state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop3state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop3state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop3state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop4cfg(pub u32);
    impl Loop4cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop4cfg {
        #[inline(always)]
        fn default() -> Loop4cfg {
            Loop4cfg(0)
        }
    }
    impl core::fmt::Debug for Loop4cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop4cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop4cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop4cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop4rst(pub u32);
    impl Loop4rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop4rst {
        #[inline(always)]
        fn default() -> Loop4rst {
            Loop4rst(0)
        }
    }
    impl core::fmt::Debug for Loop4rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop4rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop4rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop4rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop4state(pub u32);
    impl Loop4state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop4state {
        #[inline(always)]
        fn default() -> Loop4state {
            Loop4state(0)
        }
    }
    impl core::fmt::Debug for Loop4state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop4state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop4state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop4state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop5cfg(pub u32);
    impl Loop5cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop5cfg {
        #[inline(always)]
        fn default() -> Loop5cfg {
            Loop5cfg(0)
        }
    }
    impl core::fmt::Debug for Loop5cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop5cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop5cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop5cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop5rst(pub u32);
    impl Loop5rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop5rst {
        #[inline(always)]
        fn default() -> Loop5rst {
            Loop5rst(0)
        }
    }
    impl core::fmt::Debug for Loop5rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop5rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop5rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop5rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop5state(pub u32);
    impl Loop5state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop5state {
        #[inline(always)]
        fn default() -> Loop5state {
            Loop5state(0)
        }
    }
    impl core::fmt::Debug for Loop5state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop5state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop5state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop5state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop6cfg(pub u32);
    impl Loop6cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop6cfg {
        #[inline(always)]
        fn default() -> Loop6cfg {
            Loop6cfg(0)
        }
    }
    impl core::fmt::Debug for Loop6cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop6cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop6cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop6cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop6rst(pub u32);
    impl Loop6rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop6rst {
        #[inline(always)]
        fn default() -> Loop6rst {
            Loop6rst(0)
        }
    }
    impl core::fmt::Debug for Loop6rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop6rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop6rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop6rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop6state(pub u32);
    impl Loop6state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop6state {
        #[inline(always)]
        fn default() -> Loop6state {
            Loop6state(0)
        }
    }
    impl core::fmt::Debug for Loop6state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop6state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop6state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop6state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Loop N Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop7cfg(pub u32);
    impl Loop7cfg {
        #[doc = "Number of Iterations."]
        #[must_use]
        #[inline(always)]
        pub const fn numiters(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Number of Iterations."]
        #[inline(always)]
        pub const fn set_numiters(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array0incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array0incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0incrdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array0incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array1incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array1incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1incrdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array1incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array2incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array2incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2incrdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array2incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array3incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array3incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3incrdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array3incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Increment Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 0."]
        #[inline(always)]
        pub const fn set_array4incrdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Increment Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 1."]
        #[inline(always)]
        pub const fn set_array4incrdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Increment Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4incrdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Increment Dimension 2."]
        #[inline(always)]
        pub const fn set_array4incrdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop7cfg {
        #[inline(always)]
        fn default() -> Loop7cfg {
            Loop7cfg(0)
        }
    }
    impl core::fmt::Debug for Loop7cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop7cfg")
                .field("numiters", &self.numiters())
                .field("array0incrdim0", &self.array0incrdim0())
                .field("array0incrdim1", &self.array0incrdim1())
                .field("array0incrdim2", &self.array0incrdim2())
                .field("array1incrdim0", &self.array1incrdim0())
                .field("array1incrdim1", &self.array1incrdim1())
                .field("array1incrdim2", &self.array1incrdim2())
                .field("array2incrdim0", &self.array2incrdim0())
                .field("array2incrdim1", &self.array2incrdim1())
                .field("array2incrdim2", &self.array2incrdim2())
                .field("array3incrdim0", &self.array3incrdim0())
                .field("array3incrdim1", &self.array3incrdim1())
                .field("array3incrdim2", &self.array3incrdim2())
                .field("array4incrdim0", &self.array4incrdim0())
                .field("array4incrdim1", &self.array4incrdim1())
                .field("array4incrdim2", &self.array4incrdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop7cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop7cfg {{ numiters: {=u16:?}, array0incrdim0: {=bool:?}, array0incrdim1: {=bool:?}, array0incrdim2: {=bool:?}, array1incrdim0: {=bool:?}, array1incrdim1: {=bool:?}, array1incrdim2: {=bool:?}, array2incrdim0: {=bool:?}, array2incrdim1: {=bool:?}, array2incrdim2: {=bool:?}, array3incrdim0: {=bool:?}, array3incrdim1: {=bool:?}, array3incrdim2: {=bool:?}, array4incrdim0: {=bool:?}, array4incrdim1: {=bool:?}, array4incrdim2: {=bool:?} }}",
                self.numiters(),
                self.array0incrdim0(),
                self.array0incrdim1(),
                self.array0incrdim2(),
                self.array1incrdim0(),
                self.array1incrdim1(),
                self.array1incrdim2(),
                self.array2incrdim0(),
                self.array2incrdim1(),
                self.array2incrdim2(),
                self.array3incrdim0(),
                self.array3incrdim1(),
                self.array3incrdim2(),
                self.array4incrdim0(),
                self.array4incrdim1(),
                self.array4incrdim2()
            )
        }
    }
    #[doc = "Loop N Reset Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop7rst(pub u32);
    impl Loop7rst {
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array0resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array0resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array0resetdim2(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array0resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array1resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array1resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array1resetdim2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array1resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array2resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim1(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array2resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array2resetdim2(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array2resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array3resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array3resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array3resetdim2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array3resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset Dimension 0."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 0."]
        #[inline(always)]
        pub const fn set_array4resetdim0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset Dimension 1."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 1."]
        #[inline(always)]
        pub const fn set_array4resetdim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Reset Dimension 2."]
        #[must_use]
        #[inline(always)]
        pub const fn array4resetdim2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset Dimension 2."]
        #[inline(always)]
        pub const fn set_array4resetdim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Loop7rst {
        #[inline(always)]
        fn default() -> Loop7rst {
            Loop7rst(0)
        }
    }
    impl core::fmt::Debug for Loop7rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop7rst")
                .field("array0resetdim0", &self.array0resetdim0())
                .field("array0resetdim1", &self.array0resetdim1())
                .field("array0resetdim2", &self.array0resetdim2())
                .field("array1resetdim0", &self.array1resetdim0())
                .field("array1resetdim1", &self.array1resetdim1())
                .field("array1resetdim2", &self.array1resetdim2())
                .field("array2resetdim0", &self.array2resetdim0())
                .field("array2resetdim1", &self.array2resetdim1())
                .field("array2resetdim2", &self.array2resetdim2())
                .field("array3resetdim0", &self.array3resetdim0())
                .field("array3resetdim1", &self.array3resetdim1())
                .field("array3resetdim2", &self.array3resetdim2())
                .field("array4resetdim0", &self.array4resetdim0())
                .field("array4resetdim1", &self.array4resetdim1())
                .field("array4resetdim2", &self.array4resetdim2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop7rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop7rst {{ array0resetdim0: {=bool:?}, array0resetdim1: {=bool:?}, array0resetdim2: {=bool:?}, array1resetdim0: {=bool:?}, array1resetdim1: {=bool:?}, array1resetdim2: {=bool:?}, array2resetdim0: {=bool:?}, array2resetdim1: {=bool:?}, array2resetdim2: {=bool:?}, array3resetdim0: {=bool:?}, array3resetdim1: {=bool:?}, array3resetdim2: {=bool:?}, array4resetdim0: {=bool:?}, array4resetdim1: {=bool:?}, array4resetdim2: {=bool:?} }}",
                self.array0resetdim0(),
                self.array0resetdim1(),
                self.array0resetdim2(),
                self.array1resetdim0(),
                self.array1resetdim1(),
                self.array1resetdim2(),
                self.array2resetdim0(),
                self.array2resetdim1(),
                self.array2resetdim2(),
                self.array3resetdim0(),
                self.array3resetdim1(),
                self.array3resetdim2(),
                self.array4resetdim0(),
                self.array4resetdim1(),
                self.array4resetdim2()
            )
        }
    }
    #[doc = "Loop N State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Loop7state(pub u32);
    impl Loop7state {
        #[doc = "Loop Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Loop Counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Loop Active."]
        #[must_use]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Active."]
        #[inline(always)]
        pub const fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Loop Start."]
        #[must_use]
        #[inline(always)]
        pub const fn pcbegin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop Start."]
        #[inline(always)]
        pub const fn set_pcbegin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Loop7state {
        #[inline(always)]
        fn default() -> Loop7state {
            Loop7state(0)
        }
    }
    impl core::fmt::Debug for Loop7state {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Loop7state")
                .field("cnt", &self.cnt())
                .field("active", &self.active())
                .field("pcbegin", &self.pcbegin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Loop7state {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Loop7state {{ cnt: {=u16:?}, active: {=bool:?}, pcbegin: {=u8:?} }}",
                self.cnt(),
                self.active(),
                self.pcbegin()
            )
        }
    }
    #[doc = "Run Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perf0cnt(pub u32);
    impl Perf0cnt {
        #[doc = "Performance Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Performance Counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Perf0cnt {
        #[inline(always)]
        fn default() -> Perf0cnt {
            Perf0cnt(0)
        }
    }
    impl core::fmt::Debug for Perf0cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perf0cnt").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perf0cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Perf0cnt {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "Run Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Perf1cnt(pub u32);
    impl Perf1cnt {
        #[doc = "Performance Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Performance Counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Perf1cnt {
        #[inline(always)]
        fn default() -> Perf1cnt {
            Perf1cnt(0)
        }
    }
    impl core::fmt::Debug for Perf1cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Perf1cnt").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Perf1cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Perf1cnt {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "Program State Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Programstate(pub u32);
    impl Programstate {
        #[doc = "Program Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn pc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Program Counter."]
        #[inline(always)]
        pub const fn set_pc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Programstate {
        #[inline(always)]
        fn default() -> Programstate {
            Programstate(0)
        }
    }
    impl core::fmt::Debug for Programstate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Programstate").field("pc", &self.pc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Programstate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Programstate {{ pc: {=u8:?} }}", self.pc())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Running Status."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Running Status."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Paused Status."]
        #[must_use]
        #[inline(always)]
        pub const fn paused(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Paused Status."]
        #[inline(always)]
        pub const fn set_paused(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Idle Status."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Idle Status."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("running", &self.running())
                .field("paused", &self.paused())
                .field("idle", &self.idle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ running: {=bool:?}, paused: {=bool:?}, idle: {=bool:?} }}",
                self.running(),
                self.paused(),
                self.idle()
            )
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
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array0dim0cfgBasetype {
        #[doc = "Data is unsigned 8-bit integer (can only be used for loads)."]
        Uint8 = 0x0,
        #[doc = "Data is signed 8-bit integer (can only be used for loads)."]
        Int8 = 0x01,
        #[doc = "Data is 16-bit float."]
        Binary16 = 0x02,
        #[doc = "Reserved. Invalid data if this is specified."]
        Reserved = 0x03,
    }
    impl Array0dim0cfgBasetype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array0dim0cfgBasetype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array0dim0cfgBasetype {
        #[inline(always)]
        fn from(val: u8) -> Array0dim0cfgBasetype {
            Array0dim0cfgBasetype::from_bits(val)
        }
    }
    impl From<Array0dim0cfgBasetype> for u8 {
        #[inline(always)]
        fn from(val: Array0dim0cfgBasetype) -> u8 {
            Array0dim0cfgBasetype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array0dim0cfgComplex {
        #[doc = "Data represents a scalar number."]
        Scalar = 0x0,
        #[doc = "Data represents a complex pair or packed pair of reals."]
        Complex = 0x01,
    }
    impl Array0dim0cfgComplex {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array0dim0cfgComplex {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array0dim0cfgComplex {
        #[inline(always)]
        fn from(val: u8) -> Array0dim0cfgComplex {
            Array0dim0cfgComplex::from_bits(val)
        }
    }
    impl From<Array0dim0cfgComplex> for u8 {
        #[inline(always)]
        fn from(val: Array0dim0cfgComplex) -> u8 {
            Array0dim0cfgComplex::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array1dim0cfgBasetype {
        #[doc = "Data is unsigned 8-bit integer (can only be used for loads)."]
        Uint8 = 0x0,
        #[doc = "Data is signed 8-bit integer (can only be used for loads)."]
        Int8 = 0x01,
        #[doc = "Data is 16-bit float."]
        Binary16 = 0x02,
        #[doc = "Reserved. Invalid data if this is specified."]
        Reserved = 0x03,
    }
    impl Array1dim0cfgBasetype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array1dim0cfgBasetype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array1dim0cfgBasetype {
        #[inline(always)]
        fn from(val: u8) -> Array1dim0cfgBasetype {
            Array1dim0cfgBasetype::from_bits(val)
        }
    }
    impl From<Array1dim0cfgBasetype> for u8 {
        #[inline(always)]
        fn from(val: Array1dim0cfgBasetype) -> u8 {
            Array1dim0cfgBasetype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array1dim0cfgComplex {
        #[doc = "Data represents a scalar number."]
        Scalar = 0x0,
        #[doc = "Data represents a complex pair or packed pair of reals."]
        Complex = 0x01,
    }
    impl Array1dim0cfgComplex {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array1dim0cfgComplex {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array1dim0cfgComplex {
        #[inline(always)]
        fn from(val: u8) -> Array1dim0cfgComplex {
            Array1dim0cfgComplex::from_bits(val)
        }
    }
    impl From<Array1dim0cfgComplex> for u8 {
        #[inline(always)]
        fn from(val: Array1dim0cfgComplex) -> u8 {
            Array1dim0cfgComplex::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array2dim0cfgBasetype {
        #[doc = "Data is unsigned 8-bit integer (can only be used for loads)."]
        Uint8 = 0x0,
        #[doc = "Data is signed 8-bit integer (can only be used for loads)."]
        Int8 = 0x01,
        #[doc = "Data is 16-bit float."]
        Binary16 = 0x02,
        #[doc = "Reserved. Invalid data if this is specified."]
        Reserved = 0x03,
    }
    impl Array2dim0cfgBasetype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array2dim0cfgBasetype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array2dim0cfgBasetype {
        #[inline(always)]
        fn from(val: u8) -> Array2dim0cfgBasetype {
            Array2dim0cfgBasetype::from_bits(val)
        }
    }
    impl From<Array2dim0cfgBasetype> for u8 {
        #[inline(always)]
        fn from(val: Array2dim0cfgBasetype) -> u8 {
            Array2dim0cfgBasetype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array2dim0cfgComplex {
        #[doc = "Data represents a scalar number."]
        Scalar = 0x0,
        #[doc = "Data represents a complex pair or packed pair of reals."]
        Complex = 0x01,
    }
    impl Array2dim0cfgComplex {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array2dim0cfgComplex {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array2dim0cfgComplex {
        #[inline(always)]
        fn from(val: u8) -> Array2dim0cfgComplex {
            Array2dim0cfgComplex::from_bits(val)
        }
    }
    impl From<Array2dim0cfgComplex> for u8 {
        #[inline(always)]
        fn from(val: Array2dim0cfgComplex) -> u8 {
            Array2dim0cfgComplex::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array3dim0cfgBasetype {
        #[doc = "Data is unsigned 8-bit integer (can only be used for loads)."]
        Uint8 = 0x0,
        #[doc = "Data is signed 8-bit integer (can only be used for loads)."]
        Int8 = 0x01,
        #[doc = "Data is 16-bit float."]
        Binary16 = 0x02,
        #[doc = "Reserved. Invalid data if this is specified."]
        Reserved = 0x03,
    }
    impl Array3dim0cfgBasetype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array3dim0cfgBasetype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array3dim0cfgBasetype {
        #[inline(always)]
        fn from(val: u8) -> Array3dim0cfgBasetype {
            Array3dim0cfgBasetype::from_bits(val)
        }
    }
    impl From<Array3dim0cfgBasetype> for u8 {
        #[inline(always)]
        fn from(val: Array3dim0cfgBasetype) -> u8 {
            Array3dim0cfgBasetype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array3dim0cfgComplex {
        #[doc = "Data represents a scalar number."]
        Scalar = 0x0,
        #[doc = "Data represents a complex pair or packed pair of reals."]
        Complex = 0x01,
    }
    impl Array3dim0cfgComplex {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array3dim0cfgComplex {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array3dim0cfgComplex {
        #[inline(always)]
        fn from(val: u8) -> Array3dim0cfgComplex {
            Array3dim0cfgComplex::from_bits(val)
        }
    }
    impl From<Array3dim0cfgComplex> for u8 {
        #[inline(always)]
        fn from(val: Array3dim0cfgComplex) -> u8 {
            Array3dim0cfgComplex::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array4dim0cfgBasetype {
        #[doc = "Data is unsigned 8-bit integer (can only be used for loads)."]
        Uint8 = 0x0,
        #[doc = "Data is signed 8-bit integer (can only be used for loads)."]
        Int8 = 0x01,
        #[doc = "Data is 16-bit float."]
        Binary16 = 0x02,
        #[doc = "Reserved. Invalid data if this is specified."]
        Reserved = 0x03,
    }
    impl Array4dim0cfgBasetype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array4dim0cfgBasetype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array4dim0cfgBasetype {
        #[inline(always)]
        fn from(val: u8) -> Array4dim0cfgBasetype {
            Array4dim0cfgBasetype::from_bits(val)
        }
    }
    impl From<Array4dim0cfgBasetype> for u8 {
        #[inline(always)]
        fn from(val: Array4dim0cfgBasetype) -> u8 {
            Array4dim0cfgBasetype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Array4dim0cfgComplex {
        #[doc = "Data represents a scalar number."]
        Scalar = 0x0,
        #[doc = "Data represents a complex pair or packed pair of reals."]
        Complex = 0x01,
    }
    impl Array4dim0cfgComplex {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Array4dim0cfgComplex {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Array4dim0cfgComplex {
        #[inline(always)]
        fn from(val: u8) -> Array4dim0cfgComplex {
            Array4dim0cfgComplex::from_bits(val)
        }
    }
    impl From<Array4dim0cfgComplex> for u8 {
        #[inline(always)]
        fn from(val: Array4dim0cfgComplex) -> u8 {
            Array4dim0cfgComplex::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Faultbus {
        #[doc = "NONE."]
        None = 0x0,
        #[doc = "LOAD0STREAM."]
        Load0stream = 0x01,
        #[doc = "LOAD1STREAM."]
        Load1stream = 0x02,
        #[doc = "STORESTREAM."]
        Storestream = 0x03,
    }
    impl Faultbus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Faultbus {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Faultbus {
        #[inline(always)]
        fn from(val: u8) -> Faultbus {
            Faultbus::from_bits(val)
        }
    }
    impl From<Faultbus> for u8 {
        #[inline(always)]
        fn from(val: Faultbus) -> u8 {
            Faultbus::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr0cfg2Aluop(u16);
    impl Instr0cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr0cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr0cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr0cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr0cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr0cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr0cfg2Aluop {
            Instr0cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr0cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr0cfg2Aluop) -> u16 {
            Instr0cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr1cfg2Aluop(u16);
    impl Instr1cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr1cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr1cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr1cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr1cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr1cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr1cfg2Aluop {
            Instr1cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr1cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr1cfg2Aluop) -> u16 {
            Instr1cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr2cfg2Aluop(u16);
    impl Instr2cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr2cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr2cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr2cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr2cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr2cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr2cfg2Aluop {
            Instr2cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr2cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr2cfg2Aluop) -> u16 {
            Instr2cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr3cfg2Aluop(u16);
    impl Instr3cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr3cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr3cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr3cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr3cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr3cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr3cfg2Aluop {
            Instr3cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr3cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr3cfg2Aluop) -> u16 {
            Instr3cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr4cfg2Aluop(u16);
    impl Instr4cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr4cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr4cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr4cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr4cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr4cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr4cfg2Aluop {
            Instr4cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr4cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr4cfg2Aluop) -> u16 {
            Instr4cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr5cfg2Aluop(u16);
    impl Instr5cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr5cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr5cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr5cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr5cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr5cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr5cfg2Aluop {
            Instr5cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr5cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr5cfg2Aluop) -> u16 {
            Instr5cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr6cfg2Aluop(u16);
    impl Instr6cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr6cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr6cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr6cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr6cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr6cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr6cfg2Aluop {
            Instr6cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr6cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr6cfg2Aluop) -> u16 {
            Instr6cfg2Aluop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Instr7cfg2Aluop(u16);
    impl Instr7cfg2Aluop {
        #[doc = "No Operation."]
        pub const Noop: Self = Self(0x0);
        #[doc = "Clear register (set to +0)."]
        pub const Clear: Self = Self(0x01);
        #[doc = "Copy operation."]
        pub const Copy: Self = Self(0x41);
        #[doc = "Swap operation."]
        pub const Swap: Self = Self(0x42);
        #[doc = "Double operation (multiply by 2)."]
        pub const Dbl: Self = Self(0x43);
        #[doc = "Load real and imag (form A)."]
        pub const Fana: Self = Self(0x44);
        #[doc = "Load real and imag (form B)."]
        pub const Fanb: Self = Self(0x45);
        #[doc = "ReLU of real (max of real and +0)."]
        pub const Relu2: Self = Self(0x46);
        #[doc = "Min of real and -0."]
        pub const Nrelu2: Self = Self(0x47);
        #[doc = "Increment by 1.0."]
        pub const Inc2: Self = Self(0x48);
        #[doc = "Decrement by 1.0."]
        pub const Dec2: Self = Self(0x49);
        #[doc = "Addition of 2 reals."]
        pub const Addr: Self = Self(0x4a);
        #[doc = "Maximum of 2 reals."]
        pub const Max: Self = Self(0x4b);
        #[doc = "Minimum of 2 reals."]
        pub const Min: Self = Self(0x4c);
        #[doc = "Square of real (form B)."]
        pub const Rsqr2b: Self = Self(0x0124);
        #[doc = "Add Complex."]
        pub const Addc: Self = Self(0x014e);
        #[doc = "Max of reals (form A)."]
        pub const Max2a: Self = Self(0x0153);
        #[doc = "Min of reals (form A)."]
        pub const Min2a: Self = Self(0x0154);
        #[doc = "Extract real from complex."]
        pub const Xrealc2: Self = Self(0x015e);
        #[doc = "Extract imag from complex."]
        pub const Ximagc2: Self = Self(0x015f);
        #[doc = "Add reals (form B)."]
        pub const Addr2b: Self = Self(0x0161);
        #[doc = "Max of reals (form B)."]
        pub const Max2b: Self = Self(0x0162);
        #[doc = "Min of reals (form B)."]
        pub const Min2b: Self = Self(0x0163);
        #[doc = "Multiply Complex."]
        pub const Mulc: Self = Self(0x018d);
        #[doc = "Multiply reals (form A)."]
        pub const Mulr2a: Self = Self(0x0197);
        #[doc = "Multiply reals (form B)."]
        pub const Mulr2b: Self = Self(0x0198);
        #[doc = "Add 4 reals."]
        pub const Addr4: Self = Self(0x019a);
        #[doc = "Max of 4 reals."]
        pub const Max4: Self = Self(0x019b);
        #[doc = "Min of 4 reals."]
        pub const Min4: Self = Self(0x019c);
        #[doc = "Squared magnitude Complex."]
        pub const Sqrmagc2: Self = Self(0x019d);
        #[doc = "Parametric ReLU (form B)."]
        pub const Prelu2b: Self = Self(0x01a0);
        #[doc = "Multiply Accumulate Complex."]
        pub const Macc: Self = Self(0x01cd);
        #[doc = "Add Accumulate Complex."]
        pub const Aacc: Self = Self(0x01ce);
        #[doc = "part of ELU activation (form A)."]
        pub const Elu2a: Self = Self(0x01cf);
        #[doc = "part of ELU activation (form B)."]
        pub const Elu2b: Self = Self(0x01d0);
        #[doc = "If A then X else Y (form A)."]
        pub const Ifr2a: Self = Self(0x01d1);
        #[doc = "If A then X else Y (form B)."]
        pub const Ifr2b: Self = Self(0x01d2);
        #[doc = "Max of reals and accumulator."]
        pub const Maxac2: Self = Self(0x01d3);
        #[doc = "Min of reals and accumulators."]
        pub const Minac2: Self = Self(0x01d4);
        #[doc = "Clipping activation (form A)."]
        pub const Clip2a: Self = Self(0x01d5);
        #[doc = "Clipping activation (form B)."]
        pub const Clip2b: Self = Self(0x01d6);
        #[doc = "Multiply accumulate reals (form A)."]
        pub const Macr2a: Self = Self(0x01d7);
        #[doc = "Multiply accumulate reals (form B)."]
        pub const Macr2b: Self = Self(0x01d8);
        #[doc = "If A then X else Y (complex)."]
        pub const Ifc: Self = Self(0x01d9);
    }
    impl Instr7cfg2Aluop {
        pub const fn from_bits(val: u16) -> Instr7cfg2Aluop {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Instr7cfg2Aluop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Noop"),
                0x01 => f.write_str("Clear"),
                0x41 => f.write_str("Copy"),
                0x42 => f.write_str("Swap"),
                0x43 => f.write_str("Dbl"),
                0x44 => f.write_str("Fana"),
                0x45 => f.write_str("Fanb"),
                0x46 => f.write_str("Relu2"),
                0x47 => f.write_str("Nrelu2"),
                0x48 => f.write_str("Inc2"),
                0x49 => f.write_str("Dec2"),
                0x4a => f.write_str("Addr"),
                0x4b => f.write_str("Max"),
                0x4c => f.write_str("Min"),
                0x0124 => f.write_str("Rsqr2b"),
                0x014e => f.write_str("Addc"),
                0x0153 => f.write_str("Max2a"),
                0x0154 => f.write_str("Min2a"),
                0x015e => f.write_str("Xrealc2"),
                0x015f => f.write_str("Ximagc2"),
                0x0161 => f.write_str("Addr2b"),
                0x0162 => f.write_str("Max2b"),
                0x0163 => f.write_str("Min2b"),
                0x018d => f.write_str("Mulc"),
                0x0197 => f.write_str("Mulr2a"),
                0x0198 => f.write_str("Mulr2b"),
                0x019a => f.write_str("Addr4"),
                0x019b => f.write_str("Max4"),
                0x019c => f.write_str("Min4"),
                0x019d => f.write_str("Sqrmagc2"),
                0x01a0 => f.write_str("Prelu2b"),
                0x01cd => f.write_str("Macc"),
                0x01ce => f.write_str("Aacc"),
                0x01cf => f.write_str("Elu2a"),
                0x01d0 => f.write_str("Elu2b"),
                0x01d1 => f.write_str("Ifr2a"),
                0x01d2 => f.write_str("Ifr2b"),
                0x01d3 => f.write_str("Maxac2"),
                0x01d4 => f.write_str("Minac2"),
                0x01d5 => f.write_str("Clip2a"),
                0x01d6 => f.write_str("Clip2b"),
                0x01d7 => f.write_str("Macr2a"),
                0x01d8 => f.write_str("Macr2b"),
                0x01d9 => f.write_str("Ifc"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr7cfg2Aluop {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Noop"),
                0x01 => defmt::write!(f, "Clear"),
                0x41 => defmt::write!(f, "Copy"),
                0x42 => defmt::write!(f, "Swap"),
                0x43 => defmt::write!(f, "Dbl"),
                0x44 => defmt::write!(f, "Fana"),
                0x45 => defmt::write!(f, "Fanb"),
                0x46 => defmt::write!(f, "Relu2"),
                0x47 => defmt::write!(f, "Nrelu2"),
                0x48 => defmt::write!(f, "Inc2"),
                0x49 => defmt::write!(f, "Dec2"),
                0x4a => defmt::write!(f, "Addr"),
                0x4b => defmt::write!(f, "Max"),
                0x4c => defmt::write!(f, "Min"),
                0x0124 => defmt::write!(f, "Rsqr2b"),
                0x014e => defmt::write!(f, "Addc"),
                0x0153 => defmt::write!(f, "Max2a"),
                0x0154 => defmt::write!(f, "Min2a"),
                0x015e => defmt::write!(f, "Xrealc2"),
                0x015f => defmt::write!(f, "Ximagc2"),
                0x0161 => defmt::write!(f, "Addr2b"),
                0x0162 => defmt::write!(f, "Max2b"),
                0x0163 => defmt::write!(f, "Min2b"),
                0x018d => defmt::write!(f, "Mulc"),
                0x0197 => defmt::write!(f, "Mulr2a"),
                0x0198 => defmt::write!(f, "Mulr2b"),
                0x019a => defmt::write!(f, "Addr4"),
                0x019b => defmt::write!(f, "Max4"),
                0x019c => defmt::write!(f, "Min4"),
                0x019d => defmt::write!(f, "Sqrmagc2"),
                0x01a0 => defmt::write!(f, "Prelu2b"),
                0x01cd => defmt::write!(f, "Macc"),
                0x01ce => defmt::write!(f, "Aacc"),
                0x01cf => defmt::write!(f, "Elu2a"),
                0x01d0 => defmt::write!(f, "Elu2b"),
                0x01d1 => defmt::write!(f, "Ifr2a"),
                0x01d2 => defmt::write!(f, "Ifr2b"),
                0x01d3 => defmt::write!(f, "Maxac2"),
                0x01d4 => defmt::write!(f, "Minac2"),
                0x01d5 => defmt::write!(f, "Clip2a"),
                0x01d6 => defmt::write!(f, "Clip2b"),
                0x01d7 => defmt::write!(f, "Macr2a"),
                0x01d8 => defmt::write!(f, "Macr2b"),
                0x01d9 => defmt::write!(f, "Ifc"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Instr7cfg2Aluop {
        #[inline(always)]
        fn from(val: u16) -> Instr7cfg2Aluop {
            Instr7cfg2Aluop::from_bits(val)
        }
    }
    impl From<Instr7cfg2Aluop> for u16 {
        #[inline(always)]
        fn from(val: Instr7cfg2Aluop) -> u16 {
            Instr7cfg2Aluop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Perf0cntsel {
        #[doc = "Total run count."]
        Run = 0x0,
        #[doc = "Total Commands Issued."]
        Cmd = 0x01,
        #[doc = "Total stall count (at sequencer issue)."]
        Stall = 0x02,
        #[doc = "NOOP ALU Op counter."]
        Noop = 0x03,
        #[doc = "Count cycles that ALU is active (not stalled), excluding NOOPs."]
        Aluactive = 0x04,
        #[doc = "Stalls caused by register and resource conflicts within the ALU."]
        Pipestall = 0x05,
        #[doc = "Count stall cycles caused by memory hazards."]
        Iofencestall = 0x06,
        #[doc = "Count stall cycles when accessing memory from load stream 0."]
        Load0stall = 0x07,
        #[doc = "Count stall cycles when accessing memory from load stream 1."]
        Load1stall = 0x08,
        #[doc = "Count stall cycles when writing memory from store stream."]
        Storestall = 0x09,
        #[doc = "Count cycles where any of previous 3 events is occurring."]
        Busstall = 0x0a,
        #[doc = "All stall cycles on load bus 0 AHB interface."]
        Load0ahbstall = 0x0b,
        #[doc = "All stall cycles on load bus 1 AHB interface."]
        Load1ahbstall = 0x0c,
        #[doc = "LOAD0 Fence Stall cycles."]
        Load0fencestall = 0x0d,
        #[doc = "LOAD1 Fence Stall cycles."]
        Load1fencestall = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Perf0cntsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Perf0cntsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Perf0cntsel {
        #[inline(always)]
        fn from(val: u8) -> Perf0cntsel {
            Perf0cntsel::from_bits(val)
        }
    }
    impl From<Perf0cntsel> for u8 {
        #[inline(always)]
        fn from(val: Perf0cntsel) -> u8 {
            Perf0cntsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Perf1cntsel {
        #[doc = "Total run count."]
        Run = 0x0,
        #[doc = "Total Commands Issued."]
        Cmd = 0x01,
        #[doc = "Total stall count (at sequencer issue)."]
        Stall = 0x02,
        #[doc = "NOOP ALU Op counter."]
        Noop = 0x03,
        #[doc = "Count cycles that ALU is active (not stalled), excluding NOOPs."]
        Aluactive = 0x04,
        #[doc = "Stalls caused by register and resource conflicts within the ALU."]
        Pipestall = 0x05,
        #[doc = "Count stall cycles caused by memory hazards."]
        Iofencestall = 0x06,
        #[doc = "Count stall cycles when accessing memory from load stream 0."]
        Load0stall = 0x07,
        #[doc = "Count stall cycles when accessing memory from load stream 1."]
        Load1stall = 0x08,
        #[doc = "Count stall cycles when writing memory from store stream."]
        Storestall = 0x09,
        #[doc = "Count cycles where any of previous 3 events is occurring."]
        Busstall = 0x0a,
        #[doc = "All stall cycles on load bus 0 AHB interface."]
        Load0ahbstall = 0x0b,
        #[doc = "All stall cycles on load bus 1 AHB interface."]
        Load1ahbstall = 0x0c,
        #[doc = "LOAD0 Fence Stall cycles."]
        Load0fencestall = 0x0d,
        #[doc = "LOAD1 Fence Stall cycles."]
        Load1fencestall = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Perf1cntsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Perf1cntsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Perf1cntsel {
        #[inline(always)]
        fn from(val: u8) -> Perf1cntsel {
            Perf1cntsel::from_bits(val)
        }
    }
    impl From<Perf1cntsel> for u8 {
        #[inline(always)]
        fn from(val: Perf1cntsel) -> u8 {
            Perf1cntsel::to_bits(val)
        }
    }
}
