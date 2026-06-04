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
            BlockItem {
                name: "cfgnsrpuratd0",
                description: Some(
                    "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.",
                ),
                array: None,
                byte_offset: 1544,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgnsrpuratd0"),
                }),
            },
            BlockItem {
                name: "cfgnsrpuratd12",
                description: Some(
                    "Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4.",
                ),
                array: None,
                byte_offset: 1592,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cfgnsrpuratd12"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cfgnsrpuratd0",
            extends: None,
            description: Some("Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ratdcfgnstcalib",
                    description: Some("CFGNSTCALIB Protection Bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ratdcfgnssystic",
                    description: Some("CFGNSSYSTIC Protection Bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgnsrpuratd12",
            extends: None,
            description: Some("Protected register address = (RPURATD register index X 32 + RPURATD bit index) X 4."),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ratdrootnsdata0",
                    description: Some("DATA0 Protection Bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ratdrootnsdata1",
                    description: Some("DATA1 Protection Bit."),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
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
