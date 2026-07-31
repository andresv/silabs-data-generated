// Per-chip PAC content: peripheral module decls, typed peripheral
// consts, interrupt enum + cortex-m-rt glue, memory map.
// Generated for EFR32MG26B510F3200IM68.
//
// This file is `include!`d at the metapac crate root by `lib.rs`
// (selected via the `SILABS_METAPAC_PAC_PATH` env var emitted from
// `build.rs`). Mirrors `stm32-metapac`'s `chips/<chip>/pac.rs`
// layout — `#[path]` resolves relative to *this* file, so the
// `../../peripherals/...` paths below reach the shared chiptool
// peripheral modules under `src/peripherals/`.

// Chiptool peripheral modules (shared register/field types).
#[path = "../../peripherals/acmp_v2.rs"]
pub mod acmp_v2;
#[path = "../../peripherals/amuxcp_v2.rs"]
pub mod amuxcp_v2;
#[path = "../../peripherals/buram_v0.rs"]
pub mod buram_v0;
#[path = "../../peripherals/burtc_v1.rs"]
pub mod burtc_v1;
#[path = "../../peripherals/cmu_v7.rs"]
pub mod cmu_v7;
#[path = "../../peripherals/dcdc_v2.rs"]
pub mod dcdc_v2;
#[path = "../../peripherals/devinfo_v0_mg26.rs"]
pub mod devinfo_v0_mg26;
#[path = "../../peripherals/dmem_v2.rs"]
pub mod dmem_v2;
#[path = "../../peripherals/dpll_v1.rs"]
pub mod dpll_v1;
#[path = "../../peripherals/emu_v3.rs"]
pub mod emu_v3;
#[path = "../../peripherals/eusart_v2.rs"]
pub mod eusart_v2;
#[path = "../../peripherals/eusart_v2_lf.rs"]
pub mod eusart_v2_lf;
#[path = "../../peripherals/fsrco_v0.rs"]
pub mod fsrco_v0;
#[path = "../../peripherals/gpcrc_v0.rs"]
pub mod gpcrc_v0;
#[path = "../../peripherals/gpio_v7.rs"]
pub mod gpio_v7;
#[path = "../../peripherals/hfrco_v2.rs"]
pub mod hfrco_v2;
#[path = "../../peripherals/hfrcoem_v2.rs"]
pub mod hfrcoem_v2;
#[path = "../../peripherals/hfxo_v3.rs"]
pub mod hfxo_v3;
#[path = "../../peripherals/hostmailbox_v0.rs"]
pub mod hostmailbox_v0;
#[path = "../../peripherals/i2c_v0.rs"]
pub mod i2c_v0;
#[path = "../../peripherals/iadc_v3_ha.rs"]
pub mod iadc_v3_ha;
#[path = "../../peripherals/icache_v0.rs"]
pub mod icache_v0;
#[path = "../../peripherals/keyscan_v1.rs"]
pub mod keyscan_v1;
#[path = "../../peripherals/lcd_v3.rs"]
pub mod lcd_v3;
#[path = "../../peripherals/lcdrf_v0.rs"]
pub mod lcdrf_v0;
#[path = "../../peripherals/ldma_v0.rs"]
pub mod ldma_v0;
#[path = "../../peripherals/ldmaxbar_v7.rs"]
pub mod ldmaxbar_v7;
#[path = "../../peripherals/letimer_v1.rs"]
pub mod letimer_v1;
#[path = "../../peripherals/lfrco_v2.rs"]
pub mod lfrco_v2;
#[path = "../../peripherals/lfxo_v1.rs"]
pub mod lfxo_v1;
#[path = "../../peripherals/msc_v9.rs"]
pub mod msc_v9;
#[path = "../../peripherals/mvp_v2.rs"]
pub mod mvp_v2;
#[path = "../../peripherals/pcnt_v1.rs"]
pub mod pcnt_v1;
#[path = "../../peripherals/prs_v6.rs"]
pub mod prs_v6;
#[path = "../../peripherals/radioaes_v1.rs"]
pub mod radioaes_v1;
#[path = "../../peripherals/semailbox_ns_host_v1.rs"]
pub mod semailbox_ns_host_v1;
#[path = "../../peripherals/smu_v7_mvp.rs"]
pub mod smu_v7_mvp;
#[path = "../../peripherals/smu_ns_cfgns_v7_mvp.rs"]
pub mod smu_ns_cfgns_v7_mvp;
#[path = "../../peripherals/syscfg_v9_mvp.rs"]
pub mod syscfg_v9_mvp;
#[path = "../../peripherals/syscfg_ns_cfgns_v9.rs"]
pub mod syscfg_ns_cfgns_v9;
#[path = "../../peripherals/sysrtc_v1.rs"]
pub mod sysrtc_v1;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer_v1;
#[path = "../../peripherals/timer_v1_w.rs"]
pub mod timer_v1_w;
#[path = "../../peripherals/ulfrco_v1.rs"]
pub mod ulfrco_v1;
#[path = "../../peripherals/usart_v0.rs"]
pub mod usart_v0;
#[path = "../../peripherals/vdac_v2.rs"]
pub mod vdac_v2;
#[path = "../../peripherals/wdog_v1.rs"]
pub mod wdog_v1;

// Version-neutral aliases for single-version kinds.
pub use acmp_v2 as acmp;
pub use amuxcp_v2 as amuxcp;
pub use buram_v0 as buram;
pub use burtc_v1 as burtc;
pub use cmu_v7 as cmu;
pub use dcdc_v2 as dcdc;
pub use devinfo_v0_mg26 as devinfo;
pub use dmem_v2 as dmem;
pub use dpll_v1 as dpll;
pub use emu_v3 as emu;
pub use fsrco_v0 as fsrco;
pub use gpcrc_v0 as gpcrc;
pub use gpio_v7 as gpio;
pub use hfrco_v2 as hfrco;
pub use hfrcoem_v2 as hfrcoem;
pub use hfxo_v3 as hfxo;
pub use hostmailbox_v0 as hostmailbox;
pub use i2c_v0 as i2c;
pub use iadc_v3_ha as iadc;
pub use icache_v0 as icache;
pub use keyscan_v1 as keyscan;
pub use lcd_v3 as lcd;
pub use lcdrf_v0 as lcdrf;
pub use ldma_v0 as ldma;
pub use ldmaxbar_v7 as ldmaxbar;
pub use letimer_v1 as letimer;
pub use lfrco_v2 as lfrco;
pub use lfxo_v1 as lfxo;
pub use msc_v9 as msc;
pub use mvp_v2 as mvp;
pub use pcnt_v1 as pcnt;
pub use prs_v6 as prs;
pub use radioaes_v1 as radioaes;
pub use semailbox_ns_host_v1 as semailbox_ns_host;
pub use smu_v7_mvp as smu;
pub use smu_ns_cfgns_v7_mvp as smu_ns_cfgns;
pub use syscfg_v9_mvp as syscfg;
pub use syscfg_ns_cfgns_v9 as syscfg_ns_cfgns;
pub use sysrtc_v1 as sysrtc;
pub use ulfrco_v1 as ulfrco;
pub use usart_v0 as usart;
pub use vdac_v2 as vdac;
pub use wdog_v1 as wdog;

/// Memory map (flash/RAM regions, from the CMSIS pdsc).
pub mod memory {
    pub const IROM1_BASE: usize = 0x08000000;
    pub const IROM1_SIZE: usize = 0x00320000;
    pub const IRAM1_BASE: usize = 0x20000000;
    pub const IRAM1_SIZE: usize = 0x00080000;
}

/// Typed peripheral instance constants.
///
/// The canonical unsuffixed name uses the non-secure address for a paired
/// TrustZone peripheral, and an explicit `_S` constant uses the secure SVD
/// address. Infix vendor names such as `_NS_HOST` remain unchanged.
pub const ACMP0: crate::acmp_v2::Acmp = unsafe { crate::acmp_v2::Acmp::from_ptr(0x59008000 as *mut ()) };
pub const ACMP0_S: crate::acmp_v2::Acmp = unsafe { crate::acmp_v2::Acmp::from_ptr(0x49008000 as *mut ()) };
pub const ACMP1: crate::acmp_v2::Acmp = unsafe { crate::acmp_v2::Acmp::from_ptr(0x5900C000 as *mut ()) };
pub const ACMP1_S: crate::acmp_v2::Acmp = unsafe { crate::acmp_v2::Acmp::from_ptr(0x4900C000 as *mut ()) };
pub const AMUXCP0: crate::amuxcp_v2::Amuxcp = unsafe { crate::amuxcp_v2::Amuxcp::from_ptr(0x59020000 as *mut ()) };
pub const AMUXCP0_S: crate::amuxcp_v2::Amuxcp = unsafe { crate::amuxcp_v2::Amuxcp::from_ptr(0x49020000 as *mut ()) };
pub const BURAM: crate::buram_v0::Buram = unsafe { crate::buram_v0::Buram::from_ptr(0x50084000 as *mut ()) };
pub const BURAM_S: crate::buram_v0::Buram = unsafe { crate::buram_v0::Buram::from_ptr(0x40084000 as *mut ()) };
pub const BURTC: crate::burtc_v1::Burtc = unsafe { crate::burtc_v1::Burtc::from_ptr(0x5000C000 as *mut ()) };
pub const BURTC_S: crate::burtc_v1::Burtc = unsafe { crate::burtc_v1::Burtc::from_ptr(0x4000C000 as *mut ()) };
pub const CMU: crate::cmu_v7::Cmu = unsafe { crate::cmu_v7::Cmu::from_ptr(0x50008000 as *mut ()) };
pub const CMU_S: crate::cmu_v7::Cmu = unsafe { crate::cmu_v7::Cmu::from_ptr(0x40008000 as *mut ()) };
pub const DCDC: crate::dcdc_v2::Dcdc = unsafe { crate::dcdc_v2::Dcdc::from_ptr(0x50098000 as *mut ()) };
pub const DCDC_S: crate::dcdc_v2::Dcdc = unsafe { crate::dcdc_v2::Dcdc::from_ptr(0x40098000 as *mut ()) };
pub const DEVINFO: crate::devinfo_v0_mg26::Devinfo = unsafe { crate::devinfo_v0_mg26::Devinfo::from_ptr(0x0FE08000 as *mut ()) };
pub const DMEM0: crate::dmem_v2::Dmem = unsafe { crate::dmem_v2::Dmem::from_ptr(0x50074000 as *mut ()) };
pub const DMEM0_S: crate::dmem_v2::Dmem = unsafe { crate::dmem_v2::Dmem::from_ptr(0x40074000 as *mut ()) };
pub const DMEM1: crate::dmem_v2::Dmem = unsafe { crate::dmem_v2::Dmem::from_ptr(0x50078000 as *mut ()) };
pub const DMEM1_S: crate::dmem_v2::Dmem = unsafe { crate::dmem_v2::Dmem::from_ptr(0x40078000 as *mut ()) };
pub const DPLL0: crate::dpll_v1::Dpll = unsafe { crate::dpll_v1::Dpll::from_ptr(0x5001C000 as *mut ()) };
pub const DPLL0_S: crate::dpll_v1::Dpll = unsafe { crate::dpll_v1::Dpll::from_ptr(0x4001C000 as *mut ()) };
pub const EMU: crate::emu_v3::Emu = unsafe { crate::emu_v3::Emu::from_ptr(0x50004000 as *mut ()) };
pub const EMU_S: crate::emu_v3::Emu = unsafe { crate::emu_v3::Emu::from_ptr(0x40004000 as *mut ()) };
pub const EUSART0: crate::eusart_v2_lf::Eusart = unsafe { crate::eusart_v2_lf::Eusart::from_ptr(0x5B010000 as *mut ()) };
pub const EUSART0_S: crate::eusart_v2_lf::Eusart = unsafe { crate::eusart_v2_lf::Eusart::from_ptr(0x4B010000 as *mut ()) };
pub const EUSART1: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x5008C000 as *mut ()) };
pub const EUSART1_S: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x4008C000 as *mut ()) };
pub const EUSART2: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x50090000 as *mut ()) };
pub const EUSART2_S: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x40090000 as *mut ()) };
pub const EUSART3: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x50094000 as *mut ()) };
pub const EUSART3_S: crate::eusart_v2::Eusart = unsafe { crate::eusart_v2::Eusart::from_ptr(0x40094000 as *mut ()) };
pub const FSRCO: crate::fsrco_v0::Fsrco = unsafe { crate::fsrco_v0::Fsrco::from_ptr(0x50018000 as *mut ()) };
pub const FSRCO_S: crate::fsrco_v0::Fsrco = unsafe { crate::fsrco_v0::Fsrco::from_ptr(0x40018000 as *mut ()) };
pub const GPCRC: crate::gpcrc_v0::Gpcrc = unsafe { crate::gpcrc_v0::Gpcrc::from_ptr(0x50088000 as *mut ()) };
pub const GPCRC_S: crate::gpcrc_v0::Gpcrc = unsafe { crate::gpcrc_v0::Gpcrc::from_ptr(0x40088000 as *mut ()) };
pub const GPIO: crate::gpio_v7::Gpio = unsafe { crate::gpio_v7::Gpio::from_ptr(0x5003C000 as *mut ()) };
pub const GPIO_S: crate::gpio_v7::Gpio = unsafe { crate::gpio_v7::Gpio::from_ptr(0x4003C000 as *mut ()) };
pub const HFRCO0: crate::hfrco_v2::Hfrco = unsafe { crate::hfrco_v2::Hfrco::from_ptr(0x50010000 as *mut ()) };
pub const HFRCO0_S: crate::hfrco_v2::Hfrco = unsafe { crate::hfrco_v2::Hfrco::from_ptr(0x40010000 as *mut ()) };
pub const HFRCOEM23: crate::hfrcoem_v2::Hfrcoem = unsafe { crate::hfrcoem_v2::Hfrcoem::from_ptr(0x5A000000 as *mut ()) };
pub const HFRCOEM23_S: crate::hfrcoem_v2::Hfrcoem = unsafe { crate::hfrcoem_v2::Hfrcoem::from_ptr(0x4A000000 as *mut ()) };
pub const HFXO0: crate::hfxo_v3::Hfxo = unsafe { crate::hfxo_v3::Hfxo::from_ptr(0x5A004000 as *mut ()) };
pub const HFXO0_S: crate::hfxo_v3::Hfxo = unsafe { crate::hfxo_v3::Hfxo::from_ptr(0x4A004000 as *mut ()) };
pub const HOSTMAILBOX: crate::hostmailbox_v0::Hostmailbox = unsafe { crate::hostmailbox_v0::Hostmailbox::from_ptr(0x5009C000 as *mut ()) };
pub const HOSTMAILBOX_S: crate::hostmailbox_v0::Hostmailbox = unsafe { crate::hostmailbox_v0::Hostmailbox::from_ptr(0x4009C000 as *mut ()) };
pub const I2C0: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x5B000000 as *mut ()) };
pub const I2C0_S: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x4B000000 as *mut ()) };
pub const I2C1: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x500B0000 as *mut ()) };
pub const I2C1_S: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x400B0000 as *mut ()) };
pub const I2C2: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x500B4000 as *mut ()) };
pub const I2C2_S: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x400B4000 as *mut ()) };
pub const I2C3: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x500B8000 as *mut ()) };
pub const I2C3_S: crate::i2c_v0::I2c = unsafe { crate::i2c_v0::I2c::from_ptr(0x400B8000 as *mut ()) };
pub const IADC0: crate::iadc_v3_ha::Iadc = unsafe { crate::iadc_v3_ha::Iadc::from_ptr(0x59004000 as *mut ()) };
pub const IADC0_S: crate::iadc_v3_ha::Iadc = unsafe { crate::iadc_v3_ha::Iadc::from_ptr(0x49004000 as *mut ()) };
pub const ICACHE0: crate::icache_v0::Icache = unsafe { crate::icache_v0::Icache::from_ptr(0x50034000 as *mut ()) };
pub const ICACHE0_S: crate::icache_v0::Icache = unsafe { crate::icache_v0::Icache::from_ptr(0x40034000 as *mut ()) };
pub const KEYSCAN: crate::keyscan_v1::Keyscan = unsafe { crate::keyscan_v1::Keyscan::from_ptr(0x5002C000 as *mut ()) };
pub const KEYSCAN_S: crate::keyscan_v1::Keyscan = unsafe { crate::keyscan_v1::Keyscan::from_ptr(0x4002C000 as *mut ()) };
pub const LCD: crate::lcd_v3::Lcd = unsafe { crate::lcd_v3::Lcd::from_ptr(0x500BC000 as *mut ()) };
pub const LCD_S: crate::lcd_v3::Lcd = unsafe { crate::lcd_v3::Lcd::from_ptr(0x400BC000 as *mut ()) };
pub const LCDRF: crate::lcdrf_v0::Lcdrf = unsafe { crate::lcdrf_v0::Lcdrf::from_ptr(0x500C0000 as *mut ()) };
pub const LCDRF_S: crate::lcdrf_v0::Lcdrf = unsafe { crate::lcdrf_v0::Lcdrf::from_ptr(0x400C0000 as *mut ()) };
pub const LDMA: crate::ldma_v0::Ldma = unsafe { crate::ldma_v0::Ldma::from_ptr(0x50040000 as *mut ()) };
pub const LDMA_S: crate::ldma_v0::Ldma = unsafe { crate::ldma_v0::Ldma::from_ptr(0x40040000 as *mut ()) };
pub const LDMAXBAR: crate::ldmaxbar_v7::Ldmaxbar = unsafe { crate::ldmaxbar_v7::Ldmaxbar::from_ptr(0x50044000 as *mut ()) };
pub const LDMAXBAR_S: crate::ldmaxbar_v7::Ldmaxbar = unsafe { crate::ldmaxbar_v7::Ldmaxbar::from_ptr(0x40044000 as *mut ()) };
pub const LETIMER0: crate::letimer_v1::Letimer = unsafe { crate::letimer_v1::Letimer::from_ptr(0x59000000 as *mut ()) };
pub const LETIMER0_S: crate::letimer_v1::Letimer = unsafe { crate::letimer_v1::Letimer::from_ptr(0x49000000 as *mut ()) };
pub const LFRCO: crate::lfrco_v2::Lfrco = unsafe { crate::lfrco_v2::Lfrco::from_ptr(0x50024000 as *mut ()) };
pub const LFRCO_S: crate::lfrco_v2::Lfrco = unsafe { crate::lfrco_v2::Lfrco::from_ptr(0x40024000 as *mut ()) };
pub const LFXO: crate::lfxo_v1::Lfxo = unsafe { crate::lfxo_v1::Lfxo::from_ptr(0x50020000 as *mut ()) };
pub const LFXO_S: crate::lfxo_v1::Lfxo = unsafe { crate::lfxo_v1::Lfxo::from_ptr(0x40020000 as *mut ()) };
pub const MSC: crate::msc_v9::Msc = unsafe { crate::msc_v9::Msc::from_ptr(0x50030000 as *mut ()) };
pub const MSC_S: crate::msc_v9::Msc = unsafe { crate::msc_v9::Msc::from_ptr(0x40030000 as *mut ()) };
pub const MVP: crate::mvp_v2::Mvp = unsafe { crate::mvp_v2::Mvp::from_ptr(0x5D000000 as *mut ()) };
pub const MVP_S: crate::mvp_v2::Mvp = unsafe { crate::mvp_v2::Mvp::from_ptr(0x4D000000 as *mut ()) };
pub const PCNT0: crate::pcnt_v1::Pcnt = unsafe { crate::pcnt_v1::Pcnt::from_ptr(0x59030000 as *mut ()) };
pub const PCNT0_S: crate::pcnt_v1::Pcnt = unsafe { crate::pcnt_v1::Pcnt::from_ptr(0x49030000 as *mut ()) };
pub const PRS: crate::prs_v6::Prs = unsafe { crate::prs_v6::Prs::from_ptr(0x50038000 as *mut ()) };
pub const PRS_S: crate::prs_v6::Prs = unsafe { crate::prs_v6::Prs::from_ptr(0x40038000 as *mut ()) };
pub const RADIOAES: crate::radioaes_v1::Radioaes = unsafe { crate::radioaes_v1::Radioaes::from_ptr(0x54000000 as *mut ()) };
pub const RADIOAES_S: crate::radioaes_v1::Radioaes = unsafe { crate::radioaes_v1::Radioaes::from_ptr(0x44000000 as *mut ()) };
pub const SEMAILBOX_NS_HOST: crate::semailbox_ns_host_v1::SemailboxNsHost = unsafe { crate::semailbox_ns_host_v1::SemailboxNsHost::from_ptr(0x5C000000 as *mut ()) };
pub const SEMAILBOX_S_HOST: crate::semailbox_ns_host_v1::SemailboxNsHost = unsafe { crate::semailbox_ns_host_v1::SemailboxNsHost::from_ptr(0x4C000000 as *mut ()) };
pub const SMU: crate::smu_v7_mvp::Smu = unsafe { crate::smu_v7_mvp::Smu::from_ptr(0x54008000 as *mut ()) };
pub const SMU_S: crate::smu_v7_mvp::Smu = unsafe { crate::smu_v7_mvp::Smu::from_ptr(0x44008000 as *mut ()) };
pub const SMU_NS_CFGNS: crate::smu_ns_cfgns_v7_mvp::SmuNsCfgns = unsafe { crate::smu_ns_cfgns_v7_mvp::SmuNsCfgns::from_ptr(0x5400C000 as *mut ()) };
pub const SMU_S_CFGNS: crate::smu_ns_cfgns_v7_mvp::SmuNsCfgns = unsafe { crate::smu_ns_cfgns_v7_mvp::SmuNsCfgns::from_ptr(0x4400C000 as *mut ()) };
pub const SYSCFG: crate::syscfg_v9_mvp::Syscfg = unsafe { crate::syscfg_v9_mvp::Syscfg::from_ptr(0x50080000 as *mut ()) };
pub const SYSCFG_S: crate::syscfg_v9_mvp::Syscfg = unsafe { crate::syscfg_v9_mvp::Syscfg::from_ptr(0x40080000 as *mut ()) };
pub const SYSCFG_NS_CFGNS: crate::syscfg_ns_cfgns_v9::SyscfgNsCfgns = unsafe { crate::syscfg_ns_cfgns_v9::SyscfgNsCfgns::from_ptr(0x5007C000 as *mut ()) };
pub const SYSCFG_S_CFGNS: crate::syscfg_ns_cfgns_v9::SyscfgNsCfgns = unsafe { crate::syscfg_ns_cfgns_v9::SyscfgNsCfgns::from_ptr(0x4007C000 as *mut ()) };
pub const SYSRTC0: crate::sysrtc_v1::Sysrtc = unsafe { crate::sysrtc_v1::Sysrtc::from_ptr(0x500AC000 as *mut ()) };
pub const SYSRTC0_S: crate::sysrtc_v1::Sysrtc = unsafe { crate::sysrtc_v1::Sysrtc::from_ptr(0x400AC000 as *mut ()) };
pub const TIMER0: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x50048000 as *mut ()) };
pub const TIMER0_S: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x40048000 as *mut ()) };
pub const TIMER1: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x5004C000 as *mut ()) };
pub const TIMER1_S: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x4004C000 as *mut ()) };
pub const TIMER2: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50050000 as *mut ()) };
pub const TIMER2_S: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x40050000 as *mut ()) };
pub const TIMER3: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50054000 as *mut ()) };
pub const TIMER3_S: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x40054000 as *mut ()) };
pub const TIMER4: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50058000 as *mut ()) };
pub const TIMER4_S: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x40058000 as *mut ()) };
pub const TIMER5: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x5005C000 as *mut ()) };
pub const TIMER5_S: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x4005C000 as *mut ()) };
pub const TIMER6: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50060000 as *mut ()) };
pub const TIMER6_S: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x40060000 as *mut ()) };
pub const TIMER7: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x50064000 as *mut ()) };
pub const TIMER7_S: crate::timer_v1::Timer = unsafe { crate::timer_v1::Timer::from_ptr(0x40064000 as *mut ()) };
pub const TIMER8: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x50068000 as *mut ()) };
pub const TIMER8_S: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x40068000 as *mut ()) };
pub const TIMER9: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x5006C000 as *mut ()) };
pub const TIMER9_S: crate::timer_v1_w::Timer = unsafe { crate::timer_v1_w::Timer::from_ptr(0x4006C000 as *mut ()) };
pub const ULFRCO: crate::ulfrco_v1::Ulfrco = unsafe { crate::ulfrco_v1::Ulfrco::from_ptr(0x50028000 as *mut ()) };
pub const ULFRCO_S: crate::ulfrco_v1::Ulfrco = unsafe { crate::ulfrco_v1::Ulfrco::from_ptr(0x40028000 as *mut ()) };
pub const USART0: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x500A0000 as *mut ()) };
pub const USART0_S: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x400A0000 as *mut ()) };
pub const USART1: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x500A4000 as *mut ()) };
pub const USART1_S: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x400A4000 as *mut ()) };
pub const USART2: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x500A8000 as *mut ()) };
pub const USART2_S: crate::usart_v0::Usart = unsafe { crate::usart_v0::Usart::from_ptr(0x400A8000 as *mut ()) };
pub const VDAC0: crate::vdac_v2::Vdac = unsafe { crate::vdac_v2::Vdac::from_ptr(0x59024000 as *mut ()) };
pub const VDAC0_S: crate::vdac_v2::Vdac = unsafe { crate::vdac_v2::Vdac::from_ptr(0x49024000 as *mut ()) };
pub const VDAC1: crate::vdac_v2::Vdac = unsafe { crate::vdac_v2::Vdac::from_ptr(0x59028000 as *mut ()) };
pub const VDAC1_S: crate::vdac_v2::Vdac = unsafe { crate::vdac_v2::Vdac::from_ptr(0x49028000 as *mut ()) };
pub const WDOG0: crate::wdog_v1::Wdog = unsafe { crate::wdog_v1::Wdog::from_ptr(0x5B004000 as *mut ()) };
pub const WDOG0_S: crate::wdog_v1::Wdog = unsafe { crate::wdog_v1::Wdog::from_ptr(0x4B004000 as *mut ()) };
pub const WDOG1: crate::wdog_v1::Wdog = unsafe { crate::wdog_v1::Wdog::from_ptr(0x5B008000 as *mut ()) };
pub const WDOG1_S: crate::wdog_v1::Wdog = unsafe { crate::wdog_v1::Wdog::from_ptr(0x4B008000 as *mut ()) };

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
    TIMER4 = 8,
    TIMER5 = 9,
    TIMER6 = 10,
    TIMER7 = 11,
    TIMER8 = 12,
    TIMER9 = 13,
    USART0_RX = 14,
    USART0_TX = 15,
    USART1_RX = 16,
    USART1_TX = 17,
    USART2_RX = 18,
    USART2_TX = 19,
    EUSART0_RX = 20,
    EUSART0_TX = 21,
    EUSART1_RX = 22,
    EUSART1_TX = 23,
    EUSART2_RX = 24,
    EUSART2_TX = 25,
    EUSART3_RX = 26,
    EUSART3_TX = 27,
    MVP = 28,
    ICACHE0 = 29,
    BURTC = 30,
    LETIMER0 = 31,
    SYSCFG = 32,
    MPAHBRAM0 = 33,
    MPAHBRAM1 = 34,
    LDMA = 35,
    LFXO = 36,
    LFRCO = 37,
    ULFRCO = 38,
    GPIO_ODD = 39,
    GPIO_EVEN = 40,
    I2C0 = 41,
    I2C1 = 42,
    I2C2 = 43,
    I2C3 = 44,
    EMUDG = 45,
    AGC = 46,
    BUFC = 47,
    FRC_PRI = 48,
    FRC = 49,
    MODEM = 50,
    PROTIMER = 51,
    RAC_RSM = 52,
    RAC_SEQ = 53,
    HOSTMAILBOX = 54,
    SYNTH = 55,
    ACMP0 = 56,
    ACMP1 = 57,
    WDOG0 = 58,
    WDOG1 = 59,
    HFXO0 = 60,
    HFRCO0 = 61,
    HFRCOEM23 = 62,
    CMU = 63,
    AES = 64,
    IADC = 65,
    MSC = 66,
    DPLL0 = 67,
    EMUEFP = 68,
    DCDC = 69,
    PCNT0 = 70,
    SW0 = 71,
    SW1 = 72,
    SW2 = 73,
    SW3 = 74,
    KERNEL0 = 75,
    KERNEL1 = 76,
    M33CTI0 = 77,
    M33CTI1 = 78,
    FPUEXH = 79,
    SETAMPERHOST = 80,
    SEMBRX = 81,
    SEMBTX = 82,
    SYSRTC_APP = 83,
    SYSRTC_SEQ = 84,
    KEYSCAN = 85,
    RFECA0 = 86,
    RFECA1 = 87,
    VDAC0 = 88,
    VDAC1 = 89,
    AHB2AHB0 = 90,
    AHB2AHB1 = 91,
    LCD = 92,
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
        fn TIMER4();
        fn TIMER5();
        fn TIMER6();
        fn TIMER7();
        fn TIMER8();
        fn TIMER9();
        fn USART0_RX();
        fn USART0_TX();
        fn USART1_RX();
        fn USART1_TX();
        fn USART2_RX();
        fn USART2_TX();
        fn EUSART0_RX();
        fn EUSART0_TX();
        fn EUSART1_RX();
        fn EUSART1_TX();
        fn EUSART2_RX();
        fn EUSART2_TX();
        fn EUSART3_RX();
        fn EUSART3_TX();
        fn MVP();
        fn ICACHE0();
        fn BURTC();
        fn LETIMER0();
        fn SYSCFG();
        fn MPAHBRAM0();
        fn MPAHBRAM1();
        fn LDMA();
        fn LFXO();
        fn LFRCO();
        fn ULFRCO();
        fn GPIO_ODD();
        fn GPIO_EVEN();
        fn I2C0();
        fn I2C1();
        fn I2C2();
        fn I2C3();
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
        fn EMUEFP();
        fn DCDC();
        fn PCNT0();
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
        fn SYSRTC_APP();
        fn SYSRTC_SEQ();
        fn KEYSCAN();
        fn RFECA0();
        fn RFECA1();
        fn VDAC0();
        fn VDAC1();
        fn AHB2AHB0();
        fn AHB2AHB1();
        fn LCD();
    }

    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }

    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 93] = [
        Vector { _handler: SMU_SECURE },
        Vector { _handler: SMU_S_PRIVILEGED },
        Vector { _handler: SMU_NS_PRIVILEGED },
        Vector { _handler: EMU },
        Vector { _handler: TIMER0 },
        Vector { _handler: TIMER1 },
        Vector { _handler: TIMER2 },
        Vector { _handler: TIMER3 },
        Vector { _handler: TIMER4 },
        Vector { _handler: TIMER5 },
        Vector { _handler: TIMER6 },
        Vector { _handler: TIMER7 },
        Vector { _handler: TIMER8 },
        Vector { _handler: TIMER9 },
        Vector { _handler: USART0_RX },
        Vector { _handler: USART0_TX },
        Vector { _handler: USART1_RX },
        Vector { _handler: USART1_TX },
        Vector { _handler: USART2_RX },
        Vector { _handler: USART2_TX },
        Vector { _handler: EUSART0_RX },
        Vector { _handler: EUSART0_TX },
        Vector { _handler: EUSART1_RX },
        Vector { _handler: EUSART1_TX },
        Vector { _handler: EUSART2_RX },
        Vector { _handler: EUSART2_TX },
        Vector { _handler: EUSART3_RX },
        Vector { _handler: EUSART3_TX },
        Vector { _handler: MVP },
        Vector { _handler: ICACHE0 },
        Vector { _handler: BURTC },
        Vector { _handler: LETIMER0 },
        Vector { _handler: SYSCFG },
        Vector { _handler: MPAHBRAM0 },
        Vector { _handler: MPAHBRAM1 },
        Vector { _handler: LDMA },
        Vector { _handler: LFXO },
        Vector { _handler: LFRCO },
        Vector { _handler: ULFRCO },
        Vector { _handler: GPIO_ODD },
        Vector { _handler: GPIO_EVEN },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: I2C3 },
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
        Vector { _handler: EMUEFP },
        Vector { _handler: DCDC },
        Vector { _handler: PCNT0 },
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
        Vector { _handler: SYSRTC_APP },
        Vector { _handler: SYSRTC_SEQ },
        Vector { _handler: KEYSCAN },
        Vector { _handler: RFECA0 },
        Vector { _handler: RFECA1 },
        Vector { _handler: VDAC0 },
        Vector { _handler: VDAC1 },
        Vector { _handler: AHB2AHB0 },
        Vector { _handler: AHB2AHB1 },
        Vector { _handler: LCD },
    ];
}

/// Number available in the NVIC for configuring priority.
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
