// Per-chip PAC content: peripheral module decls, typed peripheral
// consts, interrupt enum + cortex-m-rt glue, memory map.
// Generated for EFR32FG25B212F1920IM56.
//
// This file is `include!`d at the metapac crate root by `lib.rs`
// (selected via the `SILABS_METAPAC_PAC_PATH` env var emitted from
// `build.rs`). Mirrors `stm32-metapac`'s `chips/<chip>/pac.rs`
// layout — `#[path]` resolves relative to *this* file, so the
// `../../peripherals/...` paths below reach the shared chiptool
// peripheral modules under `src/peripherals/`.

// Chiptool peripheral modules (shared register/field types).
#[path = "../../peripherals/acmp_v3.rs"]
pub mod acmp_v3;
#[path = "../../peripherals/bufc_v3.rs"]
pub mod bufc_v3;
#[path = "../../peripherals/buram_v0.rs"]
pub mod buram_v0;
#[path = "../../peripherals/burtc_v1.rs"]
pub mod burtc_v1;
#[path = "../../peripherals/cmu_v4.rs"]
pub mod cmu_v4;
#[path = "../../peripherals/dcdc_v4.rs"]
pub mod dcdc_v4;
#[path = "../../peripherals/devinfo_v18.rs"]
pub mod devinfo_v18;
#[path = "../../peripherals/dmem_v2.rs"]
pub mod dmem_v2;
#[path = "../../peripherals/dmem_v2_fg25.rs"]
pub mod dmem_v2_fg25;
#[path = "../../peripherals/dpll_v1.rs"]
pub mod dpll_v1;
#[path = "../../peripherals/emu_v4.rs"]
pub mod emu_v4;
#[path = "../../peripherals/etampdet_v1.rs"]
pub mod etampdet_v1;
#[path = "../../peripherals/eusart_v2.rs"]
pub mod eusart_v2;
#[path = "../../peripherals/eusart_v2_lf.rs"]
pub mod eusart_v2_lf;
#[path = "../../peripherals/fsrco_v0.rs"]
pub mod fsrco_v0;
#[path = "../../peripherals/gpcrc_v0.rs"]
pub mod gpcrc_v0;
#[path = "../../peripherals/gpio_v4.rs"]
pub mod gpio_v4;
#[path = "../../peripherals/hfrco_v2.rs"]
pub mod hfrco_v2;
#[path = "../../peripherals/hfrcoem_v2.rs"]
pub mod hfrcoem_v2;
#[path = "../../peripherals/hfxo_v4.rs"]
pub mod hfxo_v4;
#[path = "../../peripherals/hostmailbox_v0.rs"]
pub mod hostmailbox_v0;
#[path = "../../peripherals/i2c_v0.rs"]
pub mod i2c_v0;
#[path = "../../peripherals/iadc_v3.rs"]
pub mod iadc_v3;
#[path = "../../peripherals/iadc_v3_fg25.rs"]
pub mod iadc_v3_fg25;
#[path = "../../peripherals/icache_v0.rs"]
pub mod icache_v0;
#[path = "../../peripherals/ldma_v1.rs"]
pub mod ldma_v1;
#[path = "../../peripherals/ldmaxbar_v4.rs"]
pub mod ldmaxbar_v4;
#[path = "../../peripherals/lesense_v1.rs"]
pub mod lesense_v1;
#[path = "../../peripherals/letimer_v1.rs"]
pub mod letimer_v1;
#[path = "../../peripherals/lfrco_v0.rs"]
pub mod lfrco_v0;
#[path = "../../peripherals/lfxo_v1.rs"]
pub mod lfxo_v1;
#[path = "../../peripherals/mpahbram_v2.rs"]
pub mod mpahbram_v2;
#[path = "../../peripherals/msc_v4.rs"]
pub mod msc_v4;
#[path = "../../peripherals/pcnt_v1.rs"]
pub mod pcnt_v1;
#[path = "../../peripherals/pfmxpprf_v0.rs"]
pub mod pfmxpprf_v0;
#[path = "../../peripherals/prs_v4.rs"]
pub mod prs_v4;
#[path = "../../peripherals/radioaes_v1.rs"]
pub mod radioaes_v1;
#[path = "../../peripherals/rffpll_v1.rs"]
pub mod rffpll_v1;
#[path = "../../peripherals/scratchpad_v0.rs"]
pub mod scratchpad_v0;
#[path = "../../peripherals/semailbox_ns_host_v1.rs"]
pub mod semailbox_ns_host_v1;
#[path = "../../peripherals/semailbox_s_host_v1.rs"]
pub mod semailbox_s_host_v1;
#[path = "../../peripherals/smu_v4.rs"]
pub mod smu_v4;
#[path = "../../peripherals/smu_ns_cfgns_v4.rs"]
pub mod smu_ns_cfgns_v4;
#[path = "../../peripherals/smu_s_cfgns_v4.rs"]
pub mod smu_s_cfgns_v4;
#[path = "../../peripherals/syscfg_v4.rs"]
pub mod syscfg_v4;
#[path = "../../peripherals/syscfg_ns_cfgns_v4.rs"]
pub mod syscfg_ns_cfgns_v4;
#[path = "../../peripherals/syscfg_s_cfgns_v4.rs"]
pub mod syscfg_s_cfgns_v4;
#[path = "../../peripherals/sysrtc_v1.rs"]
pub mod sysrtc_v1;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer_v1;
#[path = "../../peripherals/timer_v1_w.rs"]
pub mod timer_v1_w;
#[path = "../../peripherals/ulfrco_v1.rs"]
pub mod ulfrco_v1;
#[path = "../../peripherals/usb_ns_apbs_v1.rs"]
pub mod usb_ns_apbs_v1;
#[path = "../../peripherals/usb_s_apbs_v1.rs"]
pub mod usb_s_apbs_v1;
#[path = "../../peripherals/usbahb_ns_ahbs_v1.rs"]
pub mod usbahb_ns_ahbs_v1;
#[path = "../../peripherals/usbahb_s_ahbs_v1.rs"]
pub mod usbahb_s_ahbs_v1;
#[path = "../../peripherals/usbpll_v1.rs"]
pub mod usbpll_v1;
#[path = "../../peripherals/vdac_v2.rs"]
pub mod vdac_v2;
#[path = "../../peripherals/vdac_v2_fg25.rs"]
pub mod vdac_v2_fg25;
#[path = "../../peripherals/wdog_v1.rs"]
pub mod wdog_v1;

/// Memory map (flash/RAM regions, from the CMSIS pdsc).
pub mod memory {
    pub const IROM1_BASE: usize = 0x08000000;
    pub const IROM1_SIZE: usize = 0x001E0000;
    pub const IRAM1_BASE: usize = 0x20000000;
    pub const IRAM1_SIZE: usize = 0x00080000;
}

/// Typed peripheral instance constants.
///
/// Each peripheral is exposed once at its **non-secure** address (the alias
/// reachable from non-secure CPU state on TrustZone-enabled images). The
/// secure alias for any peripheral on Series 2 is `addr ^ 0x0100_0000`.
/// Secure-state code can XOR the bit explicitly when crossing the
/// security boundary.
pub const ACMP0: crate::acmp_v3::Acmp = unsafe { crate::acmp_v3::Acmp::from_ptr(0x59008000 as *mut ()) };
pub const ACMP1: crate::acmp_v3::Acmp = unsafe { crate::acmp_v3::Acmp::from_ptr(0x5900C000 as *mut ()) };
pub const BUFC: crate::bufc_v3::Bufc = unsafe { crate::bufc_v3::Bufc::from_ptr(0x52000000 as *mut ()) };
pub const BURAM: crate::buram_v0::Buram = unsafe { crate::buram_v0::Buram::from_ptr(0x50080000 as *mut ()) };
pub const BURTC: crate::burtc_v1::Burtc = unsafe { crate::burtc_v1::Burtc::from_ptr(0x5006C000 as *mut ()) };
pub const CMU: crate::cmu_v4::Cmu = unsafe { crate::cmu_v4::Cmu::from_ptr(0x50008000 as *mut ()) };
pub const DCDC: crate::dcdc_v4::Dcdc = unsafe { crate::dcdc_v4::Dcdc::from_ptr(0x50094000 as *mut ()) };
pub const DEVINFO: crate::devinfo_v18::Devinfo = unsafe { crate::devinfo_v18::Devinfo::from_ptr(0x0FE08000 as *mut ()) };
pub const DMEM: crate::dmem_v2_fg25::Dmem = unsafe { crate::dmem_v2_fg25::Dmem::from_ptr(0x500C0000 as *mut ()) };
pub const DPLL0: crate::dpll_v1::Dpll = unsafe { crate::dpll_v1::Dpll::from_ptr(0x5001C000 as *mut ()) };
pub const EMU: crate::emu_v4::Emu = unsafe { crate::emu_v4::Emu::from_ptr(0x50004000 as *mut ()) };
pub const ETAMPDET: crate::etampdet_v1::Etampdet = unsafe { crate::etampdet_v1::Etampdet::from_ptr(0x500D0000 as *mut ()) };
pub const EUSART0: crate::eusart_v2_lf::Eusart = unsafe { crate::eusart_v2_lf::Eusart::from_ptr(0x5B010000 as *mut ()) };
pub const EUSART1: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x500A0000 as *mut ()) };
pub const EUSART2: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x500A4000 as *mut ()) };
pub const EUSART3: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x500A8000 as *mut ()) };
pub const EUSART4: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x500AC000 as *mut ()) };
pub const FSRCO: crate::fsrco_v0::Fsrco = unsafe { crate::fsrco_v0::Fsrco::from_ptr(0x50018000 as *mut ()) };
pub const GPCRC: crate::gpcrc_v0::Gpcrc = unsafe { crate::gpcrc_v0::Gpcrc::from_ptr(0x50088000 as *mut ()) };
pub const GPIO: crate::gpio_v4::Gpio = unsafe { crate::gpio_v4::Gpio::from_ptr(0x5003C000 as *mut ()) };
pub const HFRCO0: crate::hfrco_v2::Hfrco = unsafe { crate::hfrco_v2::Hfrco::from_ptr(0x50010000 as *mut ()) };
pub const HFRCOEM23: crate::hfrcoem_v2::Hfrcoem = unsafe { crate::hfrcoem_v2::Hfrcoem::from_ptr(0x5A000000 as *mut ()) };
pub const HFXO0: crate::hfxo_v4::Hfxo = unsafe { crate::hfxo_v4::Hfxo::from_ptr(0x5A004000 as *mut ()) };
pub const HOSTMAILBOX: crate::hostmailbox_v0::Hostmailbox = unsafe { crate::hostmailbox_v0::Hostmailbox::from_ptr(0x50098000 as *mut ()) };
pub const I2C0: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x5B000000 as *mut ()) };
pub const I2C1: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x50070000 as *mut ()) };
pub const IADC0: crate::iadc_v3_fg25::Iadc = unsafe { crate::iadc_v3_fg25::Iadc::from_ptr(0x59004000 as *mut ()) };
pub const ICACHE0: crate::icache_v0::Icache = unsafe { crate::icache_v0::Icache::from_ptr(0x50034000 as *mut ()) };
pub const LDMA: crate::ldma_v1::Ldma = unsafe { crate::ldma_v1::Ldma::from_ptr(0x50040000 as *mut ()) };
pub const LDMAXBAR: crate::ldmaxbar_v4::Ldmaxbar = unsafe { crate::ldmaxbar_v4::Ldmaxbar::from_ptr(0x50044000 as *mut ()) };
pub const LESENSE: crate::lesense_v1::Lesense = unsafe { crate::lesense_v1::Lesense::from_ptr(0x59038000 as *mut ()) };
pub const LETIMER0: crate::letimer_v1::Letimer = unsafe { crate::letimer_v1::Letimer::from_ptr(0x59000000 as *mut ()) };
pub const LFRCO: crate::lfrco_v0::Lfrco = unsafe { crate::lfrco_v0::Lfrco::from_ptr(0x50024000 as *mut ()) };
pub const LFXO: crate::lfxo_v1::Lfxo = unsafe { crate::lfxo_v1::Lfxo::from_ptr(0x50020000 as *mut ()) };
pub const MPAHBRAM: crate::mpahbram_v2::Mpahbram = unsafe { crate::mpahbram_v2::Mpahbram::from_ptr(0xB6020000 as *mut ()) };
pub const MSC: crate::msc_v4::Msc = unsafe { crate::msc_v4::Msc::from_ptr(0x50030000 as *mut ()) };
pub const PCNT0: crate::pcnt_v1::Pcnt = unsafe { crate::pcnt_v1::Pcnt::from_ptr(0x59030000 as *mut ()) };
pub const PFMXPPRF: crate::pfmxpprf_v0::Pfmxpprf = unsafe { crate::pfmxpprf_v0::Pfmxpprf::from_ptr(0x500C4000 as *mut ()) };
pub const PRS: crate::prs_v4::Prs = unsafe { crate::prs_v4::Prs::from_ptr(0x50038000 as *mut ()) };
pub const RADIOAES: crate::radioaes_v1::Radioaes = unsafe { crate::radioaes_v1::Radioaes::from_ptr(0x54000000 as *mut ()) };
pub const RFFPLL0: crate::rffpll_v1::Rffpll = unsafe { crate::rffpll_v1::Rffpll::from_ptr(0x500C8000 as *mut ()) };
pub const SCRATCHPAD: crate::scratchpad_v0::Scratchpad = unsafe { crate::scratchpad_v0::Scratchpad::from_ptr(0x50000000 as *mut ()) };
pub const SEMAILBOX_NS_HOST: crate::semailbox_ns_host_v1::SemailboxNsHost = unsafe { crate::semailbox_ns_host_v1::SemailboxNsHost::from_ptr(0x5C000000 as *mut ()) };
pub const SEMAILBOX_S_HOST: crate::semailbox_s_host_v1::SemailboxSHost = unsafe { crate::semailbox_s_host_v1::SemailboxSHost::from_ptr(0x4C000000 as *mut ()) };
pub const SMU: crate::smu_v4::Smu = unsafe { crate::smu_v4::Smu::from_ptr(0x54008000 as *mut ()) };
pub const SMU_NS_CFGNS: crate::smu_ns_cfgns_v4::SmuNsCfgns = unsafe { crate::smu_ns_cfgns_v4::SmuNsCfgns::from_ptr(0x5400C000 as *mut ()) };
pub const SMU_S_CFGNS: crate::smu_s_cfgns_v4::SmuSCfgns = unsafe { crate::smu_s_cfgns_v4::SmuSCfgns::from_ptr(0x4400C000 as *mut ()) };
pub const SYSCFG: crate::syscfg_v4::Syscfg = unsafe { crate::syscfg_v4::Syscfg::from_ptr(0x5007C000 as *mut ()) };
pub const SYSCFG_NS_CFGNS: crate::syscfg_ns_cfgns_v4::SyscfgNsCfgns = unsafe { crate::syscfg_ns_cfgns_v4::SyscfgNsCfgns::from_ptr(0x50078000 as *mut ()) };
pub const SYSCFG_S_CFGNS: crate::syscfg_s_cfgns_v4::SyscfgSCfgns = unsafe { crate::syscfg_s_cfgns_v4::SyscfgSCfgns::from_ptr(0x40078000 as *mut ()) };
pub const SYSRTC0: crate::sysrtc_v1::Sysrtc = unsafe { crate::sysrtc_v1::Sysrtc::from_ptr(0x500B0000 as *mut ()) };
pub const TIMER0: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x50048000 as *mut ()) };
pub const TIMER1: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x5004C000 as *mut ()) };
pub const TIMER2: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50050000 as *mut ()) };
pub const TIMER3: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50054000 as *mut ()) };
pub const TIMER4: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50058000 as *mut ()) };
pub const TIMER5: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x5005C000 as *mut ()) };
pub const TIMER6: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50060000 as *mut ()) };
pub const TIMER7: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50064000 as *mut ()) };
pub const ULFRCO: crate::ulfrco_v1::Ulfrco = unsafe { crate::ulfrco_v1::Ulfrco::from_ptr(0x50028000 as *mut ()) };
pub const USBAHB_NS_AHBS: crate::usbahb_ns_ahbs_v1::UsbahbNsAhbs = unsafe { crate::usbahb_ns_ahbs_v1::UsbahbNsAhbs::from_ptr(0x56000000 as *mut ()) };
pub const USBAHB_S_AHBS: crate::usbahb_s_ahbs_v1::UsbahbSAhbs = unsafe { crate::usbahb_s_ahbs_v1::UsbahbSAhbs::from_ptr(0x46000000 as *mut ()) };
pub const USBPLL0: crate::usbpll_v1::Usbpll = unsafe { crate::usbpll_v1::Usbpll::from_ptr(0x57004000 as *mut ()) };
pub const USB_NS_APBS: crate::usb_ns_apbs_v1::UsbNsApbs = unsafe { crate::usb_ns_apbs_v1::UsbNsApbs::from_ptr(0x57000000 as *mut ()) };
pub const USB_S_APBS: crate::usb_s_apbs_v1::UsbSApbs = unsafe { crate::usb_s_apbs_v1::UsbSApbs::from_ptr(0x47000000 as *mut ()) };
pub const VDAC0: crate::vdac_v2_fg25::Vdac = unsafe { crate::vdac_v2_fg25::Vdac::from_ptr(0x59024000 as *mut ()) };
pub const WDOG0: crate::wdog_v1::Wdog = unsafe { crate::wdog_v1::Wdog::from_ptr(0x5B004000 as *mut ()) };
pub const WDOG1: crate::wdog_v1::Wdog = unsafe { crate::wdog_v1::Wdog::from_ptr(0x5B008000 as *mut ()) };

/// GPIO port indices, mirroring `efr32mg<NN>_gpio.h`'s
/// `#define GPIO_PORTA 0` etc. Use as `GPIO.p(gpio_port::PORTC)`
/// (or just `GPIO.p(2)` — they're equivalent).
pub mod gpio_port {
    pub const PORTA: usize = 0;
    pub const PORTB: usize = 1;
    pub const PORTC: usize = 2;
    pub const PORTD: usize = 3;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum Interrupt {
    SMU_SECURE = 0,
    SMU_S_PRIVILEGED = 1,
    SMU_NS_PRIVILEGED = 2,
    EMU = 3,
    TIMER0 = 4,
    TIMER1 = 5,
    TIMER2 = 6,
    TIMER3 = 7,
    ETAMPDET = 8,
    TIMER4 = 9,
    TIMER5 = 10,
    TIMER6 = 11,
    TIMER7 = 12,
    EUSART0_RX = 13,
    EUSART0_TX = 14,
    EUSART1_RX = 15,
    EUSART1_TX = 16,
    EUSART2_RX = 17,
    EUSART2_TX = 18,
    EUSART3_RX = 19,
    EUSART3_TX = 20,
    EUSART4_RX = 21,
    EUSART4_TX = 22,
    ICACHE0 = 23,
    BURTC = 24,
    LETIMER0 = 25,
    SYSCFG = 26,
    MPAHBRAM = 27,
    LDMA = 28,
    LFXO = 29,
    LFRCO = 30,
    ULFRCO = 31,
    GPIO_ODD = 32,
    GPIO_EVEN = 33,
    I2C0 = 34,
    I2C1 = 35,
    EMUDG = 36,
    AGC = 37,
    BUFC = 38,
    FRC_PRI = 39,
    FRC = 40,
    MODEM = 41,
    PROTIMER = 42,
    RAC_RSM = 43,
    RAC_SEQ = 44,
    HOSTMAILBOX = 45,
    SYNTH = 46,
    ACMP0 = 47,
    ACMP1 = 48,
    WDOG0 = 49,
    WDOG1 = 50,
    HFXO0 = 51,
    HFRCO0 = 52,
    HFRCOEM23 = 53,
    CMU = 54,
    AES = 55,
    IADC = 56,
    MSC = 57,
    DPLL0 = 58,
    DCDC = 60,
    VDAC = 61,
    PCNT0 = 62,
    USB = 63,
    SW0 = 64,
    SW1 = 65,
    SW2 = 66,
    SW3 = 67,
    KERNEL0 = 68,
    KERNEL1 = 69,
    M33CTI0 = 70,
    M33CTI1 = 71,
    FPUEXH = 72,
    SETAMPERHOST = 73,
    SEMBRX = 74,
    SEMBTX = 75,
    LESENSE = 76,
    SYSRTC_APP = 77,
    SYSRTC_SEQ = 78,
    RFECA0 = 79,
    RFECA1 = 80,
    RFFPLL = 81,
    USBPLL0 = 82,
    AHB2AHB0 = 83,
    AHB2AHB1 = 84,
    AHB2AHB2 = 85,
    AHB2AHB3 = 86,
    RFTIMER = 87,
    SOFTM = 88,
    RFLDMA = 89,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 { self as u16 }
}

#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn SMU_SECURE();
        fn SMU_S_PRIVILEGED();
        fn SMU_NS_PRIVILEGED();
        fn EMU();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn TIMER3();
        fn ETAMPDET();
        fn TIMER4();
        fn TIMER5();
        fn TIMER6();
        fn TIMER7();
        fn EUSART0_RX();
        fn EUSART0_TX();
        fn EUSART1_RX();
        fn EUSART1_TX();
        fn EUSART2_RX();
        fn EUSART2_TX();
        fn EUSART3_RX();
        fn EUSART3_TX();
        fn EUSART4_RX();
        fn EUSART4_TX();
        fn ICACHE0();
        fn BURTC();
        fn LETIMER0();
        fn SYSCFG();
        fn MPAHBRAM();
        fn LDMA();
        fn LFXO();
        fn LFRCO();
        fn ULFRCO();
        fn GPIO_ODD();
        fn GPIO_EVEN();
        fn I2C0();
        fn I2C1();
        fn EMUDG();
        fn AGC();
        fn BUFC();
        fn FRC_PRI();
        fn FRC();
        fn MODEM();
        fn PROTIMER();
        fn RAC_RSM();
        fn RAC_SEQ();
        fn HOSTMAILBOX();
        fn SYNTH();
        fn ACMP0();
        fn ACMP1();
        fn WDOG0();
        fn WDOG1();
        fn HFXO0();
        fn HFRCO0();
        fn HFRCOEM23();
        fn CMU();
        fn AES();
        fn IADC();
        fn MSC();
        fn DPLL0();
        fn DCDC();
        fn VDAC();
        fn PCNT0();
        fn USB();
        fn SW0();
        fn SW1();
        fn SW2();
        fn SW3();
        fn KERNEL0();
        fn KERNEL1();
        fn M33CTI0();
        fn M33CTI1();
        fn FPUEXH();
        fn SETAMPERHOST();
        fn SEMBRX();
        fn SEMBTX();
        fn LESENSE();
        fn SYSRTC_APP();
        fn SYSRTC_SEQ();
        fn RFECA0();
        fn RFECA1();
        fn RFFPLL();
        fn USBPLL0();
        fn AHB2AHB0();
        fn AHB2AHB1();
        fn AHB2AHB2();
        fn AHB2AHB3();
        fn RFTIMER();
        fn SOFTM();
        fn RFLDMA();
    }

    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }

    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 90] = [
        Vector { _handler: SMU_SECURE },
        Vector { _handler: SMU_S_PRIVILEGED },
        Vector { _handler: SMU_NS_PRIVILEGED },
        Vector { _handler: EMU },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: TIMER3 },
        Vector { _handler: ETAMPDET },
        Vector { _handler: TIMER4 },
        Vector { _handler: TIMER5 },
        Vector { _handler: TIMER6 },
        Vector { _handler: TIMER7 },
        Vector { _handler: EUSART0_RX },
        Vector { _handler: EUSART0_TX },
        Vector { _handler: EUSART1_RX },
        Vector { _handler: EUSART1_TX },
        Vector { _handler: EUSART2_RX },
        Vector { _handler: EUSART2_TX },
        Vector { _handler: EUSART3_RX },
        Vector { _handler: EUSART3_TX },
        Vector { _handler: EUSART4_RX },
        Vector { _handler: EUSART4_TX },
        Vector { _handler: ICACHE0 },
        Vector { _handler: BURTC },
        Vector { _handler: LETIMER0 },
        Vector { _handler: SYSCFG },
        Vector { _handler: MPAHBRAM },
        Vector { _handler: LDMA },
        Vector { _handler: LFXO },
        Vector { _handler: LFRCO },
        Vector { _handler: ULFRCO },
        Vector { _handler: GPIO_ODD },
        Vector { _handler: GPIO_EVEN },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: EMUDG },
        Vector { _handler: AGC },
        Vector { _handler: BUFC },
        Vector { _handler: FRC_PRI },
        Vector { _handler: FRC },
        Vector { _handler: MODEM },
        Vector { _handler: PROTIMER },
        Vector { _handler: RAC_RSM },
        Vector { _handler: RAC_SEQ },
        Vector { _handler: HOSTMAILBOX },
        Vector { _handler: SYNTH },
        Vector { _handler: ACMP0 },
        Vector { _handler: ACMP1 },
        Vector { _handler: WDOG0 },
        Vector { _handler: WDOG1 },
        Vector { _handler: HFXO0 },
        Vector { _handler: HFRCO0 },
        Vector { _handler: HFRCOEM23 },
        Vector { _handler: CMU },
        Vector { _handler: AES },
        Vector { _handler: IADC },
        Vector { _handler: MSC },
        Vector { _handler: DPLL0 },
        Vector { _reserved: 0 },
        Vector { _handler: DCDC },
        Vector { _handler: VDAC },
        Vector { _handler: PCNT0 },
        Vector { _handler: USB },
        Vector { _handler: SW0 },
        Vector { _handler: SW1 },
        Vector { _handler: SW2 },
        Vector { _handler: SW3 },
        Vector { _handler: KERNEL0 },
        Vector { _handler: KERNEL1 },
        Vector { _handler: M33CTI0 },
        Vector { _handler: M33CTI1 },
        Vector { _handler: FPUEXH },
        Vector { _handler: SETAMPERHOST },
        Vector { _handler: SEMBRX },
        Vector { _handler: SEMBTX },
        Vector { _handler: LESENSE },
        Vector { _handler: SYSRTC_APP },
        Vector { _handler: SYSRTC_SEQ },
        Vector { _handler: RFECA0 },
        Vector { _handler: RFECA1 },
        Vector { _handler: RFFPLL },
        Vector { _handler: USBPLL0 },
        Vector { _handler: AHB2AHB0 },
        Vector { _handler: AHB2AHB1 },
        Vector { _handler: AHB2AHB2 },
        Vector { _handler: AHB2AHB3 },
        Vector { _handler: RFTIMER },
        Vector { _handler: SOFTM },
        Vector { _handler: RFLDMA },
    ];
}

/// Number available in the NVIC for configuring priority.
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
