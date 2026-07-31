// Per-chip PAC content: peripheral module decls, typed peripheral
// consts, interrupt enum + cortex-m-rt glue, memory map.
// Generated for EFR32MG22E224F512IM32.
//
// This file is `include!`d at the metapac crate root by `lib.rs`
// (selected via the `SILABS_METAPAC_PAC_PATH` env var emitted from
// `build.rs`). Mirrors `stm32-metapac`'s `chips/<chip>/pac.rs`
// layout — `#[path]` resolves relative to *this* file, so the
// `../../peripherals/...` paths below reach the shared chiptool
// peripheral modules under `src/peripherals/`.

// Chiptool peripheral modules (shared register/field types).
#[path = "../../peripherals/buram_v0.rs"]
pub mod buram_v0;
#[path = "../../peripherals/burtc_v0.rs"]
pub mod burtc_v0;
#[path = "../../peripherals/cmu_v1.rs"]
pub mod cmu_v1;
#[path = "../../peripherals/cryptoacc_v1.rs"]
pub mod cryptoacc_v1;
#[path = "../../peripherals/cryptoacc_ns_pkctrl_v1.rs"]
pub mod cryptoacc_ns_pkctrl_v1;
#[path = "../../peripherals/cryptoacc_ns_rngctrl_v1.rs"]
pub mod cryptoacc_ns_rngctrl_v1;
#[path = "../../peripherals/dcdc_v0.rs"]
pub mod dcdc_v0;
#[path = "../../peripherals/devinfo_v1.rs"]
pub mod devinfo_v1;
#[path = "../../peripherals/dpll_v0.rs"]
pub mod dpll_v0;
#[path = "../../peripherals/emu_v8.rs"]
pub mod emu_v8;
#[path = "../../peripherals/euart_v0.rs"]
pub mod euart_v0;
#[path = "../../peripherals/fsrco_v1.rs"]
pub mod fsrco_v1;
#[path = "../../peripherals/gpcrc_v0.rs"]
pub mod gpcrc_v0;
#[path = "../../peripherals/gpio_v1.rs"]
pub mod gpio_v1;
#[path = "../../peripherals/hfrco_v1.rs"]
pub mod hfrco_v1;
#[path = "../../peripherals/hfxo_v2.rs"]
pub mod hfxo_v2;
#[path = "../../peripherals/i2c_v0.rs"]
pub mod i2c_v0;
#[path = "../../peripherals/iadc_v1.rs"]
pub mod iadc_v1;
#[path = "../../peripherals/icache_v0.rs"]
pub mod icache_v0;
#[path = "../../peripherals/ldma_v0.rs"]
pub mod ldma_v0;
#[path = "../../peripherals/ldmaxbar_v1.rs"]
pub mod ldmaxbar_v1;
#[path = "../../peripherals/letimer_v0.rs"]
pub mod letimer_v0;
#[path = "../../peripherals/lfrco_v3.rs"]
pub mod lfrco_v3;
#[path = "../../peripherals/lfxo_v0.rs"]
pub mod lfxo_v0;
#[path = "../../peripherals/msc_v8.rs"]
pub mod msc_v8;
#[path = "../../peripherals/pdm_v0.rs"]
pub mod pdm_v0;
#[path = "../../peripherals/prortc_v1.rs"]
pub mod prortc_v1;
#[path = "../../peripherals/prs_v1.rs"]
pub mod prs_v1;
#[path = "../../peripherals/radioaes_v1.rs"]
pub mod radioaes_v1;
#[path = "../../peripherals/rtcc_v1.rs"]
pub mod rtcc_v1;
#[path = "../../peripherals/smu_v1.rs"]
pub mod smu_v1;
#[path = "../../peripherals/smu_ns_cfgns_v1.rs"]
pub mod smu_ns_cfgns_v1;
#[path = "../../peripherals/syscfg_v8.rs"]
pub mod syscfg_v8;
#[path = "../../peripherals/syscfg_ns_cfgns_v8.rs"]
pub mod syscfg_ns_cfgns_v8;
#[path = "../../peripherals/timer_v0.rs"]
pub mod timer_v0;
#[path = "../../peripherals/timer_v0_w.rs"]
pub mod timer_v0_w;
#[path = "../../peripherals/ulfrco_v0.rs"]
pub mod ulfrco_v0;
#[path = "../../peripherals/usart_v0.rs"]
pub mod usart_v0;
#[path = "../../peripherals/wdog_v0.rs"]
pub mod wdog_v0;

// Version-neutral aliases for single-version kinds.
pub use buram_v0 as buram;
pub use burtc_v0 as burtc;
pub use cmu_v1 as cmu;
pub use cryptoacc_v1 as cryptoacc;
pub use cryptoacc_ns_pkctrl_v1 as cryptoacc_ns_pkctrl;
pub use cryptoacc_ns_rngctrl_v1 as cryptoacc_ns_rngctrl;
pub use dcdc_v0 as dcdc;
pub use devinfo_v1 as devinfo;
pub use dpll_v0 as dpll;
pub use emu_v8 as emu;
pub use euart_v0 as euart;
pub use fsrco_v1 as fsrco;
pub use gpcrc_v0 as gpcrc;
pub use gpio_v1 as gpio;
pub use hfrco_v1 as hfrco;
pub use hfxo_v2 as hfxo;
pub use i2c_v0 as i2c;
pub use iadc_v1 as iadc;
pub use icache_v0 as icache;
pub use ldma_v0 as ldma;
pub use ldmaxbar_v1 as ldmaxbar;
pub use letimer_v0 as letimer;
pub use lfrco_v3 as lfrco;
pub use lfxo_v0 as lfxo;
pub use msc_v8 as msc;
pub use pdm_v0 as pdm;
pub use prortc_v1 as prortc;
pub use prs_v1 as prs;
pub use radioaes_v1 as radioaes;
pub use rtcc_v1 as rtcc;
pub use smu_v1 as smu;
pub use smu_ns_cfgns_v1 as smu_ns_cfgns;
pub use syscfg_v8 as syscfg;
pub use syscfg_ns_cfgns_v8 as syscfg_ns_cfgns;
pub use ulfrco_v0 as ulfrco;
pub use usart_v0 as usart;
pub use wdog_v0 as wdog;

/// Memory map (flash/RAM regions, from the CMSIS pdsc).
pub mod memory {
    pub const IROM1_BASE: usize = 0x00000000;
    pub const IROM1_SIZE: usize = 0x00080000;
    pub const IRAM1_BASE: usize = 0x20000000;
    pub const IRAM1_SIZE: usize = 0x00008000;
}

/// Typed peripheral instance constants.
///
/// The canonical unsuffixed name uses the non-secure address for a paired
/// TrustZone peripheral, and an explicit `_S` constant uses the secure SVD
/// address. Infix vendor names such as `_NS_HOST` remain unchanged.
pub const BURAM: crate::buram_v0::Buram = unsafe { crate::buram_v0::Buram::from_ptr(0x50080000 as *mut ()) };
pub const BURAM_S: crate::buram_v0::Buram = unsafe { crate::buram_v0::Buram::from_ptr(0x40080000 as *mut ()) };
pub const BURTC: crate::burtc_v0::Burtc = unsafe { crate::burtc_v0::Burtc::from_ptr(0x50064000 as *mut ()) };
pub const BURTC_S: crate::burtc_v0::Burtc = unsafe { crate::burtc_v0::Burtc::from_ptr(0x40064000 as *mut ()) };
pub const CMU: crate::cmu_v1::Cmu = unsafe { crate::cmu_v1::Cmu::from_ptr(0x50008000 as *mut ()) };
pub const CMU_S: crate::cmu_v1::Cmu = unsafe { crate::cmu_v1::Cmu::from_ptr(0x40008000 as *mut ()) };
pub const CRYPTOACC: crate::cryptoacc_v1::Cryptoacc = unsafe { crate::cryptoacc_v1::Cryptoacc::from_ptr(0x5C020000 as *mut ()) };
pub const CRYPTOACC_S: crate::cryptoacc_v1::Cryptoacc = unsafe { crate::cryptoacc_v1::Cryptoacc::from_ptr(0x4C020000 as *mut ()) };
pub const CRYPTOACC_NS_PKCTRL: crate::cryptoacc_ns_pkctrl_v1::CryptoaccNsPkctrl = unsafe { crate::cryptoacc_ns_pkctrl_v1::CryptoaccNsPkctrl::from_ptr(0x5C022000 as *mut ()) };
pub const CRYPTOACC_S_PKCTRL: crate::cryptoacc_ns_pkctrl_v1::CryptoaccNsPkctrl = unsafe { crate::cryptoacc_ns_pkctrl_v1::CryptoaccNsPkctrl::from_ptr(0x4C022000 as *mut ()) };
pub const CRYPTOACC_NS_RNGCTRL: crate::cryptoacc_ns_rngctrl_v1::CryptoaccNsRngctrl = unsafe { crate::cryptoacc_ns_rngctrl_v1::CryptoaccNsRngctrl::from_ptr(0x5C021000 as *mut ()) };
pub const CRYPTOACC_S_RNGCTRL: crate::cryptoacc_ns_rngctrl_v1::CryptoaccNsRngctrl = unsafe { crate::cryptoacc_ns_rngctrl_v1::CryptoaccNsRngctrl::from_ptr(0x4C021000 as *mut ()) };
pub const DCDC: crate::dcdc_v0::Dcdc = unsafe { crate::dcdc_v0::Dcdc::from_ptr(0x50094000 as *mut ()) };
pub const DCDC_S: crate::dcdc_v0::Dcdc = unsafe { crate::dcdc_v0::Dcdc::from_ptr(0x40094000 as *mut ()) };
pub const DEVINFO: crate::devinfo_v1::Devinfo = unsafe { crate::devinfo_v1::Devinfo::from_ptr(0x0FE08000 as *mut ()) };
pub const DPLL0: crate::dpll_v0::Dpll = unsafe { crate::dpll_v0::Dpll::from_ptr(0x5001C000 as *mut ()) };
pub const DPLL0_S: crate::dpll_v0::Dpll = unsafe { crate::dpll_v0::Dpll::from_ptr(0x4001C000 as *mut ()) };
pub const EMU: crate::emu_v8::Emu = unsafe { crate::emu_v8::Emu::from_ptr(0x50004000 as *mut ()) };
pub const EMU_S: crate::emu_v8::Emu = unsafe { crate::emu_v8::Emu::from_ptr(0x40004000 as *mut ()) };
pub const EUART0: crate::euart_v0::Euart = unsafe { crate::euart_v0::Euart::from_ptr(0x5A030000 as *mut ()) };
pub const EUART0_S: crate::euart_v0::Euart = unsafe { crate::euart_v0::Euart::from_ptr(0x4A030000 as *mut ()) };
pub const FSRCO: crate::fsrco_v1::Fsrco = unsafe { crate::fsrco_v1::Fsrco::from_ptr(0x50018000 as *mut ()) };
pub const FSRCO_S: crate::fsrco_v1::Fsrco = unsafe { crate::fsrco_v1::Fsrco::from_ptr(0x40018000 as *mut ()) };
pub const GPCRC: crate::gpcrc_v0::Gpcrc = unsafe { crate::gpcrc_v0::Gpcrc::from_ptr(0x50088000 as *mut ()) };
pub const GPCRC_S: crate::gpcrc_v0::Gpcrc = unsafe { crate::gpcrc_v0::Gpcrc::from_ptr(0x40088000 as *mut ()) };
pub const GPIO: crate::gpio_v1::Gpio = unsafe { crate::gpio_v1::Gpio::from_ptr(0x5003C000 as *mut ()) };
pub const GPIO_S: crate::gpio_v1::Gpio = unsafe { crate::gpio_v1::Gpio::from_ptr(0x4003C000 as *mut ()) };
pub const HFRCO0: crate::hfrco_v1::Hfrco = unsafe { crate::hfrco_v1::Hfrco::from_ptr(0x50010000 as *mut ()) };
pub const HFRCO0_S: crate::hfrco_v1::Hfrco = unsafe { crate::hfrco_v1::Hfrco::from_ptr(0x40010000 as *mut ()) };
pub const HFXO0: crate::hfxo_v2::Hfxo = unsafe { crate::hfxo_v2::Hfxo::from_ptr(0x5000C000 as *mut ()) };
pub const HFXO0_S: crate::hfxo_v2::Hfxo = unsafe { crate::hfxo_v2::Hfxo::from_ptr(0x4000C000 as *mut ()) };
pub const I2C0: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x5A010000 as *mut ()) };
pub const I2C0_S: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x4A010000 as *mut ()) };
pub const I2C1: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x50068000 as *mut ()) };
pub const I2C1_S: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x40068000 as *mut ()) };
pub const IADC0: crate::iadc_v1::Iadc = unsafe { crate::iadc_v1::Iadc::from_ptr(0x5A004000 as *mut ()) };
pub const IADC0_S: crate::iadc_v1::Iadc = unsafe { crate::iadc_v1::Iadc::from_ptr(0x4A004000 as *mut ()) };
pub const ICACHE0: crate::icache_v0::Icache = unsafe { crate::icache_v0::Icache::from_ptr(0x50034000 as *mut ()) };
pub const ICACHE0_S: crate::icache_v0::Icache = unsafe { crate::icache_v0::Icache::from_ptr(0x40034000 as *mut ()) };
pub const LDMA: crate::ldma_v0::Ldma = unsafe { crate::ldma_v0::Ldma::from_ptr(0x50040000 as *mut ()) };
pub const LDMA_S: crate::ldma_v0::Ldma = unsafe { crate::ldma_v0::Ldma::from_ptr(0x40040000 as *mut ()) };
pub const LDMAXBAR: crate::ldmaxbar_v1::Ldmaxbar = unsafe { crate::ldmaxbar_v1::Ldmaxbar::from_ptr(0x50044000 as *mut ()) };
pub const LDMAXBAR_S: crate::ldmaxbar_v1::Ldmaxbar = unsafe { crate::ldmaxbar_v1::Ldmaxbar::from_ptr(0x40044000 as *mut ()) };
pub const LETIMER0: crate::letimer_v0::Letimer = unsafe { crate::letimer_v0::Letimer::from_ptr(0x5A000000 as *mut ()) };
pub const LETIMER0_S: crate::letimer_v0::Letimer = unsafe { crate::letimer_v0::Letimer::from_ptr(0x4A000000 as *mut ()) };
pub const LFRCO: crate::lfrco_v3::Lfrco = unsafe { crate::lfrco_v3::Lfrco::from_ptr(0x50024000 as *mut ()) };
pub const LFRCO_S: crate::lfrco_v3::Lfrco = unsafe { crate::lfrco_v3::Lfrco::from_ptr(0x40024000 as *mut ()) };
pub const LFXO: crate::lfxo_v0::Lfxo = unsafe { crate::lfxo_v0::Lfxo::from_ptr(0x50020000 as *mut ()) };
pub const LFXO_S: crate::lfxo_v0::Lfxo = unsafe { crate::lfxo_v0::Lfxo::from_ptr(0x40020000 as *mut ()) };
pub const MSC: crate::msc_v8::Msc = unsafe { crate::msc_v8::Msc::from_ptr(0x50030000 as *mut ()) };
pub const MSC_S: crate::msc_v8::Msc = unsafe { crate::msc_v8::Msc::from_ptr(0x40030000 as *mut ()) };
pub const PDM: crate::pdm_v0::Pdm = unsafe { crate::pdm_v0::Pdm::from_ptr(0x50098000 as *mut ()) };
pub const PDM_S: crate::pdm_v0::Pdm = unsafe { crate::pdm_v0::Pdm::from_ptr(0x40098000 as *mut ()) };
pub const PRORTC: crate::prortc_v1::Prortc = unsafe { crate::prortc_v1::Prortc::from_ptr(0xB8000000 as *mut ()) };
pub const PRORTC_S: crate::prortc_v1::Prortc = unsafe { crate::prortc_v1::Prortc::from_ptr(0xA8000000 as *mut ()) };
pub const PRS: crate::prs_v1::Prs = unsafe { crate::prs_v1::Prs::from_ptr(0x50038000 as *mut ()) };
pub const PRS_S: crate::prs_v1::Prs = unsafe { crate::prs_v1::Prs::from_ptr(0x40038000 as *mut ()) };
pub const RADIOAES: crate::radioaes_v1::Radioaes = unsafe { crate::radioaes_v1::Radioaes::from_ptr(0x54000000 as *mut ()) };
pub const RADIOAES_S: crate::radioaes_v1::Radioaes = unsafe { crate::radioaes_v1::Radioaes::from_ptr(0x44000000 as *mut ()) };
pub const RTCC: crate::rtcc_v1::Rtcc = unsafe { crate::rtcc_v1::Rtcc::from_ptr(0x58000000 as *mut ()) };
pub const RTCC_S: crate::rtcc_v1::Rtcc = unsafe { crate::rtcc_v1::Rtcc::from_ptr(0x48000000 as *mut ()) };
pub const SMU: crate::smu_v1::Smu = unsafe { crate::smu_v1::Smu::from_ptr(0x54008000 as *mut ()) };
pub const SMU_S: crate::smu_v1::Smu = unsafe { crate::smu_v1::Smu::from_ptr(0x44008000 as *mut ()) };
pub const SMU_NS_CFGNS: crate::smu_ns_cfgns_v1::SmuNsCfgns = unsafe { crate::smu_ns_cfgns_v1::SmuNsCfgns::from_ptr(0x5400C000 as *mut ()) };
pub const SMU_S_CFGNS: crate::smu_ns_cfgns_v1::SmuNsCfgns = unsafe { crate::smu_ns_cfgns_v1::SmuNsCfgns::from_ptr(0x4400C000 as *mut ()) };
pub const SYSCFG: crate::syscfg_v8::Syscfg = unsafe { crate::syscfg_v8::Syscfg::from_ptr(0x5007C000 as *mut ()) };
pub const SYSCFG_S: crate::syscfg_v8::Syscfg = unsafe { crate::syscfg_v8::Syscfg::from_ptr(0x4007C000 as *mut ()) };
pub const SYSCFG_NS_CFGNS: crate::syscfg_ns_cfgns_v8::SyscfgNsCfgns = unsafe { crate::syscfg_ns_cfgns_v8::SyscfgNsCfgns::from_ptr(0x50078000 as *mut ()) };
pub const SYSCFG_S_CFGNS: crate::syscfg_ns_cfgns_v8::SyscfgNsCfgns = unsafe { crate::syscfg_ns_cfgns_v8::SyscfgNsCfgns::from_ptr(0x40078000 as *mut ()) };
pub const TIMER0: crate::timer_v0_w::Timer = unsafe { crate::timer_v0_w::Timer::from_ptr(0x50048000 as *mut ()) };
pub const TIMER0_S: crate::timer_v0_w::Timer = unsafe { crate::timer_v0_w::Timer::from_ptr(0x40048000 as *mut ()) };
pub const TIMER1: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x5004C000 as *mut ()) };
pub const TIMER1_S: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x4004C000 as *mut ()) };
pub const TIMER2: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x50050000 as *mut ()) };
pub const TIMER2_S: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x40050000 as *mut ()) };
pub const TIMER3: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x50054000 as *mut ()) };
pub const TIMER3_S: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x40054000 as *mut ()) };
pub const TIMER4: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x50058000 as *mut ()) };
pub const TIMER4_S: crate::timer_v0::Timer = unsafe { crate::timer_v0::Timer::from_ptr(0x40058000 as *mut ()) };
pub const ULFRCO: crate::ulfrco_v0::Ulfrco = unsafe { crate::ulfrco_v0::Ulfrco::from_ptr(0x50028000 as *mut ()) };
pub const ULFRCO_S: crate::ulfrco_v0::Ulfrco = unsafe { crate::ulfrco_v0::Ulfrco::from_ptr(0x40028000 as *mut ()) };
pub const USART0: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x5005C000 as *mut ()) };
pub const USART0_S: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x4005C000 as *mut ()) };
pub const USART1: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x50060000 as *mut ()) };
pub const USART1_S: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x40060000 as *mut ()) };
pub const WDOG0: crate::wdog_v0::Wdog = unsafe { crate::wdog_v0::Wdog::from_ptr(0x5A018000 as *mut ()) };
pub const WDOG0_S: crate::wdog_v0::Wdog = unsafe { crate::wdog_v0::Wdog::from_ptr(0x4A018000 as *mut ()) };

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
    CRYPTOACC = 0,
    TRNG = 1,
    PKE = 2,
    SMU_SECURE = 3,
    SMU_S_PRIVILEGED = 4,
    SMU_NS_PRIVILEGED = 5,
    EMU = 6,
    TIMER0 = 7,
    TIMER1 = 8,
    TIMER2 = 9,
    TIMER3 = 10,
    TIMER4 = 11,
    RTCC = 12,
    USART0_RX = 13,
    USART0_TX = 14,
    USART1_RX = 15,
    USART1_TX = 16,
    ICACHE0 = 17,
    BURTC = 18,
    LETIMER0 = 19,
    SYSCFG = 20,
    LDMA = 21,
    LFXO = 22,
    LFRCO = 23,
    ULFRCO = 24,
    GPIO_ODD = 25,
    GPIO_EVEN = 26,
    I2C0 = 27,
    I2C1 = 28,
    EMUDG = 29,
    EMUSE = 30,
    AGC = 31,
    BUFC = 32,
    FRC_PRI = 33,
    FRC = 34,
    MODEM = 35,
    PROTIMER = 36,
    RAC_RSM = 37,
    RAC_SEQ = 38,
    RDMAILBOX = 39,
    RFSENSE = 40,
    PRORTC = 41,
    SYNTH = 42,
    WDOG0 = 43,
    HFXO0 = 44,
    HFRCO0 = 45,
    CMU = 46,
    AES = 47,
    IADC = 48,
    MSC = 49,
    DPLL0 = 50,
    PDM = 51,
    SW0 = 52,
    SW1 = 53,
    SW2 = 54,
    SW3 = 55,
    KERNEL0 = 56,
    KERNEL1 = 57,
    M33CTI0 = 58,
    M33CTI1 = 59,
    EMUEFP = 60,
    DCDC = 61,
    EUART0_RX = 62,
    EUART0_TX = 63,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 { self as u16 }
}

#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn CRYPTOACC();
        fn TRNG();
        fn PKE();
        fn SMU_SECURE();
        fn SMU_S_PRIVILEGED();
        fn SMU_NS_PRIVILEGED();
        fn EMU();
        fn TIMER0();
        fn TIMER1();
        fn TIMER2();
        fn TIMER3();
        fn TIMER4();
        fn RTCC();
        fn USART0_RX();
        fn USART0_TX();
        fn USART1_RX();
        fn USART1_TX();
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
        fn EMUSE();
        fn AGC();
        fn BUFC();
        fn FRC_PRI();
        fn FRC();
        fn MODEM();
        fn PROTIMER();
        fn RAC_RSM();
        fn RAC_SEQ();
        fn RDMAILBOX();
        fn RFSENSE();
        fn PRORTC();
        fn SYNTH();
        fn WDOG0();
        fn HFXO0();
        fn HFRCO0();
        fn CMU();
        fn AES();
        fn IADC();
        fn MSC();
        fn DPLL0();
        fn PDM();
        fn SW0();
        fn SW1();
        fn SW2();
        fn SW3();
        fn KERNEL0();
        fn KERNEL1();
        fn M33CTI0();
        fn M33CTI1();
        fn EMUEFP();
        fn DCDC();
        fn EUART0_RX();
        fn EUART0_TX();
    }

    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }

    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 64] = [
        Vector { _handler: CRYPTOACC },
        Vector { _handler: TRNG },
        Vector { _handler: PKE },
        Vector { _handler: SMU_SECURE },
        Vector { _handler: SMU_S_PRIVILEGED },
        Vector { _handler: SMU_NS_PRIVILEGED },
        Vector { _handler: EMU },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: TIMER3 },
        Vector { _handler: TIMER4 },
        Vector { _handler: RTCC },
        Vector { _handler: USART0_RX },
        Vector { _handler: USART0_TX },
        Vector { _handler: USART1_RX },
        Vector { _handler: USART1_TX },
        Vector { _handler: ICACHE0 },
        Vector { _handler: BURTC },
        Vector { _handler: LETIMER0 },
        Vector { _handler: SYSCFG },
        Vector { _handler: LDMA },
        Vector { _handler: LFXO },
        Vector { _handler: LFRCO },
        Vector { _handler: ULFRCO },
        Vector { _handler: GPIO_ODD },
        Vector { _handler: GPIO_EVEN },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: EMUDG },
        Vector { _handler: EMUSE },
        Vector { _handler: AGC },
        Vector { _handler: BUFC },
        Vector { _handler: FRC_PRI },
        Vector { _handler: FRC },
        Vector { _handler: MODEM },
        Vector { _handler: PROTIMER },
        Vector { _handler: RAC_RSM },
        Vector { _handler: RAC_SEQ },
        Vector { _handler: RDMAILBOX },
        Vector { _handler: RFSENSE },
        Vector { _handler: PRORTC },
        Vector { _handler: SYNTH },
        Vector { _handler: WDOG0 },
        Vector { _handler: HFXO0 },
        Vector { _handler: HFRCO0 },
        Vector { _handler: CMU },
        Vector { _handler: AES },
        Vector { _handler: IADC },
        Vector { _handler: MSC },
        Vector { _handler: DPLL0 },
        Vector { _handler: PDM },
        Vector { _handler: SW0 },
        Vector { _handler: SW1 },
        Vector { _handler: SW2 },
        Vector { _handler: SW3 },
        Vector { _handler: KERNEL0 },
        Vector { _handler: KERNEL1 },
        Vector { _handler: M33CTI0 },
        Vector { _handler: M33CTI1 },
        Vector { _handler: EMUEFP },
        Vector { _handler: DCDC },
        Vector { _handler: EUART0_RX },
        Vector { _handler: EUART0_TX },
    ];
}

/// Number available in the NVIC for configuring priority.
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
