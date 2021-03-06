//! Opcode definitions.

#![allow(non_upper_case_globals)]

pub const aaload: u8 = 0x32;
pub const aastore: u8 = 0x53;
pub const aconst_null: u8 = 0x1;
pub const aload: u8 = 0x19;
pub const aload_0: u8 = 0x2a;
pub const aload_1: u8 = 0x2b;
pub const aload_2: u8 = 0x2c;
pub const aload_3: u8 = 0x2d;
pub const anewarray: u8 = 0xbd;
pub const areturn: u8 = 0xb0;
pub const arraylength: u8 = 0xbe;
pub const astore: u8 = 0x3a;
pub const astore_0: u8 = 0x4b;
pub const astore_1: u8 = 0x4c;
pub const astore_2: u8 = 0x4d;
pub const astore_3: u8 = 0x4e;
pub const athrow: u8 = 0xbf;
pub const baload: u8 = 0x33;
pub const bastore: u8 = 0x54;
pub const bipush: u8 = 0x10;
pub const breakpoint: u8 = 0xca;
pub const caload: u8 = 0x34;
pub const castore: u8 = 0x55;
pub const checkcast: u8 = 0xc0;
pub const d2f: u8 = 0x90;
pub const d2i: u8 = 0x8e;
pub const d2l: u8 = 0x8f;
pub const dadd: u8 = 0x63;
pub const daload: u8 = 0x31;
pub const dastore: u8 = 0x52;
pub const dcmpg: u8 = 0x98;
pub const dcmpl: u8 = 0x97;
pub const dconst_0: u8 = 0x0e;
pub const dconst_1: u8 = 0x0f;
pub const ddiv: u8 = 0x6f;
pub const dload: u8 = 0x18;
pub const dload_0: u8 = 0x26;
pub const dload_1: u8 = 0x27;
pub const dload_2: u8 = 0x28;
pub const dload_3: u8 = 0x29;
pub const dmul: u8 = 0x6b;
pub const dneg: u8 = 0x77;
pub const drem: u8 = 0x73;
pub const dreturn: u8 = 0xaf;
pub const dstore: u8 = 0x39;
pub const dstore_0: u8 = 0x47;
pub const dstore_1: u8 = 0x48;
pub const dstore_2: u8 = 0x49;
pub const dstore_3: u8 = 0x4a;
pub const dsub: u8 = 0x67;
pub const dup: u8 = 0x59;
pub const dup2: u8 = 0x5c;
pub const dup2_x1: u8 = 0x5d;
pub const dup2_x2: u8 = 0x5e;
pub const dup_x1: u8 = 0x5a;
pub const dup_x2: u8 = 0x5b;
pub const f2d: u8 = 0x8d;
pub const f2i: u8 = 0x8b;
pub const f2l: u8 = 0x8c;
pub const fadd: u8 = 0x62;
pub const faload: u8 = 0x30;
pub const fastore: u8 = 0x51;
pub const fcmpg: u8 = 0x96;
pub const fcmpl: u8 = 0x95;
pub const fconst_0: u8 = 0x0b;
pub const fconst_1: u8 = 0x0c;
pub const fconst_2: u8 = 0x0d;
pub const fdiv: u8 = 0x6e;
pub const fload: u8 = 0x17;
pub const fload_0: u8 = 0x22;
pub const fload_1: u8 = 0x23;
pub const fload_2: u8 = 0x24;
pub const fload_3: u8 = 0x25;
pub const fmul: u8 = 0x6a;
pub const fneg: u8 = 0x76;
pub const frem: u8 = 0x72;
pub const freturn: u8 = 0xae;
pub const fstore: u8 = 0x38;
pub const fstore_0: u8 = 0x43;
pub const fstore_1: u8 = 0x44;
pub const fstore_2: u8 = 0x45;
pub const fstore_3: u8 = 0x46;
pub const fsub: u8 = 0x66;
pub const getfield: u8 = 0xb4;
pub const getstatic: u8 = 0xb2;
pub const goto: u8 = 0xa7;
pub const goto_w: u8 = 0xc8;
pub const i2b: u8 = 0x91;
pub const i2c: u8 = 0x92;
pub const i2d: u8 = 0x87;
pub const i2f: u8 = 0x86;
pub const i2l: u8 = 0x85;
pub const i2s: u8 = 0x93;
pub const iadd: u8 = 0x60;
pub const iaload: u8 = 0x2e;
pub const iand: u8 = 0x7e;
pub const iastore: u8 = 0x4f;
pub const iconst_0: u8 = 0x3;
pub const iconst_1: u8 = 0x4;
pub const iconst_2: u8 = 0x5;
pub const iconst_3: u8 = 0x6;
pub const iconst_4: u8 = 0x7;
pub const iconst_5: u8 = 0x8;
pub const iconst_m1: u8 = 0x2;
pub const idiv: u8 = 0x6c;
pub const if_acmpeq: u8 = 0xa5;
pub const if_acmpne: u8 = 0xa6;
pub const if_icmpeq: u8 = 0x9f;
pub const if_icmpge: u8 = 0xa2;
pub const if_icmpgt: u8 = 0xa3;
pub const if_icmple: u8 = 0xa4;
pub const if_icmplt: u8 = 0xa1;
pub const if_icmpne: u8 = 0xa0;
pub const ifeq: u8 = 0x99;
pub const ifge: u8 = 0x9c;
pub const ifgt: u8 = 0x9d;
pub const ifle: u8 = 0x9e;
pub const iflt: u8 = 0x9b;
pub const ifne: u8 = 0x9a;
pub const ifnonnull: u8 = 0xc7;
pub const ifnull: u8 = 0xc6;
pub const iinc: u8 = 0x84;
pub const iload: u8 = 0x15;
pub const iload_0: u8 = 0x1a;
pub const iload_1: u8 = 0x1b;
pub const iload_2: u8 = 0x1c;
pub const iload_3: u8 = 0x1d;
pub const impdep1: u8 = 0xfe;
pub const impdep2: u8 = 0xff;
pub const imul: u8 = 0x68;
pub const ineg: u8 = 0x74;
pub const instanceof: u8 = 0xc1;
pub const invokedynamic: u8 = 0xba;
pub const invokeinterface: u8 = 0xb9;
pub const invokespecial: u8 = 0xb7;
pub const invokestatic: u8 = 0xb8;
pub const invokevirtual: u8 = 0xb6;
pub const ior: u8 = 0x80;
pub const irem: u8 = 0x70;
pub const ireturn: u8 = 0xac;
pub const ishl: u8 = 0x78;
pub const ishr: u8 = 0x7a;
pub const istore: u8 = 0x36;
pub const istore_0: u8 = 0x3b;
pub const istore_1: u8 = 0x3c;
pub const istore_2: u8 = 0x3d;
pub const istore_3: u8 = 0x3e;
pub const isub: u8 = 0x64;
pub const iushr: u8 = 0x7c;
pub const ixor: u8 = 0x82;
pub const jsr: u8 = 0xa8;
pub const jsr_w: u8 = 0xc9;
pub const l2d: u8 = 0x8a;
pub const l2f: u8 = 0x89;
pub const l2i: u8 = 0x88;
pub const ladd: u8 = 0x61;
pub const laload: u8 = 0x2f;
pub const land: u8 = 0x7f;
pub const lastore: u8 = 0x50;
pub const lcmp: u8 = 0x94;
pub const lconst_0: u8 = 0x9;
pub const lconst_1: u8 = 0x0a;
pub const ldc: u8 = 0x12;
pub const ldc2_w: u8 = 0x14;
pub const ldc_w: u8 = 0x13;
pub const ldiv: u8 = 0x6d;
pub const lload: u8 = 0x16;
pub const lload_0: u8 = 0x1e;
pub const lload_1: u8 = 0x1f;
pub const lload_2: u8 = 0x20;
pub const lload_3: u8 = 0x21;
pub const lmul: u8 = 0x69;
pub const lneg: u8 = 0x75;
pub const lookupswitch: u8 = 0xab;
pub const lor: u8 = 0x81;
pub const lrem: u8 = 0x71;
pub const lreturn: u8 = 0xad;
pub const lshl: u8 = 0x79;
pub const lshr: u8 = 0x7b;
pub const lstore: u8 = 0x37;
pub const lstore_0: u8 = 0x3f;
pub const lstore_1: u8 = 0x40;
pub const lstore_2: u8 = 0x41;
pub const lstore_3: u8 = 0x42;
pub const lsub: u8 = 0x65;
pub const lushr: u8 = 0x7d;
pub const lxor: u8 = 0x83;
pub const monitorenter: u8 = 0xc2;
pub const monitorexit: u8 = 0xc3;
pub const multianewarray: u8 = 0xc5;
pub const new: u8 = 0xbb;
pub const newarray: u8 = 0xbc;
pub const pop: u8 = 0x57;
pub const pop2: u8 = 0x58;
pub const putfield: u8 = 0xb5;
pub const putstatic: u8 = 0xb3;
pub const ret: u8 = 0xa9;
pub const _return: u8 = 0xb1;
pub const saload: u8 = 0x35;
pub const sastore: u8 = 0x56;
pub const sipush: u8 = 0x11;
pub const swap: u8 = 0x5f;
pub const tableswitch: u8 = 0xaa;
pub const wide: u8 = 0xc4;
