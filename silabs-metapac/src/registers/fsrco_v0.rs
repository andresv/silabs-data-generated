use crate::metadata::ir::*;

pub static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Fsrco",
        extends: None,
        description: Some("FSRCO peripheral."),
        items: &[BlockItem {
            name: "ipversion",
            description: Some("No Description."),
            array: None,
            byte_offset: 0,
            inner: BlockItemInner::Register(Register {
                access: Access::Read,
                bit_size: 32,
                fieldset: Some("Ipversion"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Ipversion",
        extends: None,
        description: Some("No Description."),
        bit_size: 32,
        fields: &[Field {
            name: "ipversion",
            description: Some("IP Version."),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
            bit_size: 32,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
