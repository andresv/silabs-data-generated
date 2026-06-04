#[doc = "GPIO peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
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
    #[doc = "Port control."]
    #[inline(always)]
    pub const fn p_ctrl(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 48usize) as _) }
    }
    #[doc = "mode low."]
    #[inline(always)]
    pub const fn p_model(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 48usize) as _) }
    }
    #[doc = "mode high."]
    #[inline(always)]
    pub const fn p_modeh(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize + n * 48usize) as _) }
    }
    #[doc = "data out."]
    #[inline(always)]
    pub const fn p_dout(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 48usize) as _) }
    }
    #[doc = "data in."]
    #[inline(always)]
    pub const fn p_din(self, n: usize) -> crate::common::Reg<regs::PortDin, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize + n * 48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn gpiolockstatus(self) -> crate::common::Reg<regs::Gpiolockstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "A Bus allocation."]
    #[inline(always)]
    pub const fn abusalloc(self) -> crate::common::Reg<regs::Abusalloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "B Bus allocation."]
    #[inline(always)]
    pub const fn bbusalloc(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "CD Bus allocation."]
    #[inline(always)]
    pub const fn cdbusalloc(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "ABUS AODD0 Switch Register."]
    #[inline(always)]
    pub const fn aodd0switch(self) -> crate::common::Reg<regs::Aodd0switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "ABUS AODD1 Switch Register."]
    #[inline(always)]
    pub const fn aodd1switch(self) -> crate::common::Reg<regs::Aodd1switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "ABUS AEVEN0 Switch Register."]
    #[inline(always)]
    pub const fn aeven0switch(self) -> crate::common::Reg<regs::Aeven0switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "ABUS AEVEN1 Switch Register."]
    #[inline(always)]
    pub const fn aeven1switch(self) -> crate::common::Reg<regs::Aeven1switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "ABUS BODD0 Switch Register."]
    #[inline(always)]
    pub const fn bodd0switch(self) -> crate::common::Reg<regs::Bodd0switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "ABUS BODD1 Switch Register."]
    #[inline(always)]
    pub const fn bodd1switch(self) -> crate::common::Reg<regs::Bodd1switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "ABUS BEVEN0 Switch Register."]
    #[inline(always)]
    pub const fn beven0switch(self) -> crate::common::Reg<regs::Beven0switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "ABUS BEVEN1 Switch Register."]
    #[inline(always)]
    pub const fn beven1switch(self) -> crate::common::Reg<regs::Beven1switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "ABUS CDODD0 Switch Register."]
    #[inline(always)]
    pub const fn cdodd0switch(self) -> crate::common::Reg<regs::Cdodd0switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "ABUS CDODD1 Switch Register."]
    #[inline(always)]
    pub const fn cdodd1switch(self) -> crate::common::Reg<regs::Cdodd1switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "ABUS CDEVEN0 Switch Register."]
    #[inline(always)]
    pub const fn cdeven0switch(self) -> crate::common::Reg<regs::Cdeven0switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "ABUS CDEVEN1 Switch Register."]
    #[inline(always)]
    pub const fn cdeven1switch(self) -> crate::common::Reg<regs::Cdeven1switch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "External Interrupt Port Select Low."]
    #[inline(always)]
    pub const fn extipsell(self) -> crate::common::Reg<regs::Extipsell, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "External interrupt Port Select High."]
    #[inline(always)]
    pub const fn extipselh(self) -> crate::common::Reg<regs::Extipselh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low."]
    #[inline(always)]
    pub const fn extipinsell(self) -> crate::common::Reg<regs::Extipinsell, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High."]
    #[inline(always)]
    pub const fn extipinselh(self) -> crate::common::Reg<regs::Extipinselh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger."]
    #[inline(always)]
    pub const fn extirise(self) -> crate::common::Reg<regs::Extirise, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger."]
    #[inline(always)]
    pub const fn extifall(self) -> crate::common::Reg<regs::Extifall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4wuen(self) -> crate::common::Reg<regs::Em4wuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4wupol(self) -> crate::common::Reg<regs::Em4wupol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn dbgroutepen(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn traceroutepen(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn femroutepen(self) -> crate::common::Reg<regs::Femroutepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "ACMP0 pin enable."]
    #[inline(always)]
    pub const fn acmp0_routeen(self) -> crate::common::Reg<regs::Acmp0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "ACMPOUT port/pin select."]
    #[inline(always)]
    pub const fn acmp0_acmpoutroute(self) -> crate::common::Reg<regs::Acmp0Acmpoutroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "ACMP1 pin enable."]
    #[inline(always)]
    pub const fn acmp1_routeen(self) -> crate::common::Reg<regs::Acmp1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "ACMPOUT port/pin select."]
    #[inline(always)]
    pub const fn acmp1_acmpoutroute(self) -> crate::common::Reg<regs::Acmp1Acmpoutroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "CMU pin enable."]
    #[inline(always)]
    pub const fn cmu_routeen(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkin0route(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkout0route(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkout1route(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkout2route(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "DCDC pin enable."]
    #[inline(always)]
    pub const fn dcdc_routeen(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "EUSART0 pin enable."]
    #[inline(always)]
    pub const fn eusart0_routeen(self) -> crate::common::Reg<regs::Eusart0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn eusart0_csroute(self) -> crate::common::Reg<regs::Eusart0Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn eusart0_ctsroute(self) -> crate::common::Reg<regs::Eusart0Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn eusart0_rtsroute(self) -> crate::common::Reg<regs::Eusart0Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn eusart0_rxroute(self) -> crate::common::Reg<regs::Eusart0Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn eusart0_sclkroute(self) -> crate::common::Reg<regs::Eusart0Sclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn eusart0_txroute(self) -> crate::common::Reg<regs::Eusart0Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "EUSART1 pin enable."]
    #[inline(always)]
    pub const fn eusart1_routeen(self) -> crate::common::Reg<regs::Eusart1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn eusart1_csroute(self) -> crate::common::Reg<regs::Eusart1Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn eusart1_ctsroute(self) -> crate::common::Reg<regs::Eusart1Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn eusart1_rtsroute(self) -> crate::common::Reg<regs::Eusart1Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn eusart1_rxroute(self) -> crate::common::Reg<regs::Eusart1Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn eusart1_sclkroute(self) -> crate::common::Reg<regs::Eusart1Sclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn eusart1_txroute(self) -> crate::common::Reg<regs::Eusart1Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "EUSART2 pin enable."]
    #[inline(always)]
    pub const fn eusart2_routeen(self) -> crate::common::Reg<regs::Eusart2Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn eusart2_csroute(self) -> crate::common::Reg<regs::Eusart2Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn eusart2_ctsroute(self) -> crate::common::Reg<regs::Eusart2Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn eusart2_rtsroute(self) -> crate::common::Reg<regs::Eusart2Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn eusart2_rxroute(self) -> crate::common::Reg<regs::Eusart2Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn eusart2_sclkroute(self) -> crate::common::Reg<regs::Eusart2Sclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn eusart2_txroute(self) -> crate::common::Reg<regs::Eusart2Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "EUSART3 pin enable."]
    #[inline(always)]
    pub const fn eusart3_routeen(self) -> crate::common::Reg<regs::Eusart3Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn eusart3_csroute(self) -> crate::common::Reg<regs::Eusart3Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn eusart3_ctsroute(self) -> crate::common::Reg<regs::Eusart3Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn eusart3_rtsroute(self) -> crate::common::Reg<regs::Eusart3Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn eusart3_rxroute(self) -> crate::common::Reg<regs::Eusart3Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn eusart3_sclkroute(self) -> crate::common::Reg<regs::Eusart3Sclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn eusart3_txroute(self) -> crate::common::Reg<regs::Eusart3Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "EUSART4 pin enable."]
    #[inline(always)]
    pub const fn eusart4_routeen(self) -> crate::common::Reg<regs::Eusart4Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn eusart4_csroute(self) -> crate::common::Reg<regs::Eusart4Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn eusart4_ctsroute(self) -> crate::common::Reg<regs::Eusart4Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn eusart4_rtsroute(self) -> crate::common::Reg<regs::Eusart4Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn eusart4_rxroute(self) -> crate::common::Reg<regs::Eusart4Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn eusart4_sclkroute(self) -> crate::common::Reg<regs::Eusart4Sclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn eusart4_txroute(self) -> crate::common::Reg<regs::Eusart4Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "FRC pin enable."]
    #[inline(always)]
    pub const fn frc_routeen(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "DCLK port/pin select."]
    #[inline(always)]
    pub const fn frc_dclkroute(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "DFRAME port/pin select."]
    #[inline(always)]
    pub const fn frc_dframeroute(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "DOUT port/pin select."]
    #[inline(always)]
    pub const fn frc_doutroute(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "I2C0 pin enable."]
    #[inline(always)]
    pub const fn i2c0_routeen(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "SCL port/pin select."]
    #[inline(always)]
    pub const fn i2c0_sclroute(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "SDA port/pin select."]
    #[inline(always)]
    pub const fn i2c0_sdaroute(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "I2C1 pin enable."]
    #[inline(always)]
    pub const fn i2c1_routeen(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "SCL port/pin select."]
    #[inline(always)]
    pub const fn i2c1_sclroute(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "SDA port/pin select."]
    #[inline(always)]
    pub const fn i2c1_sdaroute(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "LESENSE pin enable."]
    #[inline(always)]
    pub const fn lesense_routeen(self) -> crate::common::Reg<regs::LesenseRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "CH0OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch0outroute(self) -> crate::common::Reg<regs::LesenseCh0outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "CH1OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch1outroute(self) -> crate::common::Reg<regs::LesenseCh1outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "CH2OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch2outroute(self) -> crate::common::Reg<regs::LesenseCh2outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "CH3OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch3outroute(self) -> crate::common::Reg<regs::LesenseCh3outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "CH4OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch4outroute(self) -> crate::common::Reg<regs::LesenseCh4outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "CH5OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch5outroute(self) -> crate::common::Reg<regs::LesenseCh5outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "CH6OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch6outroute(self) -> crate::common::Reg<regs::LesenseCh6outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0484usize) as _) }
    }
    #[doc = "CH7OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch7outroute(self) -> crate::common::Reg<regs::LesenseCh7outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0488usize) as _) }
    }
    #[doc = "CH8OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch8outroute(self) -> crate::common::Reg<regs::LesenseCh8outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048cusize) as _) }
    }
    #[doc = "CH9OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch9outroute(self) -> crate::common::Reg<regs::LesenseCh9outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "CH10OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch10outroute(self) -> crate::common::Reg<regs::LesenseCh10outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "CH11OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch11outroute(self) -> crate::common::Reg<regs::LesenseCh11outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "CH12OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch12outroute(self) -> crate::common::Reg<regs::LesenseCh12outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049cusize) as _) }
    }
    #[doc = "CH13OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch13outroute(self) -> crate::common::Reg<regs::LesenseCh13outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "CH14OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch14outroute(self) -> crate::common::Reg<regs::LesenseCh14outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a4usize) as _) }
    }
    #[doc = "CH15OUT port/pin select."]
    #[inline(always)]
    pub const fn lesense_ch15outroute(self) -> crate::common::Reg<regs::LesenseCh15outroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "LETIMER pin enable."]
    #[inline(always)]
    pub const fn letimer_routeen(self) -> crate::common::Reg<regs::LetimerRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select."]
    #[inline(always)]
    pub const fn letimer_out0route(self) -> crate::common::Reg<regs::LetimerOut0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select."]
    #[inline(always)]
    pub const fn letimer_out1route(self) -> crate::common::Reg<regs::LetimerOut1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "MODEM pin enable."]
    #[inline(always)]
    pub const fn modem_routeen(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "ANT0 port/pin select."]
    #[inline(always)]
    pub const fn modem_ant0route(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "ANT1 port/pin select."]
    #[inline(always)]
    pub const fn modem_ant1route(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select."]
    #[inline(always)]
    pub const fn modem_antrolloverroute(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "ANTRR0 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr0route(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr1route(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "ANTRR2 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr2route(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr3route(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "ANTRR4 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr4route(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr5route(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "ANTSWEN port/pin select."]
    #[inline(always)]
    pub const fn modem_antswenroute(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select."]
    #[inline(always)]
    pub const fn modem_antswusroute(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "ANTTRIG port/pin select."]
    #[inline(always)]
    pub const fn modem_anttrigroute(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select."]
    #[inline(always)]
    pub const fn modem_anttrigstoproute(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "DCLK port/pin select."]
    #[inline(always)]
    pub const fn modem_dclkroute(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "DIN port/pin select."]
    #[inline(always)]
    pub const fn modem_dinroute(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "DOUT port/pin select."]
    #[inline(always)]
    pub const fn modem_doutroute(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "S0IN port/pin select."]
    #[inline(always)]
    pub const fn pcnt0_s0inroute(self) -> crate::common::Reg<regs::Pcnt0S0inroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "S1IN port/pin select."]
    #[inline(always)]
    pub const fn pcnt0_s1inroute(self) -> crate::common::Reg<regs::Pcnt0S1inroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "PRS0 pin enable."]
    #[inline(always)]
    pub const fn prs0_routeen(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch0route(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch1route(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch2route(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch3route(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch4route(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch5route(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch6route(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch7route(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch8route(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch9route(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch10route(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch11route(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch0route(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "SYNCH1 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch1route(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "SYNCH2 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch2route(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch3route(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "BUFOUTREQINASYNC port/pin select."]
    #[inline(always)]
    pub const fn syxo0_bufoutreqinasyncroute(
        self,
    ) -> crate::common::Reg<regs::Syxo0Bufoutreqinasyncroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "TIMER0 pin enable."]
    #[inline(always)]
    pub const fn timer0_routeen(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cc0route(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cc1route(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cc2route(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cdti0route(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cdti1route(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cdti2route(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "TIMER1 pin enable."]
    #[inline(always)]
    pub const fn timer1_routeen(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cc0route(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cc1route(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cc2route(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cdti0route(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cdti1route(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cdti2route(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "TIMER2 pin enable."]
    #[inline(always)]
    pub const fn timer2_routeen(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cc0route(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cc1route(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cc2route(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cdti0route(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cdti1route(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cdti2route(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "TIMER3 pin enable."]
    #[inline(always)]
    pub const fn timer3_routeen(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cc0route(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cc1route(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cc2route(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cdti0route(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0638usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cdti1route(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cdti2route(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "TIMER4 pin enable."]
    #[inline(always)]
    pub const fn timer4_routeen(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cc0route(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x064cusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cc1route(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cc2route(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0654usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cdti0route(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0658usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cdti1route(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x065cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cdti2route(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "TIMER5 pin enable."]
    #[inline(always)]
    pub const fn timer5_routeen(self) -> crate::common::Reg<regs::Timer5Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0668usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer5_cc0route(self) -> crate::common::Reg<regs::Timer5Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x066cusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer5_cc1route(self) -> crate::common::Reg<regs::Timer5Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer5_cc2route(self) -> crate::common::Reg<regs::Timer5Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0674usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer5_cdti0route(self) -> crate::common::Reg<regs::Timer5Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0678usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer5_cdti1route(self) -> crate::common::Reg<regs::Timer5Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x067cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer5_cdti2route(self) -> crate::common::Reg<regs::Timer5Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "TIMER6 pin enable."]
    #[inline(always)]
    pub const fn timer6_routeen(self) -> crate::common::Reg<regs::Timer6Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0688usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer6_cc0route(self) -> crate::common::Reg<regs::Timer6Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x068cusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer6_cc1route(self) -> crate::common::Reg<regs::Timer6Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer6_cc2route(self) -> crate::common::Reg<regs::Timer6Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0694usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer6_cdti0route(self) -> crate::common::Reg<regs::Timer6Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0698usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer6_cdti1route(self) -> crate::common::Reg<regs::Timer6Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x069cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer6_cdti2route(self) -> crate::common::Reg<regs::Timer6Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "TIMER7 pin enable."]
    #[inline(always)]
    pub const fn timer7_routeen(self) -> crate::common::Reg<regs::Timer7Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a8usize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer7_cc0route(self) -> crate::common::Reg<regs::Timer7Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06acusize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer7_cc1route(self) -> crate::common::Reg<regs::Timer7Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b0usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer7_cc2route(self) -> crate::common::Reg<regs::Timer7Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer7_cdti0route(self) -> crate::common::Reg<regs::Timer7Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer7_cdti1route(self) -> crate::common::Reg<regs::Timer7Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06bcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer7_cdti2route(self) -> crate::common::Reg<regs::Timer7Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "USBVBUSSENSE port/pin select."]
    #[inline(always)]
    pub const fn usb_usbvbussenseroute(self) -> crate::common::Reg<regs::UsbUsbvbussenseroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ccusize) as _) }
    }
    #[doc = "Port control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_ctrl_set(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize + n * 48usize) as _) }
    }
    #[doc = "mode low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_model_set(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize + n * 48usize) as _) }
    }
    #[doc = "mode high. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_modeh_set(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::W> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize + n * 48usize) as _) }
    }
    #[doc = "data out. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_dout_set(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize + n * 48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "A Bus allocation. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn abusalloc_set(self) -> crate::common::Reg<regs::Abusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1220usize) as _) }
    }
    #[doc = "B Bus allocation. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bbusalloc_set(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1224usize) as _) }
    }
    #[doc = "CD Bus allocation. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cdbusalloc_set(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1228usize) as _) }
    }
    #[doc = "ABUS AODD0 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn aodd0switch_set(self) -> crate::common::Reg<regs::Aodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1230usize) as _) }
    }
    #[doc = "ABUS AODD1 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn aodd1switch_set(self) -> crate::common::Reg<regs::Aodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1234usize) as _) }
    }
    #[doc = "ABUS AEVEN0 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn aeven0switch_set(self) -> crate::common::Reg<regs::Aeven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1238usize) as _) }
    }
    #[doc = "ABUS AEVEN1 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn aeven1switch_set(self) -> crate::common::Reg<regs::Aeven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x123cusize) as _) }
    }
    #[doc = "ABUS BODD0 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bodd0switch_set(self) -> crate::common::Reg<regs::Bodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1240usize) as _) }
    }
    #[doc = "ABUS BODD1 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bodd1switch_set(self) -> crate::common::Reg<regs::Bodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1244usize) as _) }
    }
    #[doc = "ABUS BEVEN0 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn beven0switch_set(self) -> crate::common::Reg<regs::Beven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1248usize) as _) }
    }
    #[doc = "ABUS BEVEN1 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn beven1switch_set(self) -> crate::common::Reg<regs::Beven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x124cusize) as _) }
    }
    #[doc = "ABUS CDODD0 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cdodd0switch_set(self) -> crate::common::Reg<regs::Cdodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1250usize) as _) }
    }
    #[doc = "ABUS CDODD1 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cdodd1switch_set(self) -> crate::common::Reg<regs::Cdodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1254usize) as _) }
    }
    #[doc = "ABUS CDEVEN0 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cdeven0switch_set(self) -> crate::common::Reg<regs::Cdeven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1258usize) as _) }
    }
    #[doc = "ABUS CDEVEN1 Switch Register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cdeven1switch_set(self) -> crate::common::Reg<regs::Cdeven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x125cusize) as _) }
    }
    #[doc = "External Interrupt Port Select Low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipsell_set(self) -> crate::common::Reg<regs::Extipsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1300usize) as _) }
    }
    #[doc = "External interrupt Port Select High. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipselh_set(self) -> crate::common::Reg<regs::Extipselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1304usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipinsell_set(self) -> crate::common::Reg<regs::Extipinsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1308usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipinselh_set(self) -> crate::common::Reg<regs::Extipinselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x130cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extirise_set(self) -> crate::common::Reg<regs::Extirise, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1310usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extifall_set(self) -> crate::common::Reg<regs::Extifall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1314usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1320usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1324usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4wuen_set(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x132cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4wupol_set(self) -> crate::common::Reg<regs::Em4wupol, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1330usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dbgroutepen_set(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1340usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn traceroutepen_set(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1344usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn femroutepen_set(self) -> crate::common::Reg<regs::Femroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1348usize) as _) }
    }
    #[doc = "ACMP0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn acmp0_routeen_set(self) -> crate::common::Reg<regs::Acmp0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1350usize) as _) }
    }
    #[doc = "ACMPOUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn acmp0_acmpoutroute_set(self) -> crate::common::Reg<regs::Acmp0Acmpoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1354usize) as _) }
    }
    #[doc = "ACMP1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn acmp1_routeen_set(self) -> crate::common::Reg<regs::Acmp1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x135cusize) as _) }
    }
    #[doc = "ACMPOUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn acmp1_acmpoutroute_set(self) -> crate::common::Reg<regs::Acmp1Acmpoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1360usize) as _) }
    }
    #[doc = "CMU pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_routeen_set(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1368usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkin0route_set(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x136cusize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkout0route_set(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1370usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkout1route_set(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1374usize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkout2route_set(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1378usize) as _) }
    }
    #[doc = "DCDC pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dcdc_routeen_set(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1384usize) as _) }
    }
    #[doc = "EUSART0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_routeen_set(self) -> crate::common::Reg<regs::Eusart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1394usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_csroute_set(self) -> crate::common::Reg<regs::Eusart0Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1398usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_ctsroute_set(self) -> crate::common::Reg<regs::Eusart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x139cusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_rtsroute_set(self) -> crate::common::Reg<regs::Eusart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_rxroute_set(self) -> crate::common::Reg<regs::Eusart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_sclkroute_set(self) -> crate::common::Reg<regs::Eusart0Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart0_txroute_set(self) -> crate::common::Reg<regs::Eusart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13acusize) as _) }
    }
    #[doc = "EUSART1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_routeen_set(self) -> crate::common::Reg<regs::Eusart1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_csroute_set(self) -> crate::common::Reg<regs::Eusart1Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_ctsroute_set(self) -> crate::common::Reg<regs::Eusart1Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13bcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_rtsroute_set(self) -> crate::common::Reg<regs::Eusart1Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_rxroute_set(self) -> crate::common::Reg<regs::Eusart1Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_sclkroute_set(self) -> crate::common::Reg<regs::Eusart1Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart1_txroute_set(self) -> crate::common::Reg<regs::Eusart1Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13ccusize) as _) }
    }
    #[doc = "EUSART2 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_routeen_set(self) -> crate::common::Reg<regs::Eusart2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_csroute_set(self) -> crate::common::Reg<regs::Eusart2Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_ctsroute_set(self) -> crate::common::Reg<regs::Eusart2Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13dcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_rtsroute_set(self) -> crate::common::Reg<regs::Eusart2Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_rxroute_set(self) -> crate::common::Reg<regs::Eusart2Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_sclkroute_set(self) -> crate::common::Reg<regs::Eusart2Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart2_txroute_set(self) -> crate::common::Reg<regs::Eusart2Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13ecusize) as _) }
    }
    #[doc = "EUSART3 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_routeen_set(self) -> crate::common::Reg<regs::Eusart3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_csroute_set(self) -> crate::common::Reg<regs::Eusart3Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_ctsroute_set(self) -> crate::common::Reg<regs::Eusart3Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13fcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_rtsroute_set(self) -> crate::common::Reg<regs::Eusart3Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1400usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_rxroute_set(self) -> crate::common::Reg<regs::Eusart3Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1404usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_sclkroute_set(self) -> crate::common::Reg<regs::Eusart3Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1408usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart3_txroute_set(self) -> crate::common::Reg<regs::Eusart3Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x140cusize) as _) }
    }
    #[doc = "EUSART4 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_routeen_set(self) -> crate::common::Reg<regs::Eusart4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1414usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_csroute_set(self) -> crate::common::Reg<regs::Eusart4Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1418usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_ctsroute_set(self) -> crate::common::Reg<regs::Eusart4Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x141cusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_rtsroute_set(self) -> crate::common::Reg<regs::Eusart4Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1420usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_rxroute_set(self) -> crate::common::Reg<regs::Eusart4Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1424usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_sclkroute_set(self) -> crate::common::Reg<regs::Eusart4Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1428usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn eusart4_txroute_set(self) -> crate::common::Reg<regs::Eusart4Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x142cusize) as _) }
    }
    #[doc = "FRC pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_routeen_set(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1434usize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_dclkroute_set(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1438usize) as _) }
    }
    #[doc = "DFRAME port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_dframeroute_set(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x143cusize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_doutroute_set(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1440usize) as _) }
    }
    #[doc = "I2C0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c0_routeen_set(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1448usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c0_sclroute_set(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x144cusize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c0_sdaroute_set(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1450usize) as _) }
    }
    #[doc = "I2C1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c1_routeen_set(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1458usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c1_sclroute_set(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x145cusize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c1_sdaroute_set(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1460usize) as _) }
    }
    #[doc = "LESENSE pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_routeen_set(self) -> crate::common::Reg<regs::LesenseRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1468usize) as _) }
    }
    #[doc = "CH0OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch0outroute_set(self) -> crate::common::Reg<regs::LesenseCh0outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x146cusize) as _) }
    }
    #[doc = "CH1OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch1outroute_set(self) -> crate::common::Reg<regs::LesenseCh1outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1470usize) as _) }
    }
    #[doc = "CH2OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch2outroute_set(self) -> crate::common::Reg<regs::LesenseCh2outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1474usize) as _) }
    }
    #[doc = "CH3OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch3outroute_set(self) -> crate::common::Reg<regs::LesenseCh3outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1478usize) as _) }
    }
    #[doc = "CH4OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch4outroute_set(self) -> crate::common::Reg<regs::LesenseCh4outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x147cusize) as _) }
    }
    #[doc = "CH5OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch5outroute_set(self) -> crate::common::Reg<regs::LesenseCh5outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1480usize) as _) }
    }
    #[doc = "CH6OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch6outroute_set(self) -> crate::common::Reg<regs::LesenseCh6outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1484usize) as _) }
    }
    #[doc = "CH7OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch7outroute_set(self) -> crate::common::Reg<regs::LesenseCh7outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1488usize) as _) }
    }
    #[doc = "CH8OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch8outroute_set(self) -> crate::common::Reg<regs::LesenseCh8outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x148cusize) as _) }
    }
    #[doc = "CH9OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch9outroute_set(self) -> crate::common::Reg<regs::LesenseCh9outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1490usize) as _) }
    }
    #[doc = "CH10OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch10outroute_set(self) -> crate::common::Reg<regs::LesenseCh10outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1494usize) as _) }
    }
    #[doc = "CH11OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch11outroute_set(self) -> crate::common::Reg<regs::LesenseCh11outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1498usize) as _) }
    }
    #[doc = "CH12OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch12outroute_set(self) -> crate::common::Reg<regs::LesenseCh12outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x149cusize) as _) }
    }
    #[doc = "CH13OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch13outroute_set(self) -> crate::common::Reg<regs::LesenseCh13outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a0usize) as _) }
    }
    #[doc = "CH14OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch14outroute_set(self) -> crate::common::Reg<regs::LesenseCh14outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a4usize) as _) }
    }
    #[doc = "CH15OUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lesense_ch15outroute_set(self) -> crate::common::Reg<regs::LesenseCh15outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a8usize) as _) }
    }
    #[doc = "LETIMER pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn letimer_routeen_set(self) -> crate::common::Reg<regs::LetimerRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn letimer_out0route_set(self) -> crate::common::Reg<regs::LetimerOut0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn letimer_out1route_set(self) -> crate::common::Reg<regs::LetimerOut1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14b8usize) as _) }
    }
    #[doc = "MODEM pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_routeen_set(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14c0usize) as _) }
    }
    #[doc = "ANT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_ant0route_set(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14c4usize) as _) }
    }
    #[doc = "ANT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_ant1route_set(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14c8usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrolloverroute_set(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14ccusize) as _) }
    }
    #[doc = "ANTRR0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr0route_set(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14d0usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr1route_set(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14d4usize) as _) }
    }
    #[doc = "ANTRR2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr2route_set(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14d8usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr3route_set(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14dcusize) as _) }
    }
    #[doc = "ANTRR4 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr4route_set(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14e0usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr5route_set(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14e4usize) as _) }
    }
    #[doc = "ANTSWEN port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antswenroute_set(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14e8usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antswusroute_set(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14ecusize) as _) }
    }
    #[doc = "ANTTRIG port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_anttrigroute_set(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14f0usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute_set(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14f4usize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_dclkroute_set(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14f8usize) as _) }
    }
    #[doc = "DIN port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_dinroute_set(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14fcusize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_doutroute_set(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1500usize) as _) }
    }
    #[doc = "S0IN port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pcnt0_s0inroute_set(self) -> crate::common::Reg<regs::Pcnt0S0inroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x150cusize) as _) }
    }
    #[doc = "S1IN port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pcnt0_s1inroute_set(self) -> crate::common::Reg<regs::Pcnt0S1inroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1510usize) as _) }
    }
    #[doc = "PRS0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_routeen_set(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1518usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch0route_set(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x151cusize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch1route_set(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1520usize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch2route_set(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1524usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch3route_set(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1528usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch4route_set(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x152cusize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch5route_set(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1530usize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch6route_set(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1534usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch7route_set(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1538usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch8route_set(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x153cusize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch9route_set(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1540usize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch10route_set(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1544usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch11route_set(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1548usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch0route_set(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x154cusize) as _) }
    }
    #[doc = "SYNCH1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch1route_set(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1550usize) as _) }
    }
    #[doc = "SYNCH2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch2route_set(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1554usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch3route_set(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1558usize) as _) }
    }
    #[doc = "BUFOUTREQINASYNC port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn syxo0_bufoutreqinasyncroute_set(
        self,
    ) -> crate::common::Reg<regs::Syxo0Bufoutreqinasyncroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15c0usize) as _) }
    }
    #[doc = "TIMER0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_routeen_set(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15c8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cc0route_set(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15ccusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cc1route_set(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15d0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cc2route_set(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15d4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cdti0route_set(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15d8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cdti1route_set(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15dcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cdti2route_set(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15e0usize) as _) }
    }
    #[doc = "TIMER1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_routeen_set(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15e8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cc0route_set(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15ecusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cc1route_set(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15f0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cc2route_set(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15f4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cdti0route_set(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15f8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cdti1route_set(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15fcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cdti2route_set(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1600usize) as _) }
    }
    #[doc = "TIMER2 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_routeen_set(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1608usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cc0route_set(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x160cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cc1route_set(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1610usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cc2route_set(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1614usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cdti0route_set(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1618usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cdti1route_set(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x161cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cdti2route_set(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1620usize) as _) }
    }
    #[doc = "TIMER3 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_routeen_set(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1628usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cc0route_set(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x162cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cc1route_set(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1630usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cc2route_set(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1634usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cdti0route_set(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1638usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cdti1route_set(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x163cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cdti2route_set(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1640usize) as _) }
    }
    #[doc = "TIMER4 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_routeen_set(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1648usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cc0route_set(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x164cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cc1route_set(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1650usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cc2route_set(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1654usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cdti0route_set(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1658usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cdti1route_set(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x165cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cdti2route_set(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1660usize) as _) }
    }
    #[doc = "TIMER5 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_routeen_set(self) -> crate::common::Reg<regs::Timer5Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1668usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_cc0route_set(self) -> crate::common::Reg<regs::Timer5Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x166cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_cc1route_set(self) -> crate::common::Reg<regs::Timer5Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1670usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_cc2route_set(self) -> crate::common::Reg<regs::Timer5Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1674usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_cdti0route_set(self) -> crate::common::Reg<regs::Timer5Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1678usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_cdti1route_set(self) -> crate::common::Reg<regs::Timer5Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x167cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer5_cdti2route_set(self) -> crate::common::Reg<regs::Timer5Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1680usize) as _) }
    }
    #[doc = "TIMER6 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_routeen_set(self) -> crate::common::Reg<regs::Timer6Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1688usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_cc0route_set(self) -> crate::common::Reg<regs::Timer6Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x168cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_cc1route_set(self) -> crate::common::Reg<regs::Timer6Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1690usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_cc2route_set(self) -> crate::common::Reg<regs::Timer6Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1694usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_cdti0route_set(self) -> crate::common::Reg<regs::Timer6Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1698usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_cdti1route_set(self) -> crate::common::Reg<regs::Timer6Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x169cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer6_cdti2route_set(self) -> crate::common::Reg<regs::Timer6Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16a0usize) as _) }
    }
    #[doc = "TIMER7 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_routeen_set(self) -> crate::common::Reg<regs::Timer7Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16a8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_cc0route_set(self) -> crate::common::Reg<regs::Timer7Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16acusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_cc1route_set(self) -> crate::common::Reg<regs::Timer7Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16b0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_cc2route_set(self) -> crate::common::Reg<regs::Timer7Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16b4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_cdti0route_set(self) -> crate::common::Reg<regs::Timer7Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16b8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_cdti1route_set(self) -> crate::common::Reg<regs::Timer7Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16bcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer7_cdti2route_set(self) -> crate::common::Reg<regs::Timer7Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16c0usize) as _) }
    }
    #[doc = "USBVBUSSENSE port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usb_usbvbussenseroute_set(self) -> crate::common::Reg<regs::UsbUsbvbussenseroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16ccusize) as _) }
    }
    #[doc = "Port control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_ctrl_clr(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize + n * 48usize) as _) }
    }
    #[doc = "mode low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_model_clr(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize + n * 48usize) as _) }
    }
    #[doc = "mode high. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_modeh_clr(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::W> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize + n * 48usize) as _) }
    }
    #[doc = "data out. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_dout_clr(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize + n * 48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "A Bus allocation. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn abusalloc_clr(self) -> crate::common::Reg<regs::Abusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2220usize) as _) }
    }
    #[doc = "B Bus allocation. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bbusalloc_clr(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2224usize) as _) }
    }
    #[doc = "CD Bus allocation. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cdbusalloc_clr(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2228usize) as _) }
    }
    #[doc = "ABUS AODD0 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn aodd0switch_clr(self) -> crate::common::Reg<regs::Aodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2230usize) as _) }
    }
    #[doc = "ABUS AODD1 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn aodd1switch_clr(self) -> crate::common::Reg<regs::Aodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2234usize) as _) }
    }
    #[doc = "ABUS AEVEN0 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn aeven0switch_clr(self) -> crate::common::Reg<regs::Aeven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2238usize) as _) }
    }
    #[doc = "ABUS AEVEN1 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn aeven1switch_clr(self) -> crate::common::Reg<regs::Aeven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x223cusize) as _) }
    }
    #[doc = "ABUS BODD0 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bodd0switch_clr(self) -> crate::common::Reg<regs::Bodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2240usize) as _) }
    }
    #[doc = "ABUS BODD1 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bodd1switch_clr(self) -> crate::common::Reg<regs::Bodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2244usize) as _) }
    }
    #[doc = "ABUS BEVEN0 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn beven0switch_clr(self) -> crate::common::Reg<regs::Beven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2248usize) as _) }
    }
    #[doc = "ABUS BEVEN1 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn beven1switch_clr(self) -> crate::common::Reg<regs::Beven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x224cusize) as _) }
    }
    #[doc = "ABUS CDODD0 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cdodd0switch_clr(self) -> crate::common::Reg<regs::Cdodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2250usize) as _) }
    }
    #[doc = "ABUS CDODD1 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cdodd1switch_clr(self) -> crate::common::Reg<regs::Cdodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2254usize) as _) }
    }
    #[doc = "ABUS CDEVEN0 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cdeven0switch_clr(self) -> crate::common::Reg<regs::Cdeven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2258usize) as _) }
    }
    #[doc = "ABUS CDEVEN1 Switch Register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cdeven1switch_clr(self) -> crate::common::Reg<regs::Cdeven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x225cusize) as _) }
    }
    #[doc = "External Interrupt Port Select Low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipsell_clr(self) -> crate::common::Reg<regs::Extipsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize) as _) }
    }
    #[doc = "External interrupt Port Select High. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipselh_clr(self) -> crate::common::Reg<regs::Extipselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2304usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipinsell_clr(self) -> crate::common::Reg<regs::Extipinsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2308usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipinselh_clr(self) -> crate::common::Reg<regs::Extipinselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x230cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extirise_clr(self) -> crate::common::Reg<regs::Extirise, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2310usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extifall_clr(self) -> crate::common::Reg<regs::Extifall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2314usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2320usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2324usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4wuen_clr(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x232cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4wupol_clr(self) -> crate::common::Reg<regs::Em4wupol, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2330usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dbgroutepen_clr(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2340usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn traceroutepen_clr(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2344usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn femroutepen_clr(self) -> crate::common::Reg<regs::Femroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2348usize) as _) }
    }
    #[doc = "ACMP0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn acmp0_routeen_clr(self) -> crate::common::Reg<regs::Acmp0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2350usize) as _) }
    }
    #[doc = "ACMPOUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn acmp0_acmpoutroute_clr(self) -> crate::common::Reg<regs::Acmp0Acmpoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2354usize) as _) }
    }
    #[doc = "ACMP1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn acmp1_routeen_clr(self) -> crate::common::Reg<regs::Acmp1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x235cusize) as _) }
    }
    #[doc = "ACMPOUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn acmp1_acmpoutroute_clr(self) -> crate::common::Reg<regs::Acmp1Acmpoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2360usize) as _) }
    }
    #[doc = "CMU pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_routeen_clr(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2368usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkin0route_clr(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x236cusize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkout0route_clr(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2370usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkout1route_clr(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2374usize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkout2route_clr(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2378usize) as _) }
    }
    #[doc = "DCDC pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dcdc_routeen_clr(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2384usize) as _) }
    }
    #[doc = "EUSART0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_routeen_clr(self) -> crate::common::Reg<regs::Eusart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2394usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_csroute_clr(self) -> crate::common::Reg<regs::Eusart0Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2398usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_ctsroute_clr(self) -> crate::common::Reg<regs::Eusart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x239cusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_rtsroute_clr(self) -> crate::common::Reg<regs::Eusart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23a0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_rxroute_clr(self) -> crate::common::Reg<regs::Eusart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23a4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_sclkroute_clr(self) -> crate::common::Reg<regs::Eusart0Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23a8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart0_txroute_clr(self) -> crate::common::Reg<regs::Eusart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23acusize) as _) }
    }
    #[doc = "EUSART1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_routeen_clr(self) -> crate::common::Reg<regs::Eusart1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23b4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_csroute_clr(self) -> crate::common::Reg<regs::Eusart1Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23b8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_ctsroute_clr(self) -> crate::common::Reg<regs::Eusart1Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23bcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_rtsroute_clr(self) -> crate::common::Reg<regs::Eusart1Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23c0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_rxroute_clr(self) -> crate::common::Reg<regs::Eusart1Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23c4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_sclkroute_clr(self) -> crate::common::Reg<regs::Eusart1Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23c8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart1_txroute_clr(self) -> crate::common::Reg<regs::Eusart1Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23ccusize) as _) }
    }
    #[doc = "EUSART2 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_routeen_clr(self) -> crate::common::Reg<regs::Eusart2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23d4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_csroute_clr(self) -> crate::common::Reg<regs::Eusart2Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23d8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_ctsroute_clr(self) -> crate::common::Reg<regs::Eusart2Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23dcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_rtsroute_clr(self) -> crate::common::Reg<regs::Eusart2Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23e0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_rxroute_clr(self) -> crate::common::Reg<regs::Eusart2Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23e4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_sclkroute_clr(self) -> crate::common::Reg<regs::Eusart2Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23e8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart2_txroute_clr(self) -> crate::common::Reg<regs::Eusart2Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23ecusize) as _) }
    }
    #[doc = "EUSART3 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_routeen_clr(self) -> crate::common::Reg<regs::Eusart3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23f4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_csroute_clr(self) -> crate::common::Reg<regs::Eusart3Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23f8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_ctsroute_clr(self) -> crate::common::Reg<regs::Eusart3Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x23fcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_rtsroute_clr(self) -> crate::common::Reg<regs::Eusart3Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_rxroute_clr(self) -> crate::common::Reg<regs::Eusart3Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2404usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_sclkroute_clr(self) -> crate::common::Reg<regs::Eusart3Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2408usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart3_txroute_clr(self) -> crate::common::Reg<regs::Eusart3Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x240cusize) as _) }
    }
    #[doc = "EUSART4 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_routeen_clr(self) -> crate::common::Reg<regs::Eusart4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2414usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_csroute_clr(self) -> crate::common::Reg<regs::Eusart4Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2418usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_ctsroute_clr(self) -> crate::common::Reg<regs::Eusart4Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x241cusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_rtsroute_clr(self) -> crate::common::Reg<regs::Eusart4Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2420usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_rxroute_clr(self) -> crate::common::Reg<regs::Eusart4Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2424usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_sclkroute_clr(self) -> crate::common::Reg<regs::Eusart4Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2428usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn eusart4_txroute_clr(self) -> crate::common::Reg<regs::Eusart4Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x242cusize) as _) }
    }
    #[doc = "FRC pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_routeen_clr(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2434usize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_dclkroute_clr(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2438usize) as _) }
    }
    #[doc = "DFRAME port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_dframeroute_clr(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x243cusize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_doutroute_clr(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2440usize) as _) }
    }
    #[doc = "I2C0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c0_routeen_clr(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2448usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c0_sclroute_clr(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x244cusize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c0_sdaroute_clr(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2450usize) as _) }
    }
    #[doc = "I2C1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c1_routeen_clr(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2458usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c1_sclroute_clr(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x245cusize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c1_sdaroute_clr(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2460usize) as _) }
    }
    #[doc = "LESENSE pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_routeen_clr(self) -> crate::common::Reg<regs::LesenseRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2468usize) as _) }
    }
    #[doc = "CH0OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch0outroute_clr(self) -> crate::common::Reg<regs::LesenseCh0outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x246cusize) as _) }
    }
    #[doc = "CH1OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch1outroute_clr(self) -> crate::common::Reg<regs::LesenseCh1outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2470usize) as _) }
    }
    #[doc = "CH2OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch2outroute_clr(self) -> crate::common::Reg<regs::LesenseCh2outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2474usize) as _) }
    }
    #[doc = "CH3OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch3outroute_clr(self) -> crate::common::Reg<regs::LesenseCh3outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2478usize) as _) }
    }
    #[doc = "CH4OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch4outroute_clr(self) -> crate::common::Reg<regs::LesenseCh4outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x247cusize) as _) }
    }
    #[doc = "CH5OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch5outroute_clr(self) -> crate::common::Reg<regs::LesenseCh5outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2480usize) as _) }
    }
    #[doc = "CH6OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch6outroute_clr(self) -> crate::common::Reg<regs::LesenseCh6outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2484usize) as _) }
    }
    #[doc = "CH7OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch7outroute_clr(self) -> crate::common::Reg<regs::LesenseCh7outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2488usize) as _) }
    }
    #[doc = "CH8OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch8outroute_clr(self) -> crate::common::Reg<regs::LesenseCh8outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x248cusize) as _) }
    }
    #[doc = "CH9OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch9outroute_clr(self) -> crate::common::Reg<regs::LesenseCh9outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2490usize) as _) }
    }
    #[doc = "CH10OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch10outroute_clr(self) -> crate::common::Reg<regs::LesenseCh10outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2494usize) as _) }
    }
    #[doc = "CH11OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch11outroute_clr(self) -> crate::common::Reg<regs::LesenseCh11outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2498usize) as _) }
    }
    #[doc = "CH12OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch12outroute_clr(self) -> crate::common::Reg<regs::LesenseCh12outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x249cusize) as _) }
    }
    #[doc = "CH13OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch13outroute_clr(self) -> crate::common::Reg<regs::LesenseCh13outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24a0usize) as _) }
    }
    #[doc = "CH14OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch14outroute_clr(self) -> crate::common::Reg<regs::LesenseCh14outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24a4usize) as _) }
    }
    #[doc = "CH15OUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lesense_ch15outroute_clr(self) -> crate::common::Reg<regs::LesenseCh15outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24a8usize) as _) }
    }
    #[doc = "LETIMER pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn letimer_routeen_clr(self) -> crate::common::Reg<regs::LetimerRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn letimer_out0route_clr(self) -> crate::common::Reg<regs::LetimerOut0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn letimer_out1route_clr(self) -> crate::common::Reg<regs::LetimerOut1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24b8usize) as _) }
    }
    #[doc = "MODEM pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_routeen_clr(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24c0usize) as _) }
    }
    #[doc = "ANT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_ant0route_clr(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24c4usize) as _) }
    }
    #[doc = "ANT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_ant1route_clr(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24c8usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrolloverroute_clr(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24ccusize) as _) }
    }
    #[doc = "ANTRR0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr0route_clr(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24d0usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr1route_clr(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24d4usize) as _) }
    }
    #[doc = "ANTRR2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr2route_clr(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24d8usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr3route_clr(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24dcusize) as _) }
    }
    #[doc = "ANTRR4 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr4route_clr(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24e0usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr5route_clr(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24e4usize) as _) }
    }
    #[doc = "ANTSWEN port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antswenroute_clr(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24e8usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antswusroute_clr(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24ecusize) as _) }
    }
    #[doc = "ANTTRIG port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_anttrigroute_clr(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24f0usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute_clr(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24f4usize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_dclkroute_clr(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24f8usize) as _) }
    }
    #[doc = "DIN port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_dinroute_clr(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24fcusize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_doutroute_clr(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2500usize) as _) }
    }
    #[doc = "S0IN port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pcnt0_s0inroute_clr(self) -> crate::common::Reg<regs::Pcnt0S0inroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x250cusize) as _) }
    }
    #[doc = "S1IN port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pcnt0_s1inroute_clr(self) -> crate::common::Reg<regs::Pcnt0S1inroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2510usize) as _) }
    }
    #[doc = "PRS0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_routeen_clr(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2518usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch0route_clr(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x251cusize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch1route_clr(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2520usize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch2route_clr(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2524usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch3route_clr(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2528usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch4route_clr(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x252cusize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch5route_clr(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2530usize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch6route_clr(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2534usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch7route_clr(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2538usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch8route_clr(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x253cusize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch9route_clr(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2540usize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch10route_clr(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2544usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch11route_clr(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2548usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch0route_clr(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x254cusize) as _) }
    }
    #[doc = "SYNCH1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch1route_clr(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2550usize) as _) }
    }
    #[doc = "SYNCH2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch2route_clr(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2554usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch3route_clr(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2558usize) as _) }
    }
    #[doc = "BUFOUTREQINASYNC port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn syxo0_bufoutreqinasyncroute_clr(
        self,
    ) -> crate::common::Reg<regs::Syxo0Bufoutreqinasyncroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25c0usize) as _) }
    }
    #[doc = "TIMER0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_routeen_clr(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25c8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cc0route_clr(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25ccusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cc1route_clr(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25d0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cc2route_clr(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25d4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cdti0route_clr(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25d8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cdti1route_clr(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25dcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cdti2route_clr(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25e0usize) as _) }
    }
    #[doc = "TIMER1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_routeen_clr(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25e8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cc0route_clr(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25ecusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cc1route_clr(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25f0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cc2route_clr(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25f4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cdti0route_clr(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25f8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cdti1route_clr(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25fcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cdti2route_clr(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2600usize) as _) }
    }
    #[doc = "TIMER2 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_routeen_clr(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2608usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cc0route_clr(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x260cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cc1route_clr(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2610usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cc2route_clr(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2614usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cdti0route_clr(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2618usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cdti1route_clr(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x261cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cdti2route_clr(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2620usize) as _) }
    }
    #[doc = "TIMER3 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_routeen_clr(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2628usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cc0route_clr(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x262cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cc1route_clr(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2630usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cc2route_clr(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2634usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cdti0route_clr(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2638usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cdti1route_clr(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x263cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cdti2route_clr(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2640usize) as _) }
    }
    #[doc = "TIMER4 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_routeen_clr(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2648usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cc0route_clr(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x264cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cc1route_clr(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2650usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cc2route_clr(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2654usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cdti0route_clr(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2658usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cdti1route_clr(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x265cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cdti2route_clr(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2660usize) as _) }
    }
    #[doc = "TIMER5 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_routeen_clr(self) -> crate::common::Reg<regs::Timer5Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2668usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_cc0route_clr(self) -> crate::common::Reg<regs::Timer5Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x266cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_cc1route_clr(self) -> crate::common::Reg<regs::Timer5Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2670usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_cc2route_clr(self) -> crate::common::Reg<regs::Timer5Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2674usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_cdti0route_clr(self) -> crate::common::Reg<regs::Timer5Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2678usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_cdti1route_clr(self) -> crate::common::Reg<regs::Timer5Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x267cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer5_cdti2route_clr(self) -> crate::common::Reg<regs::Timer5Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2680usize) as _) }
    }
    #[doc = "TIMER6 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_routeen_clr(self) -> crate::common::Reg<regs::Timer6Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2688usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_cc0route_clr(self) -> crate::common::Reg<regs::Timer6Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x268cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_cc1route_clr(self) -> crate::common::Reg<regs::Timer6Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2690usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_cc2route_clr(self) -> crate::common::Reg<regs::Timer6Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2694usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_cdti0route_clr(self) -> crate::common::Reg<regs::Timer6Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2698usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_cdti1route_clr(self) -> crate::common::Reg<regs::Timer6Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x269cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer6_cdti2route_clr(self) -> crate::common::Reg<regs::Timer6Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26a0usize) as _) }
    }
    #[doc = "TIMER7 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_routeen_clr(self) -> crate::common::Reg<regs::Timer7Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26a8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_cc0route_clr(self) -> crate::common::Reg<regs::Timer7Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26acusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_cc1route_clr(self) -> crate::common::Reg<regs::Timer7Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26b0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_cc2route_clr(self) -> crate::common::Reg<regs::Timer7Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26b4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_cdti0route_clr(self) -> crate::common::Reg<regs::Timer7Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26b8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_cdti1route_clr(self) -> crate::common::Reg<regs::Timer7Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26bcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer7_cdti2route_clr(self) -> crate::common::Reg<regs::Timer7Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26c0usize) as _) }
    }
    #[doc = "USBVBUSSENSE port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usb_usbvbussenseroute_clr(self) -> crate::common::Reg<regs::UsbUsbvbussenseroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26ccusize) as _) }
    }
    #[doc = "Port control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_ctrl_tgl(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize + n * 48usize) as _) }
    }
    #[doc = "mode low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_model_tgl(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize + n * 48usize) as _) }
    }
    #[doc = "mode high. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_modeh_tgl(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::W> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize + n * 48usize) as _) }
    }
    #[doc = "data out. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_dout_tgl(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize + n * 48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize) as _) }
    }
    #[doc = "A Bus allocation. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn abusalloc_tgl(self) -> crate::common::Reg<regs::Abusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3220usize) as _) }
    }
    #[doc = "B Bus allocation. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bbusalloc_tgl(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3224usize) as _) }
    }
    #[doc = "CD Bus allocation. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cdbusalloc_tgl(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3228usize) as _) }
    }
    #[doc = "ABUS AODD0 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn aodd0switch_tgl(self) -> crate::common::Reg<regs::Aodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3230usize) as _) }
    }
    #[doc = "ABUS AODD1 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn aodd1switch_tgl(self) -> crate::common::Reg<regs::Aodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3234usize) as _) }
    }
    #[doc = "ABUS AEVEN0 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn aeven0switch_tgl(self) -> crate::common::Reg<regs::Aeven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3238usize) as _) }
    }
    #[doc = "ABUS AEVEN1 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn aeven1switch_tgl(self) -> crate::common::Reg<regs::Aeven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x323cusize) as _) }
    }
    #[doc = "ABUS BODD0 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bodd0switch_tgl(self) -> crate::common::Reg<regs::Bodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3240usize) as _) }
    }
    #[doc = "ABUS BODD1 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bodd1switch_tgl(self) -> crate::common::Reg<regs::Bodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3244usize) as _) }
    }
    #[doc = "ABUS BEVEN0 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn beven0switch_tgl(self) -> crate::common::Reg<regs::Beven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3248usize) as _) }
    }
    #[doc = "ABUS BEVEN1 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn beven1switch_tgl(self) -> crate::common::Reg<regs::Beven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x324cusize) as _) }
    }
    #[doc = "ABUS CDODD0 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cdodd0switch_tgl(self) -> crate::common::Reg<regs::Cdodd0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3250usize) as _) }
    }
    #[doc = "ABUS CDODD1 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cdodd1switch_tgl(self) -> crate::common::Reg<regs::Cdodd1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3254usize) as _) }
    }
    #[doc = "ABUS CDEVEN0 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cdeven0switch_tgl(self) -> crate::common::Reg<regs::Cdeven0switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3258usize) as _) }
    }
    #[doc = "ABUS CDEVEN1 Switch Register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cdeven1switch_tgl(self) -> crate::common::Reg<regs::Cdeven1switch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x325cusize) as _) }
    }
    #[doc = "External Interrupt Port Select Low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipsell_tgl(self) -> crate::common::Reg<regs::Extipsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3300usize) as _) }
    }
    #[doc = "External interrupt Port Select High. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipselh_tgl(self) -> crate::common::Reg<regs::Extipselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3304usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipinsell_tgl(self) -> crate::common::Reg<regs::Extipinsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3308usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipinselh_tgl(self) -> crate::common::Reg<regs::Extipinselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x330cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extirise_tgl(self) -> crate::common::Reg<regs::Extirise, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3310usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extifall_tgl(self) -> crate::common::Reg<regs::Extifall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3314usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3320usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3324usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4wuen_tgl(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x332cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4wupol_tgl(self) -> crate::common::Reg<regs::Em4wupol, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3330usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dbgroutepen_tgl(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3340usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn traceroutepen_tgl(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3344usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn femroutepen_tgl(self) -> crate::common::Reg<regs::Femroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3348usize) as _) }
    }
    #[doc = "ACMP0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn acmp0_routeen_tgl(self) -> crate::common::Reg<regs::Acmp0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3350usize) as _) }
    }
    #[doc = "ACMPOUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn acmp0_acmpoutroute_tgl(self) -> crate::common::Reg<regs::Acmp0Acmpoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3354usize) as _) }
    }
    #[doc = "ACMP1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn acmp1_routeen_tgl(self) -> crate::common::Reg<regs::Acmp1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x335cusize) as _) }
    }
    #[doc = "ACMPOUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn acmp1_acmpoutroute_tgl(self) -> crate::common::Reg<regs::Acmp1Acmpoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3360usize) as _) }
    }
    #[doc = "CMU pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_routeen_tgl(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3368usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkin0route_tgl(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x336cusize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkout0route_tgl(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3370usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkout1route_tgl(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3374usize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkout2route_tgl(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3378usize) as _) }
    }
    #[doc = "DCDC pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dcdc_routeen_tgl(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3384usize) as _) }
    }
    #[doc = "EUSART0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_routeen_tgl(self) -> crate::common::Reg<regs::Eusart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3394usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_csroute_tgl(self) -> crate::common::Reg<regs::Eusart0Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3398usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_ctsroute_tgl(self) -> crate::common::Reg<regs::Eusart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x339cusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_rtsroute_tgl(self) -> crate::common::Reg<regs::Eusart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33a0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_rxroute_tgl(self) -> crate::common::Reg<regs::Eusart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33a4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_sclkroute_tgl(self) -> crate::common::Reg<regs::Eusart0Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33a8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart0_txroute_tgl(self) -> crate::common::Reg<regs::Eusart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33acusize) as _) }
    }
    #[doc = "EUSART1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_routeen_tgl(self) -> crate::common::Reg<regs::Eusart1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33b4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_csroute_tgl(self) -> crate::common::Reg<regs::Eusart1Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33b8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_ctsroute_tgl(self) -> crate::common::Reg<regs::Eusart1Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33bcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_rtsroute_tgl(self) -> crate::common::Reg<regs::Eusart1Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33c0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_rxroute_tgl(self) -> crate::common::Reg<regs::Eusart1Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33c4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_sclkroute_tgl(self) -> crate::common::Reg<regs::Eusart1Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33c8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart1_txroute_tgl(self) -> crate::common::Reg<regs::Eusart1Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33ccusize) as _) }
    }
    #[doc = "EUSART2 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_routeen_tgl(self) -> crate::common::Reg<regs::Eusart2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33d4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_csroute_tgl(self) -> crate::common::Reg<regs::Eusart2Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33d8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_ctsroute_tgl(self) -> crate::common::Reg<regs::Eusart2Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33dcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_rtsroute_tgl(self) -> crate::common::Reg<regs::Eusart2Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33e0usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_rxroute_tgl(self) -> crate::common::Reg<regs::Eusart2Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33e4usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_sclkroute_tgl(self) -> crate::common::Reg<regs::Eusart2Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33e8usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart2_txroute_tgl(self) -> crate::common::Reg<regs::Eusart2Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33ecusize) as _) }
    }
    #[doc = "EUSART3 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_routeen_tgl(self) -> crate::common::Reg<regs::Eusart3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33f4usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_csroute_tgl(self) -> crate::common::Reg<regs::Eusart3Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33f8usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_ctsroute_tgl(self) -> crate::common::Reg<regs::Eusart3Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33fcusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_rtsroute_tgl(self) -> crate::common::Reg<regs::Eusart3Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3400usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_rxroute_tgl(self) -> crate::common::Reg<regs::Eusart3Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3404usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_sclkroute_tgl(self) -> crate::common::Reg<regs::Eusart3Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3408usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart3_txroute_tgl(self) -> crate::common::Reg<regs::Eusart3Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x340cusize) as _) }
    }
    #[doc = "EUSART4 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_routeen_tgl(self) -> crate::common::Reg<regs::Eusart4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3414usize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_csroute_tgl(self) -> crate::common::Reg<regs::Eusart4Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3418usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_ctsroute_tgl(self) -> crate::common::Reg<regs::Eusart4Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x341cusize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_rtsroute_tgl(self) -> crate::common::Reg<regs::Eusart4Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3420usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_rxroute_tgl(self) -> crate::common::Reg<regs::Eusart4Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3424usize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_sclkroute_tgl(self) -> crate::common::Reg<regs::Eusart4Sclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3428usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn eusart4_txroute_tgl(self) -> crate::common::Reg<regs::Eusart4Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x342cusize) as _) }
    }
    #[doc = "FRC pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_routeen_tgl(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3434usize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_dclkroute_tgl(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3438usize) as _) }
    }
    #[doc = "DFRAME port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_dframeroute_tgl(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x343cusize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_doutroute_tgl(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3440usize) as _) }
    }
    #[doc = "I2C0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c0_routeen_tgl(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3448usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c0_sclroute_tgl(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x344cusize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c0_sdaroute_tgl(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3450usize) as _) }
    }
    #[doc = "I2C1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c1_routeen_tgl(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3458usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c1_sclroute_tgl(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x345cusize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c1_sdaroute_tgl(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3460usize) as _) }
    }
    #[doc = "LESENSE pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_routeen_tgl(self) -> crate::common::Reg<regs::LesenseRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3468usize) as _) }
    }
    #[doc = "CH0OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch0outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh0outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x346cusize) as _) }
    }
    #[doc = "CH1OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch1outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh1outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3470usize) as _) }
    }
    #[doc = "CH2OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch2outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh2outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3474usize) as _) }
    }
    #[doc = "CH3OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch3outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh3outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3478usize) as _) }
    }
    #[doc = "CH4OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch4outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh4outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x347cusize) as _) }
    }
    #[doc = "CH5OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch5outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh5outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3480usize) as _) }
    }
    #[doc = "CH6OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch6outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh6outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3484usize) as _) }
    }
    #[doc = "CH7OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch7outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh7outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3488usize) as _) }
    }
    #[doc = "CH8OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch8outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh8outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x348cusize) as _) }
    }
    #[doc = "CH9OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch9outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh9outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3490usize) as _) }
    }
    #[doc = "CH10OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch10outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh10outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3494usize) as _) }
    }
    #[doc = "CH11OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch11outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh11outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3498usize) as _) }
    }
    #[doc = "CH12OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch12outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh12outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x349cusize) as _) }
    }
    #[doc = "CH13OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch13outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh13outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34a0usize) as _) }
    }
    #[doc = "CH14OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch14outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh14outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34a4usize) as _) }
    }
    #[doc = "CH15OUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lesense_ch15outroute_tgl(self) -> crate::common::Reg<regs::LesenseCh15outroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34a8usize) as _) }
    }
    #[doc = "LETIMER pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn letimer_routeen_tgl(self) -> crate::common::Reg<regs::LetimerRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn letimer_out0route_tgl(self) -> crate::common::Reg<regs::LetimerOut0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn letimer_out1route_tgl(self) -> crate::common::Reg<regs::LetimerOut1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34b8usize) as _) }
    }
    #[doc = "MODEM pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_routeen_tgl(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34c0usize) as _) }
    }
    #[doc = "ANT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_ant0route_tgl(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34c4usize) as _) }
    }
    #[doc = "ANT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_ant1route_tgl(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34c8usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrolloverroute_tgl(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34ccusize) as _) }
    }
    #[doc = "ANTRR0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr0route_tgl(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34d0usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr1route_tgl(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34d4usize) as _) }
    }
    #[doc = "ANTRR2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr2route_tgl(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34d8usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr3route_tgl(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34dcusize) as _) }
    }
    #[doc = "ANTRR4 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr4route_tgl(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34e0usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr5route_tgl(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34e4usize) as _) }
    }
    #[doc = "ANTSWEN port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antswenroute_tgl(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34e8usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antswusroute_tgl(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34ecusize) as _) }
    }
    #[doc = "ANTTRIG port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_anttrigroute_tgl(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34f0usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute_tgl(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34f4usize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_dclkroute_tgl(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34f8usize) as _) }
    }
    #[doc = "DIN port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_dinroute_tgl(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34fcusize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_doutroute_tgl(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3500usize) as _) }
    }
    #[doc = "S0IN port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pcnt0_s0inroute_tgl(self) -> crate::common::Reg<regs::Pcnt0S0inroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x350cusize) as _) }
    }
    #[doc = "S1IN port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pcnt0_s1inroute_tgl(self) -> crate::common::Reg<regs::Pcnt0S1inroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3510usize) as _) }
    }
    #[doc = "PRS0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_routeen_tgl(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3518usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch0route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x351cusize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch1route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3520usize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch2route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3524usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch3route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3528usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch4route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x352cusize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch5route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3530usize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch6route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3534usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch7route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3538usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch8route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x353cusize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch9route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3540usize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch10route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3544usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch11route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3548usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch0route_tgl(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x354cusize) as _) }
    }
    #[doc = "SYNCH1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch1route_tgl(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3550usize) as _) }
    }
    #[doc = "SYNCH2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch2route_tgl(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3554usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch3route_tgl(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3558usize) as _) }
    }
    #[doc = "BUFOUTREQINASYNC port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn syxo0_bufoutreqinasyncroute_tgl(
        self,
    ) -> crate::common::Reg<regs::Syxo0Bufoutreqinasyncroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35c0usize) as _) }
    }
    #[doc = "TIMER0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_routeen_tgl(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35c8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cc0route_tgl(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35ccusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cc1route_tgl(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35d0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cc2route_tgl(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35d4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35d8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35dcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35e0usize) as _) }
    }
    #[doc = "TIMER1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_routeen_tgl(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35e8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cc0route_tgl(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35ecusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cc1route_tgl(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35f0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cc2route_tgl(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35f4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35f8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35fcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3600usize) as _) }
    }
    #[doc = "TIMER2 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_routeen_tgl(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3608usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cc0route_tgl(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x360cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cc1route_tgl(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3610usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cc2route_tgl(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3614usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3618usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x361cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3620usize) as _) }
    }
    #[doc = "TIMER3 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_routeen_tgl(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3628usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cc0route_tgl(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x362cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cc1route_tgl(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3630usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cc2route_tgl(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3634usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3638usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x363cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3640usize) as _) }
    }
    #[doc = "TIMER4 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_routeen_tgl(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3648usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cc0route_tgl(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x364cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cc1route_tgl(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3650usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cc2route_tgl(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3654usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3658usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x365cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3660usize) as _) }
    }
    #[doc = "TIMER5 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_routeen_tgl(self) -> crate::common::Reg<regs::Timer5Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3668usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_cc0route_tgl(self) -> crate::common::Reg<regs::Timer5Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x366cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_cc1route_tgl(self) -> crate::common::Reg<regs::Timer5Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3670usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_cc2route_tgl(self) -> crate::common::Reg<regs::Timer5Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3674usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer5Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3678usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer5Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x367cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer5_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer5Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3680usize) as _) }
    }
    #[doc = "TIMER6 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_routeen_tgl(self) -> crate::common::Reg<regs::Timer6Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3688usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_cc0route_tgl(self) -> crate::common::Reg<regs::Timer6Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x368cusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_cc1route_tgl(self) -> crate::common::Reg<regs::Timer6Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3690usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_cc2route_tgl(self) -> crate::common::Reg<regs::Timer6Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3694usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer6Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3698usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer6Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x369cusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer6_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer6Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36a0usize) as _) }
    }
    #[doc = "TIMER7 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_routeen_tgl(self) -> crate::common::Reg<regs::Timer7Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36a8usize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_cc0route_tgl(self) -> crate::common::Reg<regs::Timer7Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36acusize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_cc1route_tgl(self) -> crate::common::Reg<regs::Timer7Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36b0usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_cc2route_tgl(self) -> crate::common::Reg<regs::Timer7Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36b4usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer7Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36b8usize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer7Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36bcusize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer7_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer7Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36c0usize) as _) }
    }
    #[doc = "USBVBUSSENSE port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usb_usbvbussenseroute_tgl(self) -> crate::common::Reg<regs::UsbUsbvbussenseroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36ccusize) as _) }
    }
}
pub mod regs {
    #[doc = "A Bus allocation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Abusalloc(pub u32);
    impl Abusalloc {
        #[doc = "A Bus Even 0."]
        #[must_use]
        #[inline(always)]
        pub const fn aeven0(&self) -> super::vals::Aeven0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Aeven0::from_bits(val as u8)
        }
        #[doc = "A Bus Even 0."]
        #[inline(always)]
        pub const fn set_aeven0(&mut self, val: super::vals::Aeven0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "A Bus Even 1."]
        #[must_use]
        #[inline(always)]
        pub const fn aeven1(&self) -> super::vals::Aeven1 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Aeven1::from_bits(val as u8)
        }
        #[doc = "A Bus Even 1."]
        #[inline(always)]
        pub const fn set_aeven1(&mut self, val: super::vals::Aeven1) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "A Bus Odd 0."]
        #[must_use]
        #[inline(always)]
        pub const fn aodd0(&self) -> super::vals::Aodd0 {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Aodd0::from_bits(val as u8)
        }
        #[doc = "A Bus Odd 0."]
        #[inline(always)]
        pub const fn set_aodd0(&mut self, val: super::vals::Aodd0) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "A Bus Odd 1."]
        #[must_use]
        #[inline(always)]
        pub const fn aodd1(&self) -> super::vals::Aodd1 {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Aodd1::from_bits(val as u8)
        }
        #[doc = "A Bus Odd 1."]
        #[inline(always)]
        pub const fn set_aodd1(&mut self, val: super::vals::Aodd1) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Abusalloc {
        #[inline(always)]
        fn default() -> Abusalloc {
            Abusalloc(0)
        }
    }
    impl core::fmt::Debug for Abusalloc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Abusalloc")
                .field("aeven0", &self.aeven0())
                .field("aeven1", &self.aeven1())
                .field("aodd0", &self.aodd0())
                .field("aodd1", &self.aodd1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Abusalloc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Abusalloc {{ aeven0: {:?}, aeven1: {:?}, aodd0: {:?}, aodd1: {:?} }}",
                self.aeven0(),
                self.aeven1(),
                self.aodd0(),
                self.aodd1()
            )
        }
    }
    #[doc = "ACMPOUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acmp0Acmpoutroute(pub u32);
    impl Acmp0Acmpoutroute {
        #[doc = "ACMPOUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ACMPOUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ACMPOUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ACMPOUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Acmp0Acmpoutroute {
        #[inline(always)]
        fn default() -> Acmp0Acmpoutroute {
            Acmp0Acmpoutroute(0)
        }
    }
    impl core::fmt::Debug for Acmp0Acmpoutroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acmp0Acmpoutroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acmp0Acmpoutroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acmp0Acmpoutroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ACMP0 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acmp0Routeen(pub u32);
    impl Acmp0Routeen {
        #[doc = "ACMPOUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn acmpoutpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ACMPOUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_acmpoutpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Acmp0Routeen {
        #[inline(always)]
        fn default() -> Acmp0Routeen {
            Acmp0Routeen(0)
        }
    }
    impl core::fmt::Debug for Acmp0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acmp0Routeen")
                .field("acmpoutpen", &self.acmpoutpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acmp0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Acmp0Routeen {{ acmpoutpen: {=bool:?} }}", self.acmpoutpen())
        }
    }
    #[doc = "ACMPOUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acmp1Acmpoutroute(pub u32);
    impl Acmp1Acmpoutroute {
        #[doc = "ACMPOUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ACMPOUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ACMPOUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ACMPOUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Acmp1Acmpoutroute {
        #[inline(always)]
        fn default() -> Acmp1Acmpoutroute {
            Acmp1Acmpoutroute(0)
        }
    }
    impl core::fmt::Debug for Acmp1Acmpoutroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acmp1Acmpoutroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acmp1Acmpoutroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acmp1Acmpoutroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ACMP1 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acmp1Routeen(pub u32);
    impl Acmp1Routeen {
        #[doc = "ACMPOUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn acmpoutpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ACMPOUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_acmpoutpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Acmp1Routeen {
        #[inline(always)]
        fn default() -> Acmp1Routeen {
            Acmp1Routeen(0)
        }
    }
    impl core::fmt::Debug for Acmp1Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acmp1Routeen")
                .field("acmpoutpen", &self.acmpoutpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acmp1Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Acmp1Routeen {{ acmpoutpen: {=bool:?} }}", self.acmpoutpen())
        }
    }
    #[doc = "ABUS AEVEN0 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aeven0switch(pub u32);
    impl Aeven0switch {
        #[doc = "AEVEN0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn aeven0switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "AEVEN0 switch register."]
        #[inline(always)]
        pub const fn set_aeven0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Aeven0switch {
        #[inline(always)]
        fn default() -> Aeven0switch {
            Aeven0switch(0)
        }
    }
    impl core::fmt::Debug for Aeven0switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aeven0switch")
                .field("aeven0switch", &self.aeven0switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aeven0switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aeven0switch {{ aeven0switch: {=u8:?} }}", self.aeven0switch())
        }
    }
    #[doc = "ABUS AEVEN1 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aeven1switch(pub u32);
    impl Aeven1switch {
        #[doc = "AEVEN1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn aeven1switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "AEVEN1 switch register."]
        #[inline(always)]
        pub const fn set_aeven1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Aeven1switch {
        #[inline(always)]
        fn default() -> Aeven1switch {
            Aeven1switch(0)
        }
    }
    impl core::fmt::Debug for Aeven1switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aeven1switch")
                .field("aeven1switch", &self.aeven1switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aeven1switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aeven1switch {{ aeven1switch: {=u8:?} }}", self.aeven1switch())
        }
    }
    #[doc = "ABUS AODD0 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aodd0switch(pub u32);
    impl Aodd0switch {
        #[doc = "AODD0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn aodd0switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "AODD0 switch register."]
        #[inline(always)]
        pub const fn set_aodd0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Aodd0switch {
        #[inline(always)]
        fn default() -> Aodd0switch {
            Aodd0switch(0)
        }
    }
    impl core::fmt::Debug for Aodd0switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aodd0switch")
                .field("aodd0switch", &self.aodd0switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aodd0switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aodd0switch {{ aodd0switch: {=u8:?} }}", self.aodd0switch())
        }
    }
    #[doc = "ABUS AODD1 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aodd1switch(pub u32);
    impl Aodd1switch {
        #[doc = "AODD1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn aodd1switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "AODD1 switch register."]
        #[inline(always)]
        pub const fn set_aodd1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Aodd1switch {
        #[inline(always)]
        fn default() -> Aodd1switch {
            Aodd1switch(0)
        }
    }
    impl core::fmt::Debug for Aodd1switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aodd1switch")
                .field("aodd1switch", &self.aodd1switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Aodd1switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Aodd1switch {{ aodd1switch: {=u8:?} }}", self.aodd1switch())
        }
    }
    #[doc = "B Bus allocation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bbusalloc(pub u32);
    impl Bbusalloc {
        #[doc = "B Bus Even 0."]
        #[must_use]
        #[inline(always)]
        pub const fn beven0(&self) -> super::vals::Beven0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Beven0::from_bits(val as u8)
        }
        #[doc = "B Bus Even 0."]
        #[inline(always)]
        pub const fn set_beven0(&mut self, val: super::vals::Beven0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "B Bus Even 1."]
        #[must_use]
        #[inline(always)]
        pub const fn beven1(&self) -> super::vals::Beven1 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Beven1::from_bits(val as u8)
        }
        #[doc = "B Bus Even 1."]
        #[inline(always)]
        pub const fn set_beven1(&mut self, val: super::vals::Beven1) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "B Bus Odd 0."]
        #[must_use]
        #[inline(always)]
        pub const fn bodd0(&self) -> super::vals::Bodd0 {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Bodd0::from_bits(val as u8)
        }
        #[doc = "B Bus Odd 0."]
        #[inline(always)]
        pub const fn set_bodd0(&mut self, val: super::vals::Bodd0) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "B Bus Odd 1."]
        #[must_use]
        #[inline(always)]
        pub const fn bodd1(&self) -> super::vals::Bodd1 {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Bodd1::from_bits(val as u8)
        }
        #[doc = "B Bus Odd 1."]
        #[inline(always)]
        pub const fn set_bodd1(&mut self, val: super::vals::Bodd1) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Bbusalloc {
        #[inline(always)]
        fn default() -> Bbusalloc {
            Bbusalloc(0)
        }
    }
    impl core::fmt::Debug for Bbusalloc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bbusalloc")
                .field("beven0", &self.beven0())
                .field("beven1", &self.beven1())
                .field("bodd0", &self.bodd0())
                .field("bodd1", &self.bodd1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bbusalloc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bbusalloc {{ beven0: {:?}, beven1: {:?}, bodd0: {:?}, bodd1: {:?} }}",
                self.beven0(),
                self.beven1(),
                self.bodd0(),
                self.bodd1()
            )
        }
    }
    #[doc = "ABUS BEVEN0 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Beven0switch(pub u32);
    impl Beven0switch {
        #[doc = "BEVEN0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn beven0switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "BEVEN0 switch register."]
        #[inline(always)]
        pub const fn set_beven0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Beven0switch {
        #[inline(always)]
        fn default() -> Beven0switch {
            Beven0switch(0)
        }
    }
    impl core::fmt::Debug for Beven0switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Beven0switch")
                .field("beven0switch", &self.beven0switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Beven0switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Beven0switch {{ beven0switch: {=u8:?} }}", self.beven0switch())
        }
    }
    #[doc = "ABUS BEVEN1 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Beven1switch(pub u32);
    impl Beven1switch {
        #[doc = "BEVEN1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn beven1switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "BEVEN1 switch register."]
        #[inline(always)]
        pub const fn set_beven1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Beven1switch {
        #[inline(always)]
        fn default() -> Beven1switch {
            Beven1switch(0)
        }
    }
    impl core::fmt::Debug for Beven1switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Beven1switch")
                .field("beven1switch", &self.beven1switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Beven1switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Beven1switch {{ beven1switch: {=u8:?} }}", self.beven1switch())
        }
    }
    #[doc = "ABUS BODD0 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bodd0switch(pub u32);
    impl Bodd0switch {
        #[doc = "BODD0 Switch Reg."]
        #[must_use]
        #[inline(always)]
        pub const fn bodd0switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "BODD0 Switch Reg."]
        #[inline(always)]
        pub const fn set_bodd0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Bodd0switch {
        #[inline(always)]
        fn default() -> Bodd0switch {
            Bodd0switch(0)
        }
    }
    impl core::fmt::Debug for Bodd0switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bodd0switch")
                .field("bodd0switch", &self.bodd0switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bodd0switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bodd0switch {{ bodd0switch: {=u8:?} }}", self.bodd0switch())
        }
    }
    #[doc = "ABUS BODD1 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bodd1switch(pub u32);
    impl Bodd1switch {
        #[doc = "BODD1 Switch Reg."]
        #[must_use]
        #[inline(always)]
        pub const fn bodd1switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "BODD1 Switch Reg."]
        #[inline(always)]
        pub const fn set_bodd1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Bodd1switch {
        #[inline(always)]
        fn default() -> Bodd1switch {
            Bodd1switch(0)
        }
    }
    impl core::fmt::Debug for Bodd1switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bodd1switch")
                .field("bodd1switch", &self.bodd1switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bodd1switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bodd1switch {{ bodd1switch: {=u8:?} }}", self.bodd1switch())
        }
    }
    #[doc = "CD Bus allocation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdbusalloc(pub u32);
    impl Cdbusalloc {
        #[doc = "CD Bus Even 0."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeven0(&self) -> super::vals::Cdeven0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Cdeven0::from_bits(val as u8)
        }
        #[doc = "CD Bus Even 0."]
        #[inline(always)]
        pub const fn set_cdeven0(&mut self, val: super::vals::Cdeven0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "CD Bus Even 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeven1(&self) -> super::vals::Cdeven1 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Cdeven1::from_bits(val as u8)
        }
        #[doc = "CD Bus Even 1."]
        #[inline(always)]
        pub const fn set_cdeven1(&mut self, val: super::vals::Cdeven1) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "CD Bus Odd 0."]
        #[must_use]
        #[inline(always)]
        pub const fn cdodd0(&self) -> super::vals::Cdodd0 {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Cdodd0::from_bits(val as u8)
        }
        #[doc = "CD Bus Odd 0."]
        #[inline(always)]
        pub const fn set_cdodd0(&mut self, val: super::vals::Cdodd0) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "CD Bus Odd 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cdodd1(&self) -> super::vals::Cdodd1 {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Cdodd1::from_bits(val as u8)
        }
        #[doc = "CD Bus Odd 1."]
        #[inline(always)]
        pub const fn set_cdodd1(&mut self, val: super::vals::Cdodd1) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Cdbusalloc {
        #[inline(always)]
        fn default() -> Cdbusalloc {
            Cdbusalloc(0)
        }
    }
    impl core::fmt::Debug for Cdbusalloc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdbusalloc")
                .field("cdeven0", &self.cdeven0())
                .field("cdeven1", &self.cdeven1())
                .field("cdodd0", &self.cdodd0())
                .field("cdodd1", &self.cdodd1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdbusalloc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdbusalloc {{ cdeven0: {:?}, cdeven1: {:?}, cdodd0: {:?}, cdodd1: {:?} }}",
                self.cdeven0(),
                self.cdeven1(),
                self.cdodd0(),
                self.cdodd1()
            )
        }
    }
    #[doc = "ABUS CDEVEN0 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdeven0switch(pub u32);
    impl Cdeven0switch {
        #[doc = "CEVEN0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn ceven0switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "CEVEN0 switch register."]
        #[inline(always)]
        pub const fn set_ceven0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DEVEN0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn deven0switch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DEVEN0 switch register."]
        #[inline(always)]
        pub const fn set_deven0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cdeven0switch {
        #[inline(always)]
        fn default() -> Cdeven0switch {
            Cdeven0switch(0)
        }
    }
    impl core::fmt::Debug for Cdeven0switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdeven0switch")
                .field("ceven0switch", &self.ceven0switch())
                .field("deven0switch", &self.deven0switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdeven0switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdeven0switch {{ ceven0switch: {=u8:?}, deven0switch: {=u8:?} }}",
                self.ceven0switch(),
                self.deven0switch()
            )
        }
    }
    #[doc = "ABUS CDEVEN1 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdeven1switch(pub u32);
    impl Cdeven1switch {
        #[doc = "CEVEN1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn ceven1switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "CEVEN1 switch register."]
        #[inline(always)]
        pub const fn set_ceven1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "DEVEN1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn deven1switch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DEVEN1 switch register."]
        #[inline(always)]
        pub const fn set_deven1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cdeven1switch {
        #[inline(always)]
        fn default() -> Cdeven1switch {
            Cdeven1switch(0)
        }
    }
    impl core::fmt::Debug for Cdeven1switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdeven1switch")
                .field("ceven1switch", &self.ceven1switch())
                .field("deven1switch", &self.deven1switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdeven1switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdeven1switch {{ ceven1switch: {=u8:?}, deven1switch: {=u8:?} }}",
                self.ceven1switch(),
                self.deven1switch()
            )
        }
    }
    #[doc = "ABUS CDODD0 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdodd0switch(pub u32);
    impl Cdodd0switch {
        #[doc = "CODD0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn codd0switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "CODD0 switch register."]
        #[inline(always)]
        pub const fn set_codd0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DODD0 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn dodd0switch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DODD0 switch register."]
        #[inline(always)]
        pub const fn set_dodd0switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cdodd0switch {
        #[inline(always)]
        fn default() -> Cdodd0switch {
            Cdodd0switch(0)
        }
    }
    impl core::fmt::Debug for Cdodd0switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdodd0switch")
                .field("codd0switch", &self.codd0switch())
                .field("dodd0switch", &self.dodd0switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdodd0switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdodd0switch {{ codd0switch: {=u8:?}, dodd0switch: {=u8:?} }}",
                self.codd0switch(),
                self.dodd0switch()
            )
        }
    }
    #[doc = "ABUS CDODD1 Switch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdodd1switch(pub u32);
    impl Cdodd1switch {
        #[doc = "CODD1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn codd1switch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "CODD1 switch register."]
        #[inline(always)]
        pub const fn set_codd1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DODD1 switch register."]
        #[must_use]
        #[inline(always)]
        pub const fn dodd1switch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DODD1 switch register."]
        #[inline(always)]
        pub const fn set_dodd1switch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Cdodd1switch {
        #[inline(always)]
        fn default() -> Cdodd1switch {
            Cdodd1switch(0)
        }
    }
    impl core::fmt::Debug for Cdodd1switch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdodd1switch")
                .field("codd1switch", &self.codd1switch())
                .field("dodd1switch", &self.dodd1switch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdodd1switch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cdodd1switch {{ codd1switch: {=u8:?}, dodd1switch: {=u8:?} }}",
                self.codd1switch(),
                self.dodd1switch()
            )
        }
    }
    #[doc = "CLKIN0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmuClkin0route(pub u32);
    impl CmuClkin0route {
        #[doc = "CLKIN0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CLKIN0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CLKIN0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CLKIN0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CmuClkin0route {
        #[inline(always)]
        fn default() -> CmuClkin0route {
            CmuClkin0route(0)
        }
    }
    impl core::fmt::Debug for CmuClkin0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmuClkin0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmuClkin0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmuClkin0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CLKOUT0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmuClkout0route(pub u32);
    impl CmuClkout0route {
        #[doc = "CLKOUT0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CLKOUT0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CLKOUT0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CLKOUT0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CmuClkout0route {
        #[inline(always)]
        fn default() -> CmuClkout0route {
            CmuClkout0route(0)
        }
    }
    impl core::fmt::Debug for CmuClkout0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmuClkout0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmuClkout0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmuClkout0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CLKOUT1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmuClkout1route(pub u32);
    impl CmuClkout1route {
        #[doc = "CLKOUT1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CLKOUT1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CLKOUT1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CLKOUT1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CmuClkout1route {
        #[inline(always)]
        fn default() -> CmuClkout1route {
            CmuClkout1route(0)
        }
    }
    impl core::fmt::Debug for CmuClkout1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmuClkout1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmuClkout1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmuClkout1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CLKOUT2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmuClkout2route(pub u32);
    impl CmuClkout2route {
        #[doc = "CLKOUT2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CLKOUT2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CLKOUT2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CLKOUT2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CmuClkout2route {
        #[inline(always)]
        fn default() -> CmuClkout2route {
            CmuClkout2route(0)
        }
    }
    impl core::fmt::Debug for CmuClkout2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmuClkout2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmuClkout2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmuClkout2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CMU pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmuRouteen(pub u32);
    impl CmuRouteen {
        #[doc = "CLKOUT0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clkout0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CLKOUT0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_clkout0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CLKOUT1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clkout1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CLKOUT1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_clkout1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CLKOUT2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clkout2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CLKOUT2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_clkout2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CmuRouteen {
        #[inline(always)]
        fn default() -> CmuRouteen {
            CmuRouteen(0)
        }
    }
    impl core::fmt::Debug for CmuRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmuRouteen")
                .field("clkout0pen", &self.clkout0pen())
                .field("clkout1pen", &self.clkout1pen())
                .field("clkout2pen", &self.clkout2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmuRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmuRouteen {{ clkout0pen: {=bool:?}, clkout1pen: {=bool:?}, clkout2pen: {=bool:?} }}",
                self.clkout0pen(),
                self.clkout1pen(),
                self.clkout2pen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgroutepen(pub u32);
    impl Dbgroutepen {
        #[doc = "Route Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn swclktckpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Route Pin Enable."]
        #[inline(always)]
        pub const fn set_swclktckpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Route Location 0."]
        #[must_use]
        #[inline(always)]
        pub const fn swdiotmspen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Route Location 0."]
        #[inline(always)]
        pub const fn set_swdiotmspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "JTAG Test Debug Output Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tdopen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "JTAG Test Debug Output Pin Enable."]
        #[inline(always)]
        pub const fn set_tdopen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "JTAG Test Debug Input Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tdipen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JTAG Test Debug Input Pin Enable."]
        #[inline(always)]
        pub const fn set_tdipen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Dbgroutepen {
        #[inline(always)]
        fn default() -> Dbgroutepen {
            Dbgroutepen(0)
        }
    }
    impl core::fmt::Debug for Dbgroutepen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgroutepen")
                .field("swclktckpen", &self.swclktckpen())
                .field("swdiotmspen", &self.swdiotmspen())
                .field("tdopen", &self.tdopen())
                .field("tdipen", &self.tdipen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgroutepen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dbgroutepen {{ swclktckpen: {=bool:?}, swdiotmspen: {=bool:?}, tdopen: {=bool:?}, tdipen: {=bool:?} }}",
                self.swclktckpen(),
                self.swdiotmspen(),
                self.tdopen(),
                self.tdipen()
            )
        }
    }
    #[doc = "DCDC pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcRouteen(pub u32);
    impl DcdcRouteen {
        #[doc = "DCDCCOREHIDDEN pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdccorehiddenpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCDCCOREHIDDEN pin enable control bit."]
        #[inline(always)]
        pub const fn set_dcdccorehiddenpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DcdcRouteen {
        #[inline(always)]
        fn default() -> DcdcRouteen {
            DcdcRouteen(0)
        }
    }
    impl core::fmt::Debug for DcdcRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcRouteen")
                .field("dcdccorehiddenpen", &self.dcdccorehiddenpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcRouteen {{ dcdccorehiddenpen: {=bool:?} }}",
                self.dcdccorehiddenpen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em4wuen(pub u32);
    impl Em4wuen {
        #[doc = "EM4 wake up enable."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuen(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "EM4 wake up enable."]
        #[inline(always)]
        pub const fn set_em4wuen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Em4wuen {
        #[inline(always)]
        fn default() -> Em4wuen {
            Em4wuen(0)
        }
    }
    impl core::fmt::Debug for Em4wuen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em4wuen").field("em4wuen", &self.em4wuen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em4wuen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Em4wuen {{ em4wuen: {=u16:?} }}", self.em4wuen())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Em4wupol(pub u32);
    impl Em4wupol {
        #[doc = "EM4 Wake-Up Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wupol(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "EM4 Wake-Up Polarity."]
        #[inline(always)]
        pub const fn set_em4wupol(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Em4wupol {
        #[inline(always)]
        fn default() -> Em4wupol {
            Em4wupol(0)
        }
    }
    impl core::fmt::Debug for Em4wupol {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Em4wupol").field("em4wupol", &self.em4wupol()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Em4wupol {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Em4wupol {{ em4wupol: {=u16:?} }}", self.em4wupol())
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Csroute(pub u32);
    impl Eusart0Csroute {
        #[doc = "CS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart0Csroute {
        #[inline(always)]
        fn default() -> Eusart0Csroute {
            Eusart0Csroute(0)
        }
    }
    impl core::fmt::Debug for Eusart0Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Ctsroute(pub u32);
    impl Eusart0Ctsroute {
        #[doc = "CTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart0Ctsroute {
        #[inline(always)]
        fn default() -> Eusart0Ctsroute {
            Eusart0Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart0Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "EUSART0 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Routeen(pub u32);
    impl Eusart0Routeen {
        #[doc = "CS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CS pin enable control bit."]
        #[inline(always)]
        pub const fn set_cspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rtspen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTS pin enable control bit."]
        #[inline(always)]
        pub const fn set_rtspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin enable control bit."]
        #[inline(always)]
        pub const fn set_rxpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin enable control bit."]
        #[inline(always)]
        pub const fn set_txpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Eusart0Routeen {
        #[inline(always)]
        fn default() -> Eusart0Routeen {
            Eusart0Routeen(0)
        }
    }
    impl core::fmt::Debug for Eusart0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("sclkpen", &self.sclkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, sclkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.sclkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Rtsroute(pub u32);
    impl Eusart0Rtsroute {
        #[doc = "RTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart0Rtsroute {
        #[inline(always)]
        fn default() -> Eusart0Rtsroute {
            Eusart0Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart0Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Rxroute(pub u32);
    impl Eusart0Rxroute {
        #[doc = "RX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart0Rxroute {
        #[inline(always)]
        fn default() -> Eusart0Rxroute {
            Eusart0Rxroute(0)
        }
    }
    impl core::fmt::Debug for Eusart0Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Sclkroute(pub u32);
    impl Eusart0Sclkroute {
        #[doc = "SCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart0Sclkroute {
        #[inline(always)]
        fn default() -> Eusart0Sclkroute {
            Eusart0Sclkroute(0)
        }
    }
    impl core::fmt::Debug for Eusart0Sclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Sclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Sclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Sclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart0Txroute(pub u32);
    impl Eusart0Txroute {
        #[doc = "TX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "TX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "TX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart0Txroute {
        #[inline(always)]
        fn default() -> Eusart0Txroute {
            Eusart0Txroute(0)
        }
    }
    impl core::fmt::Debug for Eusart0Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart0Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart0Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart0Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Csroute(pub u32);
    impl Eusart1Csroute {
        #[doc = "CS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart1Csroute {
        #[inline(always)]
        fn default() -> Eusart1Csroute {
            Eusart1Csroute(0)
        }
    }
    impl core::fmt::Debug for Eusart1Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Ctsroute(pub u32);
    impl Eusart1Ctsroute {
        #[doc = "CTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart1Ctsroute {
        #[inline(always)]
        fn default() -> Eusart1Ctsroute {
            Eusart1Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart1Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "EUSART1 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Routeen(pub u32);
    impl Eusart1Routeen {
        #[doc = "CS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CS pin enable control bit."]
        #[inline(always)]
        pub const fn set_cspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rtspen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTS pin enable control bit."]
        #[inline(always)]
        pub const fn set_rtspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin enable control bit."]
        #[inline(always)]
        pub const fn set_rxpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin enable control bit."]
        #[inline(always)]
        pub const fn set_txpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Eusart1Routeen {
        #[inline(always)]
        fn default() -> Eusart1Routeen {
            Eusart1Routeen(0)
        }
    }
    impl core::fmt::Debug for Eusart1Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("sclkpen", &self.sclkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, sclkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.sclkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Rtsroute(pub u32);
    impl Eusart1Rtsroute {
        #[doc = "RTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart1Rtsroute {
        #[inline(always)]
        fn default() -> Eusart1Rtsroute {
            Eusart1Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart1Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Rxroute(pub u32);
    impl Eusart1Rxroute {
        #[doc = "RX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart1Rxroute {
        #[inline(always)]
        fn default() -> Eusart1Rxroute {
            Eusart1Rxroute(0)
        }
    }
    impl core::fmt::Debug for Eusart1Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Sclkroute(pub u32);
    impl Eusart1Sclkroute {
        #[doc = "SCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart1Sclkroute {
        #[inline(always)]
        fn default() -> Eusart1Sclkroute {
            Eusart1Sclkroute(0)
        }
    }
    impl core::fmt::Debug for Eusart1Sclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Sclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Sclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Sclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart1Txroute(pub u32);
    impl Eusart1Txroute {
        #[doc = "TX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "TX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "TX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart1Txroute {
        #[inline(always)]
        fn default() -> Eusart1Txroute {
            Eusart1Txroute(0)
        }
    }
    impl core::fmt::Debug for Eusart1Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart1Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart1Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart1Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Csroute(pub u32);
    impl Eusart2Csroute {
        #[doc = "CS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart2Csroute {
        #[inline(always)]
        fn default() -> Eusart2Csroute {
            Eusart2Csroute(0)
        }
    }
    impl core::fmt::Debug for Eusart2Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Ctsroute(pub u32);
    impl Eusart2Ctsroute {
        #[doc = "CTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart2Ctsroute {
        #[inline(always)]
        fn default() -> Eusart2Ctsroute {
            Eusart2Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart2Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "EUSART2 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Routeen(pub u32);
    impl Eusart2Routeen {
        #[doc = "CS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CS pin enable control bit."]
        #[inline(always)]
        pub const fn set_cspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rtspen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTS pin enable control bit."]
        #[inline(always)]
        pub const fn set_rtspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin enable control bit."]
        #[inline(always)]
        pub const fn set_rxpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin enable control bit."]
        #[inline(always)]
        pub const fn set_txpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Eusart2Routeen {
        #[inline(always)]
        fn default() -> Eusart2Routeen {
            Eusart2Routeen(0)
        }
    }
    impl core::fmt::Debug for Eusart2Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("sclkpen", &self.sclkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, sclkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.sclkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Rtsroute(pub u32);
    impl Eusart2Rtsroute {
        #[doc = "RTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart2Rtsroute {
        #[inline(always)]
        fn default() -> Eusart2Rtsroute {
            Eusart2Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart2Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Rxroute(pub u32);
    impl Eusart2Rxroute {
        #[doc = "RX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart2Rxroute {
        #[inline(always)]
        fn default() -> Eusart2Rxroute {
            Eusart2Rxroute(0)
        }
    }
    impl core::fmt::Debug for Eusart2Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Sclkroute(pub u32);
    impl Eusart2Sclkroute {
        #[doc = "SCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart2Sclkroute {
        #[inline(always)]
        fn default() -> Eusart2Sclkroute {
            Eusart2Sclkroute(0)
        }
    }
    impl core::fmt::Debug for Eusart2Sclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Sclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Sclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Sclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart2Txroute(pub u32);
    impl Eusart2Txroute {
        #[doc = "TX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "TX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "TX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart2Txroute {
        #[inline(always)]
        fn default() -> Eusart2Txroute {
            Eusart2Txroute(0)
        }
    }
    impl core::fmt::Debug for Eusart2Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart2Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart2Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart2Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Csroute(pub u32);
    impl Eusart3Csroute {
        #[doc = "CS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart3Csroute {
        #[inline(always)]
        fn default() -> Eusart3Csroute {
            Eusart3Csroute(0)
        }
    }
    impl core::fmt::Debug for Eusart3Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Ctsroute(pub u32);
    impl Eusart3Ctsroute {
        #[doc = "CTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart3Ctsroute {
        #[inline(always)]
        fn default() -> Eusart3Ctsroute {
            Eusart3Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart3Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "EUSART3 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Routeen(pub u32);
    impl Eusart3Routeen {
        #[doc = "CS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CS pin enable control bit."]
        #[inline(always)]
        pub const fn set_cspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rtspen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTS pin enable control bit."]
        #[inline(always)]
        pub const fn set_rtspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin enable control bit."]
        #[inline(always)]
        pub const fn set_rxpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin enable control bit."]
        #[inline(always)]
        pub const fn set_txpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Eusart3Routeen {
        #[inline(always)]
        fn default() -> Eusart3Routeen {
            Eusart3Routeen(0)
        }
    }
    impl core::fmt::Debug for Eusart3Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("sclkpen", &self.sclkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, sclkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.sclkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Rtsroute(pub u32);
    impl Eusart3Rtsroute {
        #[doc = "RTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart3Rtsroute {
        #[inline(always)]
        fn default() -> Eusart3Rtsroute {
            Eusart3Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart3Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Rxroute(pub u32);
    impl Eusart3Rxroute {
        #[doc = "RX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart3Rxroute {
        #[inline(always)]
        fn default() -> Eusart3Rxroute {
            Eusart3Rxroute(0)
        }
    }
    impl core::fmt::Debug for Eusart3Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Sclkroute(pub u32);
    impl Eusart3Sclkroute {
        #[doc = "SCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart3Sclkroute {
        #[inline(always)]
        fn default() -> Eusart3Sclkroute {
            Eusart3Sclkroute(0)
        }
    }
    impl core::fmt::Debug for Eusart3Sclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Sclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Sclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Sclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart3Txroute(pub u32);
    impl Eusart3Txroute {
        #[doc = "TX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "TX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "TX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart3Txroute {
        #[inline(always)]
        fn default() -> Eusart3Txroute {
            Eusart3Txroute(0)
        }
    }
    impl core::fmt::Debug for Eusart3Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart3Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart3Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart3Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Csroute(pub u32);
    impl Eusart4Csroute {
        #[doc = "CS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart4Csroute {
        #[inline(always)]
        fn default() -> Eusart4Csroute {
            Eusart4Csroute(0)
        }
    }
    impl core::fmt::Debug for Eusart4Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Ctsroute(pub u32);
    impl Eusart4Ctsroute {
        #[doc = "CTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart4Ctsroute {
        #[inline(always)]
        fn default() -> Eusart4Ctsroute {
            Eusart4Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart4Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "EUSART4 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Routeen(pub u32);
    impl Eusart4Routeen {
        #[doc = "CS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CS pin enable control bit."]
        #[inline(always)]
        pub const fn set_cspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rtspen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTS pin enable control bit."]
        #[inline(always)]
        pub const fn set_rtspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rxpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin enable control bit."]
        #[inline(always)]
        pub const fn set_rxpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin enable control bit."]
        #[inline(always)]
        pub const fn set_txpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Eusart4Routeen {
        #[inline(always)]
        fn default() -> Eusart4Routeen {
            Eusart4Routeen(0)
        }
    }
    impl core::fmt::Debug for Eusart4Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("sclkpen", &self.sclkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, sclkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.sclkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Rtsroute(pub u32);
    impl Eusart4Rtsroute {
        #[doc = "RTS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RTS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RTS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RTS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart4Rtsroute {
        #[inline(always)]
        fn default() -> Eusart4Rtsroute {
            Eusart4Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Eusart4Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Rxroute(pub u32);
    impl Eusart4Rxroute {
        #[doc = "RX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "RX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart4Rxroute {
        #[inline(always)]
        fn default() -> Eusart4Rxroute {
            Eusart4Rxroute(0)
        }
    }
    impl core::fmt::Debug for Eusart4Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Sclkroute(pub u32);
    impl Eusart4Sclkroute {
        #[doc = "SCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart4Sclkroute {
        #[inline(always)]
        fn default() -> Eusart4Sclkroute {
            Eusart4Sclkroute(0)
        }
    }
    impl core::fmt::Debug for Eusart4Sclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Sclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Sclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Sclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eusart4Txroute(pub u32);
    impl Eusart4Txroute {
        #[doc = "TX port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "TX port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "TX pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "TX pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Eusart4Txroute {
        #[inline(always)]
        fn default() -> Eusart4Txroute {
            Eusart4Txroute(0)
        }
    }
    impl core::fmt::Debug for Eusart4Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eusart4Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eusart4Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eusart4Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "External Interrupt Falling Edge Trigger."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extifall(pub u32);
    impl Extifall {
        #[doc = "EXT Int FALL."]
        #[must_use]
        #[inline(always)]
        pub const fn extifall(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "EXT Int FALL."]
        #[inline(always)]
        pub const fn set_extifall(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Extifall {
        #[inline(always)]
        fn default() -> Extifall {
            Extifall(0)
        }
    }
    impl core::fmt::Debug for Extifall {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extifall").field("extifall", &self.extifall()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extifall {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Extifall {{ extifall: {=u16:?} }}", self.extifall())
        }
    }
    #[doc = "External Interrupt Pin Select High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extipinselh(pub u32);
    impl Extipinselh {
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel0(&self) -> super::vals::ExtipinselhExtipinsel0 {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::ExtipinselhExtipinsel0::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel0(&mut self, val: super::vals::ExtipinselhExtipinsel0) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel1(&self) -> super::vals::ExtipinselhExtipinsel1 {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::ExtipinselhExtipinsel1::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel1(&mut self, val: super::vals::ExtipinselhExtipinsel1) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel2(&self) -> super::vals::ExtipinselhExtipinsel2 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ExtipinselhExtipinsel2::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel2(&mut self, val: super::vals::ExtipinselhExtipinsel2) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel3(&self) -> super::vals::ExtipinselhExtipinsel3 {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::ExtipinselhExtipinsel3::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel3(&mut self, val: super::vals::ExtipinselhExtipinsel3) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Extipinselh {
        #[inline(always)]
        fn default() -> Extipinselh {
            Extipinselh(0)
        }
    }
    impl core::fmt::Debug for Extipinselh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extipinselh")
                .field("extipinsel0", &self.extipinsel0())
                .field("extipinsel1", &self.extipinsel1())
                .field("extipinsel2", &self.extipinsel2())
                .field("extipinsel3", &self.extipinsel3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extipinselh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Extipinselh {{ extipinsel0: {:?}, extipinsel1: {:?}, extipinsel2: {:?}, extipinsel3: {:?} }}",
                self.extipinsel0(),
                self.extipinsel1(),
                self.extipinsel2(),
                self.extipinsel3()
            )
        }
    }
    #[doc = "External Interrupt Pin Select Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extipinsell(pub u32);
    impl Extipinsell {
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel0(&self) -> super::vals::ExtipinsellExtipinsel0 {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::ExtipinsellExtipinsel0::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel0(&mut self, val: super::vals::ExtipinsellExtipinsel0) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel1(&self) -> super::vals::ExtipinsellExtipinsel1 {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::ExtipinsellExtipinsel1::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel1(&mut self, val: super::vals::ExtipinsellExtipinsel1) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel2(&self) -> super::vals::ExtipinsellExtipinsel2 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ExtipinsellExtipinsel2::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel2(&mut self, val: super::vals::ExtipinsellExtipinsel2) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel3(&self) -> super::vals::ExtipinsellExtipinsel3 {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::ExtipinsellExtipinsel3::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel3(&mut self, val: super::vals::ExtipinsellExtipinsel3) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel4(&self) -> super::vals::Extipinsel4 {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Extipinsel4::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel4(&mut self, val: super::vals::Extipinsel4) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel5(&self) -> super::vals::Extipinsel5 {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Extipinsel5::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel5(&mut self, val: super::vals::Extipinsel5) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel6(&self) -> super::vals::Extipinsel6 {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Extipinsel6::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel6(&mut self, val: super::vals::Extipinsel6) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "External Interrupt Pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipinsel7(&self) -> super::vals::Extipinsel7 {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Extipinsel7::from_bits(val as u8)
        }
        #[doc = "External Interrupt Pin select."]
        #[inline(always)]
        pub const fn set_extipinsel7(&mut self, val: super::vals::Extipinsel7) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Extipinsell {
        #[inline(always)]
        fn default() -> Extipinsell {
            Extipinsell(0)
        }
    }
    impl core::fmt::Debug for Extipinsell {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extipinsell")
                .field("extipinsel0", &self.extipinsel0())
                .field("extipinsel1", &self.extipinsel1())
                .field("extipinsel2", &self.extipinsel2())
                .field("extipinsel3", &self.extipinsel3())
                .field("extipinsel4", &self.extipinsel4())
                .field("extipinsel5", &self.extipinsel5())
                .field("extipinsel6", &self.extipinsel6())
                .field("extipinsel7", &self.extipinsel7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extipinsell {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Extipinsell {{ extipinsel0: {:?}, extipinsel1: {:?}, extipinsel2: {:?}, extipinsel3: {:?}, extipinsel4: {:?}, extipinsel5: {:?}, extipinsel6: {:?}, extipinsel7: {:?} }}",
                self.extipinsel0(),
                self.extipinsel1(),
                self.extipinsel2(),
                self.extipinsel3(),
                self.extipinsel4(),
                self.extipinsel5(),
                self.extipinsel6(),
                self.extipinsel7()
            )
        }
    }
    #[doc = "External interrupt Port Select High."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extipselh(pub u32);
    impl Extipselh {
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel0(&self) -> super::vals::ExtipselhExtipsel0 {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::ExtipselhExtipsel0::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel0(&mut self, val: super::vals::ExtipselhExtipsel0) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel1(&self) -> super::vals::ExtipselhExtipsel1 {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::ExtipselhExtipsel1::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel1(&mut self, val: super::vals::ExtipselhExtipsel1) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel2(&self) -> super::vals::ExtipselhExtipsel2 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ExtipselhExtipsel2::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel2(&mut self, val: super::vals::ExtipselhExtipsel2) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel3(&self) -> super::vals::ExtipselhExtipsel3 {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::ExtipselhExtipsel3::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel3(&mut self, val: super::vals::ExtipselhExtipsel3) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Extipselh {
        #[inline(always)]
        fn default() -> Extipselh {
            Extipselh(0)
        }
    }
    impl core::fmt::Debug for Extipselh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extipselh")
                .field("extipsel0", &self.extipsel0())
                .field("extipsel1", &self.extipsel1())
                .field("extipsel2", &self.extipsel2())
                .field("extipsel3", &self.extipsel3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extipselh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Extipselh {{ extipsel0: {:?}, extipsel1: {:?}, extipsel2: {:?}, extipsel3: {:?} }}",
                self.extipsel0(),
                self.extipsel1(),
                self.extipsel2(),
                self.extipsel3()
            )
        }
    }
    #[doc = "External Interrupt Port Select Low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extipsell(pub u32);
    impl Extipsell {
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel0(&self) -> super::vals::ExtipsellExtipsel0 {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::ExtipsellExtipsel0::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel0(&mut self, val: super::vals::ExtipsellExtipsel0) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel1(&self) -> super::vals::ExtipsellExtipsel1 {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::ExtipsellExtipsel1::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel1(&mut self, val: super::vals::ExtipsellExtipsel1) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel2(&self) -> super::vals::ExtipsellExtipsel2 {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ExtipsellExtipsel2::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel2(&mut self, val: super::vals::ExtipsellExtipsel2) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel3(&self) -> super::vals::ExtipsellExtipsel3 {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::ExtipsellExtipsel3::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel3(&mut self, val: super::vals::ExtipsellExtipsel3) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel4(&self) -> super::vals::Extipsel4 {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Extipsel4::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel4(&mut self, val: super::vals::Extipsel4) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel5(&self) -> super::vals::Extipsel5 {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Extipsel5::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel5(&mut self, val: super::vals::Extipsel5) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel6(&self) -> super::vals::Extipsel6 {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Extipsel6::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel6(&mut self, val: super::vals::Extipsel6) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "External Interrupt Port Select."]
        #[must_use]
        #[inline(always)]
        pub const fn extipsel7(&self) -> super::vals::Extipsel7 {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Extipsel7::from_bits(val as u8)
        }
        #[doc = "External Interrupt Port Select."]
        #[inline(always)]
        pub const fn set_extipsel7(&mut self, val: super::vals::Extipsel7) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Extipsell {
        #[inline(always)]
        fn default() -> Extipsell {
            Extipsell(0)
        }
    }
    impl core::fmt::Debug for Extipsell {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extipsell")
                .field("extipsel0", &self.extipsel0())
                .field("extipsel1", &self.extipsel1())
                .field("extipsel2", &self.extipsel2())
                .field("extipsel3", &self.extipsel3())
                .field("extipsel4", &self.extipsel4())
                .field("extipsel5", &self.extipsel5())
                .field("extipsel6", &self.extipsel6())
                .field("extipsel7", &self.extipsel7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extipsell {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Extipsell {{ extipsel0: {:?}, extipsel1: {:?}, extipsel2: {:?}, extipsel3: {:?}, extipsel4: {:?}, extipsel5: {:?}, extipsel6: {:?}, extipsel7: {:?} }}",
                self.extipsel0(),
                self.extipsel1(),
                self.extipsel2(),
                self.extipsel3(),
                self.extipsel4(),
                self.extipsel5(),
                self.extipsel6(),
                self.extipsel7()
            )
        }
    }
    #[doc = "External Interrupt Rising Edge Trigger."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extirise(pub u32);
    impl Extirise {
        #[doc = "EXT Int Rise."]
        #[must_use]
        #[inline(always)]
        pub const fn extirise(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "EXT Int Rise."]
        #[inline(always)]
        pub const fn set_extirise(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Extirise {
        #[inline(always)]
        fn default() -> Extirise {
            Extirise(0)
        }
    }
    impl core::fmt::Debug for Extirise {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Extirise").field("extirise", &self.extirise()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Extirise {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Extirise {{ extirise: {=u16:?} }}", self.extirise())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Femroutepen(pub u32);
    impl Femroutepen {
        #[doc = "FEM Data0 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn femdata0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FEM Data0 Pin Enable."]
        #[inline(always)]
        pub const fn set_femdata0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FEM Data1 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn femdata1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FEM Data1 Pin Enable."]
        #[inline(always)]
        pub const fn set_femdata1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FEM Data2 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn femdata2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FEM Data2 Pin Enable."]
        #[inline(always)]
        pub const fn set_femdata2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FEM Data3 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn femdata3pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FEM Data3 Pin Enable."]
        #[inline(always)]
        pub const fn set_femdata3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Femroutepen {
        #[inline(always)]
        fn default() -> Femroutepen {
            Femroutepen(0)
        }
    }
    impl core::fmt::Debug for Femroutepen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Femroutepen")
                .field("femdata0pen", &self.femdata0pen())
                .field("femdata1pen", &self.femdata1pen())
                .field("femdata2pen", &self.femdata2pen())
                .field("femdata3pen", &self.femdata3pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Femroutepen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Femroutepen {{ femdata0pen: {=bool:?}, femdata1pen: {=bool:?}, femdata2pen: {=bool:?}, femdata3pen: {=bool:?} }}",
                self.femdata0pen(),
                self.femdata1pen(),
                self.femdata2pen(),
                self.femdata3pen()
            )
        }
    }
    #[doc = "DCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FrcDclkroute(pub u32);
    impl FrcDclkroute {
        #[doc = "DCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FrcDclkroute {
        #[inline(always)]
        fn default() -> FrcDclkroute {
            FrcDclkroute(0)
        }
    }
    impl core::fmt::Debug for FrcDclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FrcDclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FrcDclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FrcDclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DFRAME port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FrcDframeroute(pub u32);
    impl FrcDframeroute {
        #[doc = "DFRAME port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DFRAME port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DFRAME pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DFRAME pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FrcDframeroute {
        #[inline(always)]
        fn default() -> FrcDframeroute {
            FrcDframeroute(0)
        }
    }
    impl core::fmt::Debug for FrcDframeroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FrcDframeroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FrcDframeroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FrcDframeroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DOUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FrcDoutroute(pub u32);
    impl FrcDoutroute {
        #[doc = "DOUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DOUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DOUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DOUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for FrcDoutroute {
        #[inline(always)]
        fn default() -> FrcDoutroute {
            FrcDoutroute(0)
        }
    }
    impl core::fmt::Debug for FrcDoutroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FrcDoutroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FrcDoutroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FrcDoutroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "FRC pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FrcRouteen(pub u32);
    impl FrcRouteen {
        #[doc = "DCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dclkpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_dclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DFRAME pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dframepen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DFRAME pin enable control bit."]
        #[inline(always)]
        pub const fn set_dframepen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DOUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn doutpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DOUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_doutpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for FrcRouteen {
        #[inline(always)]
        fn default() -> FrcRouteen {
            FrcRouteen(0)
        }
    }
    impl core::fmt::Debug for FrcRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FrcRouteen")
                .field("dclkpen", &self.dclkpen())
                .field("dframepen", &self.dframepen())
                .field("doutpen", &self.doutpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FrcRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FrcRouteen {{ dclkpen: {=bool:?}, dframepen: {=bool:?}, doutpen: {=bool:?} }}",
                self.dclkpen(),
                self.dframepen(),
                self.doutpen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpiolockstatus(pub u32);
    impl Gpiolockstatus {
        #[doc = "GPIO LOCK status."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO LOCK status."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Gpiolockstatus {
        #[inline(always)]
        fn default() -> Gpiolockstatus {
            Gpiolockstatus(0)
        }
    }
    impl core::fmt::Debug for Gpiolockstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpiolockstatus").field("lock", &self.lock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpiolockstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpiolockstatus {{ lock: {=bool:?} }}", self.lock())
        }
    }
    #[doc = "I2C0 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c0Routeen(pub u32);
    impl I2c0Routeen {
        #[doc = "SCL pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SCL pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SDA pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sdapen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SDA pin enable control bit."]
        #[inline(always)]
        pub const fn set_sdapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for I2c0Routeen {
        #[inline(always)]
        fn default() -> I2c0Routeen {
            I2c0Routeen(0)
        }
    }
    impl core::fmt::Debug for I2c0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2c0Routeen")
                .field("sclpen", &self.sclpen())
                .field("sdapen", &self.sdapen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2c0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2c0Routeen {{ sclpen: {=bool:?}, sdapen: {=bool:?} }}",
                self.sclpen(),
                self.sdapen()
            )
        }
    }
    #[doc = "SCL port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c0Sclroute(pub u32);
    impl I2c0Sclroute {
        #[doc = "SCL port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCL port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCL pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCL pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for I2c0Sclroute {
        #[inline(always)]
        fn default() -> I2c0Sclroute {
            I2c0Sclroute(0)
        }
    }
    impl core::fmt::Debug for I2c0Sclroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2c0Sclroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2c0Sclroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2c0Sclroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SDA port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c0Sdaroute(pub u32);
    impl I2c0Sdaroute {
        #[doc = "SDA port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SDA port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SDA pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SDA pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for I2c0Sdaroute {
        #[inline(always)]
        fn default() -> I2c0Sdaroute {
            I2c0Sdaroute(0)
        }
    }
    impl core::fmt::Debug for I2c0Sdaroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2c0Sdaroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2c0Sdaroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2c0Sdaroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "I2C1 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c1Routeen(pub u32);
    impl I2c1Routeen {
        #[doc = "SCL pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sclpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SCL pin enable control bit."]
        #[inline(always)]
        pub const fn set_sclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SDA pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sdapen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SDA pin enable control bit."]
        #[inline(always)]
        pub const fn set_sdapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for I2c1Routeen {
        #[inline(always)]
        fn default() -> I2c1Routeen {
            I2c1Routeen(0)
        }
    }
    impl core::fmt::Debug for I2c1Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2c1Routeen")
                .field("sclpen", &self.sclpen())
                .field("sdapen", &self.sdapen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2c1Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2c1Routeen {{ sclpen: {=bool:?}, sdapen: {=bool:?} }}",
                self.sclpen(),
                self.sdapen()
            )
        }
    }
    #[doc = "SCL port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c1Sclroute(pub u32);
    impl I2c1Sclroute {
        #[doc = "SCL port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SCL port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SCL pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SCL pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for I2c1Sclroute {
        #[inline(always)]
        fn default() -> I2c1Sclroute {
            I2c1Sclroute(0)
        }
    }
    impl core::fmt::Debug for I2c1Sclroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2c1Sclroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2c1Sclroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2c1Sclroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SDA port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2c1Sdaroute(pub u32);
    impl I2c1Sdaroute {
        #[doc = "SDA port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SDA port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SDA pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SDA pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for I2c1Sdaroute {
        #[inline(always)]
        fn default() -> I2c1Sdaroute {
            I2c1Sdaroute(0)
        }
    }
    impl core::fmt::Debug for I2c1Sdaroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2c1Sdaroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2c1Sdaroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I2c1Sdaroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "Interrupt Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ien(pub u32);
    impl Ien {
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "External Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extien11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Enable."]
        #[inline(always)]
        pub const fn set_extien11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wuien11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "EM4 Wake Up Interrupt En."]
        #[inline(always)]
        pub const fn set_em4wuien11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("extien0", &self.extien0())
                .field("extien1", &self.extien1())
                .field("extien2", &self.extien2())
                .field("extien3", &self.extien3())
                .field("extien4", &self.extien4())
                .field("extien5", &self.extien5())
                .field("extien6", &self.extien6())
                .field("extien7", &self.extien7())
                .field("extien8", &self.extien8())
                .field("extien9", &self.extien9())
                .field("extien10", &self.extien10())
                .field("extien11", &self.extien11())
                .field("em4wuien0", &self.em4wuien0())
                .field("em4wuien1", &self.em4wuien1())
                .field("em4wuien2", &self.em4wuien2())
                .field("em4wuien3", &self.em4wuien3())
                .field("em4wuien4", &self.em4wuien4())
                .field("em4wuien5", &self.em4wuien5())
                .field("em4wuien6", &self.em4wuien6())
                .field("em4wuien7", &self.em4wuien7())
                .field("em4wuien8", &self.em4wuien8())
                .field("em4wuien9", &self.em4wuien9())
                .field("em4wuien10", &self.em4wuien10())
                .field("em4wuien11", &self.em4wuien11())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ien {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ien {{ extien0: {=bool:?}, extien1: {=bool:?}, extien2: {=bool:?}, extien3: {=bool:?}, extien4: {=bool:?}, extien5: {=bool:?}, extien6: {=bool:?}, extien7: {=bool:?}, extien8: {=bool:?}, extien9: {=bool:?}, extien10: {=bool:?}, extien11: {=bool:?}, em4wuien0: {=bool:?}, em4wuien1: {=bool:?}, em4wuien2: {=bool:?}, em4wuien3: {=bool:?}, em4wuien4: {=bool:?}, em4wuien5: {=bool:?}, em4wuien6: {=bool:?}, em4wuien7: {=bool:?}, em4wuien8: {=bool:?}, em4wuien9: {=bool:?}, em4wuien10: {=bool:?}, em4wuien11: {=bool:?} }}",
                self.extien0(),
                self.extien1(),
                self.extien2(),
                self.extien3(),
                self.extien4(),
                self.extien5(),
                self.extien6(),
                self.extien7(),
                self.extien8(),
                self.extien9(),
                self.extien10(),
                self.extien11(),
                self.em4wuien0(),
                self.em4wuien1(),
                self.em4wuien2(),
                self.em4wuien3(),
                self.em4wuien4(),
                self.em4wuien5(),
                self.em4wuien6(),
                self.em4wuien7(),
                self.em4wuien8(),
                self.em4wuien9(),
                self.em4wuien10(),
                self.em4wuien11()
            )
        }
    }
    #[doc = "Interrupt Flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct If(pub u32);
    impl If {
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "External Pin Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn extif11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "External Pin Flag."]
        #[inline(always)]
        pub const fn set_extif11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "EM4 wake up."]
        #[must_use]
        #[inline(always)]
        pub const fn em4wu(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "EM4 wake up."]
        #[inline(always)]
        pub const fn set_em4wu(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
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
                .field("extif0", &self.extif0())
                .field("extif1", &self.extif1())
                .field("extif2", &self.extif2())
                .field("extif3", &self.extif3())
                .field("extif4", &self.extif4())
                .field("extif5", &self.extif5())
                .field("extif6", &self.extif6())
                .field("extif7", &self.extif7())
                .field("extif8", &self.extif8())
                .field("extif9", &self.extif9())
                .field("extif10", &self.extif10())
                .field("extif11", &self.extif11())
                .field("em4wu", &self.em4wu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for If {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "If {{ extif0: {=bool:?}, extif1: {=bool:?}, extif2: {=bool:?}, extif3: {=bool:?}, extif4: {=bool:?}, extif5: {=bool:?}, extif6: {=bool:?}, extif7: {=bool:?}, extif8: {=bool:?}, extif9: {=bool:?}, extif10: {=bool:?}, extif11: {=bool:?}, em4wu: {=u16:?} }}",
                self.extif0(),
                self.extif1(),
                self.extif2(),
                self.extif3(),
                self.extif4(),
                self.extif5(),
                self.extif6(),
                self.extif7(),
                self.extif8(),
                self.extif9(),
                self.extif10(),
                self.extif11(),
                self.em4wu()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "ip version id."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ip version id."]
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
    #[doc = "CH0OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh0outroute(pub u32);
    impl LesenseCh0outroute {
        #[doc = "CH0OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH0OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH0OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH0OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh0outroute {
        #[inline(always)]
        fn default() -> LesenseCh0outroute {
            LesenseCh0outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh0outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh0outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh0outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh0outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH10OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh10outroute(pub u32);
    impl LesenseCh10outroute {
        #[doc = "CH10OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH10OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH10OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH10OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh10outroute {
        #[inline(always)]
        fn default() -> LesenseCh10outroute {
            LesenseCh10outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh10outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh10outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh10outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh10outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH11OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh11outroute(pub u32);
    impl LesenseCh11outroute {
        #[doc = "CH11OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH11OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH11OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH11OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh11outroute {
        #[inline(always)]
        fn default() -> LesenseCh11outroute {
            LesenseCh11outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh11outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh11outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh11outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh11outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH12OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh12outroute(pub u32);
    impl LesenseCh12outroute {
        #[doc = "CH12OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH12OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH12OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH12OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh12outroute {
        #[inline(always)]
        fn default() -> LesenseCh12outroute {
            LesenseCh12outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh12outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh12outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh12outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh12outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH13OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh13outroute(pub u32);
    impl LesenseCh13outroute {
        #[doc = "CH13OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH13OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH13OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH13OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh13outroute {
        #[inline(always)]
        fn default() -> LesenseCh13outroute {
            LesenseCh13outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh13outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh13outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh13outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh13outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH14OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh14outroute(pub u32);
    impl LesenseCh14outroute {
        #[doc = "CH14OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH14OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH14OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH14OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh14outroute {
        #[inline(always)]
        fn default() -> LesenseCh14outroute {
            LesenseCh14outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh14outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh14outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh14outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh14outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH15OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh15outroute(pub u32);
    impl LesenseCh15outroute {
        #[doc = "CH15OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH15OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH15OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH15OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh15outroute {
        #[inline(always)]
        fn default() -> LesenseCh15outroute {
            LesenseCh15outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh15outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh15outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh15outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh15outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH1OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh1outroute(pub u32);
    impl LesenseCh1outroute {
        #[doc = "CH1OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH1OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH1OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH1OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh1outroute {
        #[inline(always)]
        fn default() -> LesenseCh1outroute {
            LesenseCh1outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh1outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh1outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh1outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh1outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH2OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh2outroute(pub u32);
    impl LesenseCh2outroute {
        #[doc = "CH2OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH2OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH2OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH2OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh2outroute {
        #[inline(always)]
        fn default() -> LesenseCh2outroute {
            LesenseCh2outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh2outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh2outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh2outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh2outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH3OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh3outroute(pub u32);
    impl LesenseCh3outroute {
        #[doc = "CH3OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH3OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH3OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH3OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh3outroute {
        #[inline(always)]
        fn default() -> LesenseCh3outroute {
            LesenseCh3outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh3outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh3outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh3outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh3outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH4OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh4outroute(pub u32);
    impl LesenseCh4outroute {
        #[doc = "CH4OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH4OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH4OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH4OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh4outroute {
        #[inline(always)]
        fn default() -> LesenseCh4outroute {
            LesenseCh4outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh4outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh4outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh4outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh4outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH5OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh5outroute(pub u32);
    impl LesenseCh5outroute {
        #[doc = "CH5OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH5OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH5OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH5OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh5outroute {
        #[inline(always)]
        fn default() -> LesenseCh5outroute {
            LesenseCh5outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh5outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh5outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh5outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh5outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH6OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh6outroute(pub u32);
    impl LesenseCh6outroute {
        #[doc = "CH6OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH6OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH6OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH6OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh6outroute {
        #[inline(always)]
        fn default() -> LesenseCh6outroute {
            LesenseCh6outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh6outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh6outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh6outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh6outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH7OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh7outroute(pub u32);
    impl LesenseCh7outroute {
        #[doc = "CH7OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH7OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH7OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH7OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh7outroute {
        #[inline(always)]
        fn default() -> LesenseCh7outroute {
            LesenseCh7outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh7outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh7outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh7outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh7outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH8OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh8outroute(pub u32);
    impl LesenseCh8outroute {
        #[doc = "CH8OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH8OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH8OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH8OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh8outroute {
        #[inline(always)]
        fn default() -> LesenseCh8outroute {
            LesenseCh8outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh8outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh8outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh8outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh8outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CH9OUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseCh9outroute(pub u32);
    impl LesenseCh9outroute {
        #[doc = "CH9OUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CH9OUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CH9OUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CH9OUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LesenseCh9outroute {
        #[inline(always)]
        fn default() -> LesenseCh9outroute {
            LesenseCh9outroute(0)
        }
    }
    impl core::fmt::Debug for LesenseCh9outroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseCh9outroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseCh9outroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseCh9outroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "LESENSE pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LesenseRouteen(pub u32);
    impl LesenseRouteen {
        #[doc = "CH0OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0outpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CH0OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch0outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CH1OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1outpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CH1OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch1outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CH2OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2outpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CH2OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch2outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CH3OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3outpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CH3OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch3outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CH4OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4outpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CH4OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch4outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CH5OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5outpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CH5OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch5outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CH6OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6outpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CH6OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch6outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CH7OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7outpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CH7OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch7outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CH8OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8outpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CH8OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch8outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CH9OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9outpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CH9OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch9outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CH10OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10outpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CH10OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch10outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CH11OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11outpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CH11OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch11outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CH12OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch12outpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CH12OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch12outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CH13OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch13outpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CH13OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch13outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CH14OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch14outpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CH14OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch14outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CH15OUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ch15outpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CH15OUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_ch15outpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for LesenseRouteen {
        #[inline(always)]
        fn default() -> LesenseRouteen {
            LesenseRouteen(0)
        }
    }
    impl core::fmt::Debug for LesenseRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LesenseRouteen")
                .field("ch0outpen", &self.ch0outpen())
                .field("ch1outpen", &self.ch1outpen())
                .field("ch2outpen", &self.ch2outpen())
                .field("ch3outpen", &self.ch3outpen())
                .field("ch4outpen", &self.ch4outpen())
                .field("ch5outpen", &self.ch5outpen())
                .field("ch6outpen", &self.ch6outpen())
                .field("ch7outpen", &self.ch7outpen())
                .field("ch8outpen", &self.ch8outpen())
                .field("ch9outpen", &self.ch9outpen())
                .field("ch10outpen", &self.ch10outpen())
                .field("ch11outpen", &self.ch11outpen())
                .field("ch12outpen", &self.ch12outpen())
                .field("ch13outpen", &self.ch13outpen())
                .field("ch14outpen", &self.ch14outpen())
                .field("ch15outpen", &self.ch15outpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LesenseRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LesenseRouteen {{ ch0outpen: {=bool:?}, ch1outpen: {=bool:?}, ch2outpen: {=bool:?}, ch3outpen: {=bool:?}, ch4outpen: {=bool:?}, ch5outpen: {=bool:?}, ch6outpen: {=bool:?}, ch7outpen: {=bool:?}, ch8outpen: {=bool:?}, ch9outpen: {=bool:?}, ch10outpen: {=bool:?}, ch11outpen: {=bool:?}, ch12outpen: {=bool:?}, ch13outpen: {=bool:?}, ch14outpen: {=bool:?}, ch15outpen: {=bool:?} }}",
                self.ch0outpen(),
                self.ch1outpen(),
                self.ch2outpen(),
                self.ch3outpen(),
                self.ch4outpen(),
                self.ch5outpen(),
                self.ch6outpen(),
                self.ch7outpen(),
                self.ch8outpen(),
                self.ch9outpen(),
                self.ch10outpen(),
                self.ch11outpen(),
                self.ch12outpen(),
                self.ch13outpen(),
                self.ch14outpen(),
                self.ch15outpen()
            )
        }
    }
    #[doc = "OUT0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LetimerOut0route(pub u32);
    impl LetimerOut0route {
        #[doc = "OUT0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "OUT0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "OUT0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "OUT0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LetimerOut0route {
        #[inline(always)]
        fn default() -> LetimerOut0route {
            LetimerOut0route(0)
        }
    }
    impl core::fmt::Debug for LetimerOut0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LetimerOut0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LetimerOut0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LetimerOut0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "OUT1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LetimerOut1route(pub u32);
    impl LetimerOut1route {
        #[doc = "OUT1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "OUT1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "OUT1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "OUT1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LetimerOut1route {
        #[inline(always)]
        fn default() -> LetimerOut1route {
            LetimerOut1route(0)
        }
    }
    impl core::fmt::Debug for LetimerOut1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LetimerOut1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LetimerOut1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LetimerOut1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "LETIMER pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LetimerRouteen(pub u32);
    impl LetimerRouteen {
        #[doc = "OUT0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn out0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OUT0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_out0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OUT1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn out1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OUT1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_out1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for LetimerRouteen {
        #[inline(always)]
        fn default() -> LetimerRouteen {
            LetimerRouteen(0)
        }
    }
    impl core::fmt::Debug for LetimerRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LetimerRouteen")
                .field("out0pen", &self.out0pen())
                .field("out1pen", &self.out1pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LetimerRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LetimerRouteen {{ out0pen: {=bool:?}, out1pen: {=bool:?} }}",
                self.out0pen(),
                self.out1pen()
            )
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
        pub const fn lockkey(&self) -> super::vals::Lockkey {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Lockkey::from_bits(val as u16)
        }
        #[doc = "Configuration Lock Key."]
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
    #[doc = "ANT0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAnt0route(pub u32);
    impl ModemAnt0route {
        #[doc = "ANT0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANT0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANT0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANT0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAnt0route {
        #[inline(always)]
        fn default() -> ModemAnt0route {
            ModemAnt0route(0)
        }
    }
    impl core::fmt::Debug for ModemAnt0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAnt0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAnt0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAnt0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANT1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAnt1route(pub u32);
    impl ModemAnt1route {
        #[doc = "ANT1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANT1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANT1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANT1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAnt1route {
        #[inline(always)]
        fn default() -> ModemAnt1route {
            ModemAnt1route(0)
        }
    }
    impl core::fmt::Debug for ModemAnt1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAnt1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAnt1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAnt1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTROLLOVER port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrolloverroute(pub u32);
    impl ModemAntrolloverroute {
        #[doc = "ANTROLLOVER port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTROLLOVER port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTROLLOVER pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTROLLOVER pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrolloverroute {
        #[inline(always)]
        fn default() -> ModemAntrolloverroute {
            ModemAntrolloverroute(0)
        }
    }
    impl core::fmt::Debug for ModemAntrolloverroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrolloverroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrolloverroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrolloverroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTRR0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrr0route(pub u32);
    impl ModemAntrr0route {
        #[doc = "ANTRR0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTRR0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTRR0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTRR0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrr0route {
        #[inline(always)]
        fn default() -> ModemAntrr0route {
            ModemAntrr0route(0)
        }
    }
    impl core::fmt::Debug for ModemAntrr0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrr0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrr0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrr0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTRR1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrr1route(pub u32);
    impl ModemAntrr1route {
        #[doc = "ANTRR1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTRR1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTRR1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTRR1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrr1route {
        #[inline(always)]
        fn default() -> ModemAntrr1route {
            ModemAntrr1route(0)
        }
    }
    impl core::fmt::Debug for ModemAntrr1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrr1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrr1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrr1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTRR2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrr2route(pub u32);
    impl ModemAntrr2route {
        #[doc = "ANTRR2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTRR2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTRR2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTRR2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrr2route {
        #[inline(always)]
        fn default() -> ModemAntrr2route {
            ModemAntrr2route(0)
        }
    }
    impl core::fmt::Debug for ModemAntrr2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrr2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrr2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrr2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTRR3 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrr3route(pub u32);
    impl ModemAntrr3route {
        #[doc = "ANTRR3 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTRR3 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTRR3 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTRR3 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrr3route {
        #[inline(always)]
        fn default() -> ModemAntrr3route {
            ModemAntrr3route(0)
        }
    }
    impl core::fmt::Debug for ModemAntrr3route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrr3route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrr3route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrr3route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTRR4 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrr4route(pub u32);
    impl ModemAntrr4route {
        #[doc = "ANTRR4 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTRR4 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTRR4 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTRR4 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrr4route {
        #[inline(always)]
        fn default() -> ModemAntrr4route {
            ModemAntrr4route(0)
        }
    }
    impl core::fmt::Debug for ModemAntrr4route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrr4route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrr4route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrr4route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTRR5 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntrr5route(pub u32);
    impl ModemAntrr5route {
        #[doc = "ANTRR5 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTRR5 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTRR5 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTRR5 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntrr5route {
        #[inline(always)]
        fn default() -> ModemAntrr5route {
            ModemAntrr5route(0)
        }
    }
    impl core::fmt::Debug for ModemAntrr5route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntrr5route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntrr5route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntrr5route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTSWEN port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntswenroute(pub u32);
    impl ModemAntswenroute {
        #[doc = "ANTSWEN port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTSWEN port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTSWEN pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTSWEN pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntswenroute {
        #[inline(always)]
        fn default() -> ModemAntswenroute {
            ModemAntswenroute(0)
        }
    }
    impl core::fmt::Debug for ModemAntswenroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntswenroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntswenroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntswenroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTSWUS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAntswusroute(pub u32);
    impl ModemAntswusroute {
        #[doc = "ANTSWUS port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTSWUS port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTSWUS pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTSWUS pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAntswusroute {
        #[inline(always)]
        fn default() -> ModemAntswusroute {
            ModemAntswusroute(0)
        }
    }
    impl core::fmt::Debug for ModemAntswusroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAntswusroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAntswusroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAntswusroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTTRIG port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAnttrigroute(pub u32);
    impl ModemAnttrigroute {
        #[doc = "ANTTRIG port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTTRIG port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTTRIG pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTTRIG pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAnttrigroute {
        #[inline(always)]
        fn default() -> ModemAnttrigroute {
            ModemAnttrigroute(0)
        }
    }
    impl core::fmt::Debug for ModemAnttrigroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAnttrigroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAnttrigroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAnttrigroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ANTTRIGSTOP port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemAnttrigstoproute(pub u32);
    impl ModemAnttrigstoproute {
        #[doc = "ANTTRIGSTOP port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ANTTRIGSTOP port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ANTTRIGSTOP pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ANTTRIGSTOP pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemAnttrigstoproute {
        #[inline(always)]
        fn default() -> ModemAnttrigstoproute {
            ModemAnttrigstoproute(0)
        }
    }
    impl core::fmt::Debug for ModemAnttrigstoproute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemAnttrigstoproute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemAnttrigstoproute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemAnttrigstoproute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemDclkroute(pub u32);
    impl ModemDclkroute {
        #[doc = "DCLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DCLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DCLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DCLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemDclkroute {
        #[inline(always)]
        fn default() -> ModemDclkroute {
            ModemDclkroute(0)
        }
    }
    impl core::fmt::Debug for ModemDclkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemDclkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemDclkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemDclkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DIN port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemDinroute(pub u32);
    impl ModemDinroute {
        #[doc = "DIN port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DIN port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DIN pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DIN pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemDinroute {
        #[inline(always)]
        fn default() -> ModemDinroute {
            ModemDinroute(0)
        }
    }
    impl core::fmt::Debug for ModemDinroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemDinroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemDinroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemDinroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DOUT port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemDoutroute(pub u32);
    impl ModemDoutroute {
        #[doc = "DOUT port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DOUT port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DOUT pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DOUT pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for ModemDoutroute {
        #[inline(always)]
        fn default() -> ModemDoutroute {
            ModemDoutroute(0)
        }
    }
    impl core::fmt::Debug for ModemDoutroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemDoutroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemDoutroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemDoutroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "MODEM pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModemRouteen(pub u32);
    impl ModemRouteen {
        #[doc = "ANT0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ant0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ANT0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ant0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ANT1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ant1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ANT1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ant1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ANTROLLOVER pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrolloverpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ANTROLLOVER pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrolloverpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ANTRR0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrr0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ANTRR0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrr0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ANTRR1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrr1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ANTRR1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrr1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ANTRR2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrr2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ANTRR2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrr2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ANTRR3 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrr3pen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ANTRR3 pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrr3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ANTRR4 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrr4pen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ANTRR4 pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrr4pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ANTRR5 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antrr5pen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ANTRR5 pin enable control bit."]
        #[inline(always)]
        pub const fn set_antrr5pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ANTSWEN pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antswenpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ANTSWEN pin enable control bit."]
        #[inline(always)]
        pub const fn set_antswenpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ANTSWUS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn antswuspen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ANTSWUS pin enable control bit."]
        #[inline(always)]
        pub const fn set_antswuspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ANTTRIG pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn anttrigpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ANTTRIG pin enable control bit."]
        #[inline(always)]
        pub const fn set_anttrigpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ANTTRIGSTOP pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn anttrigstoppen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ANTTRIGSTOP pin enable control bit."]
        #[inline(always)]
        pub const fn set_anttrigstoppen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DCLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dclkpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_dclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DOUT pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn doutpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DOUT pin enable control bit."]
        #[inline(always)]
        pub const fn set_doutpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for ModemRouteen {
        #[inline(always)]
        fn default() -> ModemRouteen {
            ModemRouteen(0)
        }
    }
    impl core::fmt::Debug for ModemRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModemRouteen")
                .field("ant0pen", &self.ant0pen())
                .field("ant1pen", &self.ant1pen())
                .field("antrolloverpen", &self.antrolloverpen())
                .field("antrr0pen", &self.antrr0pen())
                .field("antrr1pen", &self.antrr1pen())
                .field("antrr2pen", &self.antrr2pen())
                .field("antrr3pen", &self.antrr3pen())
                .field("antrr4pen", &self.antrr4pen())
                .field("antrr5pen", &self.antrr5pen())
                .field("antswenpen", &self.antswenpen())
                .field("antswuspen", &self.antswuspen())
                .field("anttrigpen", &self.anttrigpen())
                .field("anttrigstoppen", &self.anttrigstoppen())
                .field("dclkpen", &self.dclkpen())
                .field("doutpen", &self.doutpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ModemRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ModemRouteen {{ ant0pen: {=bool:?}, ant1pen: {=bool:?}, antrolloverpen: {=bool:?}, antrr0pen: {=bool:?}, antrr1pen: {=bool:?}, antrr2pen: {=bool:?}, antrr3pen: {=bool:?}, antrr4pen: {=bool:?}, antrr5pen: {=bool:?}, antswenpen: {=bool:?}, antswuspen: {=bool:?}, anttrigpen: {=bool:?}, anttrigstoppen: {=bool:?}, dclkpen: {=bool:?}, doutpen: {=bool:?} }}",
                self.ant0pen(),
                self.ant1pen(),
                self.antrolloverpen(),
                self.antrr0pen(),
                self.antrr1pen(),
                self.antrr2pen(),
                self.antrr3pen(),
                self.antrr4pen(),
                self.antrr5pen(),
                self.antswenpen(),
                self.antswuspen(),
                self.anttrigpen(),
                self.anttrigstoppen(),
                self.dclkpen(),
                self.doutpen()
            )
        }
    }
    #[doc = "S0IN port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnt0S0inroute(pub u32);
    impl Pcnt0S0inroute {
        #[doc = "S0IN port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "S0IN port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "S0IN pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "S0IN pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Pcnt0S0inroute {
        #[inline(always)]
        fn default() -> Pcnt0S0inroute {
            Pcnt0S0inroute(0)
        }
    }
    impl core::fmt::Debug for Pcnt0S0inroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnt0S0inroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcnt0S0inroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcnt0S0inroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "S1IN port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcnt0S1inroute(pub u32);
    impl Pcnt0S1inroute {
        #[doc = "S1IN port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "S1IN port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "S1IN pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "S1IN pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Pcnt0S1inroute {
        #[inline(always)]
        fn default() -> Pcnt0S1inroute {
            Pcnt0S1inroute(0)
        }
    }
    impl core::fmt::Debug for Pcnt0S1inroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcnt0S1inroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcnt0S1inroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcnt0S1inroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "Port control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortCtrl(pub u32);
    impl PortCtrl {
        #[doc = "Slew Rate."]
        #[must_use]
        #[inline(always)]
        pub const fn slewrate(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Slew Rate."]
        #[inline(always)]
        pub const fn set_slewrate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Data In Disable."]
        #[must_use]
        #[inline(always)]
        pub const fn dindis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data In Disable."]
        #[inline(always)]
        pub const fn set_dindis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Slew Rate Alt."]
        #[must_use]
        #[inline(always)]
        pub const fn slewratealt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Slew Rate Alt."]
        #[inline(always)]
        pub const fn set_slewratealt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Data In Disable Alt."]
        #[must_use]
        #[inline(always)]
        pub const fn dindisalt(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Data In Disable Alt."]
        #[inline(always)]
        pub const fn set_dindisalt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for PortCtrl {
        #[inline(always)]
        fn default() -> PortCtrl {
            PortCtrl(0)
        }
    }
    impl core::fmt::Debug for PortCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortCtrl")
                .field("slewrate", &self.slewrate())
                .field("dindis", &self.dindis())
                .field("slewratealt", &self.slewratealt())
                .field("dindisalt", &self.dindisalt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PortCtrl {{ slewrate: {=u8:?}, dindis: {=bool:?}, slewratealt: {=u8:?}, dindisalt: {=bool:?} }}",
                self.slewrate(),
                self.dindis(),
                self.slewratealt(),
                self.dindisalt()
            )
        }
    }
    #[doc = "data in."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortDin(pub u32);
    impl PortDin {
        #[doc = "Data input."]
        #[must_use]
        #[inline(always)]
        pub const fn din(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data input."]
        #[inline(always)]
        pub const fn set_din(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for PortDin {
        #[inline(always)]
        fn default() -> PortDin {
            PortDin(0)
        }
    }
    impl core::fmt::Debug for PortDin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortDin").field("din", &self.din()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortDin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PortDin {{ din: {=u16:?} }}", self.din())
        }
    }
    #[doc = "data out."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortDout(pub u32);
    impl PortDout {
        #[doc = "Data output."]
        #[must_use]
        #[inline(always)]
        pub const fn dout(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data output."]
        #[inline(always)]
        pub const fn set_dout(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for PortDout {
        #[inline(always)]
        fn default() -> PortDout {
            PortDout(0)
        }
    }
    impl core::fmt::Debug for PortDout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortDout").field("dout", &self.dout()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortDout {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PortDout {{ dout: {=u16:?} }}", self.dout())
        }
    }
    #[doc = "mode high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortModeh(pub u32);
    impl PortModeh {
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode0(&self) -> super::vals::PortaModehMode0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::PortaModehMode0::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode0(&mut self, val: super::vals::PortaModehMode0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> super::vals::PortaModehMode1 {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::PortaModehMode1::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: super::vals::PortaModehMode1) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> super::vals::PortaModehMode2 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::PortaModehMode2::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: super::vals::PortaModehMode2) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode3(&self) -> super::vals::PortaModehMode3 {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::PortaModehMode3::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode3(&mut self, val: super::vals::PortaModehMode3) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for PortModeh {
        #[inline(always)]
        fn default() -> PortModeh {
            PortModeh(0)
        }
    }
    impl core::fmt::Debug for PortModeh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortModeh")
                .field("mode0", &self.mode0())
                .field("mode1", &self.mode1())
                .field("mode2", &self.mode2())
                .field("mode3", &self.mode3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortModeh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PortModeh {{ mode0: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?} }}",
                self.mode0(),
                self.mode1(),
                self.mode2(),
                self.mode3()
            )
        }
    }
    #[doc = "mode low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortModel(pub u32);
    impl PortModel {
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode0(&self) -> super::vals::PortaModelMode0 {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::PortaModelMode0::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode0(&mut self, val: super::vals::PortaModelMode0) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> super::vals::PortaModelMode1 {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::PortaModelMode1::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: super::vals::PortaModelMode1) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> super::vals::PortaModelMode2 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::PortaModelMode2::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: super::vals::PortaModelMode2) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode3(&self) -> super::vals::PortaModelMode3 {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::PortaModelMode3::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode3(&mut self, val: super::vals::PortaModelMode3) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode4(&self) -> super::vals::PortaModelMode4 {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::PortaModelMode4::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode4(&mut self, val: super::vals::PortaModelMode4) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode5(&self) -> super::vals::PortaModelMode5 {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::PortaModelMode5::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode5(&mut self, val: super::vals::PortaModelMode5) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode6(&self) -> super::vals::PortaModelMode6 {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::PortaModelMode6::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode6(&mut self, val: super::vals::PortaModelMode6) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "MODE n."]
        #[must_use]
        #[inline(always)]
        pub const fn mode7(&self) -> super::vals::PortaModelMode7 {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::PortaModelMode7::from_bits(val as u8)
        }
        #[doc = "MODE n."]
        #[inline(always)]
        pub const fn set_mode7(&mut self, val: super::vals::PortaModelMode7) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for PortModel {
        #[inline(always)]
        fn default() -> PortModel {
            PortModel(0)
        }
    }
    impl core::fmt::Debug for PortModel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortModel")
                .field("mode0", &self.mode0())
                .field("mode1", &self.mode1())
                .field("mode2", &self.mode2())
                .field("mode3", &self.mode3())
                .field("mode4", &self.mode4())
                .field("mode5", &self.mode5())
                .field("mode6", &self.mode6())
                .field("mode7", &self.mode7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortModel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PortModel {{ mode0: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, mode4: {:?}, mode5: {:?}, mode6: {:?}, mode7: {:?} }}",
                self.mode0(),
                self.mode1(),
                self.mode2(),
                self.mode3(),
                self.mode4(),
                self.mode5(),
                self.mode6(),
                self.mode7()
            )
        }
    }
    #[doc = "ASYNCH0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch0route(pub u32);
    impl Prs0Asynch0route {
        #[doc = "ASYNCH0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch0route {
        #[inline(always)]
        fn default() -> Prs0Asynch0route {
            Prs0Asynch0route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH10 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch10route(pub u32);
    impl Prs0Asynch10route {
        #[doc = "ASYNCH10 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH10 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH10 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH10 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch10route {
        #[inline(always)]
        fn default() -> Prs0Asynch10route {
            Prs0Asynch10route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch10route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch10route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch10route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch10route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH11 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch11route(pub u32);
    impl Prs0Asynch11route {
        #[doc = "ASYNCH11 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH11 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH11 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH11 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch11route {
        #[inline(always)]
        fn default() -> Prs0Asynch11route {
            Prs0Asynch11route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch11route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch11route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch11route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch11route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch1route(pub u32);
    impl Prs0Asynch1route {
        #[doc = "ASYNCH1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch1route {
        #[inline(always)]
        fn default() -> Prs0Asynch1route {
            Prs0Asynch1route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch2route(pub u32);
    impl Prs0Asynch2route {
        #[doc = "ASYNCH2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch2route {
        #[inline(always)]
        fn default() -> Prs0Asynch2route {
            Prs0Asynch2route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH3 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch3route(pub u32);
    impl Prs0Asynch3route {
        #[doc = "ASYNCH3 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH3 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH3 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH3 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch3route {
        #[inline(always)]
        fn default() -> Prs0Asynch3route {
            Prs0Asynch3route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch3route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch3route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch3route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch3route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH4 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch4route(pub u32);
    impl Prs0Asynch4route {
        #[doc = "ASYNCH4 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH4 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH4 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH4 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch4route {
        #[inline(always)]
        fn default() -> Prs0Asynch4route {
            Prs0Asynch4route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch4route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch4route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch4route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch4route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH5 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch5route(pub u32);
    impl Prs0Asynch5route {
        #[doc = "ASYNCH5 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH5 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH5 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH5 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch5route {
        #[inline(always)]
        fn default() -> Prs0Asynch5route {
            Prs0Asynch5route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch5route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch5route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch5route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch5route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH6 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch6route(pub u32);
    impl Prs0Asynch6route {
        #[doc = "ASYNCH6 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH6 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH6 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH6 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch6route {
        #[inline(always)]
        fn default() -> Prs0Asynch6route {
            Prs0Asynch6route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch6route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch6route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch6route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch6route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH7 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch7route(pub u32);
    impl Prs0Asynch7route {
        #[doc = "ASYNCH7 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH7 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH7 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH7 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch7route {
        #[inline(always)]
        fn default() -> Prs0Asynch7route {
            Prs0Asynch7route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch7route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch7route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch7route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch7route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH8 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch8route(pub u32);
    impl Prs0Asynch8route {
        #[doc = "ASYNCH8 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH8 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH8 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH8 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch8route {
        #[inline(always)]
        fn default() -> Prs0Asynch8route {
            Prs0Asynch8route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch8route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch8route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch8route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch8route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "ASYNCH9 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Asynch9route(pub u32);
    impl Prs0Asynch9route {
        #[doc = "ASYNCH9 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "ASYNCH9 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "ASYNCH9 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCH9 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Asynch9route {
        #[inline(always)]
        fn default() -> Prs0Asynch9route {
            Prs0Asynch9route(0)
        }
    }
    impl core::fmt::Debug for Prs0Asynch9route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Asynch9route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Asynch9route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Asynch9route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "PRS0 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Routeen(pub u32);
    impl Prs0Routeen {
        #[doc = "ASYNCH0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ASYNCH1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ASYNCH2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ASYNCH3 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch3pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH3 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ASYNCH4 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch4pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH4 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch4pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ASYNCH5 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch5pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH5 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch5pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ASYNCH6 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch6pen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH6 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch6pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ASYNCH7 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch7pen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH7 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch7pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ASYNCH8 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch8pen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH8 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch8pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ASYNCH9 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch9pen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH9 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch9pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ASYNCH10 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch10pen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH10 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch10pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ASYNCH11 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn asynch11pen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ASYNCH11 pin enable control bit."]
        #[inline(always)]
        pub const fn set_asynch11pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SYNCH0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn synch0pen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCH0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_synch0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SYNCH1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn synch1pen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCH1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_synch1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SYNCH2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn synch2pen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCH2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_synch2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SYNCH3 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn synch3pen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SYNCH3 pin enable control bit."]
        #[inline(always)]
        pub const fn set_synch3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Prs0Routeen {
        #[inline(always)]
        fn default() -> Prs0Routeen {
            Prs0Routeen(0)
        }
    }
    impl core::fmt::Debug for Prs0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Routeen")
                .field("asynch0pen", &self.asynch0pen())
                .field("asynch1pen", &self.asynch1pen())
                .field("asynch2pen", &self.asynch2pen())
                .field("asynch3pen", &self.asynch3pen())
                .field("asynch4pen", &self.asynch4pen())
                .field("asynch5pen", &self.asynch5pen())
                .field("asynch6pen", &self.asynch6pen())
                .field("asynch7pen", &self.asynch7pen())
                .field("asynch8pen", &self.asynch8pen())
                .field("asynch9pen", &self.asynch9pen())
                .field("asynch10pen", &self.asynch10pen())
                .field("asynch11pen", &self.asynch11pen())
                .field("synch0pen", &self.synch0pen())
                .field("synch1pen", &self.synch1pen())
                .field("synch2pen", &self.synch2pen())
                .field("synch3pen", &self.synch3pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Routeen {{ asynch0pen: {=bool:?}, asynch1pen: {=bool:?}, asynch2pen: {=bool:?}, asynch3pen: {=bool:?}, asynch4pen: {=bool:?}, asynch5pen: {=bool:?}, asynch6pen: {=bool:?}, asynch7pen: {=bool:?}, asynch8pen: {=bool:?}, asynch9pen: {=bool:?}, asynch10pen: {=bool:?}, asynch11pen: {=bool:?}, synch0pen: {=bool:?}, synch1pen: {=bool:?}, synch2pen: {=bool:?}, synch3pen: {=bool:?} }}",
                self.asynch0pen(),
                self.asynch1pen(),
                self.asynch2pen(),
                self.asynch3pen(),
                self.asynch4pen(),
                self.asynch5pen(),
                self.asynch6pen(),
                self.asynch7pen(),
                self.asynch8pen(),
                self.asynch9pen(),
                self.asynch10pen(),
                self.asynch11pen(),
                self.synch0pen(),
                self.synch1pen(),
                self.synch2pen(),
                self.synch3pen()
            )
        }
    }
    #[doc = "SYNCH0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Synch0route(pub u32);
    impl Prs0Synch0route {
        #[doc = "SYNCH0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SYNCH0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SYNCH0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SYNCH0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Synch0route {
        #[inline(always)]
        fn default() -> Prs0Synch0route {
            Prs0Synch0route(0)
        }
    }
    impl core::fmt::Debug for Prs0Synch0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Synch0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Synch0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Synch0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SYNCH1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Synch1route(pub u32);
    impl Prs0Synch1route {
        #[doc = "SYNCH1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SYNCH1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SYNCH1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SYNCH1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Synch1route {
        #[inline(always)]
        fn default() -> Prs0Synch1route {
            Prs0Synch1route(0)
        }
    }
    impl core::fmt::Debug for Prs0Synch1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Synch1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Synch1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Synch1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SYNCH2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Synch2route(pub u32);
    impl Prs0Synch2route {
        #[doc = "SYNCH2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SYNCH2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SYNCH2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SYNCH2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Synch2route {
        #[inline(always)]
        fn default() -> Prs0Synch2route {
            Prs0Synch2route(0)
        }
    }
    impl core::fmt::Debug for Prs0Synch2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Synch2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Synch2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Synch2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SYNCH3 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prs0Synch3route(pub u32);
    impl Prs0Synch3route {
        #[doc = "SYNCH3 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SYNCH3 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SYNCH3 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SYNCH3 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Prs0Synch3route {
        #[inline(always)]
        fn default() -> Prs0Synch3route {
            Prs0Synch3route(0)
        }
    }
    impl core::fmt::Debug for Prs0Synch3route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prs0Synch3route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prs0Synch3route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Prs0Synch3route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "BUFOUTREQINASYNC port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Syxo0Bufoutreqinasyncroute(pub u32);
    impl Syxo0Bufoutreqinasyncroute {
        #[doc = "BUFOUTREQINASYNC port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "BUFOUTREQINASYNC port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "BUFOUTREQINASYNC pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "BUFOUTREQINASYNC pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Syxo0Bufoutreqinasyncroute {
        #[inline(always)]
        fn default() -> Syxo0Bufoutreqinasyncroute {
            Syxo0Bufoutreqinasyncroute(0)
        }
    }
    impl core::fmt::Debug for Syxo0Bufoutreqinasyncroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Syxo0Bufoutreqinasyncroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Syxo0Bufoutreqinasyncroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Syxo0Bufoutreqinasyncroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Cc0route(pub u32);
    impl Timer0Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer0Cc0route {
        #[inline(always)]
        fn default() -> Timer0Cc0route {
            Timer0Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer0Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Cc1route(pub u32);
    impl Timer0Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer0Cc1route {
        #[inline(always)]
        fn default() -> Timer0Cc1route {
            Timer0Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer0Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Cc2route(pub u32);
    impl Timer0Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer0Cc2route {
        #[inline(always)]
        fn default() -> Timer0Cc2route {
            Timer0Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer0Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Cdti0route(pub u32);
    impl Timer0Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer0Cdti0route {
        #[inline(always)]
        fn default() -> Timer0Cdti0route {
            Timer0Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer0Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Cdti1route(pub u32);
    impl Timer0Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer0Cdti1route {
        #[inline(always)]
        fn default() -> Timer0Cdti1route {
            Timer0Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer0Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Cdti2route(pub u32);
    impl Timer0Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer0Cdti2route {
        #[inline(always)]
        fn default() -> Timer0Cdti2route {
            Timer0Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer0Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER0 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer0Routeen(pub u32);
    impl Timer0Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer0Routeen {
        #[inline(always)]
        fn default() -> Timer0Routeen {
            Timer0Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer0Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer0Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Cc0route(pub u32);
    impl Timer1Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer1Cc0route {
        #[inline(always)]
        fn default() -> Timer1Cc0route {
            Timer1Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer1Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Cc1route(pub u32);
    impl Timer1Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer1Cc1route {
        #[inline(always)]
        fn default() -> Timer1Cc1route {
            Timer1Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer1Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Cc2route(pub u32);
    impl Timer1Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer1Cc2route {
        #[inline(always)]
        fn default() -> Timer1Cc2route {
            Timer1Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer1Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Cdti0route(pub u32);
    impl Timer1Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer1Cdti0route {
        #[inline(always)]
        fn default() -> Timer1Cdti0route {
            Timer1Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer1Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Cdti1route(pub u32);
    impl Timer1Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer1Cdti1route {
        #[inline(always)]
        fn default() -> Timer1Cdti1route {
            Timer1Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer1Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Cdti2route(pub u32);
    impl Timer1Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer1Cdti2route {
        #[inline(always)]
        fn default() -> Timer1Cdti2route {
            Timer1Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer1Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER1 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer1Routeen(pub u32);
    impl Timer1Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer1Routeen {
        #[inline(always)]
        fn default() -> Timer1Routeen {
            Timer1Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer1Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer1Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer1Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer1Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Cc0route(pub u32);
    impl Timer2Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer2Cc0route {
        #[inline(always)]
        fn default() -> Timer2Cc0route {
            Timer2Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer2Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Cc1route(pub u32);
    impl Timer2Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer2Cc1route {
        #[inline(always)]
        fn default() -> Timer2Cc1route {
            Timer2Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer2Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Cc2route(pub u32);
    impl Timer2Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer2Cc2route {
        #[inline(always)]
        fn default() -> Timer2Cc2route {
            Timer2Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer2Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Cdti0route(pub u32);
    impl Timer2Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer2Cdti0route {
        #[inline(always)]
        fn default() -> Timer2Cdti0route {
            Timer2Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer2Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Cdti1route(pub u32);
    impl Timer2Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer2Cdti1route {
        #[inline(always)]
        fn default() -> Timer2Cdti1route {
            Timer2Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer2Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Cdti2route(pub u32);
    impl Timer2Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer2Cdti2route {
        #[inline(always)]
        fn default() -> Timer2Cdti2route {
            Timer2Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer2Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER2 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer2Routeen(pub u32);
    impl Timer2Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer2Routeen {
        #[inline(always)]
        fn default() -> Timer2Routeen {
            Timer2Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer2Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer2Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer2Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer2Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Cc0route(pub u32);
    impl Timer3Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer3Cc0route {
        #[inline(always)]
        fn default() -> Timer3Cc0route {
            Timer3Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer3Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Cc1route(pub u32);
    impl Timer3Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer3Cc1route {
        #[inline(always)]
        fn default() -> Timer3Cc1route {
            Timer3Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer3Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Cc2route(pub u32);
    impl Timer3Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer3Cc2route {
        #[inline(always)]
        fn default() -> Timer3Cc2route {
            Timer3Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer3Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Cdti0route(pub u32);
    impl Timer3Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer3Cdti0route {
        #[inline(always)]
        fn default() -> Timer3Cdti0route {
            Timer3Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer3Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Cdti1route(pub u32);
    impl Timer3Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer3Cdti1route {
        #[inline(always)]
        fn default() -> Timer3Cdti1route {
            Timer3Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer3Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Cdti2route(pub u32);
    impl Timer3Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer3Cdti2route {
        #[inline(always)]
        fn default() -> Timer3Cdti2route {
            Timer3Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer3Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER3 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer3Routeen(pub u32);
    impl Timer3Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer3Routeen {
        #[inline(always)]
        fn default() -> Timer3Routeen {
            Timer3Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer3Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer3Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer3Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer3Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Cc0route(pub u32);
    impl Timer4Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer4Cc0route {
        #[inline(always)]
        fn default() -> Timer4Cc0route {
            Timer4Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer4Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Cc1route(pub u32);
    impl Timer4Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer4Cc1route {
        #[inline(always)]
        fn default() -> Timer4Cc1route {
            Timer4Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer4Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Cc2route(pub u32);
    impl Timer4Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer4Cc2route {
        #[inline(always)]
        fn default() -> Timer4Cc2route {
            Timer4Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer4Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Cdti0route(pub u32);
    impl Timer4Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer4Cdti0route {
        #[inline(always)]
        fn default() -> Timer4Cdti0route {
            Timer4Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer4Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Cdti1route(pub u32);
    impl Timer4Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer4Cdti1route {
        #[inline(always)]
        fn default() -> Timer4Cdti1route {
            Timer4Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer4Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Cdti2route(pub u32);
    impl Timer4Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer4Cdti2route {
        #[inline(always)]
        fn default() -> Timer4Cdti2route {
            Timer4Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer4Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER4 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer4Routeen(pub u32);
    impl Timer4Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer4Routeen {
        #[inline(always)]
        fn default() -> Timer4Routeen {
            Timer4Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer4Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer4Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer4Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer4Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Cc0route(pub u32);
    impl Timer5Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer5Cc0route {
        #[inline(always)]
        fn default() -> Timer5Cc0route {
            Timer5Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer5Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Cc1route(pub u32);
    impl Timer5Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer5Cc1route {
        #[inline(always)]
        fn default() -> Timer5Cc1route {
            Timer5Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer5Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Cc2route(pub u32);
    impl Timer5Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer5Cc2route {
        #[inline(always)]
        fn default() -> Timer5Cc2route {
            Timer5Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer5Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Cdti0route(pub u32);
    impl Timer5Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer5Cdti0route {
        #[inline(always)]
        fn default() -> Timer5Cdti0route {
            Timer5Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer5Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Cdti1route(pub u32);
    impl Timer5Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer5Cdti1route {
        #[inline(always)]
        fn default() -> Timer5Cdti1route {
            Timer5Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer5Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Cdti2route(pub u32);
    impl Timer5Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer5Cdti2route {
        #[inline(always)]
        fn default() -> Timer5Cdti2route {
            Timer5Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer5Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER5 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer5Routeen(pub u32);
    impl Timer5Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer5Routeen {
        #[inline(always)]
        fn default() -> Timer5Routeen {
            Timer5Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer5Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer5Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer5Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer5Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Cc0route(pub u32);
    impl Timer6Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer6Cc0route {
        #[inline(always)]
        fn default() -> Timer6Cc0route {
            Timer6Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer6Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Cc1route(pub u32);
    impl Timer6Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer6Cc1route {
        #[inline(always)]
        fn default() -> Timer6Cc1route {
            Timer6Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer6Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Cc2route(pub u32);
    impl Timer6Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer6Cc2route {
        #[inline(always)]
        fn default() -> Timer6Cc2route {
            Timer6Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer6Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Cdti0route(pub u32);
    impl Timer6Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer6Cdti0route {
        #[inline(always)]
        fn default() -> Timer6Cdti0route {
            Timer6Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer6Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Cdti1route(pub u32);
    impl Timer6Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer6Cdti1route {
        #[inline(always)]
        fn default() -> Timer6Cdti1route {
            Timer6Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer6Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Cdti2route(pub u32);
    impl Timer6Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer6Cdti2route {
        #[inline(always)]
        fn default() -> Timer6Cdti2route {
            Timer6Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer6Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER6 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer6Routeen(pub u32);
    impl Timer6Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer6Routeen {
        #[inline(always)]
        fn default() -> Timer6Routeen {
            Timer6Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer6Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer6Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer6Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer6Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "CC0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Cc0route(pub u32);
    impl Timer7Cc0route {
        #[doc = "CC0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer7Cc0route {
        #[inline(always)]
        fn default() -> Timer7Cc0route {
            Timer7Cc0route(0)
        }
    }
    impl core::fmt::Debug for Timer7Cc0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Cc0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Cc0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Cc0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Cc1route(pub u32);
    impl Timer7Cc1route {
        #[doc = "CC1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer7Cc1route {
        #[inline(always)]
        fn default() -> Timer7Cc1route {
            Timer7Cc1route(0)
        }
    }
    impl core::fmt::Debug for Timer7Cc1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Cc1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Cc1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Cc1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CC2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Cc2route(pub u32);
    impl Timer7Cc2route {
        #[doc = "CC2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CC2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer7Cc2route {
        #[inline(always)]
        fn default() -> Timer7Cc2route {
            Timer7Cc2route(0)
        }
    }
    impl core::fmt::Debug for Timer7Cc2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Cc2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Cc2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Cc2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Cdti0route(pub u32);
    impl Timer7Cdti0route {
        #[doc = "CDTI0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer7Cdti0route {
        #[inline(always)]
        fn default() -> Timer7Cdti0route {
            Timer7Cdti0route(0)
        }
    }
    impl core::fmt::Debug for Timer7Cdti0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Cdti0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Cdti0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Cdti0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Cdti1route(pub u32);
    impl Timer7Cdti1route {
        #[doc = "CDTI1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer7Cdti1route {
        #[inline(always)]
        fn default() -> Timer7Cdti1route {
            Timer7Cdti1route(0)
        }
    }
    impl core::fmt::Debug for Timer7Cdti1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Cdti1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Cdti1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Cdti1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CDTI2 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Cdti2route(pub u32);
    impl Timer7Cdti2route {
        #[doc = "CDTI2 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CDTI2 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CDTI2 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CDTI2 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Timer7Cdti2route {
        #[inline(always)]
        fn default() -> Timer7Cdti2route {
            Timer7Cdti2route(0)
        }
    }
    impl core::fmt::Debug for Timer7Cdti2route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Cdti2route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Cdti2route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Cdti2route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TIMER7 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer7Routeen(pub u32);
    impl Timer7Routeen {
        #[doc = "CC0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc0pen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CC0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CC1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc1pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CC1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CC2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cc2pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CC2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_cc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc0pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI0 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc1pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI1 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc2pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CDTI2 pin enable control bit."]
        #[inline(always)]
        pub const fn set_ccc2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Timer7Routeen {
        #[inline(always)]
        fn default() -> Timer7Routeen {
            Timer7Routeen(0)
        }
    }
    impl core::fmt::Debug for Timer7Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timer7Routeen")
                .field("cc0pen", &self.cc0pen())
                .field("cc1pen", &self.cc1pen())
                .field("cc2pen", &self.cc2pen())
                .field("ccc0pen", &self.ccc0pen())
                .field("ccc1pen", &self.ccc1pen())
                .field("ccc2pen", &self.ccc2pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timer7Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timer7Routeen {{ cc0pen: {=bool:?}, cc1pen: {=bool:?}, cc2pen: {=bool:?}, ccc0pen: {=bool:?}, ccc1pen: {=bool:?}, ccc2pen: {=bool:?} }}",
                self.cc0pen(),
                self.cc1pen(),
                self.cc2pen(),
                self.ccc0pen(),
                self.ccc1pen(),
                self.ccc2pen()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Traceroutepen(pub u32);
    impl Traceroutepen {
        #[doc = "Serial Wire Viewer Output Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn swvpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Serial Wire Viewer Output Pin Enable."]
        #[inline(always)]
        pub const fn set_swvpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Trace Clk Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn traceclkpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Trace Clk Pin Enable."]
        #[inline(always)]
        pub const fn set_traceclkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Trace Data0 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tracedata0pen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Trace Data0 Pin Enable."]
        #[inline(always)]
        pub const fn set_tracedata0pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Trace Data1 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tracedata1pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Trace Data1 Pin Enable."]
        #[inline(always)]
        pub const fn set_tracedata1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Trace Data2 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tracedata2pen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Trace Data2 Pin Enable."]
        #[inline(always)]
        pub const fn set_tracedata2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trace Data3 Pin Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tracedata3pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trace Data3 Pin Enable."]
        #[inline(always)]
        pub const fn set_tracedata3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Traceroutepen {
        #[inline(always)]
        fn default() -> Traceroutepen {
            Traceroutepen(0)
        }
    }
    impl core::fmt::Debug for Traceroutepen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Traceroutepen")
                .field("swvpen", &self.swvpen())
                .field("traceclkpen", &self.traceclkpen())
                .field("tracedata0pen", &self.tracedata0pen())
                .field("tracedata1pen", &self.tracedata1pen())
                .field("tracedata2pen", &self.tracedata2pen())
                .field("tracedata3pen", &self.tracedata3pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Traceroutepen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Traceroutepen {{ swvpen: {=bool:?}, traceclkpen: {=bool:?}, tracedata0pen: {=bool:?}, tracedata1pen: {=bool:?}, tracedata2pen: {=bool:?}, tracedata3pen: {=bool:?} }}",
                self.swvpen(),
                self.traceclkpen(),
                self.tracedata0pen(),
                self.tracedata1pen(),
                self.tracedata2pen(),
                self.tracedata3pen()
            )
        }
    }
    #[doc = "USBVBUSSENSE port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UsbUsbvbussenseroute(pub u32);
    impl UsbUsbvbussenseroute {
        #[doc = "USBVBUSSENSE port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "USBVBUSSENSE port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "USBVBUSSENSE pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "USBVBUSSENSE pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for UsbUsbvbussenseroute {
        #[inline(always)]
        fn default() -> UsbUsbvbussenseroute {
            UsbUsbvbussenseroute(0)
        }
    }
    impl core::fmt::Debug for UsbUsbvbussenseroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UsbUsbvbussenseroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UsbUsbvbussenseroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UsbUsbvbussenseroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Aeven0 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH0."]
        Vdac0ch0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Aeven0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Aeven0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Aeven0 {
        #[inline(always)]
        fn from(val: u8) -> Aeven0 {
            Aeven0::from_bits(val)
        }
    }
    impl From<Aeven0> for u8 {
        #[inline(always)]
        fn from(val: Aeven0) -> u8 {
            Aeven0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Aeven1 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH1."]
        Vdac0ch1 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Aeven1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Aeven1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Aeven1 {
        #[inline(always)]
        fn from(val: u8) -> Aeven1 {
            Aeven1::from_bits(val)
        }
    }
    impl From<Aeven1> for u8 {
        #[inline(always)]
        fn from(val: Aeven1) -> u8 {
            Aeven1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Aodd0 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH0."]
        Vdac0ch0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Aodd0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Aodd0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Aodd0 {
        #[inline(always)]
        fn from(val: u8) -> Aodd0 {
            Aodd0::from_bits(val)
        }
    }
    impl From<Aodd0> for u8 {
        #[inline(always)]
        fn from(val: Aodd0) -> u8 {
            Aodd0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Aodd1 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH1."]
        Vdac0ch1 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Aodd1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Aodd1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Aodd1 {
        #[inline(always)]
        fn from(val: u8) -> Aodd1 {
            Aodd1::from_bits(val)
        }
    }
    impl From<Aodd1> for u8 {
        #[inline(always)]
        fn from(val: Aodd1) -> u8 {
            Aodd1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Beven0 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH0."]
        Vdac0ch0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Beven0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Beven0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Beven0 {
        #[inline(always)]
        fn from(val: u8) -> Beven0 {
            Beven0::from_bits(val)
        }
    }
    impl From<Beven0> for u8 {
        #[inline(always)]
        fn from(val: Beven0) -> u8 {
            Beven0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Beven1 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH1."]
        Vdac0ch1 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Beven1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Beven1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Beven1 {
        #[inline(always)]
        fn from(val: u8) -> Beven1 {
            Beven1::from_bits(val)
        }
    }
    impl From<Beven1> for u8 {
        #[inline(always)]
        fn from(val: Beven1) -> u8 {
            Beven1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bodd0 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH0."]
        Vdac0ch0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Bodd0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bodd0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bodd0 {
        #[inline(always)]
        fn from(val: u8) -> Bodd0 {
            Bodd0::from_bits(val)
        }
    }
    impl From<Bodd0> for u8 {
        #[inline(always)]
        fn from(val: Bodd0) -> u8 {
            Bodd0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bodd1 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH1."]
        Vdac0ch1 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Bodd1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bodd1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bodd1 {
        #[inline(always)]
        fn from(val: u8) -> Bodd1 {
            Bodd1::from_bits(val)
        }
    }
    impl From<Bodd1> for u8 {
        #[inline(always)]
        fn from(val: Bodd1) -> u8 {
            Bodd1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cdeven0 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH0."]
        Vdac0ch0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Cdeven0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdeven0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdeven0 {
        #[inline(always)]
        fn from(val: u8) -> Cdeven0 {
            Cdeven0::from_bits(val)
        }
    }
    impl From<Cdeven0> for u8 {
        #[inline(always)]
        fn from(val: Cdeven0) -> u8 {
            Cdeven0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cdeven1 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH1."]
        Vdac0ch1 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Cdeven1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdeven1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdeven1 {
        #[inline(always)]
        fn from(val: u8) -> Cdeven1 {
            Cdeven1::from_bits(val)
        }
    }
    impl From<Cdeven1> for u8 {
        #[inline(always)]
        fn from(val: Cdeven1) -> u8 {
            Cdeven1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cdodd0 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH0."]
        Vdac0ch0 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Cdodd0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdodd0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdodd0 {
        #[inline(always)]
        fn from(val: u8) -> Cdodd0 {
            Cdodd0::from_bits(val)
        }
    }
    impl From<Cdodd0> for u8 {
        #[inline(always)]
        fn from(val: Cdodd0) -> u8 {
            Cdodd0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cdodd1 {
        #[doc = "The bus is not allocated."]
        Tristate = 0x0,
        #[doc = "The bus is allocated to ADC0."]
        Adc0 = 0x01,
        #[doc = "The bus is allocated to ACMP0."]
        Acmp0 = 0x02,
        #[doc = "The bus is allocated to ACMP1."]
        Acmp1 = 0x03,
        #[doc = "The bus is allocated to VDAC0 CH1."]
        Vdac0ch1 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Cdodd1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdodd1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdodd1 {
        #[inline(always)]
        fn from(val: u8) -> Cdodd1 {
            Cdodd1::from_bits(val)
        }
    }
    impl From<Cdodd1> for u8 {
        #[inline(always)]
        fn from(val: Cdodd1) -> u8 {
            Cdodd1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipinsel4 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl Extipinsel4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipinsel4 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipinsel4 {
        #[inline(always)]
        fn from(val: u8) -> Extipinsel4 {
            Extipinsel4::from_bits(val)
        }
    }
    impl From<Extipinsel4> for u8 {
        #[inline(always)]
        fn from(val: Extipinsel4) -> u8 {
            Extipinsel4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipinsel5 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl Extipinsel5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipinsel5 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipinsel5 {
        #[inline(always)]
        fn from(val: u8) -> Extipinsel5 {
            Extipinsel5::from_bits(val)
        }
    }
    impl From<Extipinsel5> for u8 {
        #[inline(always)]
        fn from(val: Extipinsel5) -> u8 {
            Extipinsel5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipinsel6 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl Extipinsel6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipinsel6 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipinsel6 {
        #[inline(always)]
        fn from(val: u8) -> Extipinsel6 {
            Extipinsel6::from_bits(val)
        }
    }
    impl From<Extipinsel6> for u8 {
        #[inline(always)]
        fn from(val: Extipinsel6) -> u8 {
            Extipinsel6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipinsel7 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl Extipinsel7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipinsel7 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipinsel7 {
        #[inline(always)]
        fn from(val: u8) -> Extipinsel7 {
            Extipinsel7::from_bits(val)
        }
    }
    impl From<Extipinsel7> for u8 {
        #[inline(always)]
        fn from(val: Extipinsel7) -> u8 {
            Extipinsel7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinselhExtipinsel0 {
        #[doc = "OFFSET=8."]
        Pin8 = 0x0,
        #[doc = "OFFSET=9."]
        Pin9 = 0x01,
        #[doc = "OFFSET=10."]
        Pin10 = 0x02,
        #[doc = "OFFSET=11."]
        Pin11 = 0x03,
    }
    impl ExtipinselhExtipinsel0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinselhExtipinsel0 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinselhExtipinsel0 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinselhExtipinsel0 {
            ExtipinselhExtipinsel0::from_bits(val)
        }
    }
    impl From<ExtipinselhExtipinsel0> for u8 {
        #[inline(always)]
        fn from(val: ExtipinselhExtipinsel0) -> u8 {
            ExtipinselhExtipinsel0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinselhExtipinsel1 {
        #[doc = "OFFSET=8."]
        Pin8 = 0x0,
        #[doc = "OFFSET=9."]
        Pin9 = 0x01,
        #[doc = "OFFSET=10."]
        Pin10 = 0x02,
        #[doc = "OFFSET=11."]
        Pin11 = 0x03,
    }
    impl ExtipinselhExtipinsel1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinselhExtipinsel1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinselhExtipinsel1 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinselhExtipinsel1 {
            ExtipinselhExtipinsel1::from_bits(val)
        }
    }
    impl From<ExtipinselhExtipinsel1> for u8 {
        #[inline(always)]
        fn from(val: ExtipinselhExtipinsel1) -> u8 {
            ExtipinselhExtipinsel1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinselhExtipinsel2 {
        #[doc = "OFFSET=8."]
        Pin8 = 0x0,
        #[doc = "OFFSET=9."]
        Pin9 = 0x01,
        #[doc = "OFFSET=10."]
        Pin10 = 0x02,
        #[doc = "OFFSET=11."]
        Pin11 = 0x03,
    }
    impl ExtipinselhExtipinsel2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinselhExtipinsel2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinselhExtipinsel2 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinselhExtipinsel2 {
            ExtipinselhExtipinsel2::from_bits(val)
        }
    }
    impl From<ExtipinselhExtipinsel2> for u8 {
        #[inline(always)]
        fn from(val: ExtipinselhExtipinsel2) -> u8 {
            ExtipinselhExtipinsel2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinselhExtipinsel3 {
        #[doc = "OFFSET=8."]
        Pin8 = 0x0,
        #[doc = "OFFSET=9."]
        Pin9 = 0x01,
        #[doc = "OFFSET=10."]
        Pin10 = 0x02,
        #[doc = "OFFSET=11."]
        Pin11 = 0x03,
    }
    impl ExtipinselhExtipinsel3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinselhExtipinsel3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinselhExtipinsel3 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinselhExtipinsel3 {
            ExtipinselhExtipinsel3::from_bits(val)
        }
    }
    impl From<ExtipinselhExtipinsel3> for u8 {
        #[inline(always)]
        fn from(val: ExtipinselhExtipinsel3) -> u8 {
            ExtipinselhExtipinsel3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinsellExtipinsel0 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl ExtipinsellExtipinsel0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinsellExtipinsel0 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinsellExtipinsel0 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinsellExtipinsel0 {
            ExtipinsellExtipinsel0::from_bits(val)
        }
    }
    impl From<ExtipinsellExtipinsel0> for u8 {
        #[inline(always)]
        fn from(val: ExtipinsellExtipinsel0) -> u8 {
            ExtipinsellExtipinsel0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinsellExtipinsel1 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl ExtipinsellExtipinsel1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinsellExtipinsel1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinsellExtipinsel1 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinsellExtipinsel1 {
            ExtipinsellExtipinsel1::from_bits(val)
        }
    }
    impl From<ExtipinsellExtipinsel1> for u8 {
        #[inline(always)]
        fn from(val: ExtipinsellExtipinsel1) -> u8 {
            ExtipinsellExtipinsel1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinsellExtipinsel2 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl ExtipinsellExtipinsel2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinsellExtipinsel2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinsellExtipinsel2 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinsellExtipinsel2 {
            ExtipinsellExtipinsel2::from_bits(val)
        }
    }
    impl From<ExtipinsellExtipinsel2> for u8 {
        #[inline(always)]
        fn from(val: ExtipinsellExtipinsel2) -> u8 {
            ExtipinsellExtipinsel2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipinsellExtipinsel3 {
        #[doc = "OFFSET=0."]
        Pin0 = 0x0,
        #[doc = "OFFSET=1."]
        Pin1 = 0x01,
        #[doc = "OFFSET=2."]
        Pin2 = 0x02,
        #[doc = "OFFSET=3."]
        Pin3 = 0x03,
    }
    impl ExtipinsellExtipinsel3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipinsellExtipinsel3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipinsellExtipinsel3 {
        #[inline(always)]
        fn from(val: u8) -> ExtipinsellExtipinsel3 {
            ExtipinsellExtipinsel3::from_bits(val)
        }
    }
    impl From<ExtipinsellExtipinsel3> for u8 {
        #[inline(always)]
        fn from(val: ExtipinsellExtipinsel3) -> u8 {
            ExtipinsellExtipinsel3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipsel4 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl Extipsel4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipsel4 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipsel4 {
        #[inline(always)]
        fn from(val: u8) -> Extipsel4 {
            Extipsel4::from_bits(val)
        }
    }
    impl From<Extipsel4> for u8 {
        #[inline(always)]
        fn from(val: Extipsel4) -> u8 {
            Extipsel4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipsel5 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl Extipsel5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipsel5 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipsel5 {
        #[inline(always)]
        fn from(val: u8) -> Extipsel5 {
            Extipsel5::from_bits(val)
        }
    }
    impl From<Extipsel5> for u8 {
        #[inline(always)]
        fn from(val: Extipsel5) -> u8 {
            Extipsel5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipsel6 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl Extipsel6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipsel6 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipsel6 {
        #[inline(always)]
        fn from(val: u8) -> Extipsel6 {
            Extipsel6::from_bits(val)
        }
    }
    impl From<Extipsel6> for u8 {
        #[inline(always)]
        fn from(val: Extipsel6) -> u8 {
            Extipsel6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Extipsel7 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl Extipsel7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Extipsel7 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Extipsel7 {
        #[inline(always)]
        fn from(val: u8) -> Extipsel7 {
            Extipsel7::from_bits(val)
        }
    }
    impl From<Extipsel7> for u8 {
        #[inline(always)]
        fn from(val: Extipsel7) -> u8 {
            Extipsel7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipselhExtipsel0 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipselhExtipsel0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipselhExtipsel0 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipselhExtipsel0 {
        #[inline(always)]
        fn from(val: u8) -> ExtipselhExtipsel0 {
            ExtipselhExtipsel0::from_bits(val)
        }
    }
    impl From<ExtipselhExtipsel0> for u8 {
        #[inline(always)]
        fn from(val: ExtipselhExtipsel0) -> u8 {
            ExtipselhExtipsel0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipselhExtipsel1 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipselhExtipsel1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipselhExtipsel1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipselhExtipsel1 {
        #[inline(always)]
        fn from(val: u8) -> ExtipselhExtipsel1 {
            ExtipselhExtipsel1::from_bits(val)
        }
    }
    impl From<ExtipselhExtipsel1> for u8 {
        #[inline(always)]
        fn from(val: ExtipselhExtipsel1) -> u8 {
            ExtipselhExtipsel1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipselhExtipsel2 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipselhExtipsel2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipselhExtipsel2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipselhExtipsel2 {
        #[inline(always)]
        fn from(val: u8) -> ExtipselhExtipsel2 {
            ExtipselhExtipsel2::from_bits(val)
        }
    }
    impl From<ExtipselhExtipsel2> for u8 {
        #[inline(always)]
        fn from(val: ExtipselhExtipsel2) -> u8 {
            ExtipselhExtipsel2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipselhExtipsel3 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipselhExtipsel3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipselhExtipsel3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipselhExtipsel3 {
        #[inline(always)]
        fn from(val: u8) -> ExtipselhExtipsel3 {
            ExtipselhExtipsel3::from_bits(val)
        }
    }
    impl From<ExtipselhExtipsel3> for u8 {
        #[inline(always)]
        fn from(val: ExtipselhExtipsel3) -> u8 {
            ExtipselhExtipsel3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipsellExtipsel0 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipsellExtipsel0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipsellExtipsel0 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipsellExtipsel0 {
        #[inline(always)]
        fn from(val: u8) -> ExtipsellExtipsel0 {
            ExtipsellExtipsel0::from_bits(val)
        }
    }
    impl From<ExtipsellExtipsel0> for u8 {
        #[inline(always)]
        fn from(val: ExtipsellExtipsel0) -> u8 {
            ExtipsellExtipsel0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipsellExtipsel1 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipsellExtipsel1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipsellExtipsel1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipsellExtipsel1 {
        #[inline(always)]
        fn from(val: u8) -> ExtipsellExtipsel1 {
            ExtipsellExtipsel1::from_bits(val)
        }
    }
    impl From<ExtipsellExtipsel1> for u8 {
        #[inline(always)]
        fn from(val: ExtipsellExtipsel1) -> u8 {
            ExtipsellExtipsel1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipsellExtipsel2 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipsellExtipsel2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipsellExtipsel2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipsellExtipsel2 {
        #[inline(always)]
        fn from(val: u8) -> ExtipsellExtipsel2 {
            ExtipsellExtipsel2::from_bits(val)
        }
    }
    impl From<ExtipsellExtipsel2> for u8 {
        #[inline(always)]
        fn from(val: ExtipsellExtipsel2) -> u8 {
            ExtipsellExtipsel2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ExtipsellExtipsel3 {
        #[doc = "Port A group selected."]
        Porta = 0x0,
        #[doc = "Port B group selected."]
        Portb = 0x01,
        #[doc = "Port C group selected."]
        Portc = 0x02,
        #[doc = "Port D group selected."]
        Portd = 0x03,
    }
    impl ExtipsellExtipsel3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ExtipsellExtipsel3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ExtipsellExtipsel3 {
        #[inline(always)]
        fn from(val: u8) -> ExtipsellExtipsel3 {
            ExtipsellExtipsel3::from_bits(val)
        }
    }
    impl From<ExtipsellExtipsel3> for u8 {
        #[inline(always)]
        fn from(val: ExtipsellExtipsel3) -> u8 {
            ExtipsellExtipsel3::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lockkey(u16);
    impl Lockkey {
        #[doc = "Unlock code."]
        pub const Unlock: Self = Self(0xa534);
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
                0xa534 => f.write_str("Unlock"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockkey {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xa534 => defmt::write!(f, "Unlock"),
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
    pub enum PortaModehMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModehMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModehMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModehMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortaModehMode0 {
            PortaModehMode0::from_bits(val)
        }
    }
    impl From<PortaModehMode0> for u8 {
        #[inline(always)]
        fn from(val: PortaModehMode0) -> u8 {
            PortaModehMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModehMode1 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModehMode1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModehMode1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModehMode1 {
        #[inline(always)]
        fn from(val: u8) -> PortaModehMode1 {
            PortaModehMode1::from_bits(val)
        }
    }
    impl From<PortaModehMode1> for u8 {
        #[inline(always)]
        fn from(val: PortaModehMode1) -> u8 {
            PortaModehMode1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModehMode2 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModehMode2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModehMode2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModehMode2 {
        #[inline(always)]
        fn from(val: u8) -> PortaModehMode2 {
            PortaModehMode2::from_bits(val)
        }
    }
    impl From<PortaModehMode2> for u8 {
        #[inline(always)]
        fn from(val: PortaModehMode2) -> u8 {
            PortaModehMode2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModehMode3 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModehMode3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModehMode3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModehMode3 {
        #[inline(always)]
        fn from(val: u8) -> PortaModehMode3 {
            PortaModehMode3::from_bits(val)
        }
    }
    impl From<PortaModehMode3> for u8 {
        #[inline(always)]
        fn from(val: PortaModehMode3) -> u8 {
            PortaModehMode3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode0 {
            PortaModelMode0::from_bits(val)
        }
    }
    impl From<PortaModelMode0> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode0) -> u8 {
            PortaModelMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode1 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode1 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode1 {
            PortaModelMode1::from_bits(val)
        }
    }
    impl From<PortaModelMode1> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode1) -> u8 {
            PortaModelMode1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode2 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode2 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode2 {
            PortaModelMode2::from_bits(val)
        }
    }
    impl From<PortaModelMode2> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode2) -> u8 {
            PortaModelMode2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode3 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode3 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode3 {
            PortaModelMode3::from_bits(val)
        }
    }
    impl From<PortaModelMode3> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode3) -> u8 {
            PortaModelMode3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode4 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode4 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode4 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode4 {
            PortaModelMode4::from_bits(val)
        }
    }
    impl From<PortaModelMode4> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode4) -> u8 {
            PortaModelMode4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode5 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode5 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode5 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode5 {
            PortaModelMode5::from_bits(val)
        }
    }
    impl From<PortaModelMode5> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode5) -> u8 {
            PortaModelMode5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode6 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode6 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode6 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode6 {
            PortaModelMode6::from_bits(val)
        }
    }
    impl From<PortaModelMode6> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode6) -> u8 {
            PortaModelMode6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortaModelMode7 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortaModelMode7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortaModelMode7 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortaModelMode7 {
        #[inline(always)]
        fn from(val: u8) -> PortaModelMode7 {
            PortaModelMode7::from_bits(val)
        }
    }
    impl From<PortaModelMode7> for u8 {
        #[inline(always)]
        fn from(val: PortaModelMode7) -> u8 {
            PortaModelMode7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModehMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModehMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModehMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModehMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortbModehMode0 {
            PortbModehMode0::from_bits(val)
        }
    }
    impl From<PortbModehMode0> for u8 {
        #[inline(always)]
        fn from(val: PortbModehMode0) -> u8 {
            PortbModehMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode0 {
            PortbModelMode0::from_bits(val)
        }
    }
    impl From<PortbModelMode0> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode0) -> u8 {
            PortbModelMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode1 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode1 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode1 {
            PortbModelMode1::from_bits(val)
        }
    }
    impl From<PortbModelMode1> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode1) -> u8 {
            PortbModelMode1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode2 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode2 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode2 {
            PortbModelMode2::from_bits(val)
        }
    }
    impl From<PortbModelMode2> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode2) -> u8 {
            PortbModelMode2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode3 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode3 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode3 {
            PortbModelMode3::from_bits(val)
        }
    }
    impl From<PortbModelMode3> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode3) -> u8 {
            PortbModelMode3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode4 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode4 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode4 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode4 {
            PortbModelMode4::from_bits(val)
        }
    }
    impl From<PortbModelMode4> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode4) -> u8 {
            PortbModelMode4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode5 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode5 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode5 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode5 {
            PortbModelMode5::from_bits(val)
        }
    }
    impl From<PortbModelMode5> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode5) -> u8 {
            PortbModelMode5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode6 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode6 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode6 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode6 {
            PortbModelMode6::from_bits(val)
        }
    }
    impl From<PortbModelMode6> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode6) -> u8 {
            PortbModelMode6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortbModelMode7 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortbModelMode7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortbModelMode7 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortbModelMode7 {
        #[inline(always)]
        fn from(val: u8) -> PortbModelMode7 {
            PortbModelMode7::from_bits(val)
        }
    }
    impl From<PortbModelMode7> for u8 {
        #[inline(always)]
        fn from(val: PortbModelMode7) -> u8 {
            PortbModelMode7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModehMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModehMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModehMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModehMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortcModehMode0 {
            PortcModehMode0::from_bits(val)
        }
    }
    impl From<PortcModehMode0> for u8 {
        #[inline(always)]
        fn from(val: PortcModehMode0) -> u8 {
            PortcModehMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModehMode1 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModehMode1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModehMode1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModehMode1 {
        #[inline(always)]
        fn from(val: u8) -> PortcModehMode1 {
            PortcModehMode1::from_bits(val)
        }
    }
    impl From<PortcModehMode1> for u8 {
        #[inline(always)]
        fn from(val: PortcModehMode1) -> u8 {
            PortcModehMode1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModehMode2 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModehMode2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModehMode2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModehMode2 {
        #[inline(always)]
        fn from(val: u8) -> PortcModehMode2 {
            PortcModehMode2::from_bits(val)
        }
    }
    impl From<PortcModehMode2> for u8 {
        #[inline(always)]
        fn from(val: PortcModehMode2) -> u8 {
            PortcModehMode2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode0 {
            PortcModelMode0::from_bits(val)
        }
    }
    impl From<PortcModelMode0> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode0) -> u8 {
            PortcModelMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode1 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode1 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode1 {
            PortcModelMode1::from_bits(val)
        }
    }
    impl From<PortcModelMode1> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode1) -> u8 {
            PortcModelMode1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode2 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode2 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode2 {
            PortcModelMode2::from_bits(val)
        }
    }
    impl From<PortcModelMode2> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode2) -> u8 {
            PortcModelMode2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode3 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode3 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode3 {
            PortcModelMode3::from_bits(val)
        }
    }
    impl From<PortcModelMode3> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode3) -> u8 {
            PortcModelMode3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode4 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode4 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode4 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode4 {
            PortcModelMode4::from_bits(val)
        }
    }
    impl From<PortcModelMode4> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode4) -> u8 {
            PortcModelMode4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode5 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode5 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode5 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode5 {
            PortcModelMode5::from_bits(val)
        }
    }
    impl From<PortcModelMode5> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode5) -> u8 {
            PortcModelMode5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode6 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode6 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode6 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode6 {
            PortcModelMode6::from_bits(val)
        }
    }
    impl From<PortcModelMode6> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode6) -> u8 {
            PortcModelMode6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortcModelMode7 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortcModelMode7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortcModelMode7 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortcModelMode7 {
        #[inline(always)]
        fn from(val: u8) -> PortcModelMode7 {
            PortcModelMode7::from_bits(val)
        }
    }
    impl From<PortcModelMode7> for u8 {
        #[inline(always)]
        fn from(val: PortcModelMode7) -> u8 {
            PortcModelMode7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode0 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode0 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode0 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode0 {
            PortdModelMode0::from_bits(val)
        }
    }
    impl From<PortdModelMode0> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode0) -> u8 {
            PortdModelMode0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode1 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode1 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode1 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode1 {
            PortdModelMode1::from_bits(val)
        }
    }
    impl From<PortdModelMode1> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode1) -> u8 {
            PortdModelMode1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode2 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode2 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode2 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode2 {
            PortdModelMode2::from_bits(val)
        }
    }
    impl From<PortdModelMode2> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode2) -> u8 {
            PortdModelMode2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode3 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode3 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode3 {
            PortdModelMode3::from_bits(val)
        }
    }
    impl From<PortdModelMode3> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode3) -> u8 {
            PortdModelMode3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode4 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode4 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode4 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode4 {
            PortdModelMode4::from_bits(val)
        }
    }
    impl From<PortdModelMode4> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode4) -> u8 {
            PortdModelMode4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode5 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode5 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode5 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode5 {
            PortdModelMode5::from_bits(val)
        }
    }
    impl From<PortdModelMode5> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode5) -> u8 {
            PortdModelMode5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode6 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode6 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode6 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode6 {
            PortdModelMode6::from_bits(val)
        }
    }
    impl From<PortdModelMode6> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode6) -> u8 {
            PortdModelMode6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PortdModelMode7 {
        #[doc = "Input disabled. Pullup if DOUT is set."]
        Disabled = 0x0,
        #[doc = "Input enabled. Filter if DOUT is set."]
        Input = 0x01,
        #[doc = "Input enabled. DOUT determines pull direction."]
        Inputpull = 0x02,
        #[doc = "Input enabled with filter. DOUT determines pull direction."]
        Inputpullfilter = 0x03,
        #[doc = "Push-pull output."]
        Pushpull = 0x04,
        #[doc = "Push-pull using alternate control."]
        Pushpullalt = 0x05,
        #[doc = "Wired-or output."]
        Wiredor = 0x06,
        #[doc = "Wired-or output with pull-down."]
        Wiredorpulldown = 0x07,
        #[doc = "Open-drain output."]
        Wiredand = 0x08,
        #[doc = "Open-drain output with filter."]
        Wiredandfilter = 0x09,
        #[doc = "Open-drain output with pullup."]
        Wiredandpullup = 0x0a,
        #[doc = "Open-drain output with filter and pullup."]
        Wiredandpullupfilter = 0x0b,
        #[doc = "Open-drain output using alternate control."]
        Wiredandalt = 0x0c,
        #[doc = "Open-drain output using alternate control with filter."]
        Wiredandaltfilter = 0x0d,
        #[doc = "Open-drain output using alternate control with pullup."]
        Wiredandaltpullup = 0x0e,
        #[doc = "Open-drain output using alternate control with filter and pullup."]
        Wiredandaltpullupfilter = 0x0f,
    }
    impl PortdModelMode7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PortdModelMode7 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PortdModelMode7 {
        #[inline(always)]
        fn from(val: u8) -> PortdModelMode7 {
            PortdModelMode7::from_bits(val)
        }
    }
    impl From<PortdModelMode7> for u8 {
        #[inline(always)]
        fn from(val: PortdModelMode7) -> u8 {
            PortdModelMode7::to_bits(val)
        }
    }
}
