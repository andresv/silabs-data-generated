// Per-chip metadata: typed peripheral consts, interrupts, memory map.
// Generated for EFR32MG24A120F1536GM48.

/// Memory map (flash/RAM regions, from the CMSIS pdsc).
pub mod memory {
    pub const IROM1_BASE: usize = 0x08000000;
    pub const IROM1_SIZE: usize = 0x00180000;
    pub const IRAM1_BASE: usize = 0x20000000;
    pub const IRAM1_SIZE: usize = 0x00040000;
}

/// Typed peripheral instance constants.
///
/// Each peripheral is exposed once at its **non-secure** address (the alias
/// reachable from non-secure CPU state on TrustZone-enabled images). The
/// secure alias for any peripheral on Series 2 is `addr ^ 0x0100_0000`.
/// Secure-state code can XOR the bit explicitly when crossing the
/// security boundary.
pub const ACMP0: crate::acmp_v2::Acmp = unsafe { crate::acmp_v2::Acmp::from_ptr(0x59008000 as *mut ()) };
pub const ACMP1: crate::acmp_v2::Acmp = unsafe { crate::acmp_v2::Acmp::from_ptr(0x5900C000 as *mut ()) };
pub const BURAM: crate::buram_v0::Buram = unsafe { crate::buram_v0::Buram::from_ptr(0x50080000 as *mut ()) };
pub const BURTC: crate::burtc_v1::Burtc = unsafe { crate::burtc_v1::Burtc::from_ptr(0x50064000 as *mut ()) };
pub const CMU: crate::cmu_v3::Cmu = unsafe { crate::cmu_v3::Cmu::from_ptr(0x50008000 as *mut ()) };
pub const DCDC: crate::dcdc_v2::Dcdc = unsafe { crate::dcdc_v2::Dcdc::from_ptr(0x50094000 as *mut ()) };
pub const DEVINFO: crate::devinfo_v0_mg24::Devinfo = unsafe { crate::devinfo_v0_mg24::Devinfo::from_ptr(0x0FE08000 as *mut ()) };
pub const DMEM: crate::dmem_v2_ws::Dmem = unsafe { crate::dmem_v2_ws::Dmem::from_ptr(0x500B4000 as *mut ()) };
pub const DPLL0: crate::dpll_v1::Dpll = unsafe { crate::dpll_v1::Dpll::from_ptr(0x5001C000 as *mut ()) };
pub const EMU: crate::emu_v3::Emu = unsafe { crate::emu_v3::Emu::from_ptr(0x50004000 as *mut ()) };
pub const EUSART0: crate::eusart_v2_lf::Eusart = unsafe { crate::eusart_v2_lf::Eusart::from_ptr(0x5B010000 as *mut ()) };
pub const EUSART1: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x500A0000 as *mut ()) };
pub const FSRCO: crate::fsrco_v0::Fsrco = unsafe { crate::fsrco_v0::Fsrco::from_ptr(0x50018000 as *mut ()) };
pub const GPCRC: crate::gpcrc_v0::Gpcrc = unsafe { crate::gpcrc_v0::Gpcrc::from_ptr(0x50088000 as *mut ()) };
pub const GPIO: crate::gpio_v3::Gpio = unsafe { crate::gpio_v3::Gpio::from_ptr(0x5003C000 as *mut ()) };
pub const HFRCO0: crate::hfrco_v2::Hfrco = unsafe { crate::hfrco_v2::Hfrco::from_ptr(0x50010000 as *mut ()) };
pub const HFRCOEM23: crate::hfrcoem_v2::Hfrcoem = unsafe { crate::hfrcoem_v2::Hfrcoem::from_ptr(0x5A000000 as *mut ()) };
pub const HFXO0: crate::hfxo_v3::Hfxo = unsafe { crate::hfxo_v3::Hfxo::from_ptr(0x5A004000 as *mut ()) };
pub const HOSTMAILBOX: crate::hostmailbox_v0::Hostmailbox = unsafe { crate::hostmailbox_v0::Hostmailbox::from_ptr(0x50098000 as *mut ()) };
pub const I2C0: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x5B000000 as *mut ()) };
pub const I2C1: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x50068000 as *mut ()) };
pub const IADC0: crate::iadc_v3_ha::Iadc = unsafe { crate::iadc_v3_ha::Iadc::from_ptr(0x59004000 as *mut ()) };
pub const ICACHE0: crate::icache_v0::Icache = unsafe { crate::icache_v0::Icache::from_ptr(0x50034000 as *mut ()) };
pub const KEYSCAN: crate::keyscan_v1::Keyscan = unsafe { crate::keyscan_v1::Keyscan::from_ptr(0x500B0000 as *mut ()) };
pub const LDMA: crate::ldma_v0::Ldma = unsafe { crate::ldma_v0::Ldma::from_ptr(0x50040000 as *mut ()) };
pub const LDMAXBAR: crate::ldmaxbar_v3::Ldmaxbar = unsafe { crate::ldmaxbar_v3::Ldmaxbar::from_ptr(0x50044000 as *mut ()) };
pub const LETIMER0: crate::letimer_v1::Letimer = unsafe { crate::letimer_v1::Letimer::from_ptr(0x59000000 as *mut ()) };
pub const LFRCO: crate::lfrco_v2::Lfrco = unsafe { crate::lfrco_v2::Lfrco::from_ptr(0x50024000 as *mut ()) };
pub const LFXO: crate::lfxo_v1::Lfxo = unsafe { crate::lfxo_v1::Lfxo::from_ptr(0x50020000 as *mut ()) };
pub const MSC: crate::msc_v3::Msc = unsafe { crate::msc_v3::Msc::from_ptr(0x50030000 as *mut ()) };
pub const PCNT0: crate::pcnt_v1::Pcnt = unsafe { crate::pcnt_v1::Pcnt::from_ptr(0x59030000 as *mut ()) };
pub const PRS: crate::prs_v3::Prs = unsafe { crate::prs_v3::Prs::from_ptr(0x50038000 as *mut ()) };
pub const RADIOAES: crate::radioaes_v1::Radioaes = unsafe { crate::radioaes_v1::Radioaes::from_ptr(0x54000000 as *mut ()) };
pub const SCRATCHPAD: crate::scratchpad_v0::Scratchpad = unsafe { crate::scratchpad_v0::Scratchpad::from_ptr(0x50000000 as *mut ()) };
pub const SEMAILBOX_NS_HOST: crate::semailbox_ns_host_v1::SemailboxNsHost = unsafe { crate::semailbox_ns_host_v1::SemailboxNsHost::from_ptr(0x5C000000 as *mut ()) };
pub const SEMAILBOX_S_HOST: crate::semailbox_s_host_v1::SemailboxSHost = unsafe { crate::semailbox_s_host_v1::SemailboxSHost::from_ptr(0x4C000000 as *mut ()) };
pub const SMU: crate::smu_v3::Smu = unsafe { crate::smu_v3::Smu::from_ptr(0x54008000 as *mut ()) };
pub const SMU_NS_CFGNS: crate::smu_ns_cfgns_v3::SmuNsCfgns = unsafe { crate::smu_ns_cfgns_v3::SmuNsCfgns::from_ptr(0x5400C000 as *mut ()) };
pub const SMU_S_CFGNS: crate::smu_s_cfgns_v3::SmuSCfgns = unsafe { crate::smu_s_cfgns_v3::SmuSCfgns::from_ptr(0x4400C000 as *mut ()) };
pub const SYSCFG: crate::syscfg_v3::Syscfg = unsafe { crate::syscfg_v3::Syscfg::from_ptr(0x5007C000 as *mut ()) };
pub const SYSCFG_NS_CFGNS: crate::syscfg_ns_cfgns_v3::SyscfgNsCfgns = unsafe { crate::syscfg_ns_cfgns_v3::SyscfgNsCfgns::from_ptr(0x50078000 as *mut ()) };
pub const SYSCFG_S_CFGNS: crate::syscfg_s_cfgns_v3::SyscfgSCfgns = unsafe { crate::syscfg_s_cfgns_v3::SyscfgSCfgns::from_ptr(0x40078000 as *mut ()) };
pub const SYSRTC0: crate::sysrtc_v1::Sysrtc = unsafe { crate::sysrtc_v1::Sysrtc::from_ptr(0x500A8000 as *mut ()) };
pub const TIMER0: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x50048000 as *mut ()) };
pub const TIMER1: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x5004C000 as *mut ()) };
pub const TIMER2: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50050000 as *mut ()) };
pub const TIMER3: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50054000 as *mut ()) };
pub const TIMER4: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50058000 as *mut ()) };
pub const ULFRCO: crate::ulfrco_v1::Ulfrco = unsafe { crate::ulfrco_v1::Ulfrco::from_ptr(0x50028000 as *mut ()) };
pub const USART0: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x5005C000 as *mut ()) };
pub const VDAC0: crate::vdac_v2::Vdac = unsafe { crate::vdac_v2::Vdac::from_ptr(0x59024000 as *mut ()) };
pub const VDAC1: crate::vdac_v2::Vdac = unsafe { crate::vdac_v2::Vdac::from_ptr(0x59028000 as *mut ()) };
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

/// Cortex-M interrupt numbers (deduped by name).
pub mod interrupts {
    pub const ACMP0: u8 = 40;
    pub const ACMP1: u8 = 41;
    pub const BURTC: u8 = 17;
    pub const CMU: u8 = 47;
    pub const DCDC: u8 = 53;
    pub const DPLL0: u8 = 51;
    pub const EMU: u8 = 3;
    pub const EMUDG: u8 = 29;
    pub const EUSART0_RX: u8 = 11;
    pub const EUSART0_TX: u8 = 12;
    pub const EUSART1_RX: u8 = 13;
    pub const EUSART1_TX: u8 = 14;
    pub const GPIO_ODD: u8 = 25;
    pub const GPIO_EVEN: u8 = 26;
    pub const HFRCO0: u8 = 45;
    pub const HFRCOEM23: u8 = 46;
    pub const HFXO0: u8 = 44;
    pub const HOSTMAILBOX: u8 = 38;
    pub const I2C0: u8 = 27;
    pub const I2C1: u8 = 28;
    pub const IADC: u8 = 49;
    pub const ICACHE0: u8 = 16;
    pub const KEYSCAN: u8 = 69;
    pub const LDMA: u8 = 21;
    pub const LETIMER0: u8 = 18;
    pub const LFRCO: u8 = 23;
    pub const LFXO: u8 = 22;
    pub const MSC: u8 = 50;
    pub const PCNT0: u8 = 54;
    pub const AES: u8 = 48;
    pub const SEMBRX: u8 = 65;
    pub const SEMBTX: u8 = 66;
    pub const SETAMPERHOST: u8 = 64;
    pub const SMU_SECURE: u8 = 0;
    pub const SMU_S_PRIVILEGED: u8 = 1;
    pub const SYSCFG: u8 = 19;
    pub const SW0: u8 = 55;
    pub const SW1: u8 = 56;
    pub const SW2: u8 = 57;
    pub const SW3: u8 = 58;
    pub const SYSRTC_APP: u8 = 67;
    pub const SYSRTC_SEQ: u8 = 68;
    pub const TIMER0: u8 = 4;
    pub const TIMER1: u8 = 5;
    pub const TIMER2: u8 = 6;
    pub const TIMER3: u8 = 7;
    pub const TIMER4: u8 = 8;
    pub const ULFRCO: u8 = 24;
    pub const USART0_RX: u8 = 9;
    pub const USART0_TX: u8 = 10;
    pub const VDAC0: u8 = 72;
    pub const VDAC1: u8 = 73;
    pub const WDOG0: u8 = 42;
    pub const WDOG1: u8 = 43;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub enum Interrupt {
    SMU_SECURE = 0,
    SMU_S_PRIVILEGED = 1,
    EMU = 3,
    TIMER0 = 4,
    TIMER1 = 5,
    TIMER2 = 6,
    TIMER3 = 7,
    TIMER4 = 8,
    USART0_RX = 9,
    USART0_TX = 10,
    EUSART0_RX = 11,
    EUSART0_TX = 12,
    EUSART1_RX = 13,
    EUSART1_TX = 14,
    ICACHE0 = 16,
    BURTC = 17,
    LETIMER0 = 18,
    SYSCFG = 19,
    LDMA = 21,
    LFXO = 22,
    LFRCO = 23,
    ULFRCO = 24,
    GPIO_ODD = 25,
    GPIO_EVEN = 26,
    I2C0 = 27,
    I2C1 = 28,
    EMUDG = 29,
    HOSTMAILBOX = 38,
    ACMP0 = 40,
    ACMP1 = 41,
    WDOG0 = 42,
    WDOG1 = 43,
    HFXO0 = 44,
    HFRCO0 = 45,
    HFRCOEM23 = 46,
    CMU = 47,
    AES = 48,
    IADC = 49,
    MSC = 50,
    DPLL0 = 51,
    DCDC = 53,
    PCNT0 = 54,
    SW0 = 55,
    SW1 = 56,
    SW2 = 57,
    SW3 = 58,
    SETAMPERHOST = 64,
    SEMBRX = 65,
    SEMBTX = 66,
    SYSRTC_APP = 67,
    SYSRTC_SEQ = 68,
    KEYSCAN = 69,
    VDAC0 = 72,
    VDAC1 = 73,
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
        fn EMU();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn TIMER3();
        fn TIMER4();
        fn USART0_RX();
        fn USART0_TX();
        fn EUSART0_RX();
        fn EUSART0_TX();
        fn EUSART1_RX();
        fn EUSART1_TX();
        fn ICACHE0();
        fn BURTC();
        fn LETIMER0();
        fn SYSCFG();
        fn LDMA();
        fn LFXO();
        fn LFRCO();
        fn ULFRCO();
        fn GPIO_ODD();
        fn GPIO_EVEN();
        fn I2C0();
        fn I2C1();
        fn EMUDG();
        fn HOSTMAILBOX();
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
        fn PCNT0();
        fn SW0();
        fn SW1();
        fn SW2();
        fn SW3();
        fn SETAMPERHOST();
        fn SEMBRX();
        fn SEMBTX();
        fn SYSRTC_APP();
        fn SYSRTC_SEQ();
        fn KEYSCAN();
        fn VDAC0();
        fn VDAC1();
    }

    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }

    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 74] = [
        Vector { _handler: SMU_SECURE },
        Vector { _handler: SMU_S_PRIVILEGED },
        Vector { _reserved: 0 },
        Vector { _handler: EMU },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: TIMER3 },
        Vector { _handler: TIMER4 },
        Vector { _handler: USART0_RX },
        Vector { _handler: USART0_TX },
        Vector { _handler: EUSART0_RX },
        Vector { _handler: EUSART0_TX },
        Vector { _handler: EUSART1_RX },
        Vector { _handler: EUSART1_TX },
        Vector { _reserved: 0 },
        Vector { _handler: ICACHE0 },
        Vector { _handler: BURTC },
        Vector { _handler: LETIMER0 },
        Vector { _handler: SYSCFG },
        Vector { _reserved: 0 },
        Vector { _handler: LDMA },
        Vector { _handler: LFXO },
        Vector { _handler: LFRCO },
        Vector { _handler: ULFRCO },
        Vector { _handler: GPIO_ODD },
        Vector { _handler: GPIO_EVEN },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: EMUDG },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: HOSTMAILBOX },
        Vector { _reserved: 0 },
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
        Vector { _handler: PCNT0 },
        Vector { _handler: SW0 },
        Vector { _handler: SW1 },
        Vector { _handler: SW2 },
        Vector { _handler: SW3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SETAMPERHOST },
        Vector { _handler: SEMBRX },
        Vector { _handler: SEMBTX },
        Vector { _handler: SYSRTC_APP },
        Vector { _handler: SYSRTC_SEQ },
        Vector { _handler: KEYSCAN },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: VDAC0 },
        Vector { _handler: VDAC1 },
    ];
}

/// Number available in the NVIC for configuring priority.
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
