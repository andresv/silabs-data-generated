use crate::metadata::ir::*;

pub static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Lcdrf",
        extends: None,
        description: Some("LCDRF peripheral."),
        items: &[
            BlockItem {
                name: "rfimlcdctrl",
                description: Some("No Description."),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rfimlcdctrl"),
                }),
            },
            BlockItem {
                name: "rfimlcdctrl_set",
                description: Some("No Description. (write-1-to-set alias)"),
                array: None,
                byte_offset: 4096,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Rfimlcdctrl"),
                }),
            },
            BlockItem {
                name: "rfimlcdctrl_clr",
                description: Some("No Description. (write-1-to-clr alias)"),
                array: None,
                byte_offset: 8192,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Rfimlcdctrl"),
                }),
            },
            BlockItem {
                name: "rfimlcdctrl_tgl",
                description: Some("No Description. (write-1-to-tgl alias)"),
                array: None,
                byte_offset: 12288,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Rfimlcdctrl"),
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "Rfimlcdctrl",
        extends: None,
        description: Some("No Description."),
        bit_size: 32,
        fields: &[
            Field {
                name: "lcdcpxoen",
                description: Some("LCD Charge Pump XO Clock Enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lcdcpxosel",
                description: Some("LCD Charge Pump XO Select."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                bit_size: 1,
                array: None,
                enumm: Some("Lcdcpxosel"),
            },
            Field {
                name: "lcdcpxoretimeen",
                description: Some("LCD Charge Pump XO Retime Enable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lcdlownoise",
                description: Some("LCD Low Noise."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 3 }),
                bit_size: 1,
                array: None,
                enumm: Some("Lcdlownoise"),
            },
            Field {
                name: "lcdcmpdout",
                description: Some("LCD Comparator Dout."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[
        Enum {
            name: "Lcdcpxosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Hfxodiv",
                    description: Some("HFXO divided 4 clock."),
                    value: 1,
                },
                EnumVariant {
                    name: "Intrco",
                    description: Some("Internal LCD CP 10Mhz RC oscillator."),
                    value: 0,
                },
            ],
        },
        Enum {
            name: "Lcdlownoise",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Normal",
                    description: Some("Normal operation."),
                    value: 0,
                },
                EnumVariant {
                    name: "Slow",
                    description: Some(
                        "slows down slew rate to reduce RF interference at a cost of additional power consumption.",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
