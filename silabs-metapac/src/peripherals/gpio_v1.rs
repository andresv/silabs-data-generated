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
    #[doc = "Port control."]
    #[inline(always)]
    pub const fn p_ctrl(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 48usize) as _) }
    }
    #[doc = "mode low."]
    #[inline(always)]
    pub const fn p_model(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 48usize) as _) }
    }
    #[doc = "mode high."]
    #[inline(always)]
    pub const fn p_modeh(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 0usize) as _) }
    }
    #[doc = "data out."]
    #[inline(always)]
    pub const fn p_dout(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 48usize) as _) }
    }
    #[doc = "data in."]
    #[inline(always)]
    pub const fn p_din(self, n: usize) -> crate::common::Reg<regs::PortDin, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn gpiolockstatus(self) -> crate::common::Reg<regs::Gpiolockstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "A Bus allocation."]
    #[inline(always)]
    pub const fn abusalloc(self) -> crate::common::Reg<regs::Abusalloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "B Bus allocation."]
    #[inline(always)]
    pub const fn bbusalloc(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "CD Bus allocation."]
    #[inline(always)]
    pub const fn cdbusalloc(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "External Interrupt Port Select Low."]
    #[inline(always)]
    pub const fn extipsell(self) -> crate::common::Reg<regs::Extipsell, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "External interrupt Port Select High."]
    #[inline(always)]
    pub const fn extipselh(self) -> crate::common::Reg<regs::Extipselh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low."]
    #[inline(always)]
    pub const fn extipinsell(self) -> crate::common::Reg<regs::Extipinsell, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High."]
    #[inline(always)]
    pub const fn extipinselh(self) -> crate::common::Reg<regs::Extipinselh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger."]
    #[inline(always)]
    pub const fn extirise(self) -> crate::common::Reg<regs::Extirise, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger."]
    #[inline(always)]
    pub const fn extifall(self) -> crate::common::Reg<regs::Extifall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn if_(self) -> crate::common::Reg<regs::If, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ien(self) -> crate::common::Reg<regs::Ien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4wuen(self) -> crate::common::Reg<regs::Em4wuen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn em4wupol(self) -> crate::common::Reg<regs::Em4wupol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn dbgroutepen(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn traceroutepen(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "CMU pin enable."]
    #[inline(always)]
    pub const fn cmu_routeen(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkin0route(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkout0route(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkout1route(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select."]
    #[inline(always)]
    pub const fn cmu_clkout2route(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "DCDC pin enable."]
    #[inline(always)]
    pub const fn dcdc_routeen(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "FRC pin enable."]
    #[inline(always)]
    pub const fn frc_routeen(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "DCLK port/pin select."]
    #[inline(always)]
    pub const fn frc_dclkroute(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "DFRAME port/pin select."]
    #[inline(always)]
    pub const fn frc_dframeroute(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0484usize) as _) }
    }
    #[doc = "DOUT port/pin select."]
    #[inline(always)]
    pub const fn frc_doutroute(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0488usize) as _) }
    }
    #[doc = "I2C0 pin enable."]
    #[inline(always)]
    pub const fn i2c0_routeen(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "SCL port/pin select."]
    #[inline(always)]
    pub const fn i2c0_sclroute(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "SDA port/pin select."]
    #[inline(always)]
    pub const fn i2c0_sdaroute(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "I2C1 pin enable."]
    #[inline(always)]
    pub const fn i2c1_routeen(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "SCL port/pin select."]
    #[inline(always)]
    pub const fn i2c1_sclroute(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a4usize) as _) }
    }
    #[doc = "SDA port/pin select."]
    #[inline(always)]
    pub const fn i2c1_sdaroute(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "LETIMER pin enable."]
    #[inline(always)]
    pub const fn letimer0_routeen(self) -> crate::common::Reg<regs::Letimer0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select."]
    #[inline(always)]
    pub const fn letimer0_out0route(self) -> crate::common::Reg<regs::Letimer0Out0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select."]
    #[inline(always)]
    pub const fn letimer0_out1route(self) -> crate::common::Reg<regs::Letimer0Out1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "EUART pin enable."]
    #[inline(always)]
    pub const fn euart0_routeen(self) -> crate::common::Reg<regs::Euart0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn euart0_ctsroute(self) -> crate::common::Reg<regs::Euart0Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn euart0_rtsroute(self) -> crate::common::Reg<regs::Euart0Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn euart0_rxroute(self) -> crate::common::Reg<regs::Euart0Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn euart0_txroute(self) -> crate::common::Reg<regs::Euart0Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "MODEM pin enable."]
    #[inline(always)]
    pub const fn modem_routeen(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "ANT0 port/pin select."]
    #[inline(always)]
    pub const fn modem_ant0route(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "ANT1 port/pin select."]
    #[inline(always)]
    pub const fn modem_ant1route(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select."]
    #[inline(always)]
    pub const fn modem_antrolloverroute(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "ANTRR0 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr0route(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr1route(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "ANTRR2 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr2route(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr3route(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "ANTRR4 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr4route(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select."]
    #[inline(always)]
    pub const fn modem_antrr5route(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "ANTSWEN port/pin select."]
    #[inline(always)]
    pub const fn modem_antswenroute(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select."]
    #[inline(always)]
    pub const fn modem_antswusroute(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "ANTTRIG port/pin select."]
    #[inline(always)]
    pub const fn modem_anttrigroute(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select."]
    #[inline(always)]
    pub const fn modem_anttrigstoproute(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "DCLK port/pin select."]
    #[inline(always)]
    pub const fn modem_dclkroute(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "DIN port/pin select."]
    #[inline(always)]
    pub const fn modem_dinroute(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "DOUT port/pin select."]
    #[inline(always)]
    pub const fn modem_doutroute(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "PDM pin enable."]
    #[inline(always)]
    pub const fn pdm_routeen(self) -> crate::common::Reg<regs::PdmRouteen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "CLK port/pin select."]
    #[inline(always)]
    pub const fn pdm_clkroute(self) -> crate::common::Reg<regs::PdmClkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "DAT0 port/pin select."]
    #[inline(always)]
    pub const fn pdm_dat0route(self) -> crate::common::Reg<regs::PdmDat0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "DAT1 port/pin select."]
    #[inline(always)]
    pub const fn pdm_dat1route(self) -> crate::common::Reg<regs::PdmDat1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "PRS0 pin enable."]
    #[inline(always)]
    pub const fn prs0_routeen(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch0route(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch1route(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch2route(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch3route(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch4route(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch5route(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch6route(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch7route(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch8route(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch9route(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch10route(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select."]
    #[inline(always)]
    pub const fn prs0_asynch11route(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch0route(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "SYNCH1 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch1route(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "SYNCH2 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch2route(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select."]
    #[inline(always)]
    pub const fn prs0_synch3route(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "TIMER0 pin enable."]
    #[inline(always)]
    pub const fn timer0_routeen(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cc0route(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cc1route(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cc2route(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cdti0route(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cdti1route(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer0_cdti2route(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "TIMER1 pin enable."]
    #[inline(always)]
    pub const fn timer1_routeen(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cc0route(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cc1route(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cc2route(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cdti0route(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cdti1route(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer1_cdti2route(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "TIMER2 pin enable."]
    #[inline(always)]
    pub const fn timer2_routeen(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cc0route(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cc1route(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cc2route(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cdti0route(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cdti1route(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer2_cdti2route(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "TIMER3 pin enable."]
    #[inline(always)]
    pub const fn timer3_routeen(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cc0route(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cc1route(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cc2route(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cdti0route(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cdti1route(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer3_cdti2route(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "TIMER4 pin enable."]
    #[inline(always)]
    pub const fn timer4_routeen(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "CC0 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cc0route(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "CC1 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cc1route(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "CC2 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cc2route(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "CDTI0 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cdti0route(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cdti1route(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "CDTI2 port/pin select."]
    #[inline(always)]
    pub const fn timer4_cdti2route(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "USART0 pin enable."]
    #[inline(always)]
    pub const fn usart0_routeen(self) -> crate::common::Reg<regs::Usart0Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn usart0_csroute(self) -> crate::common::Reg<regs::Usart0Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn usart0_ctsroute(self) -> crate::common::Reg<regs::Usart0Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn usart0_rtsroute(self) -> crate::common::Reg<regs::Usart0Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn usart0_rxroute(self) -> crate::common::Reg<regs::Usart0Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn usart0_clkroute(self) -> crate::common::Reg<regs::Usart0Clkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn usart0_txroute(self) -> crate::common::Reg<regs::Usart0Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[doc = "USART1 pin enable."]
    #[inline(always)]
    pub const fn usart1_routeen(self) -> crate::common::Reg<regs::Usart1Routeen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[doc = "CS port/pin select."]
    #[inline(always)]
    pub const fn usart1_csroute(self) -> crate::common::Reg<regs::Usart1Csroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "CTS port/pin select."]
    #[inline(always)]
    pub const fn usart1_ctsroute(self) -> crate::common::Reg<regs::Usart1Ctsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    #[doc = "RTS port/pin select."]
    #[inline(always)]
    pub const fn usart1_rtsroute(self) -> crate::common::Reg<regs::Usart1Rtsroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
    }
    #[doc = "RX port/pin select."]
    #[inline(always)]
    pub const fn usart1_rxroute(self) -> crate::common::Reg<regs::Usart1Rxroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x064cusize) as _) }
    }
    #[doc = "SCLK port/pin select."]
    #[inline(always)]
    pub const fn usart1_clkroute(self) -> crate::common::Reg<regs::Usart1Clkroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
    }
    #[doc = "TX port/pin select."]
    #[inline(always)]
    pub const fn usart1_txroute(self) -> crate::common::Reg<regs::Usart1Txroute, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0654usize) as _) }
    }
    #[doc = "Port control. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_ctrl_set(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 48usize) as _) }
    }
    #[doc = "mode low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_model_set(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize + n * 48usize) as _) }
    }
    #[doc = "mode high. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_modeh_set(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::W> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize + n * 0usize) as _) }
    }
    #[doc = "data out. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn p_dout_set(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize + n * 48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn lock_set(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1300usize) as _) }
    }
    #[doc = "A Bus allocation. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn abusalloc_set(self) -> crate::common::Reg<regs::Abusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1320usize) as _) }
    }
    #[doc = "B Bus allocation. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn bbusalloc_set(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1324usize) as _) }
    }
    #[doc = "CD Bus allocation. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cdbusalloc_set(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1328usize) as _) }
    }
    #[doc = "External Interrupt Port Select Low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipsell_set(self) -> crate::common::Reg<regs::Extipsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1400usize) as _) }
    }
    #[doc = "External interrupt Port Select High. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipselh_set(self) -> crate::common::Reg<regs::Extipselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1404usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipinsell_set(self) -> crate::common::Reg<regs::Extipinsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1408usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extipinselh_set(self) -> crate::common::Reg<regs::Extipinselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x140cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extirise_set(self) -> crate::common::Reg<regs::Extirise, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1410usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn extifall_set(self) -> crate::common::Reg<regs::Extifall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1414usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn if_set(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1420usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn ien_set(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1424usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4wuen_set(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x142cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn em4wupol_set(self) -> crate::common::Reg<regs::Em4wupol, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1430usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dbgroutepen_set(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1440usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn traceroutepen_set(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1444usize) as _) }
    }
    #[doc = "CMU pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_routeen_set(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1450usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkin0route_set(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1454usize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkout0route_set(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1458usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkout1route_set(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x145cusize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn cmu_clkout2route_set(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1460usize) as _) }
    }
    #[doc = "DCDC pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn dcdc_routeen_set(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x146cusize) as _) }
    }
    #[doc = "FRC pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_routeen_set(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x147cusize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_dclkroute_set(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1480usize) as _) }
    }
    #[doc = "DFRAME port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_dframeroute_set(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1484usize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn frc_doutroute_set(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1488usize) as _) }
    }
    #[doc = "I2C0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c0_routeen_set(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1490usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c0_sclroute_set(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1494usize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c0_sdaroute_set(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1498usize) as _) }
    }
    #[doc = "I2C1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c1_routeen_set(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a0usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c1_sclroute_set(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a4usize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn i2c1_sdaroute_set(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14a8usize) as _) }
    }
    #[doc = "LETIMER pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn letimer0_routeen_set(self) -> crate::common::Reg<regs::Letimer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn letimer0_out0route_set(self) -> crate::common::Reg<regs::Letimer0Out0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn letimer0_out1route_set(self) -> crate::common::Reg<regs::Letimer0Out1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14b8usize) as _) }
    }
    #[doc = "EUART pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn euart0_routeen_set(self) -> crate::common::Reg<regs::Euart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14c0usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn euart0_ctsroute_set(self) -> crate::common::Reg<regs::Euart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14c4usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn euart0_rtsroute_set(self) -> crate::common::Reg<regs::Euart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14c8usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn euart0_rxroute_set(self) -> crate::common::Reg<regs::Euart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14ccusize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn euart0_txroute_set(self) -> crate::common::Reg<regs::Euart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14d0usize) as _) }
    }
    #[doc = "MODEM pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_routeen_set(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14d8usize) as _) }
    }
    #[doc = "ANT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_ant0route_set(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14dcusize) as _) }
    }
    #[doc = "ANT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_ant1route_set(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14e0usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrolloverroute_set(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14e4usize) as _) }
    }
    #[doc = "ANTRR0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr0route_set(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14e8usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr1route_set(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14ecusize) as _) }
    }
    #[doc = "ANTRR2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr2route_set(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14f0usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr3route_set(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14f4usize) as _) }
    }
    #[doc = "ANTRR4 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr4route_set(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14f8usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antrr5route_set(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14fcusize) as _) }
    }
    #[doc = "ANTSWEN port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antswenroute_set(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1500usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_antswusroute_set(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1504usize) as _) }
    }
    #[doc = "ANTTRIG port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_anttrigroute_set(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1508usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute_set(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x150cusize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_dclkroute_set(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1510usize) as _) }
    }
    #[doc = "DIN port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_dinroute_set(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1514usize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn modem_doutroute_set(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1518usize) as _) }
    }
    #[doc = "PDM pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pdm_routeen_set(self) -> crate::common::Reg<regs::PdmRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1520usize) as _) }
    }
    #[doc = "CLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pdm_clkroute_set(self) -> crate::common::Reg<regs::PdmClkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1524usize) as _) }
    }
    #[doc = "DAT0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pdm_dat0route_set(self) -> crate::common::Reg<regs::PdmDat0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1528usize) as _) }
    }
    #[doc = "DAT1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn pdm_dat1route_set(self) -> crate::common::Reg<regs::PdmDat1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x152cusize) as _) }
    }
    #[doc = "PRS0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_routeen_set(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1534usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch0route_set(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1538usize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch1route_set(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x153cusize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch2route_set(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1540usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch3route_set(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1544usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch4route_set(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1548usize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch5route_set(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x154cusize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch6route_set(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1550usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch7route_set(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1554usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch8route_set(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1558usize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch9route_set(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x155cusize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch10route_set(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1560usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_asynch11route_set(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1564usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch0route_set(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1568usize) as _) }
    }
    #[doc = "SYNCH1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch1route_set(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x156cusize) as _) }
    }
    #[doc = "SYNCH2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch2route_set(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1570usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn prs0_synch3route_set(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1574usize) as _) }
    }
    #[doc = "TIMER0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_routeen_set(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x157cusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cc0route_set(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1580usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cc1route_set(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1584usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cc2route_set(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1588usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cdti0route_set(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x158cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cdti1route_set(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1590usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer0_cdti2route_set(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1594usize) as _) }
    }
    #[doc = "TIMER1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_routeen_set(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x159cusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cc0route_set(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15a0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cc1route_set(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15a4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cc2route_set(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15a8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cdti0route_set(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15acusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cdti1route_set(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15b0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer1_cdti2route_set(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15b4usize) as _) }
    }
    #[doc = "TIMER2 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_routeen_set(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15bcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cc0route_set(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15c0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cc1route_set(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15c4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cc2route_set(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15c8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cdti0route_set(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15ccusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cdti1route_set(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15d0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer2_cdti2route_set(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15d4usize) as _) }
    }
    #[doc = "TIMER3 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_routeen_set(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15dcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cc0route_set(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15e0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cc1route_set(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15e4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cc2route_set(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15e8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cdti0route_set(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15ecusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cdti1route_set(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15f0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer3_cdti2route_set(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15f4usize) as _) }
    }
    #[doc = "TIMER4 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_routeen_set(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15fcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cc0route_set(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1600usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cc1route_set(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1604usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cc2route_set(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1608usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cdti0route_set(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x160cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cdti1route_set(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1610usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn timer4_cdti2route_set(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1614usize) as _) }
    }
    #[doc = "USART0 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_routeen_set(self) -> crate::common::Reg<regs::Usart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x161cusize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_csroute_set(self) -> crate::common::Reg<regs::Usart0Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1620usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_ctsroute_set(self) -> crate::common::Reg<regs::Usart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1624usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_rtsroute_set(self) -> crate::common::Reg<regs::Usart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1628usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_rxroute_set(self) -> crate::common::Reg<regs::Usart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x162cusize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_clkroute_set(self) -> crate::common::Reg<regs::Usart0Clkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1630usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart0_txroute_set(self) -> crate::common::Reg<regs::Usart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1634usize) as _) }
    }
    #[doc = "USART1 pin enable. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_routeen_set(self) -> crate::common::Reg<regs::Usart1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x163cusize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_csroute_set(self) -> crate::common::Reg<regs::Usart1Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1640usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_ctsroute_set(self) -> crate::common::Reg<regs::Usart1Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1644usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_rtsroute_set(self) -> crate::common::Reg<regs::Usart1Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1648usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_rxroute_set(self) -> crate::common::Reg<regs::Usart1Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x164cusize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_clkroute_set(self) -> crate::common::Reg<regs::Usart1Clkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1650usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn usart1_txroute_set(self) -> crate::common::Reg<regs::Usart1Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1654usize) as _) }
    }
    #[doc = "Port control. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_ctrl_clr(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize + n * 48usize) as _) }
    }
    #[doc = "mode low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_model_clr(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize + n * 48usize) as _) }
    }
    #[doc = "mode high. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_modeh_clr(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::W> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize + n * 0usize) as _) }
    }
    #[doc = "data out. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn p_dout_clr(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize + n * 48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn lock_clr(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize) as _) }
    }
    #[doc = "A Bus allocation. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn abusalloc_clr(self) -> crate::common::Reg<regs::Abusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2320usize) as _) }
    }
    #[doc = "B Bus allocation. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn bbusalloc_clr(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2324usize) as _) }
    }
    #[doc = "CD Bus allocation. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cdbusalloc_clr(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2328usize) as _) }
    }
    #[doc = "External Interrupt Port Select Low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipsell_clr(self) -> crate::common::Reg<regs::Extipsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize) as _) }
    }
    #[doc = "External interrupt Port Select High. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipselh_clr(self) -> crate::common::Reg<regs::Extipselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2404usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipinsell_clr(self) -> crate::common::Reg<regs::Extipinsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2408usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extipinselh_clr(self) -> crate::common::Reg<regs::Extipinselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x240cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extirise_clr(self) -> crate::common::Reg<regs::Extirise, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2410usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn extifall_clr(self) -> crate::common::Reg<regs::Extifall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2414usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn if_clr(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2420usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn ien_clr(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2424usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4wuen_clr(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x242cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn em4wupol_clr(self) -> crate::common::Reg<regs::Em4wupol, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2430usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dbgroutepen_clr(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2440usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn traceroutepen_clr(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2444usize) as _) }
    }
    #[doc = "CMU pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_routeen_clr(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2450usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkin0route_clr(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2454usize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkout0route_clr(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2458usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkout1route_clr(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x245cusize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn cmu_clkout2route_clr(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2460usize) as _) }
    }
    #[doc = "DCDC pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn dcdc_routeen_clr(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x246cusize) as _) }
    }
    #[doc = "FRC pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_routeen_clr(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x247cusize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_dclkroute_clr(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2480usize) as _) }
    }
    #[doc = "DFRAME port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_dframeroute_clr(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2484usize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn frc_doutroute_clr(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2488usize) as _) }
    }
    #[doc = "I2C0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c0_routeen_clr(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2490usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c0_sclroute_clr(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2494usize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c0_sdaroute_clr(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2498usize) as _) }
    }
    #[doc = "I2C1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c1_routeen_clr(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24a0usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c1_sclroute_clr(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24a4usize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn i2c1_sdaroute_clr(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24a8usize) as _) }
    }
    #[doc = "LETIMER pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn letimer0_routeen_clr(self) -> crate::common::Reg<regs::Letimer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn letimer0_out0route_clr(self) -> crate::common::Reg<regs::Letimer0Out0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn letimer0_out1route_clr(self) -> crate::common::Reg<regs::Letimer0Out1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24b8usize) as _) }
    }
    #[doc = "EUART pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn euart0_routeen_clr(self) -> crate::common::Reg<regs::Euart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24c0usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn euart0_ctsroute_clr(self) -> crate::common::Reg<regs::Euart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24c4usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn euart0_rtsroute_clr(self) -> crate::common::Reg<regs::Euart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24c8usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn euart0_rxroute_clr(self) -> crate::common::Reg<regs::Euart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24ccusize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn euart0_txroute_clr(self) -> crate::common::Reg<regs::Euart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24d0usize) as _) }
    }
    #[doc = "MODEM pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_routeen_clr(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24d8usize) as _) }
    }
    #[doc = "ANT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_ant0route_clr(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24dcusize) as _) }
    }
    #[doc = "ANT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_ant1route_clr(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24e0usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrolloverroute_clr(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24e4usize) as _) }
    }
    #[doc = "ANTRR0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr0route_clr(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24e8usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr1route_clr(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24ecusize) as _) }
    }
    #[doc = "ANTRR2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr2route_clr(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24f0usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr3route_clr(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24f4usize) as _) }
    }
    #[doc = "ANTRR4 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr4route_clr(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24f8usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antrr5route_clr(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24fcusize) as _) }
    }
    #[doc = "ANTSWEN port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antswenroute_clr(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2500usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_antswusroute_clr(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2504usize) as _) }
    }
    #[doc = "ANTTRIG port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_anttrigroute_clr(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2508usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute_clr(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x250cusize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_dclkroute_clr(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2510usize) as _) }
    }
    #[doc = "DIN port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_dinroute_clr(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2514usize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn modem_doutroute_clr(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2518usize) as _) }
    }
    #[doc = "PDM pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pdm_routeen_clr(self) -> crate::common::Reg<regs::PdmRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2520usize) as _) }
    }
    #[doc = "CLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pdm_clkroute_clr(self) -> crate::common::Reg<regs::PdmClkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2524usize) as _) }
    }
    #[doc = "DAT0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pdm_dat0route_clr(self) -> crate::common::Reg<regs::PdmDat0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2528usize) as _) }
    }
    #[doc = "DAT1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn pdm_dat1route_clr(self) -> crate::common::Reg<regs::PdmDat1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x252cusize) as _) }
    }
    #[doc = "PRS0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_routeen_clr(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2534usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch0route_clr(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2538usize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch1route_clr(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x253cusize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch2route_clr(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2540usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch3route_clr(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2544usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch4route_clr(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2548usize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch5route_clr(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x254cusize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch6route_clr(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2550usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch7route_clr(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2554usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch8route_clr(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2558usize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch9route_clr(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x255cusize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch10route_clr(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2560usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_asynch11route_clr(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2564usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch0route_clr(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2568usize) as _) }
    }
    #[doc = "SYNCH1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch1route_clr(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x256cusize) as _) }
    }
    #[doc = "SYNCH2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch2route_clr(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2570usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn prs0_synch3route_clr(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2574usize) as _) }
    }
    #[doc = "TIMER0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_routeen_clr(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x257cusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cc0route_clr(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2580usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cc1route_clr(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2584usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cc2route_clr(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2588usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cdti0route_clr(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x258cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cdti1route_clr(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2590usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer0_cdti2route_clr(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2594usize) as _) }
    }
    #[doc = "TIMER1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_routeen_clr(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x259cusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cc0route_clr(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25a0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cc1route_clr(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25a4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cc2route_clr(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25a8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cdti0route_clr(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25acusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cdti1route_clr(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25b0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer1_cdti2route_clr(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25b4usize) as _) }
    }
    #[doc = "TIMER2 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_routeen_clr(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25bcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cc0route_clr(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25c0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cc1route_clr(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25c4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cc2route_clr(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25c8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cdti0route_clr(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25ccusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cdti1route_clr(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25d0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer2_cdti2route_clr(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25d4usize) as _) }
    }
    #[doc = "TIMER3 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_routeen_clr(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25dcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cc0route_clr(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25e0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cc1route_clr(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25e4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cc2route_clr(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25e8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cdti0route_clr(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25ecusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cdti1route_clr(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25f0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer3_cdti2route_clr(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25f4usize) as _) }
    }
    #[doc = "TIMER4 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_routeen_clr(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25fcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cc0route_clr(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2600usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cc1route_clr(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2604usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cc2route_clr(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2608usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cdti0route_clr(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x260cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cdti1route_clr(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2610usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn timer4_cdti2route_clr(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2614usize) as _) }
    }
    #[doc = "USART0 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_routeen_clr(self) -> crate::common::Reg<regs::Usart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x261cusize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_csroute_clr(self) -> crate::common::Reg<regs::Usart0Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2620usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_ctsroute_clr(self) -> crate::common::Reg<regs::Usart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2624usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_rtsroute_clr(self) -> crate::common::Reg<regs::Usart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2628usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_rxroute_clr(self) -> crate::common::Reg<regs::Usart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x262cusize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_clkroute_clr(self) -> crate::common::Reg<regs::Usart0Clkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2630usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart0_txroute_clr(self) -> crate::common::Reg<regs::Usart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2634usize) as _) }
    }
    #[doc = "USART1 pin enable. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_routeen_clr(self) -> crate::common::Reg<regs::Usart1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x263cusize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_csroute_clr(self) -> crate::common::Reg<regs::Usart1Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2640usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_ctsroute_clr(self) -> crate::common::Reg<regs::Usart1Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2644usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_rtsroute_clr(self) -> crate::common::Reg<regs::Usart1Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2648usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_rxroute_clr(self) -> crate::common::Reg<regs::Usart1Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x264cusize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_clkroute_clr(self) -> crate::common::Reg<regs::Usart1Clkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2650usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn usart1_txroute_clr(self) -> crate::common::Reg<regs::Usart1Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2654usize) as _) }
    }
    #[doc = "Port control. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_ctrl_tgl(self, n: usize) -> crate::common::Reg<regs::PortCtrl, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize + n * 48usize) as _) }
    }
    #[doc = "mode low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_model_tgl(self, n: usize) -> crate::common::Reg<regs::PortModel, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize + n * 48usize) as _) }
    }
    #[doc = "mode high. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_modeh_tgl(self, n: usize) -> crate::common::Reg<regs::PortModeh, crate::common::W> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize + n * 0usize) as _) }
    }
    #[doc = "data out. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn p_dout_tgl(self, n: usize) -> crate::common::Reg<regs::PortDout, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize + n * 48usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn lock_tgl(self) -> crate::common::Reg<regs::Lock, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3300usize) as _) }
    }
    #[doc = "A Bus allocation. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn abusalloc_tgl(self) -> crate::common::Reg<regs::Abusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3320usize) as _) }
    }
    #[doc = "B Bus allocation. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn bbusalloc_tgl(self) -> crate::common::Reg<regs::Bbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3324usize) as _) }
    }
    #[doc = "CD Bus allocation. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cdbusalloc_tgl(self) -> crate::common::Reg<regs::Cdbusalloc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3328usize) as _) }
    }
    #[doc = "External Interrupt Port Select Low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipsell_tgl(self) -> crate::common::Reg<regs::Extipsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3400usize) as _) }
    }
    #[doc = "External interrupt Port Select High. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipselh_tgl(self) -> crate::common::Reg<regs::Extipselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3404usize) as _) }
    }
    #[doc = "External Interrupt Pin Select Low. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipinsell_tgl(self) -> crate::common::Reg<regs::Extipinsell, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3408usize) as _) }
    }
    #[doc = "External Interrupt Pin Select High. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extipinselh_tgl(self) -> crate::common::Reg<regs::Extipinselh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x340cusize) as _) }
    }
    #[doc = "External Interrupt Rising Edge Trigger. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extirise_tgl(self) -> crate::common::Reg<regs::Extirise, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3410usize) as _) }
    }
    #[doc = "External Interrupt Falling Edge Trigger. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn extifall_tgl(self) -> crate::common::Reg<regs::Extifall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3414usize) as _) }
    }
    #[doc = "Interrupt Flag. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn if_tgl(self) -> crate::common::Reg<regs::If, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3420usize) as _) }
    }
    #[doc = "Interrupt Enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn ien_tgl(self) -> crate::common::Reg<regs::Ien, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3424usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4wuen_tgl(self) -> crate::common::Reg<regs::Em4wuen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x342cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn em4wupol_tgl(self) -> crate::common::Reg<regs::Em4wupol, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3430usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dbgroutepen_tgl(self) -> crate::common::Reg<regs::Dbgroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3440usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn traceroutepen_tgl(self) -> crate::common::Reg<regs::Traceroutepen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3444usize) as _) }
    }
    #[doc = "CMU pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_routeen_tgl(self) -> crate::common::Reg<regs::CmuRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3450usize) as _) }
    }
    #[doc = "CLKIN0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkin0route_tgl(self) -> crate::common::Reg<regs::CmuClkin0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3454usize) as _) }
    }
    #[doc = "CLKOUT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkout0route_tgl(self) -> crate::common::Reg<regs::CmuClkout0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3458usize) as _) }
    }
    #[doc = "CLKOUT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkout1route_tgl(self) -> crate::common::Reg<regs::CmuClkout1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x345cusize) as _) }
    }
    #[doc = "CLKOUT2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn cmu_clkout2route_tgl(self) -> crate::common::Reg<regs::CmuClkout2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3460usize) as _) }
    }
    #[doc = "DCDC pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn dcdc_routeen_tgl(self) -> crate::common::Reg<regs::DcdcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x346cusize) as _) }
    }
    #[doc = "FRC pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_routeen_tgl(self) -> crate::common::Reg<regs::FrcRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x347cusize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_dclkroute_tgl(self) -> crate::common::Reg<regs::FrcDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3480usize) as _) }
    }
    #[doc = "DFRAME port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_dframeroute_tgl(self) -> crate::common::Reg<regs::FrcDframeroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3484usize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn frc_doutroute_tgl(self) -> crate::common::Reg<regs::FrcDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3488usize) as _) }
    }
    #[doc = "I2C0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c0_routeen_tgl(self) -> crate::common::Reg<regs::I2c0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3490usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c0_sclroute_tgl(self) -> crate::common::Reg<regs::I2c0Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3494usize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c0_sdaroute_tgl(self) -> crate::common::Reg<regs::I2c0Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3498usize) as _) }
    }
    #[doc = "I2C1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c1_routeen_tgl(self) -> crate::common::Reg<regs::I2c1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34a0usize) as _) }
    }
    #[doc = "SCL port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c1_sclroute_tgl(self) -> crate::common::Reg<regs::I2c1Sclroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34a4usize) as _) }
    }
    #[doc = "SDA port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn i2c1_sdaroute_tgl(self) -> crate::common::Reg<regs::I2c1Sdaroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34a8usize) as _) }
    }
    #[doc = "LETIMER pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn letimer0_routeen_tgl(self) -> crate::common::Reg<regs::Letimer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34b0usize) as _) }
    }
    #[doc = "OUT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn letimer0_out0route_tgl(self) -> crate::common::Reg<regs::Letimer0Out0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34b4usize) as _) }
    }
    #[doc = "OUT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn letimer0_out1route_tgl(self) -> crate::common::Reg<regs::Letimer0Out1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34b8usize) as _) }
    }
    #[doc = "EUART pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn euart0_routeen_tgl(self) -> crate::common::Reg<regs::Euart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34c0usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn euart0_ctsroute_tgl(self) -> crate::common::Reg<regs::Euart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34c4usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn euart0_rtsroute_tgl(self) -> crate::common::Reg<regs::Euart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34c8usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn euart0_rxroute_tgl(self) -> crate::common::Reg<regs::Euart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34ccusize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn euart0_txroute_tgl(self) -> crate::common::Reg<regs::Euart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34d0usize) as _) }
    }
    #[doc = "MODEM pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_routeen_tgl(self) -> crate::common::Reg<regs::ModemRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34d8usize) as _) }
    }
    #[doc = "ANT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_ant0route_tgl(self) -> crate::common::Reg<regs::ModemAnt0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34dcusize) as _) }
    }
    #[doc = "ANT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_ant1route_tgl(self) -> crate::common::Reg<regs::ModemAnt1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34e0usize) as _) }
    }
    #[doc = "ANTROLLOVER port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrolloverroute_tgl(self) -> crate::common::Reg<regs::ModemAntrolloverroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34e4usize) as _) }
    }
    #[doc = "ANTRR0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr0route_tgl(self) -> crate::common::Reg<regs::ModemAntrr0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34e8usize) as _) }
    }
    #[doc = "ANTRR1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr1route_tgl(self) -> crate::common::Reg<regs::ModemAntrr1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34ecusize) as _) }
    }
    #[doc = "ANTRR2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr2route_tgl(self) -> crate::common::Reg<regs::ModemAntrr2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34f0usize) as _) }
    }
    #[doc = "ANTRR3 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr3route_tgl(self) -> crate::common::Reg<regs::ModemAntrr3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34f4usize) as _) }
    }
    #[doc = "ANTRR4 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr4route_tgl(self) -> crate::common::Reg<regs::ModemAntrr4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34f8usize) as _) }
    }
    #[doc = "ANTRR5 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antrr5route_tgl(self) -> crate::common::Reg<regs::ModemAntrr5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34fcusize) as _) }
    }
    #[doc = "ANTSWEN port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antswenroute_tgl(self) -> crate::common::Reg<regs::ModemAntswenroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3500usize) as _) }
    }
    #[doc = "ANTSWUS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_antswusroute_tgl(self) -> crate::common::Reg<regs::ModemAntswusroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3504usize) as _) }
    }
    #[doc = "ANTTRIG port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_anttrigroute_tgl(self) -> crate::common::Reg<regs::ModemAnttrigroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3508usize) as _) }
    }
    #[doc = "ANTTRIGSTOP port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_anttrigstoproute_tgl(self) -> crate::common::Reg<regs::ModemAnttrigstoproute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x350cusize) as _) }
    }
    #[doc = "DCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_dclkroute_tgl(self) -> crate::common::Reg<regs::ModemDclkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3510usize) as _) }
    }
    #[doc = "DIN port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_dinroute_tgl(self) -> crate::common::Reg<regs::ModemDinroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3514usize) as _) }
    }
    #[doc = "DOUT port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn modem_doutroute_tgl(self) -> crate::common::Reg<regs::ModemDoutroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3518usize) as _) }
    }
    #[doc = "PDM pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pdm_routeen_tgl(self) -> crate::common::Reg<regs::PdmRouteen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3520usize) as _) }
    }
    #[doc = "CLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pdm_clkroute_tgl(self) -> crate::common::Reg<regs::PdmClkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3524usize) as _) }
    }
    #[doc = "DAT0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pdm_dat0route_tgl(self) -> crate::common::Reg<regs::PdmDat0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3528usize) as _) }
    }
    #[doc = "DAT1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn pdm_dat1route_tgl(self) -> crate::common::Reg<regs::PdmDat1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x352cusize) as _) }
    }
    #[doc = "PRS0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_routeen_tgl(self) -> crate::common::Reg<regs::Prs0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3534usize) as _) }
    }
    #[doc = "ASYNCH0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch0route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3538usize) as _) }
    }
    #[doc = "ASYNCH1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch1route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x353cusize) as _) }
    }
    #[doc = "ASYNCH2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch2route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3540usize) as _) }
    }
    #[doc = "ASYNCH3 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch3route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3544usize) as _) }
    }
    #[doc = "ASYNCH4 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch4route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch4route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3548usize) as _) }
    }
    #[doc = "ASYNCH5 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch5route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch5route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x354cusize) as _) }
    }
    #[doc = "ASYNCH6 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch6route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch6route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3550usize) as _) }
    }
    #[doc = "ASYNCH7 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch7route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch7route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3554usize) as _) }
    }
    #[doc = "ASYNCH8 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch8route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch8route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3558usize) as _) }
    }
    #[doc = "ASYNCH9 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch9route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch9route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x355cusize) as _) }
    }
    #[doc = "ASYNCH10 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch10route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch10route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3560usize) as _) }
    }
    #[doc = "ASYNCH11 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_asynch11route_tgl(self) -> crate::common::Reg<regs::Prs0Asynch11route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3564usize) as _) }
    }
    #[doc = "SYNCH0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch0route_tgl(self) -> crate::common::Reg<regs::Prs0Synch0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3568usize) as _) }
    }
    #[doc = "SYNCH1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch1route_tgl(self) -> crate::common::Reg<regs::Prs0Synch1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x356cusize) as _) }
    }
    #[doc = "SYNCH2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch2route_tgl(self) -> crate::common::Reg<regs::Prs0Synch2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3570usize) as _) }
    }
    #[doc = "SYNCH3 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn prs0_synch3route_tgl(self) -> crate::common::Reg<regs::Prs0Synch3route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3574usize) as _) }
    }
    #[doc = "TIMER0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_routeen_tgl(self) -> crate::common::Reg<regs::Timer0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x357cusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cc0route_tgl(self) -> crate::common::Reg<regs::Timer0Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3580usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cc1route_tgl(self) -> crate::common::Reg<regs::Timer0Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3584usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cc2route_tgl(self) -> crate::common::Reg<regs::Timer0Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3588usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer0Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x358cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer0Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3590usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer0_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer0Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3594usize) as _) }
    }
    #[doc = "TIMER1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_routeen_tgl(self) -> crate::common::Reg<regs::Timer1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x359cusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cc0route_tgl(self) -> crate::common::Reg<regs::Timer1Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35a0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cc1route_tgl(self) -> crate::common::Reg<regs::Timer1Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35a4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cc2route_tgl(self) -> crate::common::Reg<regs::Timer1Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35a8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer1Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35acusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer1Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35b0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer1_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer1Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35b4usize) as _) }
    }
    #[doc = "TIMER2 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_routeen_tgl(self) -> crate::common::Reg<regs::Timer2Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35bcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cc0route_tgl(self) -> crate::common::Reg<regs::Timer2Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35c0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cc1route_tgl(self) -> crate::common::Reg<regs::Timer2Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35c4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cc2route_tgl(self) -> crate::common::Reg<regs::Timer2Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35c8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer2Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35ccusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer2Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35d0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer2_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer2Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35d4usize) as _) }
    }
    #[doc = "TIMER3 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_routeen_tgl(self) -> crate::common::Reg<regs::Timer3Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35dcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cc0route_tgl(self) -> crate::common::Reg<regs::Timer3Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35e0usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cc1route_tgl(self) -> crate::common::Reg<regs::Timer3Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35e4usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cc2route_tgl(self) -> crate::common::Reg<regs::Timer3Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35e8usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer3Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35ecusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer3Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35f0usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer3_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer3Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35f4usize) as _) }
    }
    #[doc = "TIMER4 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_routeen_tgl(self) -> crate::common::Reg<regs::Timer4Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x35fcusize) as _) }
    }
    #[doc = "CC0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cc0route_tgl(self) -> crate::common::Reg<regs::Timer4Cc0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3600usize) as _) }
    }
    #[doc = "CC1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cc1route_tgl(self) -> crate::common::Reg<regs::Timer4Cc1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3604usize) as _) }
    }
    #[doc = "CC2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cc2route_tgl(self) -> crate::common::Reg<regs::Timer4Cc2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3608usize) as _) }
    }
    #[doc = "CDTI0 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cdti0route_tgl(self) -> crate::common::Reg<regs::Timer4Cdti0route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x360cusize) as _) }
    }
    #[doc = "CDTI1 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cdti1route_tgl(self) -> crate::common::Reg<regs::Timer4Cdti1route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3610usize) as _) }
    }
    #[doc = "CDTI2 port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn timer4_cdti2route_tgl(self) -> crate::common::Reg<regs::Timer4Cdti2route, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3614usize) as _) }
    }
    #[doc = "USART0 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_routeen_tgl(self) -> crate::common::Reg<regs::Usart0Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x361cusize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_csroute_tgl(self) -> crate::common::Reg<regs::Usart0Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3620usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_ctsroute_tgl(self) -> crate::common::Reg<regs::Usart0Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3624usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_rtsroute_tgl(self) -> crate::common::Reg<regs::Usart0Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3628usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_rxroute_tgl(self) -> crate::common::Reg<regs::Usart0Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x362cusize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_clkroute_tgl(self) -> crate::common::Reg<regs::Usart0Clkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3630usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart0_txroute_tgl(self) -> crate::common::Reg<regs::Usart0Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3634usize) as _) }
    }
    #[doc = "USART1 pin enable. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_routeen_tgl(self) -> crate::common::Reg<regs::Usart1Routeen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x363cusize) as _) }
    }
    #[doc = "CS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_csroute_tgl(self) -> crate::common::Reg<regs::Usart1Csroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3640usize) as _) }
    }
    #[doc = "CTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_ctsroute_tgl(self) -> crate::common::Reg<regs::Usart1Ctsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3644usize) as _) }
    }
    #[doc = "RTS port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_rtsroute_tgl(self) -> crate::common::Reg<regs::Usart1Rtsroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3648usize) as _) }
    }
    #[doc = "RX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_rxroute_tgl(self) -> crate::common::Reg<regs::Usart1Rxroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x364cusize) as _) }
    }
    #[doc = "SCLK port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_clkroute_tgl(self) -> crate::common::Reg<regs::Usart1Clkroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3650usize) as _) }
    }
    #[doc = "TX port/pin select. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn usart1_txroute_tgl(self) -> crate::common::Reg<regs::Usart1Txroute, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3654usize) as _) }
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
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Euart0Ctsroute(pub u32);
    impl Euart0Ctsroute {
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
    impl Default for Euart0Ctsroute {
        #[inline(always)]
        fn default() -> Euart0Ctsroute {
            Euart0Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Euart0Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Euart0Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Euart0Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Euart0Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "EUART pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Euart0Routeen(pub u32);
    impl Euart0Routeen {
        #[doc = "RTS pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rtspen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTS pin enable control bit."]
        #[inline(always)]
        pub const fn set_rtspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin enable control bit."]
        #[inline(always)]
        pub const fn set_txpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Euart0Routeen {
        #[inline(always)]
        fn default() -> Euart0Routeen {
            Euart0Routeen(0)
        }
    }
    impl core::fmt::Debug for Euart0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Euart0Routeen")
                .field("rtspen", &self.rtspen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Euart0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Euart0Routeen {{ rtspen: {=bool:?}, txpen: {=bool:?} }}",
                self.rtspen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Euart0Rtsroute(pub u32);
    impl Euart0Rtsroute {
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
    impl Default for Euart0Rtsroute {
        #[inline(always)]
        fn default() -> Euart0Rtsroute {
            Euart0Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Euart0Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Euart0Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Euart0Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Euart0Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Euart0Rxroute(pub u32);
    impl Euart0Rxroute {
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
    impl Default for Euart0Rxroute {
        #[inline(always)]
        fn default() -> Euart0Rxroute {
            Euart0Rxroute(0)
        }
    }
    impl core::fmt::Debug for Euart0Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Euart0Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Euart0Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Euart0Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Euart0Txroute(pub u32);
    impl Euart0Txroute {
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
    impl Default for Euart0Txroute {
        #[inline(always)]
        fn default() -> Euart0Txroute {
            Euart0Txroute(0)
        }
    }
    impl core::fmt::Debug for Euart0Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Euart0Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Euart0Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Euart0Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
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
    #[doc = "OUT0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Letimer0Out0route(pub u32);
    impl Letimer0Out0route {
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
    impl Default for Letimer0Out0route {
        #[inline(always)]
        fn default() -> Letimer0Out0route {
            Letimer0Out0route(0)
        }
    }
    impl core::fmt::Debug for Letimer0Out0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Letimer0Out0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Letimer0Out0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Letimer0Out0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "OUT1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Letimer0Out1route(pub u32);
    impl Letimer0Out1route {
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
    impl Default for Letimer0Out1route {
        #[inline(always)]
        fn default() -> Letimer0Out1route {
            Letimer0Out1route(0)
        }
    }
    impl core::fmt::Debug for Letimer0Out1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Letimer0Out1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Letimer0Out1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Letimer0Out1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "LETIMER pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Letimer0Routeen(pub u32);
    impl Letimer0Routeen {
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
    impl Default for Letimer0Routeen {
        #[inline(always)]
        fn default() -> Letimer0Routeen {
            Letimer0Routeen(0)
        }
    }
    impl core::fmt::Debug for Letimer0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Letimer0Routeen")
                .field("out0pen", &self.out0pen())
                .field("out1pen", &self.out1pen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Letimer0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Letimer0Routeen {{ out0pen: {=bool:?}, out1pen: {=bool:?} }}",
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
    #[doc = "CLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdmClkroute(pub u32);
    impl PdmClkroute {
        #[doc = "CLK port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "CLK port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CLK pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CLK pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for PdmClkroute {
        #[inline(always)]
        fn default() -> PdmClkroute {
            PdmClkroute(0)
        }
    }
    impl core::fmt::Debug for PdmClkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdmClkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdmClkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PdmClkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DAT0 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdmDat0route(pub u32);
    impl PdmDat0route {
        #[doc = "DAT0 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DAT0 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DAT0 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DAT0 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for PdmDat0route {
        #[inline(always)]
        fn default() -> PdmDat0route {
            PdmDat0route(0)
        }
    }
    impl core::fmt::Debug for PdmDat0route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdmDat0route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdmDat0route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PdmDat0route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "DAT1 port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdmDat1route(pub u32);
    impl PdmDat1route {
        #[doc = "DAT1 port select register."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "DAT1 port select register."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "DAT1 pin select register."]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "DAT1 pin select register."]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for PdmDat1route {
        #[inline(always)]
        fn default() -> PdmDat1route {
            PdmDat1route(0)
        }
    }
    impl core::fmt::Debug for PdmDat1route {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdmDat1route")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdmDat1route {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PdmDat1route {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "PDM pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdmRouteen(pub u32);
    impl PdmRouteen {
        #[doc = "CLK pin enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clkpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_clkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PdmRouteen {
        #[inline(always)]
        fn default() -> PdmRouteen {
            PdmRouteen(0)
        }
    }
    impl core::fmt::Debug for PdmRouteen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdmRouteen").field("clkpen", &self.clkpen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdmRouteen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PdmRouteen {{ clkpen: {=bool:?} }}", self.clkpen())
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
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Data input."]
        #[inline(always)]
        pub const fn set_din(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Data output."]
        #[inline(always)]
        pub const fn set_dout(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
    }
    impl Default for PortModeh {
        #[inline(always)]
        fn default() -> PortModeh {
            PortModeh(0)
        }
    }
    impl core::fmt::Debug for PortModeh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortModeh").field("mode0", &self.mode0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortModeh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PortModeh {{ mode0: {:?} }}", self.mode0())
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Traceroutepen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Traceroutepen {{ swvpen: {=bool:?}, traceclkpen: {=bool:?}, tracedata0pen: {=bool:?} }}",
                self.swvpen(),
                self.traceclkpen(),
                self.tracedata0pen()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Clkroute(pub u32);
    impl Usart0Clkroute {
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
    impl Default for Usart0Clkroute {
        #[inline(always)]
        fn default() -> Usart0Clkroute {
            Usart0Clkroute(0)
        }
    }
    impl core::fmt::Debug for Usart0Clkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Clkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Clkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Clkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Csroute(pub u32);
    impl Usart0Csroute {
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
    impl Default for Usart0Csroute {
        #[inline(always)]
        fn default() -> Usart0Csroute {
            Usart0Csroute(0)
        }
    }
    impl core::fmt::Debug for Usart0Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Ctsroute(pub u32);
    impl Usart0Ctsroute {
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
    impl Default for Usart0Ctsroute {
        #[inline(always)]
        fn default() -> Usart0Ctsroute {
            Usart0Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Usart0Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "USART0 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Routeen(pub u32);
    impl Usart0Routeen {
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
        pub const fn clkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_clkpen(&mut self, val: bool) {
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
    impl Default for Usart0Routeen {
        #[inline(always)]
        fn default() -> Usart0Routeen {
            Usart0Routeen(0)
        }
    }
    impl core::fmt::Debug for Usart0Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("clkpen", &self.clkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, clkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.clkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Rtsroute(pub u32);
    impl Usart0Rtsroute {
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
    impl Default for Usart0Rtsroute {
        #[inline(always)]
        fn default() -> Usart0Rtsroute {
            Usart0Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Usart0Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Rxroute(pub u32);
    impl Usart0Rxroute {
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
    impl Default for Usart0Rxroute {
        #[inline(always)]
        fn default() -> Usart0Rxroute {
            Usart0Rxroute(0)
        }
    }
    impl core::fmt::Debug for Usart0Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart0Txroute(pub u32);
    impl Usart0Txroute {
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
    impl Default for Usart0Txroute {
        #[inline(always)]
        fn default() -> Usart0Txroute {
            Usart0Txroute(0)
        }
    }
    impl core::fmt::Debug for Usart0Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart0Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart0Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart0Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "SCLK port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Clkroute(pub u32);
    impl Usart1Clkroute {
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
    impl Default for Usart1Clkroute {
        #[inline(always)]
        fn default() -> Usart1Clkroute {
            Usart1Clkroute(0)
        }
    }
    impl core::fmt::Debug for Usart1Clkroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Clkroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Clkroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Clkroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Csroute(pub u32);
    impl Usart1Csroute {
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
    impl Default for Usart1Csroute {
        #[inline(always)]
        fn default() -> Usart1Csroute {
            Usart1Csroute(0)
        }
    }
    impl core::fmt::Debug for Usart1Csroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Csroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Csroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Csroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "CTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Ctsroute(pub u32);
    impl Usart1Ctsroute {
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
    impl Default for Usart1Ctsroute {
        #[inline(always)]
        fn default() -> Usart1Ctsroute {
            Usart1Ctsroute(0)
        }
    }
    impl core::fmt::Debug for Usart1Ctsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Ctsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Ctsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Ctsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "USART1 pin enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Routeen(pub u32);
    impl Usart1Routeen {
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
        pub const fn clkpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SCLK pin enable control bit."]
        #[inline(always)]
        pub const fn set_clkpen(&mut self, val: bool) {
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
    impl Default for Usart1Routeen {
        #[inline(always)]
        fn default() -> Usart1Routeen {
            Usart1Routeen(0)
        }
    }
    impl core::fmt::Debug for Usart1Routeen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Routeen")
                .field("cspen", &self.cspen())
                .field("rtspen", &self.rtspen())
                .field("rxpen", &self.rxpen())
                .field("clkpen", &self.clkpen())
                .field("txpen", &self.txpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Routeen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Routeen {{ cspen: {=bool:?}, rtspen: {=bool:?}, rxpen: {=bool:?}, clkpen: {=bool:?}, txpen: {=bool:?} }}",
                self.cspen(),
                self.rtspen(),
                self.rxpen(),
                self.clkpen(),
                self.txpen()
            )
        }
    }
    #[doc = "RTS port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Rtsroute(pub u32);
    impl Usart1Rtsroute {
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
    impl Default for Usart1Rtsroute {
        #[inline(always)]
        fn default() -> Usart1Rtsroute {
            Usart1Rtsroute(0)
        }
    }
    impl core::fmt::Debug for Usart1Rtsroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Rtsroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Rtsroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Rtsroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "RX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Rxroute(pub u32);
    impl Usart1Rxroute {
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
    impl Default for Usart1Rxroute {
        #[inline(always)]
        fn default() -> Usart1Rxroute {
            Usart1Rxroute(0)
        }
    }
    impl core::fmt::Debug for Usart1Rxroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Rxroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Rxroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Rxroute {{ port: {=u8:?}, pin: {=u8:?} }}",
                self.port(),
                self.pin()
            )
        }
    }
    #[doc = "TX port/pin select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usart1Txroute(pub u32);
    impl Usart1Txroute {
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
    impl Default for Usart1Txroute {
        #[inline(always)]
        fn default() -> Usart1Txroute {
            Usart1Txroute(0)
        }
    }
    impl core::fmt::Debug for Usart1Txroute {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usart1Txroute")
                .field("port", &self.port())
                .field("pin", &self.pin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usart1Txroute {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Usart1Txroute {{ port: {=u8:?}, pin: {=u8:?} }}",
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset8 = 0x0,
        #[doc = "OFFSET=9."]
        Offset9 = 0x01,
        #[doc = "OFFSET=10."]
        Offset10 = 0x02,
        #[doc = "OFFSET=11."]
        Offset11 = 0x03,
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
        Offset8 = 0x0,
        #[doc = "OFFSET=9."]
        Offset9 = 0x01,
        #[doc = "OFFSET=10."]
        Offset10 = 0x02,
        #[doc = "OFFSET=11."]
        Offset11 = 0x03,
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
        Offset8 = 0x0,
        #[doc = "OFFSET=9."]
        Offset9 = 0x01,
        #[doc = "OFFSET=10."]
        Offset10 = 0x02,
        #[doc = "OFFSET=11."]
        Offset11 = 0x03,
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
        Offset8 = 0x0,
        #[doc = "OFFSET=9."]
        Offset9 = 0x01,
        #[doc = "OFFSET=10."]
        Offset10 = 0x02,
        #[doc = "OFFSET=11."]
        Offset11 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
        Offset0 = 0x0,
        #[doc = "OFFSET=1."]
        Offset1 = 0x01,
        #[doc = "OFFSET=2."]
        Offset2 = 0x02,
        #[doc = "OFFSET=3."]
        Offset3 = 0x03,
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
}
