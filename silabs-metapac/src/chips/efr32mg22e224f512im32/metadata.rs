// Per-chip iterable metadata. Generated for EFR32MG22E224F512IM32.
//
// Included from `pub mod metadata` in the metapac crate root;
// type names resolve to the surrounding module — see
// silabs-metapac-gen/res/metadata.rs.

pub static METADATA: Metadata = Metadata {
    name: "EFR32MG22E224F512IM32",
    core: "Cortex-M33",
    fpu: false,
    mpu: false,
    trustzone: false,
    series: Series::Series2(2),
    memory: &[
        MemoryRegion { name: "IROM1", address: 0x00000000, size: 0x00080000, access: "rx" },
        MemoryRegion { name: "IRAM1", address: 0x20000000, size: 0x00008000, access: "rwx" },
    ],
    peripherals: &[
        Peripheral { name: "BURAM", address: 0x50080000, kind: "buram", version: "v0", block: "BURAM" },
        Peripheral { name: "BURTC", address: 0x50064000, kind: "burtc", version: "v0", block: "BURTC" },
        Peripheral { name: "CMU", address: 0x50008000, kind: "cmu", version: "v1", block: "CMU" },
        Peripheral { name: "CRYPTOACC", address: 0x5C020000, kind: "cryptoacc", version: "v1", block: "CRYPTOACC" },
        Peripheral { name: "CRYPTOACC_NS_PKCTRL", address: 0x5C022000, kind: "cryptoacc_ns_pkctrl", version: "v1", block: "CRYPTOACC_NS_PKCTRL" },
        Peripheral { name: "CRYPTOACC_NS_RNGCTRL", address: 0x5C021000, kind: "cryptoacc_ns_rngctrl", version: "v1", block: "CRYPTOACC_NS_RNGCTRL" },
        Peripheral { name: "CRYPTOACC_S_PKCTRL", address: 0x4C022000, kind: "cryptoacc_s_pkctrl", version: "v1", block: "CRYPTOACC_S_PKCTRL" },
        Peripheral { name: "CRYPTOACC_S_RNGCTRL", address: 0x4C021000, kind: "cryptoacc_s_rngctrl", version: "v1", block: "CRYPTOACC_S_RNGCTRL" },
        Peripheral { name: "DCDC", address: 0x50094000, kind: "dcdc", version: "v0", block: "DCDC" },
        Peripheral { name: "DEVINFO", address: 0x0FE08000, kind: "devinfo", version: "v1", block: "DEVINFO" },
        Peripheral { name: "DPLL0", address: 0x5001C000, kind: "dpll", version: "v0", block: "DPLL" },
        Peripheral { name: "EMU", address: 0x50004000, kind: "emu", version: "v8", block: "EMU" },
        Peripheral { name: "EUART0", address: 0x5A030000, kind: "euart", version: "v0", block: "EUART" },
        Peripheral { name: "FSRCO", address: 0x50018000, kind: "fsrco", version: "v1", block: "FSRCO" },
        Peripheral { name: "GPCRC", address: 0x50088000, kind: "gpcrc", version: "v0", block: "GPCRC" },
        Peripheral { name: "GPIO", address: 0x5003C000, kind: "gpio", version: "v1", block: "GPIO" },
        Peripheral { name: "HFRCO0", address: 0x50010000, kind: "hfrco", version: "v1", block: "HFRCO" },
        Peripheral { name: "HFXO0", address: 0x5000C000, kind: "hfxo", version: "v2", block: "HFXO" },
        Peripheral { name: "I2C0", address: 0x5A010000, kind: "i2c", version: "v0", block: "I2C" },
        Peripheral { name: "I2C1", address: 0x50068000, kind: "i2c", version: "v0", block: "I2C" },
        Peripheral { name: "IADC0", address: 0x5A004000, kind: "iadc", version: "v1", block: "IADC" },
        Peripheral { name: "ICACHE0", address: 0x50034000, kind: "icache", version: "v0", block: "ICACHE" },
        Peripheral { name: "LDMA", address: 0x50040000, kind: "ldma", version: "v0", block: "LDMA" },
        Peripheral { name: "LDMAXBAR", address: 0x50044000, kind: "ldmaxbar", version: "v1", block: "LDMAXBAR" },
        Peripheral { name: "LETIMER0", address: 0x5A000000, kind: "letimer", version: "v0", block: "LETIMER" },
        Peripheral { name: "LFRCO", address: 0x50024000, kind: "lfrco", version: "v3", block: "LFRCO" },
        Peripheral { name: "LFXO", address: 0x50020000, kind: "lfxo", version: "v0", block: "LFXO" },
        Peripheral { name: "MSC", address: 0x50030000, kind: "msc", version: "v8", block: "MSC" },
        Peripheral { name: "PDM", address: 0x50098000, kind: "pdm", version: "v0", block: "PDM" },
        Peripheral { name: "PRORTC", address: 0xB8000000, kind: "prortc", version: "v1", block: "PRORTC" },
        Peripheral { name: "PRS", address: 0x50038000, kind: "prs", version: "v1", block: "PRS" },
        Peripheral { name: "RADIOAES", address: 0x54000000, kind: "radioaes", version: "v1", block: "RADIOAES" },
        Peripheral { name: "RTCC", address: 0x58000000, kind: "rtcc", version: "v1", block: "RTCC" },
        Peripheral { name: "SMU", address: 0x54008000, kind: "smu", version: "v1", block: "SMU" },
        Peripheral { name: "SMU_NS_CFGNS", address: 0x5400C000, kind: "smu_ns_cfgns", version: "v1", block: "SMU_NS_CFGNS" },
        Peripheral { name: "SMU_S_CFGNS", address: 0x4400C000, kind: "smu_s_cfgns", version: "v1", block: "SMU_S_CFGNS" },
        Peripheral { name: "SYSCFG", address: 0x5007C000, kind: "syscfg", version: "v8", block: "SYSCFG" },
        Peripheral { name: "SYSCFG_NS_CFGNS", address: 0x50078000, kind: "syscfg_ns_cfgns", version: "v8", block: "SYSCFG_NS_CFGNS" },
        Peripheral { name: "SYSCFG_S_CFGNS", address: 0x40078000, kind: "syscfg_s_cfgns", version: "v8", block: "SYSCFG_S_CFGNS" },
        Peripheral { name: "TIMER0", address: 0x50048000, kind: "timer", version: "v0_w", block: "TIMER" },
        Peripheral { name: "TIMER1", address: 0x5004C000, kind: "timer", version: "v0", block: "TIMER" },
        Peripheral { name: "TIMER2", address: 0x50050000, kind: "timer", version: "v0", block: "TIMER" },
        Peripheral { name: "TIMER3", address: 0x50054000, kind: "timer", version: "v0", block: "TIMER" },
        Peripheral { name: "TIMER4", address: 0x50058000, kind: "timer", version: "v0", block: "TIMER" },
        Peripheral { name: "ULFRCO", address: 0x50028000, kind: "ulfrco", version: "v0", block: "ULFRCO" },
        Peripheral { name: "USART0", address: 0x5005C000, kind: "usart", version: "v0", block: "USART" },
        Peripheral { name: "USART1", address: 0x50060000, kind: "usart", version: "v0", block: "USART" },
        Peripheral { name: "WDOG0", address: 0x5A018000, kind: "wdog", version: "v0", block: "WDOG" },
    ],
    interrupts: &[
        Interrupt { name: "CRYPTOACC", number: 0 },
        Interrupt { name: "TRNG", number: 1 },
        Interrupt { name: "PKE", number: 2 },
        Interrupt { name: "SMU_SECURE", number: 3 },
        Interrupt { name: "SMU_S_PRIVILEGED", number: 4 },
        Interrupt { name: "SMU_NS_PRIVILEGED", number: 5 },
        Interrupt { name: "EMU", number: 6 },
        Interrupt { name: "TIMER0", number: 7 },
        Interrupt { name: "TIMER1", number: 8 },
        Interrupt { name: "TIMER2", number: 9 },
        Interrupt { name: "TIMER3", number: 10 },
        Interrupt { name: "TIMER4", number: 11 },
        Interrupt { name: "RTCC", number: 12 },
        Interrupt { name: "USART0_RX", number: 13 },
        Interrupt { name: "USART0_TX", number: 14 },
        Interrupt { name: "USART1_RX", number: 15 },
        Interrupt { name: "USART1_TX", number: 16 },
        Interrupt { name: "ICACHE0", number: 17 },
        Interrupt { name: "BURTC", number: 18 },
        Interrupt { name: "LETIMER0", number: 19 },
        Interrupt { name: "SYSCFG", number: 20 },
        Interrupt { name: "LDMA", number: 21 },
        Interrupt { name: "LFXO", number: 22 },
        Interrupt { name: "LFRCO", number: 23 },
        Interrupt { name: "ULFRCO", number: 24 },
        Interrupt { name: "GPIO_ODD", number: 25 },
        Interrupt { name: "GPIO_EVEN", number: 26 },
        Interrupt { name: "I2C0", number: 27 },
        Interrupt { name: "I2C1", number: 28 },
        Interrupt { name: "EMUDG", number: 29 },
        Interrupt { name: "EMUSE", number: 30 },
        Interrupt { name: "AGC", number: 31 },
        Interrupt { name: "BUFC", number: 32 },
        Interrupt { name: "FRC_PRI", number: 33 },
        Interrupt { name: "FRC", number: 34 },
        Interrupt { name: "MODEM", number: 35 },
        Interrupt { name: "PROTIMER", number: 36 },
        Interrupt { name: "RAC_RSM", number: 37 },
        Interrupt { name: "RAC_SEQ", number: 38 },
        Interrupt { name: "RDMAILBOX", number: 39 },
        Interrupt { name: "RFSENSE", number: 40 },
        Interrupt { name: "PRORTC", number: 41 },
        Interrupt { name: "SYNTH", number: 42 },
        Interrupt { name: "WDOG0", number: 43 },
        Interrupt { name: "HFXO0", number: 44 },
        Interrupt { name: "HFRCO0", number: 45 },
        Interrupt { name: "CMU", number: 46 },
        Interrupt { name: "AES", number: 47 },
        Interrupt { name: "IADC", number: 48 },
        Interrupt { name: "MSC", number: 49 },
        Interrupt { name: "DPLL0", number: 50 },
        Interrupt { name: "PDM", number: 51 },
        Interrupt { name: "SW0", number: 52 },
        Interrupt { name: "SW1", number: 53 },
        Interrupt { name: "SW2", number: 54 },
        Interrupt { name: "SW3", number: 55 },
        Interrupt { name: "KERNEL0", number: 56 },
        Interrupt { name: "KERNEL1", number: 57 },
        Interrupt { name: "M33CTI0", number: 58 },
        Interrupt { name: "M33CTI1", number: 59 },
        Interrupt { name: "EMUEFP", number: 60 },
        Interrupt { name: "DCDC", number: 61 },
        Interrupt { name: "EUART0_RX", number: 62 },
        Interrupt { name: "EUART0_TX", number: 63 },
    ],
};

// Per-kind IR statics (chiptool IR snapshots).
#[path = "../../registers/buram_v0.rs"]
pub mod buram_v0;
#[path = "../../registers/burtc_v0.rs"]
pub mod burtc_v0;
#[path = "../../registers/cmu_v1.rs"]
pub mod cmu_v1;
#[path = "../../registers/cryptoacc_v1.rs"]
pub mod cryptoacc_v1;
#[path = "../../registers/cryptoacc_ns_pkctrl_v1.rs"]
pub mod cryptoacc_ns_pkctrl_v1;
#[path = "../../registers/cryptoacc_ns_rngctrl_v1.rs"]
pub mod cryptoacc_ns_rngctrl_v1;
#[path = "../../registers/cryptoacc_s_pkctrl_v1.rs"]
pub mod cryptoacc_s_pkctrl_v1;
#[path = "../../registers/cryptoacc_s_rngctrl_v1.rs"]
pub mod cryptoacc_s_rngctrl_v1;
#[path = "../../registers/dcdc_v0.rs"]
pub mod dcdc_v0;
#[path = "../../registers/devinfo_v1.rs"]
pub mod devinfo_v1;
#[path = "../../registers/dpll_v0.rs"]
pub mod dpll_v0;
#[path = "../../registers/emu_v8.rs"]
pub mod emu_v8;
#[path = "../../registers/euart_v0.rs"]
pub mod euart_v0;
#[path = "../../registers/fsrco_v1.rs"]
pub mod fsrco_v1;
#[path = "../../registers/gpcrc_v0.rs"]
pub mod gpcrc_v0;
#[path = "../../registers/gpio_v1.rs"]
pub mod gpio_v1;
#[path = "../../registers/hfrco_v1.rs"]
pub mod hfrco_v1;
#[path = "../../registers/hfxo_v2.rs"]
pub mod hfxo_v2;
#[path = "../../registers/i2c_v0.rs"]
pub mod i2c_v0;
#[path = "../../registers/iadc_v1.rs"]
pub mod iadc_v1;
#[path = "../../registers/icache_v0.rs"]
pub mod icache_v0;
#[path = "../../registers/ldma_v0.rs"]
pub mod ldma_v0;
#[path = "../../registers/ldmaxbar_v1.rs"]
pub mod ldmaxbar_v1;
#[path = "../../registers/letimer_v0.rs"]
pub mod letimer_v0;
#[path = "../../registers/lfrco_v3.rs"]
pub mod lfrco_v3;
#[path = "../../registers/lfxo_v0.rs"]
pub mod lfxo_v0;
#[path = "../../registers/msc_v8.rs"]
pub mod msc_v8;
#[path = "../../registers/pdm_v0.rs"]
pub mod pdm_v0;
#[path = "../../registers/prortc_v1.rs"]
pub mod prortc_v1;
#[path = "../../registers/prs_v1.rs"]
pub mod prs_v1;
#[path = "../../registers/radioaes_v1.rs"]
pub mod radioaes_v1;
#[path = "../../registers/rtcc_v1.rs"]
pub mod rtcc_v1;
#[path = "../../registers/smu_v1.rs"]
pub mod smu_v1;
#[path = "../../registers/smu_ns_cfgns_v1.rs"]
pub mod smu_ns_cfgns_v1;
#[path = "../../registers/smu_s_cfgns_v1.rs"]
pub mod smu_s_cfgns_v1;
#[path = "../../registers/syscfg_v8.rs"]
pub mod syscfg_v8;
#[path = "../../registers/syscfg_ns_cfgns_v8.rs"]
pub mod syscfg_ns_cfgns_v8;
#[path = "../../registers/syscfg_s_cfgns_v8.rs"]
pub mod syscfg_s_cfgns_v8;
#[path = "../../registers/timer_v0.rs"]
pub mod timer_v0;
#[path = "../../registers/timer_v0_w.rs"]
pub mod timer_v0_w;
#[path = "../../registers/ulfrco_v0.rs"]
pub mod ulfrco_v0;
#[path = "../../registers/usart_v0.rs"]
pub mod usart_v0;
#[path = "../../registers/wdog_v0.rs"]
pub mod wdog_v0;
