use crate::metadata::ir::*;

pub static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "SyscfgNsCfgns",
        extends: None,
        description: Some("SYSCFG_NS_CFGNS peripheral."),
        items: &[
            BlockItem {
                name: "cfgnstcalib",
                description: Some("Configure to define the system tick for the M33."),
                array: None,
                byte_offset: 28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgnstcalib"),
                }),
            },
            BlockItem {
                name: "rootnsdata0",
                description: Some("Generic data space for user to pass to root, e.g., address of struct in mem."),
                array: None,
                byte_offset: 1536,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rootnsdata0"),
                }),
            },
            BlockItem {
                name: "rootnsdata1",
                description: Some("Generic data space for user to pass to root, e.g., address of struct in mem."),
                array: None,
                byte_offset: 1540,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rootnsdata1"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfgnstcalib",
            extends: None,
            description: Some("Configure to define the system tick for the M33."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tenms",
                    description: Some("Ten Milliseconds."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "skew",
                    description: Some("Skew."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 24 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "noref",
                    description: Some("No Reference."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 25 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rootnsdata0",
            extends: None,
            description: Some("Generic data space for user to pass to root, e.g., address of struct in mem."),
            bit_size: 32,
            fields: &[Field {
                name: "data",
                description: Some("Data."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Rootnsdata1",
            extends: None,
            description: Some("Generic data space for user to pass to root, e.g., address of struct in mem."),
            bit_size: 32,
            fields: &[Field {
                name: "data",
                description: Some("Data."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 32,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[],
};
