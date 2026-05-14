use crate::metadata::ir::*;

pub static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Scratchpad",
        extends: None,
        description: Some("SCRATCHPAD peripheral."),
        items: &[
            BlockItem {
                name: "sreg0",
                description: Some("Used for SIMCTRL Pointer in Verification Environment."),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sreg0"),
                }),
            },
            BlockItem {
                name: "sreg1",
                description: Some("Used for SIMCTRL Data Access in Verification Environment."),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sreg1"),
                }),
            },
            BlockItem {
                name: "sreg0_set",
                description: Some("Used for SIMCTRL Pointer in Verification Environment. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4096,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Sreg0"),
                }),
            },
            BlockItem {
                name: "sreg0_clr",
                description: Some("Used for SIMCTRL Pointer in Verification Environment. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8192,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Sreg0"),
                }),
            },
            BlockItem {
                name: "sreg0_tgl",
                description: Some("Used for SIMCTRL Pointer in Verification Environment. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12288,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Sreg0"),
                }),
            },
            BlockItem {
                name: "sreg1_set",
                description: Some("Used for SIMCTRL Data Access in Verification Environment. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4100,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Sreg1"),
                }),
            },
            BlockItem {
                name: "sreg1_clr",
                description: Some("Used for SIMCTRL Data Access in Verification Environment. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8196,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Sreg1"),
                }),
            },
            BlockItem {
                name: "sreg1_tgl",
                description: Some("Used for SIMCTRL Data Access in Verification Environment. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12292,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Sreg1"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Sreg0",
            extends: None,
            description: Some("Used for SIMCTRL Pointer in Verification Environment."),
            bit_size: 32,
            fields: &[Field {
                name: "scratch",
                description: Some("Scratch Pad Register."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sreg1",
            extends: None,
            description: Some("Used for SIMCTRL Data Access in Verification Environment."),
            bit_size: 32,
            fields: &[Field {
                name: "scratch",
                description: Some("Scratch Register."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
