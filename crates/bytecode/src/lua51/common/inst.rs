use crate::{
	lua51::common::types::SerialziedInst,
	shared::macros::{opcodes, operand, opmode},
};

#[derive(Debug)]
pub struct Inst {
	serialized: SerialziedInst,
}

impl From<SerialziedInst> for Inst {
	fn from(serialized: SerialziedInst) -> Self {
		Self { serialized }
	}
}

impl Inst {
	operand!(opcode, set_opcode, u8, 26..32);

	operand!(a, set_a, u8, 18..25);
	operand!(c, set_c, u16, 9..17);
	operand!(b, set_b, u16, 0..8);
	operand!(bx, set_bx, u32, 0..17);
	operand!(sbx, set_sbx, u32, 0..17);

	opmode!(ia, a: u8);
	opmode!(iab, a: u8, b: u16);
	opmode!(iac, a: u8, c: u16);
	opmode!(iabc, a: u8, b: u16, c: u16);

	opmode!(iabx, a: u8, bx: u32);

	opmode!(isbx, sbx: u32);
	opmode!(iasbx, a: u8, sbx: u32);
}

// as per https://stevedonovan.github.io/lua-5.1.4/lopcodes.h.html
opcodes!(
	0 => Move: iab,
	1 => LoadK: iabx,
	2 => LoadBool: iabc,
	3 => LoadNil: iab,
	4 => GetUpval: iab,

	5 => GetGlobal: iabx,
	6 => GetTable: iabc,

	7 => SetGlobal: iabx,
	8 => SetUpval: iab,
	9 => SetTable: iabc,

	10 => NewTable: iabc,

	11 => Self_: iabc,

	12 => Add: iabc,
	13 => Sub: iabc,
	14 => Mul: iabc,
	15 => Div: iabc,
	16 => Mod: iabc,
	17 => Pow: iabc,
	18 => Unm: iab,
	19 => Not: iab,
	20 => Len: iab,

	21 => Concat: iabc,

	22 => Jmp: isbx,

	23 => Eq: iabc,
	24 => Lt: iabc,
	25 => Le: iabc,

	26 => Test: iac,
	27 => TestSet: iabc,

	28 => Call: iabc,
	29 => TailCall: iabc,
	30 => Return: iab,

	31 => ForLoop: iasbx,

	32 => ForPrep: iasbx,

	33 => TForLoop: iac,

	34 => SetList: iabc,

	35 => Close: ia,
	36 => Closure: iabx,

	37 => Vararg: iab
);

mod tests {
	#![allow(unused_imports)]
	use super::{Inst, Opcode};

	#[test]
	fn test_deserialize() {
		let opcode = 0u32;
		let a = 1u32;
		let b = 2u32;
		let c = 1u32;
		let instruction = (opcode << 26 | a << 18 | c << 9 | b) as usize;

		let inst: Inst = instruction.into();
		assert_eq!(inst.iabc(), (a as u8, b as u16, c as u16));

		let opcode = Opcode::deserialize(instruction);
		println!("{:?}", opcode);
	}
}
