use crate::metadata::ir::*;

pub static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Ldma",
        extends: None,
        description: Some("LDMA peripheral."),
        items: &[
            BlockItem {
                name: "ipversion",
                description: Some("IP version register."),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Ipversion"),
                }),
            },
            BlockItem {
                name: "en",
                description: Some("Module enable disable Register."),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("En"),
                }),
            },
            BlockItem {
                name: "swrst",
                description: Some("Software Reset Register."),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Swrst"),
                }),
            },
            BlockItem {
                name: "ctrl",
                description: Some("Control Register."),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctrl"),
                }),
            },
            BlockItem {
                name: "status",
                description: Some("Status Register."),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Status"),
                }),
            },
            BlockItem {
                name: "syncswset",
                description: Some("Sync Trig Sw Set Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswset"),
                }),
            },
            BlockItem {
                name: "syncswclr",
                description: Some("Sync Trig Sw Clear register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 24,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswclr"),
                }),
            },
            BlockItem {
                name: "synchwen",
                description: Some("Sync HW trigger enable register."),
                array: None,
                byte_offset: 28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Synchwen"),
                }),
            },
            BlockItem {
                name: "synchwsel",
                description: Some("Sync HW trigger selection register."),
                array: None,
                byte_offset: 32,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Synchwsel"),
                }),
            },
            BlockItem {
                name: "syncstatus",
                description: Some("Sync Trigger Status Register."),
                array: None,
                byte_offset: 36,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Syncstatus"),
                }),
            },
            BlockItem {
                name: "chen",
                description: Some("Channel Enable Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 40,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chen"),
                }),
            },
            BlockItem {
                name: "chdis",
                description: Some("Channel Disable Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 44,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdis"),
                }),
            },
            BlockItem {
                name: "chstatus",
                description: Some("Channel Status Register."),
                array: None,
                byte_offset: 48,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Chstatus"),
                }),
            },
            BlockItem {
                name: "chbusy",
                description: Some("Channel Busy Register."),
                array: None,
                byte_offset: 52,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Chbusy"),
                }),
            },
            BlockItem {
                name: "chdone",
                description: Some("Channel Linking Done Status Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 56,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Chdone"),
                }),
            },
            BlockItem {
                name: "dbghalt",
                description: Some("Channel Debug Halt Register."),
                array: None,
                byte_offset: 60,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Dbghalt"),
                }),
            },
            BlockItem {
                name: "swreq",
                description: Some("Channel Software Transfer Request (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 64,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swreq"),
                }),
            },
            BlockItem {
                name: "reqdis",
                description: Some("Channel Request Disable Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 68,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Reqdis"),
                }),
            },
            BlockItem {
                name: "reqpend",
                description: Some("Channel Requests Pending Register."),
                array: None,
                byte_offset: 72,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Reqpend"),
                }),
            },
            BlockItem {
                name: "linkload",
                description: Some("Channel Link Load Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 76,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Linkload"),
                }),
            },
            BlockItem {
                name: "reqclear",
                description: Some("Channel Request Clear Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 80,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqclear"),
                }),
            },
            BlockItem {
                name: "if_",
                description: Some("Interrupt Flag Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 84,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("If"),
                }),
            },
            BlockItem {
                name: "ien",
                description: Some("Done Interrupt Enable Register."),
                array: None,
                byte_offset: 88,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ien"),
                }),
            },
            BlockItem {
                name: "ch0_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 96,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Cfg"),
                }),
            },
            BlockItem {
                name: "ch0_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 100,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Loop"),
                }),
            },
            BlockItem {
                name: "ch0_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 104,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Ctrl"),
                }),
            },
            BlockItem {
                name: "ch0_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 108,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Src"),
                }),
            },
            BlockItem {
                name: "ch0_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 112,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Dst"),
                }),
            },
            BlockItem {
                name: "ch0_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 116,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Link"),
                }),
            },
            BlockItem {
                name: "ch0_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 120,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Xctrl"),
                }),
            },
            BlockItem {
                name: "ch0_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 128,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch0Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch1_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 144,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Cfg"),
                }),
            },
            BlockItem {
                name: "ch1_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 148,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Loop"),
                }),
            },
            BlockItem {
                name: "ch1_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 152,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Ctrl"),
                }),
            },
            BlockItem {
                name: "ch1_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 156,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Src"),
                }),
            },
            BlockItem {
                name: "ch1_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 160,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Dst"),
                }),
            },
            BlockItem {
                name: "ch1_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 164,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Link"),
                }),
            },
            BlockItem {
                name: "ch1_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 168,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Xctrl"),
                }),
            },
            BlockItem {
                name: "ch1_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 176,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch1Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch2_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 192,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Cfg"),
                }),
            },
            BlockItem {
                name: "ch2_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 196,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Loop"),
                }),
            },
            BlockItem {
                name: "ch2_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 200,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Ctrl"),
                }),
            },
            BlockItem {
                name: "ch2_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 204,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Src"),
                }),
            },
            BlockItem {
                name: "ch2_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 208,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Dst"),
                }),
            },
            BlockItem {
                name: "ch2_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 212,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Link"),
                }),
            },
            BlockItem {
                name: "ch2_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 216,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Xctrl"),
                }),
            },
            BlockItem {
                name: "ch2_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 224,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch2Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch3_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 240,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Cfg"),
                }),
            },
            BlockItem {
                name: "ch3_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 244,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Loop"),
                }),
            },
            BlockItem {
                name: "ch3_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 248,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Ctrl"),
                }),
            },
            BlockItem {
                name: "ch3_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 252,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Src"),
                }),
            },
            BlockItem {
                name: "ch3_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 256,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Dst"),
                }),
            },
            BlockItem {
                name: "ch3_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 260,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Link"),
                }),
            },
            BlockItem {
                name: "ch3_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 264,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Xctrl"),
                }),
            },
            BlockItem {
                name: "ch3_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 272,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch3Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch4_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 288,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Cfg"),
                }),
            },
            BlockItem {
                name: "ch4_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 292,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Loop"),
                }),
            },
            BlockItem {
                name: "ch4_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 296,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Ctrl"),
                }),
            },
            BlockItem {
                name: "ch4_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 300,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Src"),
                }),
            },
            BlockItem {
                name: "ch4_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 304,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Dst"),
                }),
            },
            BlockItem {
                name: "ch4_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 308,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Link"),
                }),
            },
            BlockItem {
                name: "ch4_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 312,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Xctrl"),
                }),
            },
            BlockItem {
                name: "ch4_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 320,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch4Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch5_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 336,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Cfg"),
                }),
            },
            BlockItem {
                name: "ch5_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 340,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Loop"),
                }),
            },
            BlockItem {
                name: "ch5_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 344,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Ctrl"),
                }),
            },
            BlockItem {
                name: "ch5_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 348,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Src"),
                }),
            },
            BlockItem {
                name: "ch5_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 352,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Dst"),
                }),
            },
            BlockItem {
                name: "ch5_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 356,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Link"),
                }),
            },
            BlockItem {
                name: "ch5_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 360,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Xctrl"),
                }),
            },
            BlockItem {
                name: "ch5_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 368,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch5Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch6_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 384,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Cfg"),
                }),
            },
            BlockItem {
                name: "ch6_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 388,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Loop"),
                }),
            },
            BlockItem {
                name: "ch6_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 392,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Ctrl"),
                }),
            },
            BlockItem {
                name: "ch6_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 396,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Src"),
                }),
            },
            BlockItem {
                name: "ch6_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 400,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Dst"),
                }),
            },
            BlockItem {
                name: "ch6_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 404,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Link"),
                }),
            },
            BlockItem {
                name: "ch6_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 408,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Xctrl"),
                }),
            },
            BlockItem {
                name: "ch6_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 416,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch6Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch7_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 432,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Cfg"),
                }),
            },
            BlockItem {
                name: "ch7_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 436,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Loop"),
                }),
            },
            BlockItem {
                name: "ch7_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 440,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Ctrl"),
                }),
            },
            BlockItem {
                name: "ch7_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 444,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Src"),
                }),
            },
            BlockItem {
                name: "ch7_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 448,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Dst"),
                }),
            },
            BlockItem {
                name: "ch7_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 452,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Link"),
                }),
            },
            BlockItem {
                name: "ch7_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 456,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Xctrl"),
                }),
            },
            BlockItem {
                name: "ch7_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 464,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch7Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch8_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 480,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Cfg"),
                }),
            },
            BlockItem {
                name: "ch8_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 484,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Loop"),
                }),
            },
            BlockItem {
                name: "ch8_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 488,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Ctrl"),
                }),
            },
            BlockItem {
                name: "ch8_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 492,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Src"),
                }),
            },
            BlockItem {
                name: "ch8_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 496,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Dst"),
                }),
            },
            BlockItem {
                name: "ch8_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 500,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Link"),
                }),
            },
            BlockItem {
                name: "ch8_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 504,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Xctrl"),
                }),
            },
            BlockItem {
                name: "ch8_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 512,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch8Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch9_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 528,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Cfg"),
                }),
            },
            BlockItem {
                name: "ch9_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 532,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Loop"),
                }),
            },
            BlockItem {
                name: "ch9_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 536,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Ctrl"),
                }),
            },
            BlockItem {
                name: "ch9_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 540,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Src"),
                }),
            },
            BlockItem {
                name: "ch9_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 544,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Dst"),
                }),
            },
            BlockItem {
                name: "ch9_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 548,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Link"),
                }),
            },
            BlockItem {
                name: "ch9_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 552,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Xctrl"),
                }),
            },
            BlockItem {
                name: "ch9_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 560,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch9Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch10_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 576,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Cfg"),
                }),
            },
            BlockItem {
                name: "ch10_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 580,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Loop"),
                }),
            },
            BlockItem {
                name: "ch10_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 584,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Ctrl"),
                }),
            },
            BlockItem {
                name: "ch10_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 588,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Src"),
                }),
            },
            BlockItem {
                name: "ch10_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 592,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Dst"),
                }),
            },
            BlockItem {
                name: "ch10_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 596,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Link"),
                }),
            },
            BlockItem {
                name: "ch10_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 600,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Xctrl"),
                }),
            },
            BlockItem {
                name: "ch10_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 608,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch10Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch11_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 624,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Cfg"),
                }),
            },
            BlockItem {
                name: "ch11_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 628,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Loop"),
                }),
            },
            BlockItem {
                name: "ch11_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 632,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Ctrl"),
                }),
            },
            BlockItem {
                name: "ch11_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 636,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Src"),
                }),
            },
            BlockItem {
                name: "ch11_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 640,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Dst"),
                }),
            },
            BlockItem {
                name: "ch11_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 644,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Link"),
                }),
            },
            BlockItem {
                name: "ch11_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 648,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Xctrl"),
                }),
            },
            BlockItem {
                name: "ch11_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 656,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch11Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch12_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 672,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Cfg"),
                }),
            },
            BlockItem {
                name: "ch12_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 676,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Loop"),
                }),
            },
            BlockItem {
                name: "ch12_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 680,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Ctrl"),
                }),
            },
            BlockItem {
                name: "ch12_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 684,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Src"),
                }),
            },
            BlockItem {
                name: "ch12_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 688,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Dst"),
                }),
            },
            BlockItem {
                name: "ch12_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 692,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Link"),
                }),
            },
            BlockItem {
                name: "ch12_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 696,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Xctrl"),
                }),
            },
            BlockItem {
                name: "ch12_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 704,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch12Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch13_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 720,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Cfg"),
                }),
            },
            BlockItem {
                name: "ch13_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 724,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Loop"),
                }),
            },
            BlockItem {
                name: "ch13_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 728,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Ctrl"),
                }),
            },
            BlockItem {
                name: "ch13_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 732,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Src"),
                }),
            },
            BlockItem {
                name: "ch13_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 736,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Dst"),
                }),
            },
            BlockItem {
                name: "ch13_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 740,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Link"),
                }),
            },
            BlockItem {
                name: "ch13_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 744,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Xctrl"),
                }),
            },
            BlockItem {
                name: "ch13_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 752,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch13Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch14_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 768,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Cfg"),
                }),
            },
            BlockItem {
                name: "ch14_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 772,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Loop"),
                }),
            },
            BlockItem {
                name: "ch14_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 776,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Ctrl"),
                }),
            },
            BlockItem {
                name: "ch14_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 780,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Src"),
                }),
            },
            BlockItem {
                name: "ch14_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 784,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Dst"),
                }),
            },
            BlockItem {
                name: "ch14_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 788,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Link"),
                }),
            },
            BlockItem {
                name: "ch14_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 792,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Xctrl"),
                }),
            },
            BlockItem {
                name: "ch14_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 800,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch14Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch15_cfg",
                description: Some("Channel Configuration Register."),
                array: None,
                byte_offset: 816,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Cfg"),
                }),
            },
            BlockItem {
                name: "ch15_loop",
                description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 820,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Loop"),
                }),
            },
            BlockItem {
                name: "ch15_ctrl",
                description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 824,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Ctrl"),
                }),
            },
            BlockItem {
                name: "ch15_src",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 828,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Src"),
                }),
            },
            BlockItem {
                name: "ch15_dst",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 832,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Dst"),
                }),
            },
            BlockItem {
                name: "ch15_link",
                description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
                array: None,
                byte_offset: 836,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Link"),
                }),
            },
            BlockItem {
                name: "ch15_xctrl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 840,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Xctrl"),
                }),
            },
            BlockItem {
                name: "ch15_ilsrc",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
                ),
                array: None,
                byte_offset: 848,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ch15Ilsrc"),
                }),
            },
            BlockItem {
                name: "en_set",
                description: Some("Module enable disable Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4100,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("En"),
                }),
            },
            BlockItem {
                name: "en_clr",
                description: Some("Module enable disable Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8196,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("En"),
                }),
            },
            BlockItem {
                name: "en_tgl",
                description: Some("Module enable disable Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12292,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("En"),
                }),
            },
            BlockItem {
                name: "swrst_set",
                description: Some("Software Reset Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4104,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swrst"),
                }),
            },
            BlockItem {
                name: "swrst_clr",
                description: Some("Software Reset Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8200,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swrst"),
                }),
            },
            BlockItem {
                name: "swrst_tgl",
                description: Some("Software Reset Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12296,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swrst"),
                }),
            },
            BlockItem {
                name: "ctrl_set",
                description: Some("Control Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4108,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ctrl"),
                }),
            },
            BlockItem {
                name: "ctrl_clr",
                description: Some("Control Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8204,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ctrl"),
                }),
            },
            BlockItem {
                name: "ctrl_tgl",
                description: Some("Control Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12300,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ctrl"),
                }),
            },
            BlockItem {
                name: "syncswset_set",
                description: Some(
                    "Sync Trig Sw Set Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4116,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswset"),
                }),
            },
            BlockItem {
                name: "syncswset_clr",
                description: Some(
                    "Sync Trig Sw Set Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8212,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswset"),
                }),
            },
            BlockItem {
                name: "syncswset_tgl",
                description: Some(
                    "Sync Trig Sw Set Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12308,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswset"),
                }),
            },
            BlockItem {
                name: "syncswclr_set",
                description: Some(
                    "Sync Trig Sw Clear register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4120,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswclr"),
                }),
            },
            BlockItem {
                name: "syncswclr_clr",
                description: Some(
                    "Sync Trig Sw Clear register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8216,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswclr"),
                }),
            },
            BlockItem {
                name: "syncswclr_tgl",
                description: Some(
                    "Sync Trig Sw Clear register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12312,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Syncswclr"),
                }),
            },
            BlockItem {
                name: "synchwen_set",
                description: Some("Sync HW trigger enable register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4124,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Synchwen"),
                }),
            },
            BlockItem {
                name: "synchwen_clr",
                description: Some("Sync HW trigger enable register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8220,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Synchwen"),
                }),
            },
            BlockItem {
                name: "synchwen_tgl",
                description: Some("Sync HW trigger enable register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12316,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Synchwen"),
                }),
            },
            BlockItem {
                name: "synchwsel_set",
                description: Some("Sync HW trigger selection register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4128,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Synchwsel"),
                }),
            },
            BlockItem {
                name: "synchwsel_clr",
                description: Some("Sync HW trigger selection register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8224,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Synchwsel"),
                }),
            },
            BlockItem {
                name: "synchwsel_tgl",
                description: Some("Sync HW trigger selection register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12320,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Synchwsel"),
                }),
            },
            BlockItem {
                name: "chen_set",
                description: Some(
                    "Channel Enable Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4136,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chen"),
                }),
            },
            BlockItem {
                name: "chen_clr",
                description: Some(
                    "Channel Enable Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8232,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chen"),
                }),
            },
            BlockItem {
                name: "chen_tgl",
                description: Some(
                    "Channel Enable Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12328,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chen"),
                }),
            },
            BlockItem {
                name: "chdis_set",
                description: Some(
                    "Channel Disable Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4140,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdis"),
                }),
            },
            BlockItem {
                name: "chdis_clr",
                description: Some(
                    "Channel Disable Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8236,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdis"),
                }),
            },
            BlockItem {
                name: "chdis_tgl",
                description: Some(
                    "Channel Disable Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12332,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdis"),
                }),
            },
            BlockItem {
                name: "chdone_set",
                description: Some(
                    "Channel Linking Done Status Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4152,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdone"),
                }),
            },
            BlockItem {
                name: "chdone_clr",
                description: Some(
                    "Channel Linking Done Status Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8248,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdone"),
                }),
            },
            BlockItem {
                name: "chdone_tgl",
                description: Some(
                    "Channel Linking Done Status Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12344,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Chdone"),
                }),
            },
            BlockItem {
                name: "dbghalt_set",
                description: Some("Channel Debug Halt Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4156,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Dbghalt"),
                }),
            },
            BlockItem {
                name: "dbghalt_clr",
                description: Some("Channel Debug Halt Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8252,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Dbghalt"),
                }),
            },
            BlockItem {
                name: "dbghalt_tgl",
                description: Some("Channel Debug Halt Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12348,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Dbghalt"),
                }),
            },
            BlockItem {
                name: "swreq_set",
                description: Some(
                    "Channel Software Transfer Request (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4160,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swreq"),
                }),
            },
            BlockItem {
                name: "swreq_clr",
                description: Some(
                    "Channel Software Transfer Request (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8256,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swreq"),
                }),
            },
            BlockItem {
                name: "swreq_tgl",
                description: Some(
                    "Channel Software Transfer Request (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12352,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Swreq"),
                }),
            },
            BlockItem {
                name: "reqdis_set",
                description: Some(
                    "Channel Request Disable Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4164,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqdis"),
                }),
            },
            BlockItem {
                name: "reqdis_clr",
                description: Some(
                    "Channel Request Disable Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8260,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqdis"),
                }),
            },
            BlockItem {
                name: "reqdis_tgl",
                description: Some(
                    "Channel Request Disable Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12356,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqdis"),
                }),
            },
            BlockItem {
                name: "linkload_set",
                description: Some(
                    "Channel Link Load Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4172,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Linkload"),
                }),
            },
            BlockItem {
                name: "linkload_clr",
                description: Some(
                    "Channel Link Load Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8268,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Linkload"),
                }),
            },
            BlockItem {
                name: "linkload_tgl",
                description: Some(
                    "Channel Link Load Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12364,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Linkload"),
                }),
            },
            BlockItem {
                name: "reqclear_set",
                description: Some(
                    "Channel Request Clear Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4176,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqclear"),
                }),
            },
            BlockItem {
                name: "reqclear_clr",
                description: Some(
                    "Channel Request Clear Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8272,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqclear"),
                }),
            },
            BlockItem {
                name: "reqclear_tgl",
                description: Some(
                    "Channel Request Clear Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12368,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Reqclear"),
                }),
            },
            BlockItem {
                name: "if_set",
                description: Some(
                    "Interrupt Flag Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4180,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("If"),
                }),
            },
            BlockItem {
                name: "if_clr",
                description: Some(
                    "Interrupt Flag Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8276,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("If"),
                }),
            },
            BlockItem {
                name: "if_tgl",
                description: Some(
                    "Interrupt Flag Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12372,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("If"),
                }),
            },
            BlockItem {
                name: "ien_set",
                description: Some("Done Interrupt Enable Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4184,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ien"),
                }),
            },
            BlockItem {
                name: "ien_clr",
                description: Some("Done Interrupt Enable Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8280,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ien"),
                }),
            },
            BlockItem {
                name: "ien_tgl",
                description: Some("Done Interrupt Enable Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12376,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ien"),
                }),
            },
            BlockItem {
                name: "ch0_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4192,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Cfg"),
                }),
            },
            BlockItem {
                name: "ch0_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8288,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Cfg"),
                }),
            },
            BlockItem {
                name: "ch0_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12384,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Cfg"),
                }),
            },
            BlockItem {
                name: "ch0_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4196,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Loop"),
                }),
            },
            BlockItem {
                name: "ch0_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8292,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Loop"),
                }),
            },
            BlockItem {
                name: "ch0_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12388,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Loop"),
                }),
            },
            BlockItem {
                name: "ch0_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4200,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Ctrl"),
                }),
            },
            BlockItem {
                name: "ch0_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8296,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Ctrl"),
                }),
            },
            BlockItem {
                name: "ch0_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12392,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Ctrl"),
                }),
            },
            BlockItem {
                name: "ch0_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4204,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Src"),
                }),
            },
            BlockItem {
                name: "ch0_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8300,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Src"),
                }),
            },
            BlockItem {
                name: "ch0_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12396,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Src"),
                }),
            },
            BlockItem {
                name: "ch0_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4208,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Dst"),
                }),
            },
            BlockItem {
                name: "ch0_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8304,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Dst"),
                }),
            },
            BlockItem {
                name: "ch0_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12400,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Dst"),
                }),
            },
            BlockItem {
                name: "ch0_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4212,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Link"),
                }),
            },
            BlockItem {
                name: "ch0_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8308,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Link"),
                }),
            },
            BlockItem {
                name: "ch0_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12404,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Link"),
                }),
            },
            BlockItem {
                name: "ch0_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4216,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Xctrl"),
                }),
            },
            BlockItem {
                name: "ch0_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8312,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Xctrl"),
                }),
            },
            BlockItem {
                name: "ch0_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12408,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Xctrl"),
                }),
            },
            BlockItem {
                name: "ch0_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4224,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch0_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8320,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch0_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12416,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch0Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch1_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4240,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Cfg"),
                }),
            },
            BlockItem {
                name: "ch1_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8336,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Cfg"),
                }),
            },
            BlockItem {
                name: "ch1_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12432,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Cfg"),
                }),
            },
            BlockItem {
                name: "ch1_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4244,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Loop"),
                }),
            },
            BlockItem {
                name: "ch1_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8340,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Loop"),
                }),
            },
            BlockItem {
                name: "ch1_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12436,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Loop"),
                }),
            },
            BlockItem {
                name: "ch1_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4248,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Ctrl"),
                }),
            },
            BlockItem {
                name: "ch1_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8344,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Ctrl"),
                }),
            },
            BlockItem {
                name: "ch1_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12440,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Ctrl"),
                }),
            },
            BlockItem {
                name: "ch1_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4252,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Src"),
                }),
            },
            BlockItem {
                name: "ch1_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8348,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Src"),
                }),
            },
            BlockItem {
                name: "ch1_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12444,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Src"),
                }),
            },
            BlockItem {
                name: "ch1_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4256,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Dst"),
                }),
            },
            BlockItem {
                name: "ch1_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8352,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Dst"),
                }),
            },
            BlockItem {
                name: "ch1_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12448,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Dst"),
                }),
            },
            BlockItem {
                name: "ch1_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4260,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Link"),
                }),
            },
            BlockItem {
                name: "ch1_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8356,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Link"),
                }),
            },
            BlockItem {
                name: "ch1_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12452,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Link"),
                }),
            },
            BlockItem {
                name: "ch1_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4264,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Xctrl"),
                }),
            },
            BlockItem {
                name: "ch1_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8360,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Xctrl"),
                }),
            },
            BlockItem {
                name: "ch1_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12456,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Xctrl"),
                }),
            },
            BlockItem {
                name: "ch1_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4272,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch1_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8368,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch1_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12464,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch1Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch2_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4288,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Cfg"),
                }),
            },
            BlockItem {
                name: "ch2_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8384,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Cfg"),
                }),
            },
            BlockItem {
                name: "ch2_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12480,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Cfg"),
                }),
            },
            BlockItem {
                name: "ch2_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4292,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Loop"),
                }),
            },
            BlockItem {
                name: "ch2_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8388,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Loop"),
                }),
            },
            BlockItem {
                name: "ch2_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12484,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Loop"),
                }),
            },
            BlockItem {
                name: "ch2_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4296,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Ctrl"),
                }),
            },
            BlockItem {
                name: "ch2_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8392,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Ctrl"),
                }),
            },
            BlockItem {
                name: "ch2_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12488,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Ctrl"),
                }),
            },
            BlockItem {
                name: "ch2_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4300,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Src"),
                }),
            },
            BlockItem {
                name: "ch2_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8396,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Src"),
                }),
            },
            BlockItem {
                name: "ch2_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12492,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Src"),
                }),
            },
            BlockItem {
                name: "ch2_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4304,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Dst"),
                }),
            },
            BlockItem {
                name: "ch2_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8400,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Dst"),
                }),
            },
            BlockItem {
                name: "ch2_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12496,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Dst"),
                }),
            },
            BlockItem {
                name: "ch2_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4308,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Link"),
                }),
            },
            BlockItem {
                name: "ch2_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8404,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Link"),
                }),
            },
            BlockItem {
                name: "ch2_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12500,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Link"),
                }),
            },
            BlockItem {
                name: "ch2_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4312,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Xctrl"),
                }),
            },
            BlockItem {
                name: "ch2_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8408,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Xctrl"),
                }),
            },
            BlockItem {
                name: "ch2_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12504,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Xctrl"),
                }),
            },
            BlockItem {
                name: "ch2_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4320,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch2_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8416,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch2_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12512,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch2Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch3_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4336,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Cfg"),
                }),
            },
            BlockItem {
                name: "ch3_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8432,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Cfg"),
                }),
            },
            BlockItem {
                name: "ch3_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12528,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Cfg"),
                }),
            },
            BlockItem {
                name: "ch3_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4340,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Loop"),
                }),
            },
            BlockItem {
                name: "ch3_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8436,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Loop"),
                }),
            },
            BlockItem {
                name: "ch3_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12532,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Loop"),
                }),
            },
            BlockItem {
                name: "ch3_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4344,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Ctrl"),
                }),
            },
            BlockItem {
                name: "ch3_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8440,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Ctrl"),
                }),
            },
            BlockItem {
                name: "ch3_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12536,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Ctrl"),
                }),
            },
            BlockItem {
                name: "ch3_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4348,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Src"),
                }),
            },
            BlockItem {
                name: "ch3_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8444,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Src"),
                }),
            },
            BlockItem {
                name: "ch3_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12540,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Src"),
                }),
            },
            BlockItem {
                name: "ch3_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4352,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Dst"),
                }),
            },
            BlockItem {
                name: "ch3_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8448,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Dst"),
                }),
            },
            BlockItem {
                name: "ch3_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12544,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Dst"),
                }),
            },
            BlockItem {
                name: "ch3_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4356,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Link"),
                }),
            },
            BlockItem {
                name: "ch3_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8452,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Link"),
                }),
            },
            BlockItem {
                name: "ch3_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12548,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Link"),
                }),
            },
            BlockItem {
                name: "ch3_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4360,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Xctrl"),
                }),
            },
            BlockItem {
                name: "ch3_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8456,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Xctrl"),
                }),
            },
            BlockItem {
                name: "ch3_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12552,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Xctrl"),
                }),
            },
            BlockItem {
                name: "ch3_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4368,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch3_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8464,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch3_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12560,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch3Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch4_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4384,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Cfg"),
                }),
            },
            BlockItem {
                name: "ch4_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8480,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Cfg"),
                }),
            },
            BlockItem {
                name: "ch4_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12576,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Cfg"),
                }),
            },
            BlockItem {
                name: "ch4_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4388,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Loop"),
                }),
            },
            BlockItem {
                name: "ch4_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8484,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Loop"),
                }),
            },
            BlockItem {
                name: "ch4_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12580,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Loop"),
                }),
            },
            BlockItem {
                name: "ch4_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4392,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Ctrl"),
                }),
            },
            BlockItem {
                name: "ch4_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8488,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Ctrl"),
                }),
            },
            BlockItem {
                name: "ch4_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12584,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Ctrl"),
                }),
            },
            BlockItem {
                name: "ch4_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4396,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Src"),
                }),
            },
            BlockItem {
                name: "ch4_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8492,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Src"),
                }),
            },
            BlockItem {
                name: "ch4_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12588,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Src"),
                }),
            },
            BlockItem {
                name: "ch4_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4400,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Dst"),
                }),
            },
            BlockItem {
                name: "ch4_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8496,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Dst"),
                }),
            },
            BlockItem {
                name: "ch4_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12592,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Dst"),
                }),
            },
            BlockItem {
                name: "ch4_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4404,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Link"),
                }),
            },
            BlockItem {
                name: "ch4_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8500,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Link"),
                }),
            },
            BlockItem {
                name: "ch4_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12596,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Link"),
                }),
            },
            BlockItem {
                name: "ch4_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4408,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Xctrl"),
                }),
            },
            BlockItem {
                name: "ch4_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8504,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Xctrl"),
                }),
            },
            BlockItem {
                name: "ch4_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12600,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Xctrl"),
                }),
            },
            BlockItem {
                name: "ch4_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4416,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch4_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8512,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch4_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12608,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch4Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch5_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4432,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Cfg"),
                }),
            },
            BlockItem {
                name: "ch5_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8528,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Cfg"),
                }),
            },
            BlockItem {
                name: "ch5_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12624,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Cfg"),
                }),
            },
            BlockItem {
                name: "ch5_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4436,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Loop"),
                }),
            },
            BlockItem {
                name: "ch5_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8532,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Loop"),
                }),
            },
            BlockItem {
                name: "ch5_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12628,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Loop"),
                }),
            },
            BlockItem {
                name: "ch5_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4440,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Ctrl"),
                }),
            },
            BlockItem {
                name: "ch5_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8536,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Ctrl"),
                }),
            },
            BlockItem {
                name: "ch5_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12632,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Ctrl"),
                }),
            },
            BlockItem {
                name: "ch5_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4444,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Src"),
                }),
            },
            BlockItem {
                name: "ch5_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8540,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Src"),
                }),
            },
            BlockItem {
                name: "ch5_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12636,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Src"),
                }),
            },
            BlockItem {
                name: "ch5_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4448,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Dst"),
                }),
            },
            BlockItem {
                name: "ch5_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8544,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Dst"),
                }),
            },
            BlockItem {
                name: "ch5_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12640,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Dst"),
                }),
            },
            BlockItem {
                name: "ch5_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4452,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Link"),
                }),
            },
            BlockItem {
                name: "ch5_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8548,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Link"),
                }),
            },
            BlockItem {
                name: "ch5_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12644,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Link"),
                }),
            },
            BlockItem {
                name: "ch5_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4456,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Xctrl"),
                }),
            },
            BlockItem {
                name: "ch5_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8552,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Xctrl"),
                }),
            },
            BlockItem {
                name: "ch5_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12648,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Xctrl"),
                }),
            },
            BlockItem {
                name: "ch5_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4464,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch5_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8560,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch5_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12656,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch5Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch6_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4480,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Cfg"),
                }),
            },
            BlockItem {
                name: "ch6_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8576,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Cfg"),
                }),
            },
            BlockItem {
                name: "ch6_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12672,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Cfg"),
                }),
            },
            BlockItem {
                name: "ch6_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4484,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Loop"),
                }),
            },
            BlockItem {
                name: "ch6_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8580,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Loop"),
                }),
            },
            BlockItem {
                name: "ch6_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12676,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Loop"),
                }),
            },
            BlockItem {
                name: "ch6_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4488,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Ctrl"),
                }),
            },
            BlockItem {
                name: "ch6_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8584,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Ctrl"),
                }),
            },
            BlockItem {
                name: "ch6_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12680,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Ctrl"),
                }),
            },
            BlockItem {
                name: "ch6_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4492,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Src"),
                }),
            },
            BlockItem {
                name: "ch6_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8588,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Src"),
                }),
            },
            BlockItem {
                name: "ch6_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12684,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Src"),
                }),
            },
            BlockItem {
                name: "ch6_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4496,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Dst"),
                }),
            },
            BlockItem {
                name: "ch6_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8592,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Dst"),
                }),
            },
            BlockItem {
                name: "ch6_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12688,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Dst"),
                }),
            },
            BlockItem {
                name: "ch6_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4500,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Link"),
                }),
            },
            BlockItem {
                name: "ch6_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8596,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Link"),
                }),
            },
            BlockItem {
                name: "ch6_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12692,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Link"),
                }),
            },
            BlockItem {
                name: "ch6_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4504,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Xctrl"),
                }),
            },
            BlockItem {
                name: "ch6_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8600,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Xctrl"),
                }),
            },
            BlockItem {
                name: "ch6_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12696,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Xctrl"),
                }),
            },
            BlockItem {
                name: "ch6_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4512,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch6_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8608,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch6_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12704,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch6Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch7_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4528,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Cfg"),
                }),
            },
            BlockItem {
                name: "ch7_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8624,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Cfg"),
                }),
            },
            BlockItem {
                name: "ch7_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12720,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Cfg"),
                }),
            },
            BlockItem {
                name: "ch7_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4532,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Loop"),
                }),
            },
            BlockItem {
                name: "ch7_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8628,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Loop"),
                }),
            },
            BlockItem {
                name: "ch7_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12724,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Loop"),
                }),
            },
            BlockItem {
                name: "ch7_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4536,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Ctrl"),
                }),
            },
            BlockItem {
                name: "ch7_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8632,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Ctrl"),
                }),
            },
            BlockItem {
                name: "ch7_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12728,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Ctrl"),
                }),
            },
            BlockItem {
                name: "ch7_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4540,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Src"),
                }),
            },
            BlockItem {
                name: "ch7_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8636,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Src"),
                }),
            },
            BlockItem {
                name: "ch7_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12732,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Src"),
                }),
            },
            BlockItem {
                name: "ch7_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4544,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Dst"),
                }),
            },
            BlockItem {
                name: "ch7_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8640,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Dst"),
                }),
            },
            BlockItem {
                name: "ch7_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12736,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Dst"),
                }),
            },
            BlockItem {
                name: "ch7_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4548,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Link"),
                }),
            },
            BlockItem {
                name: "ch7_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8644,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Link"),
                }),
            },
            BlockItem {
                name: "ch7_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12740,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Link"),
                }),
            },
            BlockItem {
                name: "ch7_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4552,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Xctrl"),
                }),
            },
            BlockItem {
                name: "ch7_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8648,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Xctrl"),
                }),
            },
            BlockItem {
                name: "ch7_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12744,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Xctrl"),
                }),
            },
            BlockItem {
                name: "ch7_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4560,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch7_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8656,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch7_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12752,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch7Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch8_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4576,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Cfg"),
                }),
            },
            BlockItem {
                name: "ch8_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8672,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Cfg"),
                }),
            },
            BlockItem {
                name: "ch8_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12768,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Cfg"),
                }),
            },
            BlockItem {
                name: "ch8_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4580,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Loop"),
                }),
            },
            BlockItem {
                name: "ch8_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8676,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Loop"),
                }),
            },
            BlockItem {
                name: "ch8_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12772,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Loop"),
                }),
            },
            BlockItem {
                name: "ch8_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4584,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Ctrl"),
                }),
            },
            BlockItem {
                name: "ch8_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8680,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Ctrl"),
                }),
            },
            BlockItem {
                name: "ch8_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12776,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Ctrl"),
                }),
            },
            BlockItem {
                name: "ch8_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4588,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Src"),
                }),
            },
            BlockItem {
                name: "ch8_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8684,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Src"),
                }),
            },
            BlockItem {
                name: "ch8_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12780,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Src"),
                }),
            },
            BlockItem {
                name: "ch8_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4592,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Dst"),
                }),
            },
            BlockItem {
                name: "ch8_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8688,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Dst"),
                }),
            },
            BlockItem {
                name: "ch8_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12784,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Dst"),
                }),
            },
            BlockItem {
                name: "ch8_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4596,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Link"),
                }),
            },
            BlockItem {
                name: "ch8_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8692,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Link"),
                }),
            },
            BlockItem {
                name: "ch8_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12788,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Link"),
                }),
            },
            BlockItem {
                name: "ch8_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4600,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Xctrl"),
                }),
            },
            BlockItem {
                name: "ch8_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8696,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Xctrl"),
                }),
            },
            BlockItem {
                name: "ch8_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12792,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Xctrl"),
                }),
            },
            BlockItem {
                name: "ch8_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4608,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch8_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8704,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch8_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12800,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch8Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch9_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4624,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Cfg"),
                }),
            },
            BlockItem {
                name: "ch9_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8720,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Cfg"),
                }),
            },
            BlockItem {
                name: "ch9_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12816,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Cfg"),
                }),
            },
            BlockItem {
                name: "ch9_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4628,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Loop"),
                }),
            },
            BlockItem {
                name: "ch9_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8724,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Loop"),
                }),
            },
            BlockItem {
                name: "ch9_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12820,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Loop"),
                }),
            },
            BlockItem {
                name: "ch9_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4632,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Ctrl"),
                }),
            },
            BlockItem {
                name: "ch9_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8728,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Ctrl"),
                }),
            },
            BlockItem {
                name: "ch9_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12824,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Ctrl"),
                }),
            },
            BlockItem {
                name: "ch9_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4636,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Src"),
                }),
            },
            BlockItem {
                name: "ch9_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8732,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Src"),
                }),
            },
            BlockItem {
                name: "ch9_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12828,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Src"),
                }),
            },
            BlockItem {
                name: "ch9_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4640,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Dst"),
                }),
            },
            BlockItem {
                name: "ch9_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8736,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Dst"),
                }),
            },
            BlockItem {
                name: "ch9_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12832,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Dst"),
                }),
            },
            BlockItem {
                name: "ch9_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4644,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Link"),
                }),
            },
            BlockItem {
                name: "ch9_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8740,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Link"),
                }),
            },
            BlockItem {
                name: "ch9_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12836,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Link"),
                }),
            },
            BlockItem {
                name: "ch9_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4648,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Xctrl"),
                }),
            },
            BlockItem {
                name: "ch9_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8744,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Xctrl"),
                }),
            },
            BlockItem {
                name: "ch9_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12840,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Xctrl"),
                }),
            },
            BlockItem {
                name: "ch9_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4656,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch9_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8752,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch9_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12848,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch9Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch10_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4672,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Cfg"),
                }),
            },
            BlockItem {
                name: "ch10_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8768,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Cfg"),
                }),
            },
            BlockItem {
                name: "ch10_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12864,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Cfg"),
                }),
            },
            BlockItem {
                name: "ch10_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4676,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Loop"),
                }),
            },
            BlockItem {
                name: "ch10_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8772,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Loop"),
                }),
            },
            BlockItem {
                name: "ch10_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12868,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Loop"),
                }),
            },
            BlockItem {
                name: "ch10_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4680,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Ctrl"),
                }),
            },
            BlockItem {
                name: "ch10_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8776,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Ctrl"),
                }),
            },
            BlockItem {
                name: "ch10_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12872,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Ctrl"),
                }),
            },
            BlockItem {
                name: "ch10_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4684,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Src"),
                }),
            },
            BlockItem {
                name: "ch10_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8780,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Src"),
                }),
            },
            BlockItem {
                name: "ch10_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12876,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Src"),
                }),
            },
            BlockItem {
                name: "ch10_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4688,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Dst"),
                }),
            },
            BlockItem {
                name: "ch10_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8784,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Dst"),
                }),
            },
            BlockItem {
                name: "ch10_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12880,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Dst"),
                }),
            },
            BlockItem {
                name: "ch10_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4692,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Link"),
                }),
            },
            BlockItem {
                name: "ch10_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8788,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Link"),
                }),
            },
            BlockItem {
                name: "ch10_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12884,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Link"),
                }),
            },
            BlockItem {
                name: "ch10_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4696,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Xctrl"),
                }),
            },
            BlockItem {
                name: "ch10_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8792,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Xctrl"),
                }),
            },
            BlockItem {
                name: "ch10_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12888,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Xctrl"),
                }),
            },
            BlockItem {
                name: "ch10_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4704,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch10_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8800,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch10_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12896,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch10Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch11_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4720,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Cfg"),
                }),
            },
            BlockItem {
                name: "ch11_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8816,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Cfg"),
                }),
            },
            BlockItem {
                name: "ch11_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12912,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Cfg"),
                }),
            },
            BlockItem {
                name: "ch11_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4724,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Loop"),
                }),
            },
            BlockItem {
                name: "ch11_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8820,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Loop"),
                }),
            },
            BlockItem {
                name: "ch11_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12916,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Loop"),
                }),
            },
            BlockItem {
                name: "ch11_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4728,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Ctrl"),
                }),
            },
            BlockItem {
                name: "ch11_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8824,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Ctrl"),
                }),
            },
            BlockItem {
                name: "ch11_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12920,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Ctrl"),
                }),
            },
            BlockItem {
                name: "ch11_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4732,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Src"),
                }),
            },
            BlockItem {
                name: "ch11_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8828,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Src"),
                }),
            },
            BlockItem {
                name: "ch11_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12924,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Src"),
                }),
            },
            BlockItem {
                name: "ch11_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4736,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Dst"),
                }),
            },
            BlockItem {
                name: "ch11_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8832,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Dst"),
                }),
            },
            BlockItem {
                name: "ch11_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12928,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Dst"),
                }),
            },
            BlockItem {
                name: "ch11_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4740,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Link"),
                }),
            },
            BlockItem {
                name: "ch11_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8836,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Link"),
                }),
            },
            BlockItem {
                name: "ch11_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12932,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Link"),
                }),
            },
            BlockItem {
                name: "ch11_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4744,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Xctrl"),
                }),
            },
            BlockItem {
                name: "ch11_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8840,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Xctrl"),
                }),
            },
            BlockItem {
                name: "ch11_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12936,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Xctrl"),
                }),
            },
            BlockItem {
                name: "ch11_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4752,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch11_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8848,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch11_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12944,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch11Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch12_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4768,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Cfg"),
                }),
            },
            BlockItem {
                name: "ch12_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8864,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Cfg"),
                }),
            },
            BlockItem {
                name: "ch12_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12960,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Cfg"),
                }),
            },
            BlockItem {
                name: "ch12_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4772,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Loop"),
                }),
            },
            BlockItem {
                name: "ch12_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8868,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Loop"),
                }),
            },
            BlockItem {
                name: "ch12_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12964,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Loop"),
                }),
            },
            BlockItem {
                name: "ch12_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4776,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Ctrl"),
                }),
            },
            BlockItem {
                name: "ch12_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8872,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Ctrl"),
                }),
            },
            BlockItem {
                name: "ch12_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12968,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Ctrl"),
                }),
            },
            BlockItem {
                name: "ch12_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4780,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Src"),
                }),
            },
            BlockItem {
                name: "ch12_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8876,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Src"),
                }),
            },
            BlockItem {
                name: "ch12_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12972,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Src"),
                }),
            },
            BlockItem {
                name: "ch12_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4784,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Dst"),
                }),
            },
            BlockItem {
                name: "ch12_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8880,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Dst"),
                }),
            },
            BlockItem {
                name: "ch12_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12976,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Dst"),
                }),
            },
            BlockItem {
                name: "ch12_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4788,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Link"),
                }),
            },
            BlockItem {
                name: "ch12_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8884,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Link"),
                }),
            },
            BlockItem {
                name: "ch12_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12980,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Link"),
                }),
            },
            BlockItem {
                name: "ch12_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4792,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Xctrl"),
                }),
            },
            BlockItem {
                name: "ch12_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8888,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Xctrl"),
                }),
            },
            BlockItem {
                name: "ch12_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12984,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Xctrl"),
                }),
            },
            BlockItem {
                name: "ch12_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4800,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch12_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8896,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch12_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 12992,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch12Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch13_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4816,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Cfg"),
                }),
            },
            BlockItem {
                name: "ch13_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8912,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Cfg"),
                }),
            },
            BlockItem {
                name: "ch13_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 13008,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Cfg"),
                }),
            },
            BlockItem {
                name: "ch13_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4820,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Loop"),
                }),
            },
            BlockItem {
                name: "ch13_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8916,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Loop"),
                }),
            },
            BlockItem {
                name: "ch13_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13012,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Loop"),
                }),
            },
            BlockItem {
                name: "ch13_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4824,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Ctrl"),
                }),
            },
            BlockItem {
                name: "ch13_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8920,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Ctrl"),
                }),
            },
            BlockItem {
                name: "ch13_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13016,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Ctrl"),
                }),
            },
            BlockItem {
                name: "ch13_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4828,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Src"),
                }),
            },
            BlockItem {
                name: "ch13_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8924,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Src"),
                }),
            },
            BlockItem {
                name: "ch13_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13020,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Src"),
                }),
            },
            BlockItem {
                name: "ch13_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4832,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Dst"),
                }),
            },
            BlockItem {
                name: "ch13_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8928,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Dst"),
                }),
            },
            BlockItem {
                name: "ch13_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13024,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Dst"),
                }),
            },
            BlockItem {
                name: "ch13_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4836,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Link"),
                }),
            },
            BlockItem {
                name: "ch13_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8932,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Link"),
                }),
            },
            BlockItem {
                name: "ch13_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13028,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Link"),
                }),
            },
            BlockItem {
                name: "ch13_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4840,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Xctrl"),
                }),
            },
            BlockItem {
                name: "ch13_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8936,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Xctrl"),
                }),
            },
            BlockItem {
                name: "ch13_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13032,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Xctrl"),
                }),
            },
            BlockItem {
                name: "ch13_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4848,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch13_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8944,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch13_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13040,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch13Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch14_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4864,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Cfg"),
                }),
            },
            BlockItem {
                name: "ch14_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8960,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Cfg"),
                }),
            },
            BlockItem {
                name: "ch14_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 13056,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Cfg"),
                }),
            },
            BlockItem {
                name: "ch14_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4868,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Loop"),
                }),
            },
            BlockItem {
                name: "ch14_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8964,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Loop"),
                }),
            },
            BlockItem {
                name: "ch14_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13060,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Loop"),
                }),
            },
            BlockItem {
                name: "ch14_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4872,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Ctrl"),
                }),
            },
            BlockItem {
                name: "ch14_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8968,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Ctrl"),
                }),
            },
            BlockItem {
                name: "ch14_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13064,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Ctrl"),
                }),
            },
            BlockItem {
                name: "ch14_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4876,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Src"),
                }),
            },
            BlockItem {
                name: "ch14_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8972,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Src"),
                }),
            },
            BlockItem {
                name: "ch14_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13068,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Src"),
                }),
            },
            BlockItem {
                name: "ch14_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4880,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Dst"),
                }),
            },
            BlockItem {
                name: "ch14_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8976,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Dst"),
                }),
            },
            BlockItem {
                name: "ch14_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13072,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Dst"),
                }),
            },
            BlockItem {
                name: "ch14_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4884,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Link"),
                }),
            },
            BlockItem {
                name: "ch14_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8980,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Link"),
                }),
            },
            BlockItem {
                name: "ch14_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13076,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Link"),
                }),
            },
            BlockItem {
                name: "ch14_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4888,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Xctrl"),
                }),
            },
            BlockItem {
                name: "ch14_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8984,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Xctrl"),
                }),
            },
            BlockItem {
                name: "ch14_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13080,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Xctrl"),
                }),
            },
            BlockItem {
                name: "ch14_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4896,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch14_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 8992,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch14_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13088,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch14Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch15_cfg_set",
                description: Some("Channel Configuration Register. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4912,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Cfg"),
                }),
            },
            BlockItem {
                name: "ch15_cfg_clr",
                description: Some("Channel Configuration Register. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 9008,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Cfg"),
                }),
            },
            BlockItem {
                name: "ch15_cfg_tgl",
                description: Some("Channel Configuration Register. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 13104,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Cfg"),
                }),
            },
            BlockItem {
                name: "ch15_loop_set",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4916,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Loop"),
                }),
            },
            BlockItem {
                name: "ch15_loop_clr",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9012,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Loop"),
                }),
            },
            BlockItem {
                name: "ch15_loop_tgl",
                description: Some(
                    "Channel Loop Counter Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13108,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Loop"),
                }),
            },
            BlockItem {
                name: "ch15_ctrl_set",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4920,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Ctrl"),
                }),
            },
            BlockItem {
                name: "ch15_ctrl_clr",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9016,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Ctrl"),
                }),
            },
            BlockItem {
                name: "ch15_ctrl_tgl",
                description: Some(
                    "Channel Descriptor Control Word Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13112,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Ctrl"),
                }),
            },
            BlockItem {
                name: "ch15_src_set",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4924,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Src"),
                }),
            },
            BlockItem {
                name: "ch15_src_clr",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9020,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Src"),
                }),
            },
            BlockItem {
                name: "ch15_src_tgl",
                description: Some(
                    "Channel Descriptor Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13116,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Src"),
                }),
            },
            BlockItem {
                name: "ch15_dst_set",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4928,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Dst"),
                }),
            },
            BlockItem {
                name: "ch15_dst_clr",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9024,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Dst"),
                }),
            },
            BlockItem {
                name: "ch15_dst_tgl",
                description: Some(
                    "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13120,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Dst"),
                }),
            },
            BlockItem {
                name: "ch15_link_set",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4932,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Link"),
                }),
            },
            BlockItem {
                name: "ch15_link_clr",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9028,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Link"),
                }),
            },
            BlockItem {
                name: "ch15_link_tgl",
                description: Some(
                    "Channel Descriptor Link Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13124,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Link"),
                }),
            },
            BlockItem {
                name: "ch15_xctrl_set",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4936,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Xctrl"),
                }),
            },
            BlockItem {
                name: "ch15_xctrl_clr",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9032,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Xctrl"),
                }),
            },
            BlockItem {
                name: "ch15_xctrl_tgl",
                description: Some(
                    "Channel Extended Descriptor Control Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13128,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Xctrl"),
                }),
            },
            BlockItem {
                name: "ch15_ilsrc_set",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-set alias)",
                ),
                array: None,
                byte_offset: 4944,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch15_ilsrc_clr",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-clr alias)",
                ),
                array: None,
                byte_offset: 9040,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Ilsrc"),
                }),
            },
            BlockItem {
                name: "ch15_ilsrc_tgl",
                description: Some(
                    "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1). (write-1-to-tgl alias)",
                ),
                array: None,
                byte_offset: 13136,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Ch15Ilsrc"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Ch0Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch0CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch0Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch0CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch0CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch0CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch0CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch0CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch0Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch0Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch0Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch0LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch0Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch0Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch0XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch10Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch10CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch10Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch10CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch10CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch10CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch10CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch10CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch10Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch10Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch10Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch10LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch10Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch10Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch10Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch10XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch11Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch11CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch11Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch11CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch11CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch11CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch11CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch11CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch11Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch11Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch11Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch11LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch11Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch11Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch11Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch11XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch12Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch12CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch12Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch12CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch12CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch12CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch12CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch12CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch12Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch12Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch12Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch12LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch12Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch12Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch12Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch12XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch13Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch13CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch13Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch13CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch13CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch13CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch13CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch13CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch13Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch13Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch13Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch13LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch13Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch13Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch13Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch13XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch14Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch14CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch14Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch14CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch14CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch14CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch14CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch14CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch14Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch14Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch14Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch14LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch14Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch14Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch14Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch14XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch15Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch15CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch15Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch15CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch15CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch15CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch15CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch15CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch15Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch15Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch15Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch15LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch15Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch15Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch15Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch15XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch1CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch1Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch1CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch1CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch1CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch1CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch1CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch1Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch1Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch1Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch1LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch1Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch1Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch1XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch2Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch2CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch2Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch2CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch2CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch2CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch2CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch2CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch2Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch2Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch2Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch2LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch2Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch2Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch2Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch2XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch3Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch3CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch3Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch3CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch3CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch3CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch3CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch3CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch3Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch3Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch3Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch3LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch3Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch3Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch3Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch3XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch4Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch4CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch4Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch4CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch4CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch4CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch4CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch4CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch4Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch4Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch4Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch4LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch4Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch4Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch4Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch4XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch5Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch5CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch5Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch5CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch5CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch5CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch5CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch5CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch5Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch5Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch5Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch5LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch5Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch5Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch5Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch5XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch6Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch6CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch6Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch6CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch6CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch6CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch6CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch6CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch6Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch6Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch6Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch6LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch6Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch6Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch6Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch6XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch7Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch7CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch7Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch7CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch7CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch7CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch7CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch7CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch7Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch7Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch7Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch7LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch7Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch7Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch7Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch7XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch8Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch8CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch8Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch8CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch8CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch8CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch8CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch8CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch8Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch8Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch8Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch8LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch8Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch8Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch8Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch8XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch9Cfg",
            extends: None,
            description: Some("Channel Configuration Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "arbslots",
                    description: Some("Arbitration Slot Number Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch9CfgArbslots"),
                },
                Field {
                    name: "srcincsign",
                    description: Some("Source Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CfgSrcincsign"),
                },
                Field {
                    name: "dstincsign",
                    description: Some("Destination Address Increment Sign."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CfgDstincsign"),
                },
                Field {
                    name: "structbusport",
                    description: Some("Structure Fetch Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CfgStructbusport"),
                },
                Field {
                    name: "srcbusport",
                    description: Some("Source Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CfgSrcbusport"),
                },
                Field {
                    name: "dstbusport",
                    description: Some("Destination Bus Port."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CfgDstbusport"),
                },
            ],
        },
        FieldSet {
            name: "Ch9Ctrl",
            extends: None,
            description: Some("Channel Descriptor Control Word Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "structtype",
                    description: Some("DMA Structure Type."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch9CtrlStructtype"),
                },
                Field {
                    name: "extend",
                    description: Some("Extend."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "structreq",
                    description: Some("Structure DMA Transfer Request."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xfercnt",
                    description: Some("DMA Unit Data Transfer Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "byteswap",
                    description: Some("Endian Byte Swap."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "blocksize",
                    description: Some("Block Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 4,
                    array: None,
                    enumm: Some("Ch9CtrlBlocksize"),
                },
                Field {
                    name: "doneien",
                    description: Some("DMA Operation Done Interrupt Flag Set."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 20 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reqmode",
                    description: Some("DMA Request Transfer Mode Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 21 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CtrlReqmode"),
                },
                Field {
                    name: "decloopcnt",
                    description: Some("Decrement Loop Count."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 22 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ignoresreq",
                    description: Some("Ignore Sreq."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 23 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcinc",
                    description: Some("Source Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch9CtrlSrcinc"),
                },
                Field {
                    name: "size",
                    description: Some("Unit Data Transfer Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 26 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch9CtrlSize"),
                },
                Field {
                    name: "dstinc",
                    description: Some("Destination Address Increment Size."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 28 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch9CtrlDstinc"),
                },
                Field {
                    name: "srcmode",
                    description: Some("Source Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 30 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CtrlSrcmode"),
                },
                Field {
                    name: "dstmode",
                    description: Some("Destination Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9CtrlDstmode"),
                },
            ],
        },
        FieldSet {
            name: "Ch9Dst",
            extends: None,
            description: Some(
                "Channel Descriptor Destination Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Destination Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch9Ilsrc",
            extends: None,
            description: Some(
                "Channel Extended Descriptor Interleaving Source Address Register (Writes will only take effect when EN=1).",
            ),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Interleave Source Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch9Link",
            extends: None,
            description: Some("Channel Descriptor Link Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "linkmode",
                    description: Some("Link Structure Addressing Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ch9LinkLinkmode"),
                },
                Field {
                    name: "link",
                    description: Some("Link Next Structure."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "linkaddr",
                    description: Some("Link Structure Address."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 30,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch9Loop",
            extends: None,
            description: Some("Channel Loop Counter Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "loopcnt",
                description: Some("Linked Structure Sequence Loop Counter."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch9Src",
            extends: None,
            description: Some("Channel Descriptor Source Address Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "addr",
                description: Some("Source Data Address."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ch9Xctrl",
            extends: None,
            description: Some("Channel Extended Descriptor Control Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dstilen",
                    description: Some("Destination Interleave."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ilmode",
                    description: Some("Interleave Mode."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ch9XctrlIlmode"),
                },
                Field {
                    name: "bufferable",
                    description: Some("Allow AHB buffering."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chbusy",
            extends: None,
            description: Some("Channel Busy Register."),
            bit_size: 32,
            fields: &[Field {
                name: "busy",
                description: Some("Channels Busy."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Chdis",
            extends: None,
            description: Some("Channel Disable Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "chdis",
                description: Some("DMA Channel disable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Chdone",
            extends: None,
            description: Some("Channel Linking Done Status Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chdone0",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone1",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone2",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone3",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone4",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone5",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone6",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone7",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone8",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone9",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone10",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone11",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone12",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone13",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone14",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chdone15",
                    description: Some("Channel Linking Done Status flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chen",
            extends: None,
            description: Some("Channel Enable Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "chen",
                description: Some("Channel Enables."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Chstatus",
            extends: None,
            description: Some("Channel Status Register."),
            bit_size: 32,
            fields: &[Field {
                name: "chstatus",
                description: Some("DMA Channel Status."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some("Control Register."),
            bit_size: 32,
            fields: &[Field {
                name: "numfixed",
                description: Some("Number of Fixed Priority Channels."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                bit_size: 6,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Dbghalt",
            extends: None,
            description: Some("Channel Debug Halt Register."),
            bit_size: 32,
            fields: &[Field {
                name: "dbghalt",
                description: Some("DMA Debug Halt."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "En",
            extends: None,
            description: Some("Module enable disable Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some("Module Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "disabling",
                    description: Some("Disablement busy status."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ien",
            extends: None,
            description: Some("Done Interrupt Enable Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "done0",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done1",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done2",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done3",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done4",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done5",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done6",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done7",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done8",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done9",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done10",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done11",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done12",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done13",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done14",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done15",
                    description: Some("Done Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "error",
                    description: Some("Error Interrupt Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "If",
            extends: None,
            description: Some("Interrupt Flag Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "done0",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done1",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done2",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done3",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done4",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done5",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 5 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done6",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done7",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done8",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done9",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done10",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 10 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done11",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 11 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done12",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done13",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 13 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done14",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 14 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "done15",
                    description: Some("Done Interrupt Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 15 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "error",
                    description: Some("Error Flag."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 31 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ipversion",
            extends: None,
            description: Some("IP version register."),
            bit_size: 32,
            fields: &[Field {
                name: "ipversion",
                description: Some("IPVERSION."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Linkload",
            extends: None,
            description: Some("Channel Link Load Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "linkload",
                description: Some("DMA Link Loads."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Reqclear",
            extends: None,
            description: Some("Channel Request Clear Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "reqclear",
                description: Some("DMA Request Clear."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Reqdis",
            extends: None,
            description: Some("Channel Request Disable Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "reqdis",
                description: Some("DMA Request Disables."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Reqpend",
            extends: None,
            description: Some("Channel Requests Pending Register."),
            bit_size: 32,
            fields: &[Field {
                name: "reqpend",
                description: Some("DMA Requests Pending."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Status",
            extends: None,
            description: Some("Status Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "anybusy",
                    description: Some("Any DMA Channel Busy."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "anyreq",
                    description: Some("Any DMA Channel Request Pending."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chgrant",
                    description: Some("Granted Channel Number."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cherror",
                    description: Some("Errant Channel Number."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "chnum",
                    description: Some("Number of Channels."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swreq",
            extends: None,
            description: Some("Channel Software Transfer Request (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "swreq",
                description: Some("Software Transfer Requests."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Swrst",
            extends: None,
            description: Some("Software Reset Register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swrst",
                    description: Some("Software Reset Command."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "resetting",
                    description: Some("Software Reset Busy Status."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Synchwen",
            extends: None,
            description: Some("Sync HW trigger enable register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syncseten",
                    description: Some("Hardware Sync Trigger Set Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncclren",
                    description: Some("Hardware Sync Trigger Clear Enable."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Synchwsel",
            extends: None,
            description: Some("Sync HW trigger selection register."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syncsetedge",
                    description: Some("Hardware Sync Trigger Set Edge Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 8,
                    array: None,
                    enumm: Some("Syncsetedge"),
                },
                Field {
                    name: "syncclredge",
                    description: Some("Hardware Sync Trigger Clear Edge Select."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 16 }),
                    bit_size: 8,
                    array: None,
                    enumm: Some("Syncclredge"),
                },
            ],
        },
        FieldSet {
            name: "Syncstatus",
            extends: None,
            description: Some("Sync Trigger Status Register."),
            bit_size: 32,
            fields: &[Field {
                name: "synctrig",
                description: Some("sync trig status."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Syncswclr",
            extends: None,
            description: Some("Sync Trig Sw Clear register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "syncswclr",
                description: Some("DMA SYNC Software Trigger Clear."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Syncswset",
            extends: None,
            description: Some("Sync Trig Sw Set Register (Writes will only take effect when EN=1)."),
            bit_size: 32,
            fields: &[Field {
                name: "syncswset",
                description: Some("DMA SYNC Software Trigger Set."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 8,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[
        Enum {
            name: "Ch0CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch0CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch0CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch0LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch0XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch10CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch10CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch10CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch10LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch10XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch11CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch11CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch11CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch11LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch11XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch12CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch12CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch12CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch12LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch12XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch13CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch13CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch13CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch13LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch13XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch14CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch14CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch14CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch14LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch14XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch15CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch15CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch15CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch15LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch15XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch1CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch1CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch1CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch1LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch1XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch2CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch2CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch2CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch2LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch2XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch3CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch3CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch3CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch3LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch3XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch4CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch4CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch4CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch4LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch4XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch5CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch5CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch5CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch5LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch5XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch6CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch6CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch6CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch6LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch6XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch7CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch7CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch7CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch7LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch7XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch8CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch8CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch8CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch8LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch8XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch9CfgArbslots",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Eight",
                    description: Some("Eight arbitration slots selected."),
                    value: 3,
                },
                EnumVariant {
                    name: "Four",
                    description: Some("Four arbitration slots selected."),
                    value: 2,
                },
                EnumVariant {
                    name: "One",
                    description: Some("One arbitration slot selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Two arbitration slots selected."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CfgDstbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CfgDstincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement destination address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment destination address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch9CfgSrcbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CfgSrcincsign",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Negative",
                    description: Some("Decrement source address."),
                    value: 1,
                },
                EnumVariant {
                    name: "Positive",
                    description: Some("Increment source address."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch9CfgStructbusport",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Ahbm0",
                    description: Some("AHBM0."),
                    value: 0,
                },
                EnumVariant {
                    name: "Ahbm1",
                    description: Some("AHBM1."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlBlocksize",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("Transfer all units as specified by the XFRCNT field."),
                    value: 15,
                },
                EnumVariant {
                    name: "Unit1",
                    description: Some("1 unit transfer per arbitration."),
                    value: 0,
                },
                EnumVariant {
                    name: "Unit1024",
                    description: Some("1024 unit transfers per arbitration."),
                    value: 14,
                },
                EnumVariant {
                    name: "Unit12",
                    description: Some("12 unit transfers per arbitration."),
                    value: 6,
                },
                EnumVariant {
                    name: "Unit128",
                    description: Some("128 unit transfers per arbitration."),
                    value: 11,
                },
                EnumVariant {
                    name: "Unit16",
                    description: Some("16 unit transfers per arbitration."),
                    value: 7,
                },
                EnumVariant {
                    name: "Unit2",
                    description: Some("2 unit transfers per arbitration."),
                    value: 1,
                },
                EnumVariant {
                    name: "Unit24",
                    description: Some("24 unit transfers per arbitration."),
                    value: 8,
                },
                EnumVariant {
                    name: "Unit256",
                    description: Some("256 unit transfers per arbitration."),
                    value: 12,
                },
                EnumVariant {
                    name: "Unit3",
                    description: Some("3 unit transfers per arbitration."),
                    value: 2,
                },
                EnumVariant {
                    name: "Unit32",
                    description: Some("32 unit transfers per arbitration."),
                    value: 9,
                },
                EnumVariant {
                    name: "Unit4",
                    description: Some("4 unit transfers per arbitration."),
                    value: 3,
                },
                EnumVariant {
                    name: "Unit512",
                    description: Some("512 unit transfers per arbitration."),
                    value: 13,
                },
                EnumVariant {
                    name: "Unit6",
                    description: Some("6 unit transfers per arbitration."),
                    value: 4,
                },
                EnumVariant {
                    name: "Unit64",
                    description: Some("64 unit transfers per arbitration."),
                    value: 10,
                },
                EnumVariant {
                    name: "Unit8",
                    description: Some("8 unit transfers per arbitration."),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlDstinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment destination address by four unit data sizes after each write."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment destination address by one unit data size after each write."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment destination address by two unit data sizes after each write."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlDstmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlReqmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "All",
                    description: Some("One transfer request transfers all units as defined by the XFRCNT field."),
                    value: 1,
                },
                EnumVariant {
                    name: "Block",
                    description: Some("The LDMA transfers one BLOCKSIZE per transfer request."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlSize",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Byte",
                    description: Some("Each unit transfer is a byte."),
                    value: 0,
                },
                EnumVariant {
                    name: "Halfword",
                    description: Some("Each unit transfer is a half-word."),
                    value: 1,
                },
                EnumVariant {
                    name: "Word",
                    description: Some("Each unit transfer is a word."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlSrcinc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Four",
                    description: Some("Increment source address by four unit data sizes after each read."),
                    value: 2,
                },
                EnumVariant {
                    name: "None",
                    description: Some(
                        "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "One",
                    description: Some("Increment source address by one unit data size after each read."),
                    value: 0,
                },
                EnumVariant {
                    name: "Two",
                    description: Some("Increment source address by two unit data sizes after each read."),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlSrcmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9CtrlStructtype",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Synchronize",
                    description: Some("Synchronization structure type selected."),
                    value: 1,
                },
                EnumVariant {
                    name: "Transfer",
                    description: Some("DMA transfer structure type selected."),
                    value: 0,
                },
                EnumVariant {
                    name: "Write",
                    description: Some("Write immediate value structure type selected."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Ch9LinkLinkmode",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the absolute address of the linked descriptor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative",
                    description: Some(
                        "The LINKADDR field of LDMA_CHx_LINK contains the relative offset of the linked descriptor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ch9XctrlIlmode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "Absolute",
                    description: Some("Address determined by value in rules. Size of WORD."),
                    value: 0,
                },
                EnumVariant {
                    name: "Relative16",
                    description: Some("Address determined by adding rules to DST. Size of HALFWORD."),
                    value: 1,
                },
                EnumVariant {
                    name: "Relative8",
                    description: Some("Address determined by adding rules to DST. Size of BYTE."),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Syncclredge",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "Fall",
                    description: Some("Use falling edge detection."),
                    value: 1,
                },
                EnumVariant {
                    name: "Rise",
                    description: Some("Use rising edge detection."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Syncsetedge",
            description: None,
            bit_size: 8,
            variants: &[
                EnumVariant {
                    name: "Fall",
                    description: Some("Use falling edge detection."),
                    value: 1,
                },
                EnumVariant {
                    name: "Rise",
                    description: Some("Use rising edge detection."),
                    value: 0,
                },
            ],
        },
    ],
};
