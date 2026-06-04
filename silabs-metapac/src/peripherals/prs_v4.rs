#[doc = "PRS peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs {
    ptr: *mut u8,
}
unsafe impl Send for Prs {}
unsafe impl Sync for Prs {}
impl Prs {
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
    pub const fn async_swpulse(self) -> crate::common::Reg<regs::AsyncSwpulse, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_swlevel(self) -> crate::common::Reg<regs::AsyncSwlevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_peek(self) -> crate::common::Reg<regs::AsyncPeek, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sync_peek(self) -> crate::common::Reg<regs::SyncPeek, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch0_ctrl(self) -> crate::common::Reg<regs::AsyncCh0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch1_ctrl(self) -> crate::common::Reg<regs::AsyncCh1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch2_ctrl(self) -> crate::common::Reg<regs::AsyncCh2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch3_ctrl(self) -> crate::common::Reg<regs::AsyncCh3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch4_ctrl(self) -> crate::common::Reg<regs::AsyncCh4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch5_ctrl(self) -> crate::common::Reg<regs::AsyncCh5Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch6_ctrl(self) -> crate::common::Reg<regs::AsyncCh6Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch7_ctrl(self) -> crate::common::Reg<regs::AsyncCh7Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch8_ctrl(self) -> crate::common::Reg<regs::AsyncCh8Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch9_ctrl(self) -> crate::common::Reg<regs::AsyncCh9Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch10_ctrl(self) -> crate::common::Reg<regs::AsyncCh10Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn async_ch11_ctrl(self) -> crate::common::Reg<regs::AsyncCh11Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sync_ch0_ctrl(self) -> crate::common::Reg<regs::SyncCh0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sync_ch1_ctrl(self) -> crate::common::Reg<regs::SyncCh1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sync_ch2_ctrl(self) -> crate::common::Reg<regs::SyncCh2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub const fn sync_ch3_ctrl(self) -> crate::common::Reg<regs::SyncCh3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "CALDN consumer register."]
    #[inline(always)]
    pub const fn consumer_cmu_caldn(self) -> crate::common::Reg<regs::ConsumerCmuCaldn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "CALUP Consumer register."]
    #[inline(always)]
    pub const fn consumer_cmu_calup(self) -> crate::common::Reg<regs::ConsumerCmuCalup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "CLK consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart0_clk(self) -> crate::common::Reg<regs::ConsumerEusart0Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "RX Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart0_rx(self) -> crate::common::Reg<regs::ConsumerEusart0Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "TRIGGER Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart0_trigger(self) -> crate::common::Reg<regs::ConsumerEusart0Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "CLK consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart1_clk(self) -> crate::common::Reg<regs::ConsumerEusart1Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "RX Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart1_rx(self) -> crate::common::Reg<regs::ConsumerEusart1Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "TRIGGER Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart1_trigger(self) -> crate::common::Reg<regs::ConsumerEusart1Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "CLK consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart2_clk(self) -> crate::common::Reg<regs::ConsumerEusart2Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "RX Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart2_rx(self) -> crate::common::Reg<regs::ConsumerEusart2Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "TRIGGER Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart2_trigger(self) -> crate::common::Reg<regs::ConsumerEusart2Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "CLK consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart3_clk(self) -> crate::common::Reg<regs::ConsumerEusart3Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "RX Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart3_rx(self) -> crate::common::Reg<regs::ConsumerEusart3Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "TRIGGER Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart3_trigger(self) -> crate::common::Reg<regs::ConsumerEusart3Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "CLK consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart4_clk(self) -> crate::common::Reg<regs::ConsumerEusart4Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RX Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart4_rx(self) -> crate::common::Reg<regs::ConsumerEusart4Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "TRIGGER Consumer register."]
    #[inline(always)]
    pub const fn consumer_eusart4_trigger(self) -> crate::common::Reg<regs::ConsumerEusart4Trigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "RXRAW consumer register."]
    #[inline(always)]
    pub const fn consumer_frc_rxraw(self) -> crate::common::Reg<regs::ConsumerFrcRxraw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "SCAN consumer register."]
    #[inline(always)]
    pub const fn consumer_iadc0_scantrigger(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Scantrigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "SINGLE Consumer register."]
    #[inline(always)]
    pub const fn consumer_iadc0_singletrigger(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Singletrigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "DMAREQ0 consumer register."]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq0(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "DMAREQ1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq1(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "DECIN0 consumer register."]
    #[inline(always)]
    pub const fn consumer_lesense_decin0(self) -> crate::common::Reg<regs::ConsumerLesenseDecin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "DECIN1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_lesense_decin1(self) -> crate::common::Reg<regs::ConsumerLesenseDecin1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "DECIN2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_lesense_decin2(self) -> crate::common::Reg<regs::ConsumerLesenseDecin2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "DECIN3 Consumer register."]
    #[inline(always)]
    pub const fn consumer_lesense_decin3(self) -> crate::common::Reg<regs::ConsumerLesenseDecin3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "START Consumer register."]
    #[inline(always)]
    pub const fn consumer_lesense_start(self) -> crate::common::Reg<regs::ConsumerLesenseStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "CLEAR consumer register."]
    #[inline(always)]
    pub const fn consumer_letimer0_clear(self) -> crate::common::Reg<regs::ConsumerLetimer0Clear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "START Consumer register."]
    #[inline(always)]
    pub const fn consumer_letimer0_start(self) -> crate::common::Reg<regs::ConsumerLetimer0Start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "STOP Consumer register."]
    #[inline(always)]
    pub const fn consumer_letimer0_stop(self) -> crate::common::Reg<regs::ConsumerLetimer0Stop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "DIN consumer register."]
    #[inline(always)]
    pub const fn consumer_modem_din(self) -> crate::common::Reg<regs::ConsumerModemDin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "S0IN consumer register."]
    #[inline(always)]
    pub const fn consumer_pcnt0_s0in(self) -> crate::common::Reg<regs::ConsumerPcnt0S0in, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "S1IN Consumer register."]
    #[inline(always)]
    pub const fn consumer_pcnt0_s1in(self) -> crate::common::Reg<regs::ConsumerPcnt0S1in, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "CLR consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_clr(self) -> crate::common::Reg<regs::ConsumerRacClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_ctiin0(self) -> crate::common::Reg<regs::ConsumerRacCtiin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_ctiin1(self) -> crate::common::Reg<regs::ConsumerRacCtiin1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_ctiin2(self) -> crate::common::Reg<regs::ConsumerRacCtiin2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_ctiin3(self) -> crate::common::Reg<regs::ConsumerRacCtiin3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "FORCETX Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_forcetx(self) -> crate::common::Reg<regs::ConsumerRacForcetx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "RXDIS Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_rxdis(self) -> crate::common::Reg<regs::ConsumerRacRxdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "RXEN Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_rxen(self) -> crate::common::Reg<regs::ConsumerRacRxen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "TXEN Consumer register."]
    #[inline(always)]
    pub const fn consumer_rac_txen(self) -> crate::common::Reg<regs::ConsumerRacTxen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "TAMPERSRC26 consumer register."]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc26(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "TAMPERSRC27 Consumer register."]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc27(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "TAMPERSRC28 Consumer register."]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc28(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "TAMPERSRC29 Consumer register."]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc29(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "TAMPERSRC30 Consumer register."]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc30(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "TAMPERSRC31 Consumer register."]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc31(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "IN0 consumer register."]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in0(self) -> crate::common::Reg<regs::ConsumerSysrtc0In0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "IN1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in1(self) -> crate::common::Reg<regs::ConsumerSysrtc0In1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "OSCREQ consumer register."]
    #[inline(always)]
    pub const fn consumer_hfxo0_oscreq(self) -> crate::common::Reg<regs::ConsumerHfxo0Oscreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "TIMEOUT Consumer register."]
    #[inline(always)]
    pub const fn consumer_hfxo0_timeout(self) -> crate::common::Reg<regs::ConsumerHfxo0Timeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "CTI consumer register."]
    #[inline(always)]
    pub const fn consumer_core_ctiin0(self) -> crate::common::Reg<regs::ConsumerCoreCtiin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_core_ctiin1(self) -> crate::common::Reg<regs::ConsumerCoreCtiin1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_core_ctiin2(self) -> crate::common::Reg<regs::ConsumerCoreCtiin2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "CTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_core_ctiin3(self) -> crate::common::Reg<regs::ConsumerCoreCtiin3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "M33 Consumer register."]
    #[inline(always)]
    pub const fn consumer_core_m33rxev(self) -> crate::common::Reg<regs::ConsumerCoreM33rxev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer0_cc0(self) -> crate::common::Reg<regs::ConsumerTimer0Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer0_cc1(self) -> crate::common::Reg<regs::ConsumerTimer0Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer0_cc2(self) -> crate::common::Reg<regs::ConsumerTimer0Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer0_dti(self) -> crate::common::Reg<regs::ConsumerTimer0Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer1_cc0(self) -> crate::common::Reg<regs::ConsumerTimer1Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer1_cc1(self) -> crate::common::Reg<regs::ConsumerTimer1Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer1_cc2(self) -> crate::common::Reg<regs::ConsumerTimer1Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer1_dti(self) -> crate::common::Reg<regs::ConsumerTimer1Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer2_cc0(self) -> crate::common::Reg<regs::ConsumerTimer2Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer2_cc1(self) -> crate::common::Reg<regs::ConsumerTimer2Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer2_cc2(self) -> crate::common::Reg<regs::ConsumerTimer2Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer2_dti(self) -> crate::common::Reg<regs::ConsumerTimer2Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer3_cc0(self) -> crate::common::Reg<regs::ConsumerTimer3Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer3_cc1(self) -> crate::common::Reg<regs::ConsumerTimer3Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer3_cc2(self) -> crate::common::Reg<regs::ConsumerTimer3Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer3_dti(self) -> crate::common::Reg<regs::ConsumerTimer3Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer4_cc0(self) -> crate::common::Reg<regs::ConsumerTimer4Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer4_cc1(self) -> crate::common::Reg<regs::ConsumerTimer4Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer4_cc2(self) -> crate::common::Reg<regs::ConsumerTimer4Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer4_dti(self) -> crate::common::Reg<regs::ConsumerTimer4Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer5_cc0(self) -> crate::common::Reg<regs::ConsumerTimer5Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer5_cc1(self) -> crate::common::Reg<regs::ConsumerTimer5Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer5_cc2(self) -> crate::common::Reg<regs::ConsumerTimer5Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer5_dti(self) -> crate::common::Reg<regs::ConsumerTimer5Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer6_cc0(self) -> crate::common::Reg<regs::ConsumerTimer6Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer6_cc1(self) -> crate::common::Reg<regs::ConsumerTimer6Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer6_cc2(self) -> crate::common::Reg<regs::ConsumerTimer6Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer6_dti(self) -> crate::common::Reg<regs::ConsumerTimer6Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "CC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_timer7_cc0(self) -> crate::common::Reg<regs::ConsumerTimer7Cc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "CC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer7_cc1(self) -> crate::common::Reg<regs::ConsumerTimer7Cc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "CC2 Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer7_cc2(self) -> crate::common::Reg<regs::ConsumerTimer7Cc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer7_dti(self) -> crate::common::Reg<regs::ConsumerTimer7Dti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs1(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "DTI Consumer register."]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs2(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "ASYNCTRIG consumer register."]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch0(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "ASYNCTRIG Consumer register."]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch1(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register."]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch0(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register."]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch1(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "SRC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_wdog0_src0(self) -> crate::common::Reg<regs::ConsumerWdog0Src0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "SRC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_wdog0_src1(self) -> crate::common::Reg<regs::ConsumerWdog0Src1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "SRC0 consumer register."]
    #[inline(always)]
    pub const fn consumer_wdog1_src0(self) -> crate::common::Reg<regs::ConsumerWdog1Src0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "SRC1 Consumer register."]
    #[inline(always)]
    pub const fn consumer_wdog1_src1(self) -> crate::common::Reg<regs::ConsumerWdog1Src1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_swpulse_set(self) -> crate::common::Reg<regs::AsyncSwpulse, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_swlevel_set(self) -> crate::common::Reg<regs::AsyncSwlevel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch0_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch1_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch2_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch3_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch4_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh4Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch5_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh5Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch6_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh6Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch7_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh7Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch8_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh8Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch9_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh9Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch10_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh10Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn async_ch11_ctrl_set(self) -> crate::common::Reg<regs::AsyncCh11Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sync_ch0_ctrl_set(self) -> crate::common::Reg<regs::SyncCh0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sync_ch1_ctrl_set(self) -> crate::common::Reg<regs::SyncCh1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sync_ch2_ctrl_set(self) -> crate::common::Reg<regs::SyncCh2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn sync_ch3_ctrl_set(self) -> crate::common::Reg<regs::SyncCh3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "CALDN consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_cmu_caldn_set(self) -> crate::common::Reg<regs::ConsumerCmuCaldn, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "CALUP Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_cmu_calup_set(self) -> crate::common::Reg<regs::ConsumerCmuCalup, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_clk_set(self) -> crate::common::Reg<regs::ConsumerEusart0Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_rx_set(self) -> crate::common::Reg<regs::ConsumerEusart0Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_trigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart0Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_clk_set(self) -> crate::common::Reg<regs::ConsumerEusart1Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_rx_set(self) -> crate::common::Reg<regs::ConsumerEusart1Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_trigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart1Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_clk_set(self) -> crate::common::Reg<regs::ConsumerEusart2Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_rx_set(self) -> crate::common::Reg<regs::ConsumerEusart2Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_trigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart2Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_clk_set(self) -> crate::common::Reg<regs::ConsumerEusart3Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1084usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_rx_set(self) -> crate::common::Reg<regs::ConsumerEusart3Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_trigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart3Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x108cusize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_clk_set(self) -> crate::common::Reg<regs::ConsumerEusart4Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_rx_set(self) -> crate::common::Reg<regs::ConsumerEusart4Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_trigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart4Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "RXRAW consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_frc_rxraw_set(self) -> crate::common::Reg<regs::ConsumerFrcRxraw, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109cusize) as _) }
    }
    #[doc = "SCAN consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_iadc0_scantrigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Scantrigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "SINGLE Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_iadc0_singletrigger_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Singletrigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "DMAREQ0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq0_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "DMAREQ1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq1_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10acusize) as _) }
    }
    #[doc = "DECIN0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin0_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "DECIN1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin1_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b4usize) as _) }
    }
    #[doc = "DECIN2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin2_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b8usize) as _) }
    }
    #[doc = "DECIN3 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin3_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10bcusize) as _) }
    }
    #[doc = "START Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_start_set(self) -> crate::common::Reg<regs::ConsumerLesenseStart, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "CLEAR consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_clear_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLetimer0Clear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "START Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_start_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerLetimer0Start, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "STOP Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_stop_set(self) -> crate::common::Reg<regs::ConsumerLetimer0Stop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "DIN consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_modem_din_set(self) -> crate::common::Reg<regs::ConsumerModemDin, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "S0IN consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s0in_set(self) -> crate::common::Reg<regs::ConsumerPcnt0S0in, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "S1IN Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s1in_set(self) -> crate::common::Reg<regs::ConsumerPcnt0S1in, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "CLR consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_clr_set(self) -> crate::common::Reg<regs::ConsumerRacClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin0_set(self) -> crate::common::Reg<regs::ConsumerRacCtiin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x110cusize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin1_set(self) -> crate::common::Reg<regs::ConsumerRacCtiin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1110usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin2_set(self) -> crate::common::Reg<regs::ConsumerRacCtiin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin3_set(self) -> crate::common::Reg<regs::ConsumerRacCtiin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize) as _) }
    }
    #[doc = "FORCETX Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_forcetx_set(self) -> crate::common::Reg<regs::ConsumerRacForcetx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "RXDIS Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_rxdis_set(self) -> crate::common::Reg<regs::ConsumerRacRxdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "RXEN Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_rxen_set(self) -> crate::common::Reg<regs::ConsumerRacRxen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "TXEN Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_rac_txen_set(self) -> crate::common::Reg<regs::ConsumerRacTxen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TAMPERSRC26 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc26_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc26, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "TAMPERSRC27 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc27_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc27, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "TAMPERSRC28 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc28_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc28, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "TAMPERSRC29 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc29_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc29, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "TAMPERSRC30 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc30_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc30, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "TAMPERSRC31 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc31_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc31, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "IN0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in0_set(self) -> crate::common::Reg<regs::ConsumerSysrtc0In0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "IN1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in1_set(self) -> crate::common::Reg<regs::ConsumerSysrtc0In1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "OSCREQ consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_hfxo0_oscreq_set(self) -> crate::common::Reg<regs::ConsumerHfxo0Oscreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "TIMEOUT Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_hfxo0_timeout_set(self) -> crate::common::Reg<regs::ConsumerHfxo0Timeout, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "CTI consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin0_set(self) -> crate::common::Reg<regs::ConsumerCoreCtiin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin1_set(self) -> crate::common::Reg<regs::ConsumerCoreCtiin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1158usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin2_set(self) -> crate::common::Reg<regs::ConsumerCoreCtiin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin3_set(self) -> crate::common::Reg<regs::ConsumerCoreCtiin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "M33 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_core_m33rxev_set(self) -> crate::common::Reg<regs::ConsumerCoreM33rxev, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer0Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer0Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer0Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer0Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1174usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1178usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117cusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer1Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer1Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer1Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer1Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x118cusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1190usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer2Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1198usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer2Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119cusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer2Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer2Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a4usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11acusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer3Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer3Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer3Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer3Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer4Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer4Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ccusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer4Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer4Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d4usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11dcusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer5Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer5Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer5Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer5Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f4usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer6Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f8usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer6Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11fcusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer6Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer6Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1204usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1208usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x120cusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc0_set(self) -> crate::common::Reg<regs::ConsumerTimer7Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1210usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc1_set(self) -> crate::common::Reg<regs::ConsumerTimer7Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1214usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc2_set(self) -> crate::common::Reg<regs::ConsumerTimer7Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1218usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dti_set(self) -> crate::common::Reg<regs::ConsumerTimer7Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x121cusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs1_set(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1220usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs2_set(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1224usize) as _) }
    }
    #[doc = "ASYNCTRIG consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch0_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1228usize) as _) }
    }
    #[doc = "ASYNCTRIG Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch1_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x122cusize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch0_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1230usize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch1_set(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1234usize) as _) }
    }
    #[doc = "SRC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_wdog0_src0_set(self) -> crate::common::Reg<regs::ConsumerWdog0Src0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1238usize) as _) }
    }
    #[doc = "SRC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_wdog0_src1_set(self) -> crate::common::Reg<regs::ConsumerWdog0Src1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x123cusize) as _) }
    }
    #[doc = "SRC0 consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_wdog1_src0_set(self) -> crate::common::Reg<regs::ConsumerWdog1Src0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1240usize) as _) }
    }
    #[doc = "SRC1 Consumer register. (write-1-to-set alias)"]
    #[inline(always)]
    pub const fn consumer_wdog1_src1_set(self) -> crate::common::Reg<regs::ConsumerWdog1Src1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1244usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_swpulse_clr(self) -> crate::common::Reg<regs::AsyncSwpulse, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_swlevel_clr(self) -> crate::common::Reg<regs::AsyncSwlevel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch0_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch1_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch2_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch3_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch4_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh4Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch5_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh5Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x202cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch6_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh6Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch7_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh7Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch8_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh8Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch9_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh9Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x203cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch10_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh10Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn async_ch11_ctrl_clr(self) -> crate::common::Reg<regs::AsyncCh11Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sync_ch0_ctrl_clr(self) -> crate::common::Reg<regs::SyncCh0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sync_ch1_ctrl_clr(self) -> crate::common::Reg<regs::SyncCh1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x204cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sync_ch2_ctrl_clr(self) -> crate::common::Reg<regs::SyncCh2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn sync_ch3_ctrl_clr(self) -> crate::common::Reg<regs::SyncCh3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2054usize) as _) }
    }
    #[doc = "CALDN consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_cmu_caldn_clr(self) -> crate::common::Reg<regs::ConsumerCmuCaldn, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2058usize) as _) }
    }
    #[doc = "CALUP Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_cmu_calup_clr(self) -> crate::common::Reg<regs::ConsumerCmuCalup, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x205cusize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_clk_clr(self) -> crate::common::Reg<regs::ConsumerEusart0Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2060usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_rx_clr(self) -> crate::common::Reg<regs::ConsumerEusart0Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2064usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_trigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart0Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2068usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_clk_clr(self) -> crate::common::Reg<regs::ConsumerEusart1Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x206cusize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_rx_clr(self) -> crate::common::Reg<regs::ConsumerEusart1Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2070usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_trigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart1Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2074usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_clk_clr(self) -> crate::common::Reg<regs::ConsumerEusart2Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2078usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_rx_clr(self) -> crate::common::Reg<regs::ConsumerEusart2Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x207cusize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_trigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart2Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_clk_clr(self) -> crate::common::Reg<regs::ConsumerEusart3Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2084usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_rx_clr(self) -> crate::common::Reg<regs::ConsumerEusart3Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2088usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_trigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart3Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x208cusize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_clk_clr(self) -> crate::common::Reg<regs::ConsumerEusart4Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2090usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_rx_clr(self) -> crate::common::Reg<regs::ConsumerEusart4Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2094usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_trigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart4Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2098usize) as _) }
    }
    #[doc = "RXRAW consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_frc_rxraw_clr(self) -> crate::common::Reg<regs::ConsumerFrcRxraw, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x209cusize) as _) }
    }
    #[doc = "SCAN consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_iadc0_scantrigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Scantrigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a0usize) as _) }
    }
    #[doc = "SINGLE Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_iadc0_singletrigger_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Singletrigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a4usize) as _) }
    }
    #[doc = "DMAREQ0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq0_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20a8usize) as _) }
    }
    #[doc = "DMAREQ1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq1_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20acusize) as _) }
    }
    #[doc = "DECIN0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin0_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b0usize) as _) }
    }
    #[doc = "DECIN1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin1_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b4usize) as _) }
    }
    #[doc = "DECIN2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin2_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20b8usize) as _) }
    }
    #[doc = "DECIN3 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin3_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20bcusize) as _) }
    }
    #[doc = "START Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_start_clr(self) -> crate::common::Reg<regs::ConsumerLesenseStart, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c0usize) as _) }
    }
    #[doc = "CLEAR consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_clear_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLetimer0Clear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c4usize) as _) }
    }
    #[doc = "START Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_start_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerLetimer0Start, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20c8usize) as _) }
    }
    #[doc = "STOP Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_stop_clr(self) -> crate::common::Reg<regs::ConsumerLetimer0Stop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20ccusize) as _) }
    }
    #[doc = "DIN consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_modem_din_clr(self) -> crate::common::Reg<regs::ConsumerModemDin, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d0usize) as _) }
    }
    #[doc = "S0IN consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s0in_clr(self) -> crate::common::Reg<regs::ConsumerPcnt0S0in, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d4usize) as _) }
    }
    #[doc = "S1IN Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s1in_clr(self) -> crate::common::Reg<regs::ConsumerPcnt0S1in, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20d8usize) as _) }
    }
    #[doc = "CLR consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_clr_clr(self) -> crate::common::Reg<regs::ConsumerRacClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2108usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin0_clr(self) -> crate::common::Reg<regs::ConsumerRacCtiin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x210cusize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin1_clr(self) -> crate::common::Reg<regs::ConsumerRacCtiin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2110usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin2_clr(self) -> crate::common::Reg<regs::ConsumerRacCtiin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2114usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin3_clr(self) -> crate::common::Reg<regs::ConsumerRacCtiin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2118usize) as _) }
    }
    #[doc = "FORCETX Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_forcetx_clr(self) -> crate::common::Reg<regs::ConsumerRacForcetx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x211cusize) as _) }
    }
    #[doc = "RXDIS Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_rxdis_clr(self) -> crate::common::Reg<regs::ConsumerRacRxdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "RXEN Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_rxen_clr(self) -> crate::common::Reg<regs::ConsumerRacRxen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "TXEN Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_rac_txen_clr(self) -> crate::common::Reg<regs::ConsumerRacTxen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2128usize) as _) }
    }
    #[doc = "TAMPERSRC26 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc26_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc26, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x212cusize) as _) }
    }
    #[doc = "TAMPERSRC27 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc27_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc27, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2130usize) as _) }
    }
    #[doc = "TAMPERSRC28 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc28_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc28, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2134usize) as _) }
    }
    #[doc = "TAMPERSRC29 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc29_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc29, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2138usize) as _) }
    }
    #[doc = "TAMPERSRC30 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc30_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc30, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x213cusize) as _) }
    }
    #[doc = "TAMPERSRC31 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc31_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc31, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize) as _) }
    }
    #[doc = "IN0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in0_clr(self) -> crate::common::Reg<regs::ConsumerSysrtc0In0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2144usize) as _) }
    }
    #[doc = "IN1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in1_clr(self) -> crate::common::Reg<regs::ConsumerSysrtc0In1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2148usize) as _) }
    }
    #[doc = "OSCREQ consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_hfxo0_oscreq_clr(self) -> crate::common::Reg<regs::ConsumerHfxo0Oscreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x214cusize) as _) }
    }
    #[doc = "TIMEOUT Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_hfxo0_timeout_clr(self) -> crate::common::Reg<regs::ConsumerHfxo0Timeout, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2150usize) as _) }
    }
    #[doc = "CTI consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin0_clr(self) -> crate::common::Reg<regs::ConsumerCoreCtiin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2154usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin1_clr(self) -> crate::common::Reg<regs::ConsumerCoreCtiin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2158usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin2_clr(self) -> crate::common::Reg<regs::ConsumerCoreCtiin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x215cusize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin3_clr(self) -> crate::common::Reg<regs::ConsumerCoreCtiin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    #[doc = "M33 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_core_m33rxev_clr(self) -> crate::common::Reg<regs::ConsumerCoreM33rxev, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2164usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer0Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2168usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer0Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x216cusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer0Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer0Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2174usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2178usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x217cusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer1Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer1Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2184usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer1Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2188usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer1Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x218cusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2190usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2194usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer2Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2198usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer2Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x219cusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer2Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer2Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a4usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21acusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer3Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b0usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer3Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b4usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer3Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer3Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21bcusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c4usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer4Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21c8usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer4Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21ccusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer4Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer4Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d4usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21d8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21dcusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer5Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e0usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer5Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e4usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer5Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21e8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer5Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21ecusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f4usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer6Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21f8usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer6Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21fcusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer6Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer6Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2204usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2208usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x220cusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc0_clr(self) -> crate::common::Reg<regs::ConsumerTimer7Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2210usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc1_clr(self) -> crate::common::Reg<regs::ConsumerTimer7Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2214usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc2_clr(self) -> crate::common::Reg<regs::ConsumerTimer7Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2218usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dti_clr(self) -> crate::common::Reg<regs::ConsumerTimer7Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x221cusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs1_clr(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2220usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs2_clr(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2224usize) as _) }
    }
    #[doc = "ASYNCTRIG consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch0_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2228usize) as _) }
    }
    #[doc = "ASYNCTRIG Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch1_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x222cusize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch0_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2230usize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch1_clr(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2234usize) as _) }
    }
    #[doc = "SRC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_wdog0_src0_clr(self) -> crate::common::Reg<regs::ConsumerWdog0Src0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2238usize) as _) }
    }
    #[doc = "SRC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_wdog0_src1_clr(self) -> crate::common::Reg<regs::ConsumerWdog0Src1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x223cusize) as _) }
    }
    #[doc = "SRC0 consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_wdog1_src0_clr(self) -> crate::common::Reg<regs::ConsumerWdog1Src0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2240usize) as _) }
    }
    #[doc = "SRC1 Consumer register. (write-1-to-clr alias)"]
    #[inline(always)]
    pub const fn consumer_wdog1_src1_clr(self) -> crate::common::Reg<regs::ConsumerWdog1Src1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2244usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_swpulse_tgl(self) -> crate::common::Reg<regs::AsyncSwpulse, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_swlevel_tgl(self) -> crate::common::Reg<regs::AsyncSwlevel, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch0_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch1_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch2_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch3_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch4_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh4Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch5_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh5Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch6_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh6Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch7_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh7Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch8_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh8Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch9_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh9Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch10_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh10Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn async_ch11_ctrl_tgl(self) -> crate::common::Reg<regs::AsyncCh11Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sync_ch0_ctrl_tgl(self) -> crate::common::Reg<regs::SyncCh0Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sync_ch1_ctrl_tgl(self) -> crate::common::Reg<regs::SyncCh1Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sync_ch2_ctrl_tgl(self) -> crate::common::Reg<regs::SyncCh2Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "No Description. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn sync_ch3_ctrl_tgl(self) -> crate::common::Reg<regs::SyncCh3Ctrl, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "CALDN consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_cmu_caldn_tgl(self) -> crate::common::Reg<regs::ConsumerCmuCaldn, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3058usize) as _) }
    }
    #[doc = "CALUP Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_cmu_calup_tgl(self) -> crate::common::Reg<regs::ConsumerCmuCalup, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x305cusize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_clk_tgl(self) -> crate::common::Reg<regs::ConsumerEusart0Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_rx_tgl(self) -> crate::common::Reg<regs::ConsumerEusart0Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart0_trigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart0Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3068usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_clk_tgl(self) -> crate::common::Reg<regs::ConsumerEusart1Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_rx_tgl(self) -> crate::common::Reg<regs::ConsumerEusart1Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart1_trigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart1Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_clk_tgl(self) -> crate::common::Reg<regs::ConsumerEusart2Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3078usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_rx_tgl(self) -> crate::common::Reg<regs::ConsumerEusart2Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x307cusize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart2_trigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart2Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3080usize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_clk_tgl(self) -> crate::common::Reg<regs::ConsumerEusart3Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3084usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_rx_tgl(self) -> crate::common::Reg<regs::ConsumerEusart3Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3088usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart3_trigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart3Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x308cusize) as _) }
    }
    #[doc = "CLK consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_clk_tgl(self) -> crate::common::Reg<regs::ConsumerEusart4Clk, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3090usize) as _) }
    }
    #[doc = "RX Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_rx_tgl(self) -> crate::common::Reg<regs::ConsumerEusart4Rx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3094usize) as _) }
    }
    #[doc = "TRIGGER Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_eusart4_trigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerEusart4Trigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3098usize) as _) }
    }
    #[doc = "RXRAW consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_frc_rxraw_tgl(self) -> crate::common::Reg<regs::ConsumerFrcRxraw, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x309cusize) as _) }
    }
    #[doc = "SCAN consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_iadc0_scantrigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Scantrigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a0usize) as _) }
    }
    #[doc = "SINGLE Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_iadc0_singletrigger_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerIadc0Singletrigger, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a4usize) as _) }
    }
    #[doc = "DMAREQ0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq0_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30a8usize) as _) }
    }
    #[doc = "DMAREQ1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq1_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLdmaxbarDmareq1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30acusize) as _) }
    }
    #[doc = "DECIN0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin0_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b0usize) as _) }
    }
    #[doc = "DECIN1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin1_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b4usize) as _) }
    }
    #[doc = "DECIN2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin2_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30b8usize) as _) }
    }
    #[doc = "DECIN3 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_decin3_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLesenseDecin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30bcusize) as _) }
    }
    #[doc = "START Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_lesense_start_tgl(self) -> crate::common::Reg<regs::ConsumerLesenseStart, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c0usize) as _) }
    }
    #[doc = "CLEAR consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_clear_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLetimer0Clear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c4usize) as _) }
    }
    #[doc = "START Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_start_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerLetimer0Start, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30c8usize) as _) }
    }
    #[doc = "STOP Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_letimer0_stop_tgl(self) -> crate::common::Reg<regs::ConsumerLetimer0Stop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30ccusize) as _) }
    }
    #[doc = "DIN consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_modem_din_tgl(self) -> crate::common::Reg<regs::ConsumerModemDin, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d0usize) as _) }
    }
    #[doc = "S0IN consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s0in_tgl(self) -> crate::common::Reg<regs::ConsumerPcnt0S0in, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d4usize) as _) }
    }
    #[doc = "S1IN Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_pcnt0_s1in_tgl(self) -> crate::common::Reg<regs::ConsumerPcnt0S1in, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30d8usize) as _) }
    }
    #[doc = "CLR consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_clr_tgl(self) -> crate::common::Reg<regs::ConsumerRacClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3108usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin0_tgl(self) -> crate::common::Reg<regs::ConsumerRacCtiin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x310cusize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin1_tgl(self) -> crate::common::Reg<regs::ConsumerRacCtiin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3110usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin2_tgl(self) -> crate::common::Reg<regs::ConsumerRacCtiin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3114usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin3_tgl(self) -> crate::common::Reg<regs::ConsumerRacCtiin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3118usize) as _) }
    }
    #[doc = "FORCETX Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_forcetx_tgl(self) -> crate::common::Reg<regs::ConsumerRacForcetx, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x311cusize) as _) }
    }
    #[doc = "RXDIS Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_rxdis_tgl(self) -> crate::common::Reg<regs::ConsumerRacRxdis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3120usize) as _) }
    }
    #[doc = "RXEN Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_rxen_tgl(self) -> crate::common::Reg<regs::ConsumerRacRxen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3124usize) as _) }
    }
    #[doc = "TXEN Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_rac_txen_tgl(self) -> crate::common::Reg<regs::ConsumerRacTxen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3128usize) as _) }
    }
    #[doc = "TAMPERSRC26 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc26_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc26, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x312cusize) as _) }
    }
    #[doc = "TAMPERSRC27 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc27_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc27, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3130usize) as _) }
    }
    #[doc = "TAMPERSRC28 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc28_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc28, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3134usize) as _) }
    }
    #[doc = "TAMPERSRC29 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc29_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc29, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3138usize) as _) }
    }
    #[doc = "TAMPERSRC30 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc30_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc30, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x313cusize) as _) }
    }
    #[doc = "TAMPERSRC31 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_setamper_tampersrc31_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerSetamperTampersrc31, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3140usize) as _) }
    }
    #[doc = "IN0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in0_tgl(self) -> crate::common::Reg<regs::ConsumerSysrtc0In0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3144usize) as _) }
    }
    #[doc = "IN1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_sysrtc0_in1_tgl(self) -> crate::common::Reg<regs::ConsumerSysrtc0In1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3148usize) as _) }
    }
    #[doc = "OSCREQ consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_hfxo0_oscreq_tgl(self) -> crate::common::Reg<regs::ConsumerHfxo0Oscreq, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x314cusize) as _) }
    }
    #[doc = "TIMEOUT Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_hfxo0_timeout_tgl(self) -> crate::common::Reg<regs::ConsumerHfxo0Timeout, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3150usize) as _) }
    }
    #[doc = "CTI consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin0_tgl(self) -> crate::common::Reg<regs::ConsumerCoreCtiin0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3154usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin1_tgl(self) -> crate::common::Reg<regs::ConsumerCoreCtiin1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3158usize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin2_tgl(self) -> crate::common::Reg<regs::ConsumerCoreCtiin2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x315cusize) as _) }
    }
    #[doc = "CTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_core_ctiin3_tgl(self) -> crate::common::Reg<regs::ConsumerCoreCtiin3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3160usize) as _) }
    }
    #[doc = "M33 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_core_m33rxev_tgl(self) -> crate::common::Reg<regs::ConsumerCoreM33rxev, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3164usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer0Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3168usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer0Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x316cusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer0Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3170usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer0Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3174usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3178usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer0Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x317cusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer1Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3180usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer1Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3184usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer1Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3188usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer1Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x318cusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3190usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer1Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3194usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer2Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3198usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer2Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x319cusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer2Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer2Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a4usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31a8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer2Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31acusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer3Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b0usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer3Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b4usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer3Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31b8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer3Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31bcusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer3Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c4usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer4Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31c8usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer4Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31ccusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer4Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer4Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d4usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31d8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer4Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31dcusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer5Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e0usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer5Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e4usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer5Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31e8usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer5Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31ecusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f0usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer5_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer5Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f4usize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer6Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31f8usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer6Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31fcusize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer6Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3200usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer6Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3204usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3208usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer6_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer6Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x320cusize) as _) }
    }
    #[doc = "CC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc0_tgl(self) -> crate::common::Reg<regs::ConsumerTimer7Cc0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3210usize) as _) }
    }
    #[doc = "CC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer7Cc1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3214usize) as _) }
    }
    #[doc = "CC2 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_cc2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer7Cc2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3218usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dti_tgl(self) -> crate::common::Reg<regs::ConsumerTimer7Dti, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x321cusize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs1_tgl(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3220usize) as _) }
    }
    #[doc = "DTI Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_timer7_dtifs2_tgl(self) -> crate::common::Reg<regs::ConsumerTimer7Dtifs2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3224usize) as _) }
    }
    #[doc = "ASYNCTRIG consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch0_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3228usize) as _) }
    }
    #[doc = "ASYNCTRIG Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_asynctrigch1_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Asynctrigch1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x322cusize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch0_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3230usize) as _) }
    }
    #[doc = "SYNCTRIG Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_vdac0_synctrigch1_tgl(
        self,
    ) -> crate::common::Reg<regs::ConsumerVdac0Synctrigch1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3234usize) as _) }
    }
    #[doc = "SRC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_wdog0_src0_tgl(self) -> crate::common::Reg<regs::ConsumerWdog0Src0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3238usize) as _) }
    }
    #[doc = "SRC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_wdog0_src1_tgl(self) -> crate::common::Reg<regs::ConsumerWdog0Src1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x323cusize) as _) }
    }
    #[doc = "SRC0 consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_wdog1_src0_tgl(self) -> crate::common::Reg<regs::ConsumerWdog1Src0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3240usize) as _) }
    }
    #[doc = "SRC1 Consumer register. (write-1-to-tgl alias)"]
    #[inline(always)]
    pub const fn consumer_wdog1_src1_tgl(self) -> crate::common::Reg<regs::ConsumerWdog1Src1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3244usize) as _) }
    }
}
pub mod regs {
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh0Ctrl(pub u32);
    impl AsyncCh0Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh0CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh0CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh0CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh0CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh0CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh0CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh0Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh0Ctrl {
            AsyncCh0Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh0Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh0Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh10Ctrl(pub u32);
    impl AsyncCh10Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh10CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh10CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh10CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh10CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh10CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh10CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh10Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh10Ctrl {
            AsyncCh10Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh10Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh10Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh10Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh10Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh11Ctrl(pub u32);
    impl AsyncCh11Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh11CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh11CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh11CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh11CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh11CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh11CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh11Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh11Ctrl {
            AsyncCh11Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh11Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh11Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh11Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh11Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh1Ctrl(pub u32);
    impl AsyncCh1Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh1CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh1CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh1CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh1CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh1CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh1CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh1Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh1Ctrl {
            AsyncCh1Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh1Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh1Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh1Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh2Ctrl(pub u32);
    impl AsyncCh2Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh2CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh2CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh2CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh2CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh2CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh2CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh2Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh2Ctrl {
            AsyncCh2Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh2Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh2Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh2Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh2Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh3Ctrl(pub u32);
    impl AsyncCh3Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh3CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh3CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh3CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh3CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh3CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh3CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh3Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh3Ctrl {
            AsyncCh3Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh3Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh3Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh3Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh3Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh4Ctrl(pub u32);
    impl AsyncCh4Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh4CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh4CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh4CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh4CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh4CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh4CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh4Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh4Ctrl {
            AsyncCh4Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh4Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh4Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh4Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh4Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh5Ctrl(pub u32);
    impl AsyncCh5Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh5CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh5CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh5CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh5CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh5CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh5CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh5Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh5Ctrl {
            AsyncCh5Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh5Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh5Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh5Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh5Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh6Ctrl(pub u32);
    impl AsyncCh6Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh6CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh6CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh6CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh6CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh6CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh6CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh6Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh6Ctrl {
            AsyncCh6Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh6Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh6Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh6Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh6Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh7Ctrl(pub u32);
    impl AsyncCh7Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh7CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh7CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh7CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh7CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh7CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh7CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh7Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh7Ctrl {
            AsyncCh7Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh7Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh7Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh7Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh7Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh8Ctrl(pub u32);
    impl AsyncCh8Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh8CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh8CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh8CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh8CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh8CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh8CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh8Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh8Ctrl {
            AsyncCh8Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh8Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh8Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh8Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh8Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncCh9Ctrl(pub u32);
    impl AsyncCh9Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::AsyncCh9CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::AsyncCh9CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::AsyncCh9CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Function Select."]
        #[must_use]
        #[inline(always)]
        pub const fn fnsel(&self) -> super::vals::AsyncCh9CtrlFnsel {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::AsyncCh9CtrlFnsel::from_bits(val as u8)
        }
        #[doc = "Function Select."]
        #[inline(always)]
        pub const fn set_fnsel(&mut self, val: super::vals::AsyncCh9CtrlFnsel) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "Aux Select."]
        #[must_use]
        #[inline(always)]
        pub const fn auxsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Aux Select."]
        #[inline(always)]
        pub const fn set_auxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for AsyncCh9Ctrl {
        #[inline(always)]
        fn default() -> AsyncCh9Ctrl {
            AsyncCh9Ctrl(0)
        }
    }
    impl core::fmt::Debug for AsyncCh9Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncCh9Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .field("fnsel", &self.fnsel())
                .field("auxsel", &self.auxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncCh9Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncCh9Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?}, fnsel: {:?}, auxsel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel(),
                self.fnsel(),
                self.auxsel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncPeek(pub u32);
    impl AsyncPeek {
        #[doc = "Channel 0 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0val(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 0 Current Value."]
        #[inline(always)]
        pub const fn set_ch0val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel 1 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1val(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 1 Current Value."]
        #[inline(always)]
        pub const fn set_ch1val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel 2 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2val(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 2 Current Value."]
        #[inline(always)]
        pub const fn set_ch2val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel 3 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3val(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 3 Current Value."]
        #[inline(always)]
        pub const fn set_ch3val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel 4 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4val(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 4 Current Value."]
        #[inline(always)]
        pub const fn set_ch4val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel 5 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5val(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 5 Current Value."]
        #[inline(always)]
        pub const fn set_ch5val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel 6 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6val(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 6 Current Value."]
        #[inline(always)]
        pub const fn set_ch6val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel 7 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7val(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 7 Current Value."]
        #[inline(always)]
        pub const fn set_ch7val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel 8 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8val(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 8 Current Value."]
        #[inline(always)]
        pub const fn set_ch8val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel 9 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9val(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 9 Current Value."]
        #[inline(always)]
        pub const fn set_ch9val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel 10 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10val(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 10 Current Value."]
        #[inline(always)]
        pub const fn set_ch10val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel 11 Current Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11val(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel 11 Current Value."]
        #[inline(always)]
        pub const fn set_ch11val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for AsyncPeek {
        #[inline(always)]
        fn default() -> AsyncPeek {
            AsyncPeek(0)
        }
    }
    impl core::fmt::Debug for AsyncPeek {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncPeek")
                .field("ch0val", &self.ch0val())
                .field("ch1val", &self.ch1val())
                .field("ch2val", &self.ch2val())
                .field("ch3val", &self.ch3val())
                .field("ch4val", &self.ch4val())
                .field("ch5val", &self.ch5val())
                .field("ch6val", &self.ch6val())
                .field("ch7val", &self.ch7val())
                .field("ch8val", &self.ch8val())
                .field("ch9val", &self.ch9val())
                .field("ch10val", &self.ch10val())
                .field("ch11val", &self.ch11val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncPeek {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncPeek {{ ch0val: {=bool:?}, ch1val: {=bool:?}, ch2val: {=bool:?}, ch3val: {=bool:?}, ch4val: {=bool:?}, ch5val: {=bool:?}, ch6val: {=bool:?}, ch7val: {=bool:?}, ch8val: {=bool:?}, ch9val: {=bool:?}, ch10val: {=bool:?}, ch11val: {=bool:?} }}",
                self.ch0val(),
                self.ch1val(),
                self.ch2val(),
                self.ch3val(),
                self.ch4val(),
                self.ch5val(),
                self.ch6val(),
                self.ch7val(),
                self.ch8val(),
                self.ch9val(),
                self.ch10val(),
                self.ch11val()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncSwlevel(pub u32);
    impl AsyncSwlevel {
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0level(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch0level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1level(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch1level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2level(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch2level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3level(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch3level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4level(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch4level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5level(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch5level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6level(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch6level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7level(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch7level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8level(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch8level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9level(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch9level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10level(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch10level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel Level."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11level(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Level."]
        #[inline(always)]
        pub const fn set_ch11level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for AsyncSwlevel {
        #[inline(always)]
        fn default() -> AsyncSwlevel {
            AsyncSwlevel(0)
        }
    }
    impl core::fmt::Debug for AsyncSwlevel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncSwlevel")
                .field("ch0level", &self.ch0level())
                .field("ch1level", &self.ch1level())
                .field("ch2level", &self.ch2level())
                .field("ch3level", &self.ch3level())
                .field("ch4level", &self.ch4level())
                .field("ch5level", &self.ch5level())
                .field("ch6level", &self.ch6level())
                .field("ch7level", &self.ch7level())
                .field("ch8level", &self.ch8level())
                .field("ch9level", &self.ch9level())
                .field("ch10level", &self.ch10level())
                .field("ch11level", &self.ch11level())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncSwlevel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncSwlevel {{ ch0level: {=bool:?}, ch1level: {=bool:?}, ch2level: {=bool:?}, ch3level: {=bool:?}, ch4level: {=bool:?}, ch5level: {=bool:?}, ch6level: {=bool:?}, ch7level: {=bool:?}, ch8level: {=bool:?}, ch9level: {=bool:?}, ch10level: {=bool:?}, ch11level: {=bool:?} }}",
                self.ch0level(),
                self.ch1level(),
                self.ch2level(),
                self.ch3level(),
                self.ch4level(),
                self.ch5level(),
                self.ch6level(),
                self.ch7level(),
                self.ch8level(),
                self.ch9level(),
                self.ch10level(),
                self.ch11level()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsyncSwpulse(pub u32);
    impl AsyncSwpulse {
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0pulse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch0pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1pulse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch1pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2pulse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch2pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3pulse(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch3pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4pulse(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch4pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5pulse(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch5pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6pulse(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch6pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7pulse(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch7pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8pulse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch8pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9pulse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch9pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch10pulse(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch10pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn ch11pulse(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel pulse."]
        #[inline(always)]
        pub const fn set_ch11pulse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for AsyncSwpulse {
        #[inline(always)]
        fn default() -> AsyncSwpulse {
            AsyncSwpulse(0)
        }
    }
    impl core::fmt::Debug for AsyncSwpulse {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AsyncSwpulse")
                .field("ch0pulse", &self.ch0pulse())
                .field("ch1pulse", &self.ch1pulse())
                .field("ch2pulse", &self.ch2pulse())
                .field("ch3pulse", &self.ch3pulse())
                .field("ch4pulse", &self.ch4pulse())
                .field("ch5pulse", &self.ch5pulse())
                .field("ch6pulse", &self.ch6pulse())
                .field("ch7pulse", &self.ch7pulse())
                .field("ch8pulse", &self.ch8pulse())
                .field("ch9pulse", &self.ch9pulse())
                .field("ch10pulse", &self.ch10pulse())
                .field("ch11pulse", &self.ch11pulse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AsyncSwpulse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AsyncSwpulse {{ ch0pulse: {=bool:?}, ch1pulse: {=bool:?}, ch2pulse: {=bool:?}, ch3pulse: {=bool:?}, ch4pulse: {=bool:?}, ch5pulse: {=bool:?}, ch6pulse: {=bool:?}, ch7pulse: {=bool:?}, ch8pulse: {=bool:?}, ch9pulse: {=bool:?}, ch10pulse: {=bool:?}, ch11pulse: {=bool:?} }}",
                self.ch0pulse(),
                self.ch1pulse(),
                self.ch2pulse(),
                self.ch3pulse(),
                self.ch4pulse(),
                self.ch5pulse(),
                self.ch6pulse(),
                self.ch7pulse(),
                self.ch8pulse(),
                self.ch9pulse(),
                self.ch10pulse(),
                self.ch11pulse()
            )
        }
    }
    #[doc = "CALDN consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCmuCaldn(pub u32);
    impl ConsumerCmuCaldn {
        #[doc = "CALDN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CALDN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCmuCaldn {
        #[inline(always)]
        fn default() -> ConsumerCmuCaldn {
            ConsumerCmuCaldn(0)
        }
    }
    impl core::fmt::Debug for ConsumerCmuCaldn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCmuCaldn")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCmuCaldn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCmuCaldn {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CALUP Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCmuCalup(pub u32);
    impl ConsumerCmuCalup {
        #[doc = "CALUP async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CALUP async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCmuCalup {
        #[inline(always)]
        fn default() -> ConsumerCmuCalup {
            ConsumerCmuCalup(0)
        }
    }
    impl core::fmt::Debug for ConsumerCmuCalup {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCmuCalup")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCmuCalup {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCmuCalup {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCoreCtiin0(pub u32);
    impl ConsumerCoreCtiin0 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCoreCtiin0 {
        #[inline(always)]
        fn default() -> ConsumerCoreCtiin0 {
            ConsumerCoreCtiin0(0)
        }
    }
    impl core::fmt::Debug for ConsumerCoreCtiin0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCoreCtiin0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCoreCtiin0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCoreCtiin0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCoreCtiin1(pub u32);
    impl ConsumerCoreCtiin1 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCoreCtiin1 {
        #[inline(always)]
        fn default() -> ConsumerCoreCtiin1 {
            ConsumerCoreCtiin1(0)
        }
    }
    impl core::fmt::Debug for ConsumerCoreCtiin1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCoreCtiin1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCoreCtiin1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCoreCtiin1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCoreCtiin2(pub u32);
    impl ConsumerCoreCtiin2 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCoreCtiin2 {
        #[inline(always)]
        fn default() -> ConsumerCoreCtiin2 {
            ConsumerCoreCtiin2(0)
        }
    }
    impl core::fmt::Debug for ConsumerCoreCtiin2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCoreCtiin2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCoreCtiin2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCoreCtiin2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCoreCtiin3(pub u32);
    impl ConsumerCoreCtiin3 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCoreCtiin3 {
        #[inline(always)]
        fn default() -> ConsumerCoreCtiin3 {
            ConsumerCoreCtiin3(0)
        }
    }
    impl core::fmt::Debug for ConsumerCoreCtiin3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCoreCtiin3")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCoreCtiin3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCoreCtiin3 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "M33 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerCoreM33rxev(pub u32);
    impl ConsumerCoreM33rxev {
        #[doc = "M33 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "M33 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerCoreM33rxev {
        #[inline(always)]
        fn default() -> ConsumerCoreM33rxev {
            ConsumerCoreM33rxev(0)
        }
    }
    impl core::fmt::Debug for ConsumerCoreM33rxev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerCoreM33rxev")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerCoreM33rxev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerCoreM33rxev {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLK consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart0Clk(pub u32);
    impl ConsumerEusart0Clk {
        #[doc = "CLK async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLK async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart0Clk {
        #[inline(always)]
        fn default() -> ConsumerEusart0Clk {
            ConsumerEusart0Clk(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart0Clk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart0Clk")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart0Clk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart0Clk {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RX Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart0Rx(pub u32);
    impl ConsumerEusart0Rx {
        #[doc = "RX async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RX async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart0Rx {
        #[inline(always)]
        fn default() -> ConsumerEusart0Rx {
            ConsumerEusart0Rx(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart0Rx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart0Rx")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart0Rx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart0Rx {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TRIGGER Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart0Trigger(pub u32);
    impl ConsumerEusart0Trigger {
        #[doc = "TRIGGER async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TRIGGER async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart0Trigger {
        #[inline(always)]
        fn default() -> ConsumerEusart0Trigger {
            ConsumerEusart0Trigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart0Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart0Trigger")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart0Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart0Trigger {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLK consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart1Clk(pub u32);
    impl ConsumerEusart1Clk {
        #[doc = "CLK async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLK async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart1Clk {
        #[inline(always)]
        fn default() -> ConsumerEusart1Clk {
            ConsumerEusart1Clk(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart1Clk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart1Clk")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart1Clk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart1Clk {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RX Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart1Rx(pub u32);
    impl ConsumerEusart1Rx {
        #[doc = "RX async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RX async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart1Rx {
        #[inline(always)]
        fn default() -> ConsumerEusart1Rx {
            ConsumerEusart1Rx(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart1Rx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart1Rx")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart1Rx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart1Rx {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TRIGGER Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart1Trigger(pub u32);
    impl ConsumerEusart1Trigger {
        #[doc = "TRIGGER async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TRIGGER async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart1Trigger {
        #[inline(always)]
        fn default() -> ConsumerEusart1Trigger {
            ConsumerEusart1Trigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart1Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart1Trigger")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart1Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart1Trigger {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLK consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart2Clk(pub u32);
    impl ConsumerEusart2Clk {
        #[doc = "CLK async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLK async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart2Clk {
        #[inline(always)]
        fn default() -> ConsumerEusart2Clk {
            ConsumerEusart2Clk(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart2Clk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart2Clk")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart2Clk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart2Clk {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RX Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart2Rx(pub u32);
    impl ConsumerEusart2Rx {
        #[doc = "RX async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RX async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart2Rx {
        #[inline(always)]
        fn default() -> ConsumerEusart2Rx {
            ConsumerEusart2Rx(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart2Rx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart2Rx")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart2Rx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart2Rx {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TRIGGER Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart2Trigger(pub u32);
    impl ConsumerEusart2Trigger {
        #[doc = "TRIGGER async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TRIGGER async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart2Trigger {
        #[inline(always)]
        fn default() -> ConsumerEusart2Trigger {
            ConsumerEusart2Trigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart2Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart2Trigger")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart2Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart2Trigger {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLK consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart3Clk(pub u32);
    impl ConsumerEusart3Clk {
        #[doc = "CLK async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLK async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart3Clk {
        #[inline(always)]
        fn default() -> ConsumerEusart3Clk {
            ConsumerEusart3Clk(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart3Clk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart3Clk")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart3Clk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart3Clk {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RX Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart3Rx(pub u32);
    impl ConsumerEusart3Rx {
        #[doc = "RX async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RX async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart3Rx {
        #[inline(always)]
        fn default() -> ConsumerEusart3Rx {
            ConsumerEusart3Rx(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart3Rx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart3Rx")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart3Rx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart3Rx {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TRIGGER Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart3Trigger(pub u32);
    impl ConsumerEusart3Trigger {
        #[doc = "TRIGGER async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TRIGGER async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart3Trigger {
        #[inline(always)]
        fn default() -> ConsumerEusart3Trigger {
            ConsumerEusart3Trigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart3Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart3Trigger")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart3Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart3Trigger {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLK consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart4Clk(pub u32);
    impl ConsumerEusart4Clk {
        #[doc = "CLK async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLK async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart4Clk {
        #[inline(always)]
        fn default() -> ConsumerEusart4Clk {
            ConsumerEusart4Clk(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart4Clk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart4Clk")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart4Clk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart4Clk {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RX Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart4Rx(pub u32);
    impl ConsumerEusart4Rx {
        #[doc = "RX async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RX async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart4Rx {
        #[inline(always)]
        fn default() -> ConsumerEusart4Rx {
            ConsumerEusart4Rx(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart4Rx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart4Rx")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart4Rx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart4Rx {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TRIGGER Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerEusart4Trigger(pub u32);
    impl ConsumerEusart4Trigger {
        #[doc = "TRIGGER async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TRIGGER async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerEusart4Trigger {
        #[inline(always)]
        fn default() -> ConsumerEusart4Trigger {
            ConsumerEusart4Trigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerEusart4Trigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerEusart4Trigger")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerEusart4Trigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerEusart4Trigger {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RXRAW consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerFrcRxraw(pub u32);
    impl ConsumerFrcRxraw {
        #[doc = "RXRAW async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RXRAW async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerFrcRxraw {
        #[inline(always)]
        fn default() -> ConsumerFrcRxraw {
            ConsumerFrcRxraw(0)
        }
    }
    impl core::fmt::Debug for ConsumerFrcRxraw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerFrcRxraw")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerFrcRxraw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerFrcRxraw {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "OSCREQ consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerHfxo0Oscreq(pub u32);
    impl ConsumerHfxo0Oscreq {
        #[doc = "OSC async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "OSC async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerHfxo0Oscreq {
        #[inline(always)]
        fn default() -> ConsumerHfxo0Oscreq {
            ConsumerHfxo0Oscreq(0)
        }
    }
    impl core::fmt::Debug for ConsumerHfxo0Oscreq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerHfxo0Oscreq")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerHfxo0Oscreq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerHfxo0Oscreq {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TIMEOUT Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerHfxo0Timeout(pub u32);
    impl ConsumerHfxo0Timeout {
        #[doc = "TIMEOUT async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TIMEOUT async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerHfxo0Timeout {
        #[inline(always)]
        fn default() -> ConsumerHfxo0Timeout {
            ConsumerHfxo0Timeout(0)
        }
    }
    impl core::fmt::Debug for ConsumerHfxo0Timeout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerHfxo0Timeout")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerHfxo0Timeout {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerHfxo0Timeout {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "SCAN consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerIadc0Scantrigger(pub u32);
    impl ConsumerIadc0Scantrigger {
        #[doc = "SCAN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SCAN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SCAN sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "SCAN sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerIadc0Scantrigger {
        #[inline(always)]
        fn default() -> ConsumerIadc0Scantrigger {
            ConsumerIadc0Scantrigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerIadc0Scantrigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerIadc0Scantrigger")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerIadc0Scantrigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerIadc0Scantrigger {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "SINGLE Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerIadc0Singletrigger(pub u32);
    impl ConsumerIadc0Singletrigger {
        #[doc = "SINGLE async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SINGLE async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "SINGLE sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "SINGLE sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerIadc0Singletrigger {
        #[inline(always)]
        fn default() -> ConsumerIadc0Singletrigger {
            ConsumerIadc0Singletrigger(0)
        }
    }
    impl core::fmt::Debug for ConsumerIadc0Singletrigger {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerIadc0Singletrigger")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerIadc0Singletrigger {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerIadc0Singletrigger {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DMAREQ0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLdmaxbarDmareq0(pub u32);
    impl ConsumerLdmaxbarDmareq0 {
        #[doc = "DMAREQ0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DMAREQ0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLdmaxbarDmareq0 {
        #[inline(always)]
        fn default() -> ConsumerLdmaxbarDmareq0 {
            ConsumerLdmaxbarDmareq0(0)
        }
    }
    impl core::fmt::Debug for ConsumerLdmaxbarDmareq0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLdmaxbarDmareq0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLdmaxbarDmareq0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLdmaxbarDmareq0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DMAREQ1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLdmaxbarDmareq1(pub u32);
    impl ConsumerLdmaxbarDmareq1 {
        #[doc = "DMAREQ1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DMAREQ1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLdmaxbarDmareq1 {
        #[inline(always)]
        fn default() -> ConsumerLdmaxbarDmareq1 {
            ConsumerLdmaxbarDmareq1(0)
        }
    }
    impl core::fmt::Debug for ConsumerLdmaxbarDmareq1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLdmaxbarDmareq1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLdmaxbarDmareq1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLdmaxbarDmareq1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DECIN0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLesenseDecin0(pub u32);
    impl ConsumerLesenseDecin0 {
        #[doc = "DECIN0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DECIN0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLesenseDecin0 {
        #[inline(always)]
        fn default() -> ConsumerLesenseDecin0 {
            ConsumerLesenseDecin0(0)
        }
    }
    impl core::fmt::Debug for ConsumerLesenseDecin0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLesenseDecin0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLesenseDecin0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLesenseDecin0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DECIN1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLesenseDecin1(pub u32);
    impl ConsumerLesenseDecin1 {
        #[doc = "DECIN1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DECIN1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLesenseDecin1 {
        #[inline(always)]
        fn default() -> ConsumerLesenseDecin1 {
            ConsumerLesenseDecin1(0)
        }
    }
    impl core::fmt::Debug for ConsumerLesenseDecin1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLesenseDecin1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLesenseDecin1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLesenseDecin1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DECIN2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLesenseDecin2(pub u32);
    impl ConsumerLesenseDecin2 {
        #[doc = "DECIN2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DECIN2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLesenseDecin2 {
        #[inline(always)]
        fn default() -> ConsumerLesenseDecin2 {
            ConsumerLesenseDecin2(0)
        }
    }
    impl core::fmt::Debug for ConsumerLesenseDecin2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLesenseDecin2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLesenseDecin2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLesenseDecin2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DECIN3 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLesenseDecin3(pub u32);
    impl ConsumerLesenseDecin3 {
        #[doc = "DECIN3 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DECIN3 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLesenseDecin3 {
        #[inline(always)]
        fn default() -> ConsumerLesenseDecin3 {
            ConsumerLesenseDecin3(0)
        }
    }
    impl core::fmt::Debug for ConsumerLesenseDecin3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLesenseDecin3")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLesenseDecin3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLesenseDecin3 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "START Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLesenseStart(pub u32);
    impl ConsumerLesenseStart {
        #[doc = "START async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "START async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLesenseStart {
        #[inline(always)]
        fn default() -> ConsumerLesenseStart {
            ConsumerLesenseStart(0)
        }
    }
    impl core::fmt::Debug for ConsumerLesenseStart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLesenseStart")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLesenseStart {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLesenseStart {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLEAR consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLetimer0Clear(pub u32);
    impl ConsumerLetimer0Clear {
        #[doc = "CLEAR async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLEAR async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLetimer0Clear {
        #[inline(always)]
        fn default() -> ConsumerLetimer0Clear {
            ConsumerLetimer0Clear(0)
        }
    }
    impl core::fmt::Debug for ConsumerLetimer0Clear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLetimer0Clear")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLetimer0Clear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLetimer0Clear {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "START Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLetimer0Start(pub u32);
    impl ConsumerLetimer0Start {
        #[doc = "START async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "START async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLetimer0Start {
        #[inline(always)]
        fn default() -> ConsumerLetimer0Start {
            ConsumerLetimer0Start(0)
        }
    }
    impl core::fmt::Debug for ConsumerLetimer0Start {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLetimer0Start")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLetimer0Start {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLetimer0Start {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "STOP Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerLetimer0Stop(pub u32);
    impl ConsumerLetimer0Stop {
        #[doc = "STOP async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "STOP async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerLetimer0Stop {
        #[inline(always)]
        fn default() -> ConsumerLetimer0Stop {
            ConsumerLetimer0Stop(0)
        }
    }
    impl core::fmt::Debug for ConsumerLetimer0Stop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerLetimer0Stop")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerLetimer0Stop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerLetimer0Stop {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DIN consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerModemDin(pub u32);
    impl ConsumerModemDin {
        #[doc = "DIN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DIN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerModemDin {
        #[inline(always)]
        fn default() -> ConsumerModemDin {
            ConsumerModemDin(0)
        }
    }
    impl core::fmt::Debug for ConsumerModemDin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerModemDin")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerModemDin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerModemDin {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "S0IN consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerPcnt0S0in(pub u32);
    impl ConsumerPcnt0S0in {
        #[doc = "S0IN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "S0IN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerPcnt0S0in {
        #[inline(always)]
        fn default() -> ConsumerPcnt0S0in {
            ConsumerPcnt0S0in(0)
        }
    }
    impl core::fmt::Debug for ConsumerPcnt0S0in {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerPcnt0S0in")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerPcnt0S0in {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerPcnt0S0in {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "S1IN Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerPcnt0S1in(pub u32);
    impl ConsumerPcnt0S1in {
        #[doc = "S1IN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "S1IN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerPcnt0S1in {
        #[inline(always)]
        fn default() -> ConsumerPcnt0S1in {
            ConsumerPcnt0S1in(0)
        }
    }
    impl core::fmt::Debug for ConsumerPcnt0S1in {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerPcnt0S1in")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerPcnt0S1in {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerPcnt0S1in {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CLR consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacClr(pub u32);
    impl ConsumerRacClr {
        #[doc = "CLR async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CLR async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacClr {
        #[inline(always)]
        fn default() -> ConsumerRacClr {
            ConsumerRacClr(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacClr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacClr")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacClr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacClr {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacCtiin0(pub u32);
    impl ConsumerRacCtiin0 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacCtiin0 {
        #[inline(always)]
        fn default() -> ConsumerRacCtiin0 {
            ConsumerRacCtiin0(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacCtiin0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacCtiin0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacCtiin0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacCtiin0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacCtiin1(pub u32);
    impl ConsumerRacCtiin1 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacCtiin1 {
        #[inline(always)]
        fn default() -> ConsumerRacCtiin1 {
            ConsumerRacCtiin1(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacCtiin1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacCtiin1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacCtiin1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacCtiin1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacCtiin2(pub u32);
    impl ConsumerRacCtiin2 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacCtiin2 {
        #[inline(always)]
        fn default() -> ConsumerRacCtiin2 {
            ConsumerRacCtiin2(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacCtiin2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacCtiin2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacCtiin2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacCtiin2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacCtiin3(pub u32);
    impl ConsumerRacCtiin3 {
        #[doc = "CTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacCtiin3 {
        #[inline(always)]
        fn default() -> ConsumerRacCtiin3 {
            ConsumerRacCtiin3(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacCtiin3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacCtiin3")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacCtiin3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacCtiin3 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "FORCETX Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacForcetx(pub u32);
    impl ConsumerRacForcetx {
        #[doc = "FORCETX async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "FORCETX async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacForcetx {
        #[inline(always)]
        fn default() -> ConsumerRacForcetx {
            ConsumerRacForcetx(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacForcetx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacForcetx")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacForcetx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacForcetx {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RXDIS Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacRxdis(pub u32);
    impl ConsumerRacRxdis {
        #[doc = "RXDIS async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RXDIS async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacRxdis {
        #[inline(always)]
        fn default() -> ConsumerRacRxdis {
            ConsumerRacRxdis(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacRxdis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacRxdis")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacRxdis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacRxdis {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "RXEN Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacRxen(pub u32);
    impl ConsumerRacRxen {
        #[doc = "RXEN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RXEN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacRxen {
        #[inline(always)]
        fn default() -> ConsumerRacRxen {
            ConsumerRacRxen(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacRxen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacRxen")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacRxen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacRxen {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TXEN Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerRacTxen(pub u32);
    impl ConsumerRacTxen {
        #[doc = "TXEN async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TXEN async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerRacTxen {
        #[inline(always)]
        fn default() -> ConsumerRacTxen {
            ConsumerRacTxen(0)
        }
    }
    impl core::fmt::Debug for ConsumerRacTxen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerRacTxen")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerRacTxen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerRacTxen {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TAMPERSRC26 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSetamperTampersrc26(pub u32);
    impl ConsumerSetamperTampersrc26 {
        #[doc = "TAMPERSRC26 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPERSRC26 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSetamperTampersrc26 {
        #[inline(always)]
        fn default() -> ConsumerSetamperTampersrc26 {
            ConsumerSetamperTampersrc26(0)
        }
    }
    impl core::fmt::Debug for ConsumerSetamperTampersrc26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSetamperTampersrc26")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSetamperTampersrc26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSetamperTampersrc26 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TAMPERSRC27 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSetamperTampersrc27(pub u32);
    impl ConsumerSetamperTampersrc27 {
        #[doc = "TAMPERSRC27 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPERSRC27 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSetamperTampersrc27 {
        #[inline(always)]
        fn default() -> ConsumerSetamperTampersrc27 {
            ConsumerSetamperTampersrc27(0)
        }
    }
    impl core::fmt::Debug for ConsumerSetamperTampersrc27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSetamperTampersrc27")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSetamperTampersrc27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSetamperTampersrc27 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TAMPERSRC28 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSetamperTampersrc28(pub u32);
    impl ConsumerSetamperTampersrc28 {
        #[doc = "TAMPERSRC28 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPERSRC28 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSetamperTampersrc28 {
        #[inline(always)]
        fn default() -> ConsumerSetamperTampersrc28 {
            ConsumerSetamperTampersrc28(0)
        }
    }
    impl core::fmt::Debug for ConsumerSetamperTampersrc28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSetamperTampersrc28")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSetamperTampersrc28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSetamperTampersrc28 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TAMPERSRC29 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSetamperTampersrc29(pub u32);
    impl ConsumerSetamperTampersrc29 {
        #[doc = "TAMPERSRC29 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPERSRC29 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSetamperTampersrc29 {
        #[inline(always)]
        fn default() -> ConsumerSetamperTampersrc29 {
            ConsumerSetamperTampersrc29(0)
        }
    }
    impl core::fmt::Debug for ConsumerSetamperTampersrc29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSetamperTampersrc29")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSetamperTampersrc29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSetamperTampersrc29 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TAMPERSRC30 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSetamperTampersrc30(pub u32);
    impl ConsumerSetamperTampersrc30 {
        #[doc = "TAMPERSRC30 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPERSRC30 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSetamperTampersrc30 {
        #[inline(always)]
        fn default() -> ConsumerSetamperTampersrc30 {
            ConsumerSetamperTampersrc30(0)
        }
    }
    impl core::fmt::Debug for ConsumerSetamperTampersrc30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSetamperTampersrc30")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSetamperTampersrc30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSetamperTampersrc30 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "TAMPERSRC31 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSetamperTampersrc31(pub u32);
    impl ConsumerSetamperTampersrc31 {
        #[doc = "TAMPERSRC31 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPERSRC31 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSetamperTampersrc31 {
        #[inline(always)]
        fn default() -> ConsumerSetamperTampersrc31 {
            ConsumerSetamperTampersrc31(0)
        }
    }
    impl core::fmt::Debug for ConsumerSetamperTampersrc31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSetamperTampersrc31")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSetamperTampersrc31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSetamperTampersrc31 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "IN0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSysrtc0In0(pub u32);
    impl ConsumerSysrtc0In0 {
        #[doc = "IN0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "IN0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSysrtc0In0 {
        #[inline(always)]
        fn default() -> ConsumerSysrtc0In0 {
            ConsumerSysrtc0In0(0)
        }
    }
    impl core::fmt::Debug for ConsumerSysrtc0In0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSysrtc0In0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSysrtc0In0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSysrtc0In0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "IN1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerSysrtc0In1(pub u32);
    impl ConsumerSysrtc0In1 {
        #[doc = "IN1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "IN1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerSysrtc0In1 {
        #[inline(always)]
        fn default() -> ConsumerSysrtc0In1 {
            ConsumerSysrtc0In1(0)
        }
    }
    impl core::fmt::Debug for ConsumerSysrtc0In1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerSysrtc0In1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerSysrtc0In1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerSysrtc0In1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer0Cc0(pub u32);
    impl ConsumerTimer0Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer0Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer0Cc0 {
            ConsumerTimer0Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer0Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer0Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer0Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer0Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer0Cc1(pub u32);
    impl ConsumerTimer0Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer0Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer0Cc1 {
            ConsumerTimer0Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer0Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer0Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer0Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer0Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer0Cc2(pub u32);
    impl ConsumerTimer0Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer0Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer0Cc2 {
            ConsumerTimer0Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer0Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer0Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer0Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer0Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer0Dti(pub u32);
    impl ConsumerTimer0Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer0Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer0Dti {
            ConsumerTimer0Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer0Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer0Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer0Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer0Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer0Dtifs1(pub u32);
    impl ConsumerTimer0Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer0Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer0Dtifs1 {
            ConsumerTimer0Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer0Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer0Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer0Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer0Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer0Dtifs2(pub u32);
    impl ConsumerTimer0Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer0Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer0Dtifs2 {
            ConsumerTimer0Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer0Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer0Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer0Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer0Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer1Cc0(pub u32);
    impl ConsumerTimer1Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer1Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer1Cc0 {
            ConsumerTimer1Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer1Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer1Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer1Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer1Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer1Cc1(pub u32);
    impl ConsumerTimer1Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer1Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer1Cc1 {
            ConsumerTimer1Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer1Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer1Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer1Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer1Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer1Cc2(pub u32);
    impl ConsumerTimer1Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer1Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer1Cc2 {
            ConsumerTimer1Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer1Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer1Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer1Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer1Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer1Dti(pub u32);
    impl ConsumerTimer1Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer1Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer1Dti {
            ConsumerTimer1Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer1Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer1Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer1Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer1Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer1Dtifs1(pub u32);
    impl ConsumerTimer1Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer1Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer1Dtifs1 {
            ConsumerTimer1Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer1Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer1Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer1Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer1Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer1Dtifs2(pub u32);
    impl ConsumerTimer1Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer1Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer1Dtifs2 {
            ConsumerTimer1Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer1Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer1Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer1Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer1Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer2Cc0(pub u32);
    impl ConsumerTimer2Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer2Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer2Cc0 {
            ConsumerTimer2Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer2Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer2Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer2Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer2Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer2Cc1(pub u32);
    impl ConsumerTimer2Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer2Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer2Cc1 {
            ConsumerTimer2Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer2Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer2Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer2Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer2Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer2Cc2(pub u32);
    impl ConsumerTimer2Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer2Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer2Cc2 {
            ConsumerTimer2Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer2Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer2Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer2Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer2Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer2Dti(pub u32);
    impl ConsumerTimer2Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer2Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer2Dti {
            ConsumerTimer2Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer2Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer2Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer2Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer2Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer2Dtifs1(pub u32);
    impl ConsumerTimer2Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer2Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer2Dtifs1 {
            ConsumerTimer2Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer2Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer2Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer2Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer2Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer2Dtifs2(pub u32);
    impl ConsumerTimer2Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer2Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer2Dtifs2 {
            ConsumerTimer2Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer2Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer2Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer2Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer2Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer3Cc0(pub u32);
    impl ConsumerTimer3Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer3Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer3Cc0 {
            ConsumerTimer3Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer3Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer3Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer3Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer3Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer3Cc1(pub u32);
    impl ConsumerTimer3Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer3Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer3Cc1 {
            ConsumerTimer3Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer3Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer3Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer3Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer3Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer3Cc2(pub u32);
    impl ConsumerTimer3Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer3Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer3Cc2 {
            ConsumerTimer3Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer3Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer3Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer3Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer3Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer3Dti(pub u32);
    impl ConsumerTimer3Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer3Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer3Dti {
            ConsumerTimer3Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer3Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer3Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer3Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer3Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer3Dtifs1(pub u32);
    impl ConsumerTimer3Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer3Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer3Dtifs1 {
            ConsumerTimer3Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer3Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer3Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer3Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer3Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer3Dtifs2(pub u32);
    impl ConsumerTimer3Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer3Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer3Dtifs2 {
            ConsumerTimer3Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer3Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer3Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer3Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer3Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer4Cc0(pub u32);
    impl ConsumerTimer4Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer4Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer4Cc0 {
            ConsumerTimer4Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer4Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer4Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer4Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer4Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer4Cc1(pub u32);
    impl ConsumerTimer4Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer4Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer4Cc1 {
            ConsumerTimer4Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer4Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer4Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer4Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer4Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer4Cc2(pub u32);
    impl ConsumerTimer4Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer4Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer4Cc2 {
            ConsumerTimer4Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer4Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer4Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer4Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer4Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer4Dti(pub u32);
    impl ConsumerTimer4Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer4Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer4Dti {
            ConsumerTimer4Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer4Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer4Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer4Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer4Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer4Dtifs1(pub u32);
    impl ConsumerTimer4Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer4Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer4Dtifs1 {
            ConsumerTimer4Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer4Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer4Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer4Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer4Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer4Dtifs2(pub u32);
    impl ConsumerTimer4Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer4Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer4Dtifs2 {
            ConsumerTimer4Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer4Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer4Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer4Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer4Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer5Cc0(pub u32);
    impl ConsumerTimer5Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer5Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer5Cc0 {
            ConsumerTimer5Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer5Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer5Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer5Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer5Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer5Cc1(pub u32);
    impl ConsumerTimer5Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer5Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer5Cc1 {
            ConsumerTimer5Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer5Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer5Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer5Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer5Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer5Cc2(pub u32);
    impl ConsumerTimer5Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer5Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer5Cc2 {
            ConsumerTimer5Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer5Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer5Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer5Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer5Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer5Dti(pub u32);
    impl ConsumerTimer5Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer5Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer5Dti {
            ConsumerTimer5Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer5Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer5Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer5Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer5Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer5Dtifs1(pub u32);
    impl ConsumerTimer5Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer5Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer5Dtifs1 {
            ConsumerTimer5Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer5Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer5Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer5Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer5Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer5Dtifs2(pub u32);
    impl ConsumerTimer5Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer5Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer5Dtifs2 {
            ConsumerTimer5Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer5Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer5Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer5Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer5Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer6Cc0(pub u32);
    impl ConsumerTimer6Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer6Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer6Cc0 {
            ConsumerTimer6Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer6Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer6Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer6Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer6Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer6Cc1(pub u32);
    impl ConsumerTimer6Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer6Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer6Cc1 {
            ConsumerTimer6Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer6Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer6Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer6Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer6Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer6Cc2(pub u32);
    impl ConsumerTimer6Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer6Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer6Cc2 {
            ConsumerTimer6Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer6Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer6Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer6Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer6Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer6Dti(pub u32);
    impl ConsumerTimer6Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer6Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer6Dti {
            ConsumerTimer6Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer6Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer6Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer6Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer6Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer6Dtifs1(pub u32);
    impl ConsumerTimer6Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer6Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer6Dtifs1 {
            ConsumerTimer6Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer6Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer6Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer6Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer6Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer6Dtifs2(pub u32);
    impl ConsumerTimer6Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer6Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer6Dtifs2 {
            ConsumerTimer6Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer6Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer6Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer6Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer6Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "CC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer7Cc0(pub u32);
    impl ConsumerTimer7Cc0 {
        #[doc = "CC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC0 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC0 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer7Cc0 {
        #[inline(always)]
        fn default() -> ConsumerTimer7Cc0 {
            ConsumerTimer7Cc0(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer7Cc0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer7Cc0")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer7Cc0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer7Cc0 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer7Cc1(pub u32);
    impl ConsumerTimer7Cc1 {
        #[doc = "CC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC1 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC1 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer7Cc1 {
        #[inline(always)]
        fn default() -> ConsumerTimer7Cc1 {
            ConsumerTimer7Cc1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer7Cc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer7Cc1")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer7Cc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer7Cc1 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "CC2 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer7Cc2(pub u32);
    impl ConsumerTimer7Cc2 {
        #[doc = "CC2 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "CC2 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "CC2 sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "CC2 sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerTimer7Cc2 {
        #[inline(always)]
        fn default() -> ConsumerTimer7Cc2 {
            ConsumerTimer7Cc2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer7Cc2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer7Cc2")
                .field("prssel", &self.prssel())
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer7Cc2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ConsumerTimer7Cc2 {{ prssel: {=u8:?}, sprssel: {=u8:?} }}",
                self.prssel(),
                self.sprssel()
            )
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer7Dti(pub u32);
    impl ConsumerTimer7Dti {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer7Dti {
        #[inline(always)]
        fn default() -> ConsumerTimer7Dti {
            ConsumerTimer7Dti(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer7Dti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer7Dti")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer7Dti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer7Dti {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer7Dtifs1(pub u32);
    impl ConsumerTimer7Dtifs1 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer7Dtifs1 {
        #[inline(always)]
        fn default() -> ConsumerTimer7Dtifs1 {
            ConsumerTimer7Dtifs1(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer7Dtifs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer7Dtifs1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer7Dtifs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer7Dtifs1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "DTI Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerTimer7Dtifs2(pub u32);
    impl ConsumerTimer7Dtifs2 {
        #[doc = "DTI async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DTI async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerTimer7Dtifs2 {
        #[inline(always)]
        fn default() -> ConsumerTimer7Dtifs2 {
            ConsumerTimer7Dtifs2(0)
        }
    }
    impl core::fmt::Debug for ConsumerTimer7Dtifs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerTimer7Dtifs2")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerTimer7Dtifs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerTimer7Dtifs2 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "ASYNCTRIG consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerVdac0Asynctrigch0(pub u32);
    impl ConsumerVdac0Asynctrigch0 {
        #[doc = "ASYNCTRIG async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCTRIG async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerVdac0Asynctrigch0 {
        #[inline(always)]
        fn default() -> ConsumerVdac0Asynctrigch0 {
            ConsumerVdac0Asynctrigch0(0)
        }
    }
    impl core::fmt::Debug for ConsumerVdac0Asynctrigch0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerVdac0Asynctrigch0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerVdac0Asynctrigch0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerVdac0Asynctrigch0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "ASYNCTRIG Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerVdac0Asynctrigch1(pub u32);
    impl ConsumerVdac0Asynctrigch1 {
        #[doc = "ASYNCTRIG async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "ASYNCTRIG async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerVdac0Asynctrigch1 {
        #[inline(always)]
        fn default() -> ConsumerVdac0Asynctrigch1 {
            ConsumerVdac0Asynctrigch1(0)
        }
    }
    impl core::fmt::Debug for ConsumerVdac0Asynctrigch1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerVdac0Asynctrigch1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerVdac0Asynctrigch1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerVdac0Asynctrigch1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "SYNCTRIG Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerVdac0Synctrigch0(pub u32);
    impl ConsumerVdac0Synctrigch0 {
        #[doc = "SYNCTRIG sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "SYNCTRIG sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerVdac0Synctrigch0 {
        #[inline(always)]
        fn default() -> ConsumerVdac0Synctrigch0 {
            ConsumerVdac0Synctrigch0(0)
        }
    }
    impl core::fmt::Debug for ConsumerVdac0Synctrigch0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerVdac0Synctrigch0")
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerVdac0Synctrigch0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerVdac0Synctrigch0 {{ sprssel: {=u8:?} }}", self.sprssel())
        }
    }
    #[doc = "SYNCTRIG Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerVdac0Synctrigch1(pub u32);
    impl ConsumerVdac0Synctrigch1 {
        #[doc = "SYNCTRIG sync channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn sprssel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "SYNCTRIG sync channel select."]
        #[inline(always)]
        pub const fn set_sprssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for ConsumerVdac0Synctrigch1 {
        #[inline(always)]
        fn default() -> ConsumerVdac0Synctrigch1 {
            ConsumerVdac0Synctrigch1(0)
        }
    }
    impl core::fmt::Debug for ConsumerVdac0Synctrigch1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerVdac0Synctrigch1")
                .field("sprssel", &self.sprssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerVdac0Synctrigch1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerVdac0Synctrigch1 {{ sprssel: {=u8:?} }}", self.sprssel())
        }
    }
    #[doc = "SRC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerWdog0Src0(pub u32);
    impl ConsumerWdog0Src0 {
        #[doc = "SRC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SRC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerWdog0Src0 {
        #[inline(always)]
        fn default() -> ConsumerWdog0Src0 {
            ConsumerWdog0Src0(0)
        }
    }
    impl core::fmt::Debug for ConsumerWdog0Src0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerWdog0Src0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerWdog0Src0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerWdog0Src0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "SRC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerWdog0Src1(pub u32);
    impl ConsumerWdog0Src1 {
        #[doc = "SRC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SRC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerWdog0Src1 {
        #[inline(always)]
        fn default() -> ConsumerWdog0Src1 {
            ConsumerWdog0Src1(0)
        }
    }
    impl core::fmt::Debug for ConsumerWdog0Src1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerWdog0Src1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerWdog0Src1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerWdog0Src1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "SRC0 consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerWdog1Src0(pub u32);
    impl ConsumerWdog1Src0 {
        #[doc = "SRC0 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SRC0 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerWdog1Src0 {
        #[inline(always)]
        fn default() -> ConsumerWdog1Src0 {
            ConsumerWdog1Src0(0)
        }
    }
    impl core::fmt::Debug for ConsumerWdog1Src0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerWdog1Src0")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerWdog1Src0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerWdog1Src0 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "SRC1 Consumer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ConsumerWdog1Src1(pub u32);
    impl ConsumerWdog1Src1 {
        #[doc = "SRC1 async channel select."]
        #[must_use]
        #[inline(always)]
        pub const fn prssel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "SRC1 async channel select."]
        #[inline(always)]
        pub const fn set_prssel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for ConsumerWdog1Src1 {
        #[inline(always)]
        fn default() -> ConsumerWdog1Src1 {
            ConsumerWdog1Src1(0)
        }
    }
    impl core::fmt::Debug for ConsumerWdog1Src1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ConsumerWdog1Src1")
                .field("prssel", &self.prssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ConsumerWdog1Src1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ConsumerWdog1Src1 {{ prssel: {=u8:?} }}", self.prssel())
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipversion(pub u32);
    impl Ipversion {
        #[doc = "New BitField."]
        #[must_use]
        #[inline(always)]
        pub const fn ipversion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "New BitField."]
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
    pub struct SyncCh0Ctrl(pub u32);
    impl SyncCh0Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::SyncCh0CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SyncCh0CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::SyncCh0CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for SyncCh0Ctrl {
        #[inline(always)]
        fn default() -> SyncCh0Ctrl {
            SyncCh0Ctrl(0)
        }
    }
    impl core::fmt::Debug for SyncCh0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncCh0Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncCh0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncCh0Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncCh1Ctrl(pub u32);
    impl SyncCh1Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::SyncCh1CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SyncCh1CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::SyncCh1CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for SyncCh1Ctrl {
        #[inline(always)]
        fn default() -> SyncCh1Ctrl {
            SyncCh1Ctrl(0)
        }
    }
    impl core::fmt::Debug for SyncCh1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncCh1Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncCh1Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncCh1Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncCh2Ctrl(pub u32);
    impl SyncCh2Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::SyncCh2CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SyncCh2CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::SyncCh2CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for SyncCh2Ctrl {
        #[inline(always)]
        fn default() -> SyncCh2Ctrl {
            SyncCh2Ctrl(0)
        }
    }
    impl core::fmt::Debug for SyncCh2Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncCh2Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncCh2Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncCh2Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncCh3Ctrl(pub u32);
    impl SyncCh3Ctrl {
        #[doc = "Signal Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sigsel(&self) -> super::vals::SyncCh3CtrlSigsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::SyncCh3CtrlSigsel::from_bits(val as u8)
        }
        #[doc = "Signal Select."]
        #[inline(always)]
        pub const fn set_sigsel(&mut self, val: super::vals::SyncCh3CtrlSigsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Source Select."]
        #[must_use]
        #[inline(always)]
        pub const fn sourcesel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Source Select."]
        #[inline(always)]
        pub const fn set_sourcesel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for SyncCh3Ctrl {
        #[inline(always)]
        fn default() -> SyncCh3Ctrl {
            SyncCh3Ctrl(0)
        }
    }
    impl core::fmt::Debug for SyncCh3Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncCh3Ctrl")
                .field("sigsel", &self.sigsel())
                .field("sourcesel", &self.sourcesel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncCh3Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncCh3Ctrl {{ sigsel: {:?}, sourcesel: {=u8:?} }}",
                self.sigsel(),
                self.sourcesel()
            )
        }
    }
    #[doc = "No Description."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncPeek(pub u32);
    impl SyncPeek {
        #[doc = "Channel Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0val(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Value."]
        #[inline(always)]
        pub const fn set_ch0val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1val(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Value."]
        #[inline(always)]
        pub const fn set_ch1val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2val(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Value."]
        #[inline(always)]
        pub const fn set_ch2val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel Value."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3val(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel Value."]
        #[inline(always)]
        pub const fn set_ch3val(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SyncPeek {
        #[inline(always)]
        fn default() -> SyncPeek {
            SyncPeek(0)
        }
    }
    impl core::fmt::Debug for SyncPeek {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncPeek")
                .field("ch0val", &self.ch0val())
                .field("ch1val", &self.ch1val())
                .field("ch2val", &self.ch2val())
                .field("ch3val", &self.ch3val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncPeek {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncPeek {{ ch0val: {=bool:?}, ch1val: {=bool:?}, ch2val: {=bool:?}, ch3val: {=bool:?} }}",
                self.ch0val(),
                self.ch1val(),
                self.ch2val(),
                self.ch3val()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh0CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh0CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh0CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh0CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh0CtrlFnsel {
            AsyncCh0CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh0CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh0CtrlFnsel) -> u8 {
            AsyncCh0CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh0CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh0CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh0CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh0CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh0CtrlSigsel {
            AsyncCh0CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh0CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh0CtrlSigsel) -> u8 {
            AsyncCh0CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh10CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh10CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh10CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh10CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh10CtrlFnsel {
            AsyncCh10CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh10CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh10CtrlFnsel) -> u8 {
            AsyncCh10CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh10CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh10CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh10CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh10CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh10CtrlSigsel {
            AsyncCh10CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh10CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh10CtrlSigsel) -> u8 {
            AsyncCh10CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh11CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh11CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh11CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh11CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh11CtrlFnsel {
            AsyncCh11CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh11CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh11CtrlFnsel) -> u8 {
            AsyncCh11CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh11CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh11CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh11CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh11CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh11CtrlSigsel {
            AsyncCh11CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh11CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh11CtrlSigsel) -> u8 {
            AsyncCh11CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh1CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh1CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh1CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh1CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh1CtrlFnsel {
            AsyncCh1CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh1CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh1CtrlFnsel) -> u8 {
            AsyncCh1CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh1CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh1CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh1CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh1CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh1CtrlSigsel {
            AsyncCh1CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh1CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh1CtrlSigsel) -> u8 {
            AsyncCh1CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh2CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh2CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh2CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh2CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh2CtrlFnsel {
            AsyncCh2CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh2CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh2CtrlFnsel) -> u8 {
            AsyncCh2CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh2CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh2CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh2CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh2CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh2CtrlSigsel {
            AsyncCh2CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh2CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh2CtrlSigsel) -> u8 {
            AsyncCh2CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh3CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh3CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh3CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh3CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh3CtrlFnsel {
            AsyncCh3CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh3CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh3CtrlFnsel) -> u8 {
            AsyncCh3CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh3CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh3CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh3CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh3CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh3CtrlSigsel {
            AsyncCh3CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh3CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh3CtrlSigsel) -> u8 {
            AsyncCh3CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh4CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh4CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh4CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh4CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh4CtrlFnsel {
            AsyncCh4CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh4CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh4CtrlFnsel) -> u8 {
            AsyncCh4CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh4CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh4CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh4CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh4CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh4CtrlSigsel {
            AsyncCh4CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh4CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh4CtrlSigsel) -> u8 {
            AsyncCh4CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh5CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh5CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh5CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh5CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh5CtrlFnsel {
            AsyncCh5CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh5CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh5CtrlFnsel) -> u8 {
            AsyncCh5CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh5CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh5CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh5CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh5CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh5CtrlSigsel {
            AsyncCh5CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh5CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh5CtrlSigsel) -> u8 {
            AsyncCh5CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh6CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh6CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh6CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh6CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh6CtrlFnsel {
            AsyncCh6CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh6CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh6CtrlFnsel) -> u8 {
            AsyncCh6CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh6CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh6CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh6CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh6CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh6CtrlSigsel {
            AsyncCh6CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh6CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh6CtrlSigsel) -> u8 {
            AsyncCh6CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh7CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh7CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh7CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh7CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh7CtrlFnsel {
            AsyncCh7CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh7CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh7CtrlFnsel) -> u8 {
            AsyncCh7CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh7CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh7CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh7CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh7CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh7CtrlSigsel {
            AsyncCh7CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh7CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh7CtrlSigsel) -> u8 {
            AsyncCh7CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh8CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh8CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh8CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh8CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh8CtrlFnsel {
            AsyncCh8CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh8CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh8CtrlFnsel) -> u8 {
            AsyncCh8CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh8CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh8CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh8CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh8CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh8CtrlSigsel {
            AsyncCh8CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh8CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh8CtrlSigsel) -> u8 {
            AsyncCh8CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh9CtrlFnsel {
        #[doc = "Logical 0."]
        LogicalZero = 0x0,
        #[doc = "A NOR B."]
        ANorB = 0x01,
        #[doc = "(!A) AND B."]
        NotAAndB = 0x02,
        #[doc = "!A."]
        NotA = 0x03,
        #[doc = "A AND (!B)."]
        AAndNotB = 0x04,
        #[doc = "!B."]
        NotB = 0x05,
        #[doc = "A XOR B."]
        AXorB = 0x06,
        #[doc = "A NAND B."]
        ANandB = 0x07,
        #[doc = "A AND B."]
        AAndB = 0x08,
        #[doc = "A XNOR B."]
        AXnorB = 0x09,
        #[doc = "B."]
        B = 0x0a,
        #[doc = "(!A) OR B."]
        NotAOrB = 0x0b,
        #[doc = "A."]
        A = 0x0c,
        #[doc = "A OR (!B)."]
        AOrNotB = 0x0d,
        #[doc = "A OR B."]
        AOrB = 0x0e,
        #[doc = "Logical 1."]
        LogicalOne = 0x0f,
    }
    impl AsyncCh9CtrlFnsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh9CtrlFnsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh9CtrlFnsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh9CtrlFnsel {
            AsyncCh9CtrlFnsel::from_bits(val)
        }
    }
    impl From<AsyncCh9CtrlFnsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh9CtrlFnsel) -> u8 {
            AsyncCh9CtrlFnsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AsyncCh9CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl AsyncCh9CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AsyncCh9CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AsyncCh9CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> AsyncCh9CtrlSigsel {
            AsyncCh9CtrlSigsel::from_bits(val)
        }
    }
    impl From<AsyncCh9CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: AsyncCh9CtrlSigsel) -> u8 {
            AsyncCh9CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SyncCh0CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl SyncCh0CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SyncCh0CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SyncCh0CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> SyncCh0CtrlSigsel {
            SyncCh0CtrlSigsel::from_bits(val)
        }
    }
    impl From<SyncCh0CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: SyncCh0CtrlSigsel) -> u8 {
            SyncCh0CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SyncCh1CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl SyncCh1CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SyncCh1CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SyncCh1CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> SyncCh1CtrlSigsel {
            SyncCh1CtrlSigsel::from_bits(val)
        }
    }
    impl From<SyncCh1CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: SyncCh1CtrlSigsel) -> u8 {
            SyncCh1CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SyncCh2CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl SyncCh2CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SyncCh2CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SyncCh2CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> SyncCh2CtrlSigsel {
            SyncCh2CtrlSigsel::from_bits(val)
        }
    }
    impl From<SyncCh2CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: SyncCh2CtrlSigsel) -> u8 {
            SyncCh2CtrlSigsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SyncCh3CtrlSigsel {
        #[doc = "NONE."]
        None = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl SyncCh3CtrlSigsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SyncCh3CtrlSigsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SyncCh3CtrlSigsel {
        #[inline(always)]
        fn from(val: u8) -> SyncCh3CtrlSigsel {
            SyncCh3CtrlSigsel::from_bits(val)
        }
    }
    impl From<SyncCh3CtrlSigsel> for u8 {
        #[inline(always)]
        fn from(val: SyncCh3CtrlSigsel) -> u8 {
            SyncCh3CtrlSigsel::to_bits(val)
        }
    }
}
