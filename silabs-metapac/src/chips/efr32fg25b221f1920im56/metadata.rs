// Per-chip iterable metadata. Generated for EFR32FG25B221F1920IM56.
//
// Included from `pub mod metadata` in the metapac crate root;
// type names resolve to the surrounding module — see
// silabs-metapac-gen/res/metadata.rs.

pub static METADATA: Metadata = Metadata {
    name: "EFR32FG25B221F1920IM56",
    core: "Cortex-M33",
    fpu: false,
    mpu: false,
    trustzone: false,
    series: Series::Series2(5),
    memory: &[
        MemoryRegion { name: "IROM1", address: 0x08000000, size: 0x001E0000, access: "rx" },
        MemoryRegion { name: "IRAM1", address: 0x20000000, size: 0x00080000, access: "rwx" },
    ],
    peripherals: &[
        Peripheral { name: "ACMP0", address: 0x59008000, kind: "acmp", version: "v3", block: "ACMP" },
        Peripheral { name: "ACMP1", address: 0x5900C000, kind: "acmp", version: "v3", block: "ACMP" },
        Peripheral { name: "BUFC", address: 0x52000000, kind: "bufc", version: "v3", block: "BUFC" },
        Peripheral { name: "BURAM", address: 0x50080000, kind: "buram", version: "v0", block: "BURAM" },
        Peripheral { name: "BURTC", address: 0x5006C000, kind: "burtc", version: "v1", block: "BURTC" },
        Peripheral { name: "CMU", address: 0x50008000, kind: "cmu", version: "v4", block: "CMU" },
        Peripheral { name: "DCDC", address: 0x50094000, kind: "dcdc", version: "v4", block: "DCDC" },
        Peripheral { name: "DEVINFO", address: 0x0FE08000, kind: "devinfo", version: "v18", block: "DEVINFO" },
        Peripheral { name: "DMEM", address: 0x500C0000, kind: "dmem", version: "v2_fg25", block: "DMEM" },
        Peripheral { name: "DPLL0", address: 0x5001C000, kind: "dpll", version: "v1", block: "DPLL" },
        Peripheral { name: "EMU", address: 0x50004000, kind: "emu", version: "v4", block: "EMU" },
        Peripheral { name: "ETAMPDET", address: 0x500D0000, kind: "etampdet", version: "v1", block: "ETAMPDET" },
        Peripheral { name: "EUSART0", address: 0x5B010000, kind: "eusart", version: "v2_lf", block: "EUSART" },
        Peripheral { name: "EUSART1", address: 0x500A0000, kind: "eusart", version: "v2", block: "EUSART" },
        Peripheral { name: "EUSART2", address: 0x500A4000, kind: "eusart", version: "v2", block: "EUSART" },
        Peripheral { name: "EUSART3", address: 0x500A8000, kind: "eusart", version: "v2", block: "EUSART" },
        Peripheral { name: "EUSART4", address: 0x500AC000, kind: "eusart", version: "v2", block: "EUSART" },
        Peripheral { name: "FSRCO", address: 0x50018000, kind: "fsrco", version: "v0", block: "FSRCO" },
        Peripheral { name: "GPCRC", address: 0x50088000, kind: "gpcrc", version: "v0", block: "GPCRC" },
        Peripheral { name: "GPIO", address: 0x5003C000, kind: "gpio", version: "v4", block: "GPIO" },
        Peripheral { name: "HFRCO0", address: 0x50010000, kind: "hfrco", version: "v2", block: "HFRCO" },
        Peripheral { name: "HFRCOEM23", address: 0x5A000000, kind: "hfrcoem", version: "v2", block: "HFRCOEM" },
        Peripheral { name: "HFXO0", address: 0x5A004000, kind: "hfxo", version: "v4", block: "HFXO" },
        Peripheral { name: "HOSTMAILBOX", address: 0x50098000, kind: "hostmailbox", version: "v0", block: "HOSTMAILBOX" },
        Peripheral { name: "I2C0", address: 0x5B000000, kind: "i2c", version: "v0", block: "I2C" },
        Peripheral { name: "I2C1", address: 0x50070000, kind: "i2c", version: "v0", block: "I2C" },
        Peripheral { name: "IADC0", address: 0x59004000, kind: "iadc", version: "v3_fg25", block: "IADC" },
        Peripheral { name: "ICACHE0", address: 0x50034000, kind: "icache", version: "v0", block: "ICACHE" },
        Peripheral { name: "LDMA", address: 0x50040000, kind: "ldma", version: "v1", block: "LDMA" },
        Peripheral { name: "LDMAXBAR", address: 0x50044000, kind: "ldmaxbar", version: "v4", block: "LDMAXBAR" },
        Peripheral { name: "LESENSE", address: 0x59038000, kind: "lesense", version: "v1", block: "LESENSE" },
        Peripheral { name: "LETIMER0", address: 0x59000000, kind: "letimer", version: "v1", block: "LETIMER" },
        Peripheral { name: "LFRCO", address: 0x50024000, kind: "lfrco", version: "v0", block: "LFRCO" },
        Peripheral { name: "LFXO", address: 0x50020000, kind: "lfxo", version: "v1", block: "LFXO" },
        Peripheral { name: "MPAHBRAM", address: 0xB6020000, kind: "mpahbram", version: "v2", block: "MPAHBRAM" },
        Peripheral { name: "MSC", address: 0x50030000, kind: "msc", version: "v4", block: "MSC" },
        Peripheral { name: "PCNT0", address: 0x59030000, kind: "pcnt", version: "v1", block: "PCNT" },
        Peripheral { name: "PFMXPPRF", address: 0x500C4000, kind: "pfmxpprf", version: "v0", block: "PFMXPPRF" },
        Peripheral { name: "PRS", address: 0x50038000, kind: "prs", version: "v4", block: "PRS" },
        Peripheral { name: "RADIOAES", address: 0x54000000, kind: "radioaes", version: "v1", block: "RADIOAES" },
        Peripheral { name: "RFFPLL0", address: 0x500C8000, kind: "rffpll", version: "v1", block: "RFFPLL" },
        Peripheral { name: "SCRATCHPAD", address: 0x50000000, kind: "scratchpad", version: "v0", block: "SCRATCHPAD" },
        Peripheral { name: "SEMAILBOX_NS_HOST", address: 0x5C000000, kind: "semailbox_ns_host", version: "v1", block: "SEMAILBOX_NS_HOST" },
        Peripheral { name: "SEMAILBOX_S_HOST", address: 0x4C000000, kind: "semailbox_s_host", version: "v1", block: "SEMAILBOX_S_HOST" },
        Peripheral { name: "SMU", address: 0x54008000, kind: "smu", version: "v4", block: "SMU" },
        Peripheral { name: "SMU_NS_CFGNS", address: 0x5400C000, kind: "smu_ns_cfgns", version: "v4", block: "SMU_NS_CFGNS" },
        Peripheral { name: "SMU_S_CFGNS", address: 0x4400C000, kind: "smu_s_cfgns", version: "v4", block: "SMU_S_CFGNS" },
        Peripheral { name: "SYSCFG", address: 0x5007C000, kind: "syscfg", version: "v4", block: "SYSCFG" },
        Peripheral { name: "SYSCFG_NS_CFGNS", address: 0x50078000, kind: "syscfg_ns_cfgns", version: "v4", block: "SYSCFG_NS_CFGNS" },
        Peripheral { name: "SYSCFG_S_CFGNS", address: 0x40078000, kind: "syscfg_s_cfgns", version: "v4", block: "SYSCFG_S_CFGNS" },
        Peripheral { name: "SYSRTC0", address: 0x500B0000, kind: "sysrtc", version: "v1", block: "SYSRTC" },
        Peripheral { name: "TIMER0", address: 0x50048000, kind: "timer", version: "v1_w", block: "TIMER" },
        Peripheral { name: "TIMER1", address: 0x5004C000, kind: "timer", version: "v1_w", block: "TIMER" },
        Peripheral { name: "TIMER2", address: 0x50050000, kind: "timer", version: "v1", block: "TIMER" },
        Peripheral { name: "TIMER3", address: 0x50054000, kind: "timer", version: "v1", block: "TIMER" },
        Peripheral { name: "TIMER4", address: 0x50058000, kind: "timer", version: "v1", block: "TIMER" },
        Peripheral { name: "TIMER5", address: 0x5005C000, kind: "timer", version: "v1", block: "TIMER" },
        Peripheral { name: "TIMER6", address: 0x50060000, kind: "timer", version: "v1", block: "TIMER" },
        Peripheral { name: "TIMER7", address: 0x50064000, kind: "timer", version: "v1", block: "TIMER" },
        Peripheral { name: "ULFRCO", address: 0x50028000, kind: "ulfrco", version: "v1", block: "ULFRCO" },
        Peripheral { name: "USBAHB_NS_AHBS", address: 0x56000000, kind: "usbahb_ns_ahbs", version: "v1", block: "USBAHB_NS_AHBS" },
        Peripheral { name: "USBAHB_S_AHBS", address: 0x46000000, kind: "usbahb_s_ahbs", version: "v1", block: "USBAHB_S_AHBS" },
        Peripheral { name: "USBPLL0", address: 0x57004000, kind: "usbpll", version: "v1", block: "USBPLL" },
        Peripheral { name: "USB_NS_APBS", address: 0x57000000, kind: "usb_ns_apbs", version: "v1", block: "USB_NS_APBS" },
        Peripheral { name: "USB_S_APBS", address: 0x47000000, kind: "usb_s_apbs", version: "v1", block: "USB_S_APBS" },
        Peripheral { name: "VDAC0", address: 0x59024000, kind: "vdac", version: "v2_fg25", block: "VDAC" },
        Peripheral { name: "WDOG0", address: 0x5B004000, kind: "wdog", version: "v1", block: "WDOG" },
        Peripheral { name: "WDOG1", address: 0x5B008000, kind: "wdog", version: "v1", block: "WDOG" },
    ],
    interrupts: &[
        Interrupt { name: "SMU_SECURE", number: 0 },
        Interrupt { name: "SMU_S_PRIVILEGED", number: 1 },
        Interrupt { name: "SMU_NS_PRIVILEGED", number: 2 },
        Interrupt { name: "EMU", number: 3 },
        Interrupt { name: "TIMER0", number: 4 },
        Interrupt { name: "TIMER1", number: 5 },
        Interrupt { name: "TIMER2", number: 6 },
        Interrupt { name: "TIMER3", number: 7 },
        Interrupt { name: "ETAMPDET", number: 8 },
        Interrupt { name: "TIMER4", number: 9 },
        Interrupt { name: "TIMER5", number: 10 },
        Interrupt { name: "TIMER6", number: 11 },
        Interrupt { name: "TIMER7", number: 12 },
        Interrupt { name: "EUSART0_RX", number: 13 },
        Interrupt { name: "EUSART0_TX", number: 14 },
        Interrupt { name: "EUSART1_RX", number: 15 },
        Interrupt { name: "EUSART1_TX", number: 16 },
        Interrupt { name: "EUSART2_RX", number: 17 },
        Interrupt { name: "EUSART2_TX", number: 18 },
        Interrupt { name: "EUSART3_RX", number: 19 },
        Interrupt { name: "EUSART3_TX", number: 20 },
        Interrupt { name: "EUSART4_RX", number: 21 },
        Interrupt { name: "EUSART4_TX", number: 22 },
        Interrupt { name: "ICACHE0", number: 23 },
        Interrupt { name: "BURTC", number: 24 },
        Interrupt { name: "LETIMER0", number: 25 },
        Interrupt { name: "SYSCFG", number: 26 },
        Interrupt { name: "MPAHBRAM", number: 27 },
        Interrupt { name: "LDMA", number: 28 },
        Interrupt { name: "LFXO", number: 29 },
        Interrupt { name: "LFRCO", number: 30 },
        Interrupt { name: "ULFRCO", number: 31 },
        Interrupt { name: "GPIO_ODD", number: 32 },
        Interrupt { name: "GPIO_EVEN", number: 33 },
        Interrupt { name: "I2C0", number: 34 },
        Interrupt { name: "I2C1", number: 35 },
        Interrupt { name: "EMUDG", number: 36 },
        Interrupt { name: "AGC", number: 37 },
        Interrupt { name: "BUFC", number: 38 },
        Interrupt { name: "FRC_PRI", number: 39 },
        Interrupt { name: "FRC", number: 40 },
        Interrupt { name: "MODEM", number: 41 },
        Interrupt { name: "PROTIMER", number: 42 },
        Interrupt { name: "RAC_RSM", number: 43 },
        Interrupt { name: "RAC_SEQ", number: 44 },
        Interrupt { name: "HOSTMAILBOX", number: 45 },
        Interrupt { name: "SYNTH", number: 46 },
        Interrupt { name: "ACMP0", number: 47 },
        Interrupt { name: "ACMP1", number: 48 },
        Interrupt { name: "WDOG0", number: 49 },
        Interrupt { name: "WDOG1", number: 50 },
        Interrupt { name: "HFXO0", number: 51 },
        Interrupt { name: "HFRCO0", number: 52 },
        Interrupt { name: "HFRCOEM23", number: 53 },
        Interrupt { name: "CMU", number: 54 },
        Interrupt { name: "AES", number: 55 },
        Interrupt { name: "IADC", number: 56 },
        Interrupt { name: "MSC", number: 57 },
        Interrupt { name: "DPLL0", number: 58 },
        Interrupt { name: "DCDC", number: 60 },
        Interrupt { name: "VDAC", number: 61 },
        Interrupt { name: "PCNT0", number: 62 },
        Interrupt { name: "USB", number: 63 },
        Interrupt { name: "SW0", number: 64 },
        Interrupt { name: "SW1", number: 65 },
        Interrupt { name: "SW2", number: 66 },
        Interrupt { name: "SW3", number: 67 },
        Interrupt { name: "KERNEL0", number: 68 },
        Interrupt { name: "KERNEL1", number: 69 },
        Interrupt { name: "M33CTI0", number: 70 },
        Interrupt { name: "M33CTI1", number: 71 },
        Interrupt { name: "FPUEXH", number: 72 },
        Interrupt { name: "SETAMPERHOST", number: 73 },
        Interrupt { name: "SEMBRX", number: 74 },
        Interrupt { name: "SEMBTX", number: 75 },
        Interrupt { name: "LESENSE", number: 76 },
        Interrupt { name: "SYSRTC_APP", number: 77 },
        Interrupt { name: "SYSRTC_SEQ", number: 78 },
        Interrupt { name: "RFECA0", number: 79 },
        Interrupt { name: "RFECA1", number: 80 },
        Interrupt { name: "RFFPLL", number: 81 },
        Interrupt { name: "USBPLL0", number: 82 },
        Interrupt { name: "AHB2AHB0", number: 83 },
        Interrupt { name: "AHB2AHB1", number: 84 },
        Interrupt { name: "AHB2AHB2", number: 85 },
        Interrupt { name: "AHB2AHB3", number: 86 },
        Interrupt { name: "RFTIMER", number: 87 },
        Interrupt { name: "SOFTM", number: 88 },
        Interrupt { name: "RFLDMA", number: 89 },
    ],
};

// Per-kind IR statics (chiptool IR snapshots).
#[path = "../../registers/acmp_v3.rs"]
pub mod acmp_v3;
#[path = "../../registers/bufc_v3.rs"]
pub mod bufc_v3;
#[path = "../../registers/buram_v0.rs"]
pub mod buram_v0;
#[path = "../../registers/burtc_v1.rs"]
pub mod burtc_v1;
#[path = "../../registers/cmu_v4.rs"]
pub mod cmu_v4;
#[path = "../../registers/dcdc_v4.rs"]
pub mod dcdc_v4;
#[path = "../../registers/devinfo_v18.rs"]
pub mod devinfo_v18;
#[path = "../../registers/dmem_v2.rs"]
pub mod dmem_v2;
#[path = "../../registers/dmem_v2_fg25.rs"]
pub mod dmem_v2_fg25;
#[path = "../../registers/dpll_v1.rs"]
pub mod dpll_v1;
#[path = "../../registers/emu_v4.rs"]
pub mod emu_v4;
#[path = "../../registers/etampdet_v1.rs"]
pub mod etampdet_v1;
#[path = "../../registers/eusart_v2.rs"]
pub mod eusart_v2;
#[path = "../../registers/eusart_v2_lf.rs"]
pub mod eusart_v2_lf;
#[path = "../../registers/fsrco_v0.rs"]
pub mod fsrco_v0;
#[path = "../../registers/gpcrc_v0.rs"]
pub mod gpcrc_v0;
#[path = "../../registers/gpio_v4.rs"]
pub mod gpio_v4;
#[path = "../../registers/hfrco_v2.rs"]
pub mod hfrco_v2;
#[path = "../../registers/hfrcoem_v2.rs"]
pub mod hfrcoem_v2;
#[path = "../../registers/hfxo_v4.rs"]
pub mod hfxo_v4;
#[path = "../../registers/hostmailbox_v0.rs"]
pub mod hostmailbox_v0;
#[path = "../../registers/i2c_v0.rs"]
pub mod i2c_v0;
#[path = "../../registers/iadc_v3.rs"]
pub mod iadc_v3;
#[path = "../../registers/iadc_v3_fg25.rs"]
pub mod iadc_v3_fg25;
#[path = "../../registers/icache_v0.rs"]
pub mod icache_v0;
#[path = "../../registers/ldma_v1.rs"]
pub mod ldma_v1;
#[path = "../../registers/ldmaxbar_v4.rs"]
pub mod ldmaxbar_v4;
#[path = "../../registers/lesense_v1.rs"]
pub mod lesense_v1;
#[path = "../../registers/letimer_v1.rs"]
pub mod letimer_v1;
#[path = "../../registers/lfrco_v0.rs"]
pub mod lfrco_v0;
#[path = "../../registers/lfxo_v1.rs"]
pub mod lfxo_v1;
#[path = "../../registers/mpahbram_v2.rs"]
pub mod mpahbram_v2;
#[path = "../../registers/msc_v4.rs"]
pub mod msc_v4;
#[path = "../../registers/pcnt_v1.rs"]
pub mod pcnt_v1;
#[path = "../../registers/pfmxpprf_v0.rs"]
pub mod pfmxpprf_v0;
#[path = "../../registers/prs_v4.rs"]
pub mod prs_v4;
#[path = "../../registers/radioaes_v1.rs"]
pub mod radioaes_v1;
#[path = "../../registers/rffpll_v1.rs"]
pub mod rffpll_v1;
#[path = "../../registers/scratchpad_v0.rs"]
pub mod scratchpad_v0;
#[path = "../../registers/semailbox_ns_host_v1.rs"]
pub mod semailbox_ns_host_v1;
#[path = "../../registers/semailbox_s_host_v1.rs"]
pub mod semailbox_s_host_v1;
#[path = "../../registers/smu_v4.rs"]
pub mod smu_v4;
#[path = "../../registers/smu_ns_cfgns_v4.rs"]
pub mod smu_ns_cfgns_v4;
#[path = "../../registers/smu_s_cfgns_v4.rs"]
pub mod smu_s_cfgns_v4;
#[path = "../../registers/syscfg_v4.rs"]
pub mod syscfg_v4;
#[path = "../../registers/syscfg_ns_cfgns_v4.rs"]
pub mod syscfg_ns_cfgns_v4;
#[path = "../../registers/syscfg_s_cfgns_v4.rs"]
pub mod syscfg_s_cfgns_v4;
#[path = "../../registers/sysrtc_v1.rs"]
pub mod sysrtc_v1;
#[path = "../../registers/timer_v1.rs"]
pub mod timer_v1;
#[path = "../../registers/timer_v1_w.rs"]
pub mod timer_v1_w;
#[path = "../../registers/ulfrco_v1.rs"]
pub mod ulfrco_v1;
#[path = "../../registers/usb_ns_apbs_v1.rs"]
pub mod usb_ns_apbs_v1;
#[path = "../../registers/usb_s_apbs_v1.rs"]
pub mod usb_s_apbs_v1;
#[path = "../../registers/usbahb_ns_ahbs_v1.rs"]
pub mod usbahb_ns_ahbs_v1;
#[path = "../../registers/usbahb_s_ahbs_v1.rs"]
pub mod usbahb_s_ahbs_v1;
#[path = "../../registers/usbpll_v1.rs"]
pub mod usbpll_v1;
#[path = "../../registers/vdac_v2.rs"]
pub mod vdac_v2;
#[path = "../../registers/vdac_v2_fg25.rs"]
pub mod vdac_v2_fg25;
#[path = "../../registers/wdog_v1.rs"]
pub mod wdog_v1;
