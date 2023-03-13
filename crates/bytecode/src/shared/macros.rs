macro_rules! operand {
	($operand:ident, $write_operand:ident, $type:ty, $range:expr) => {
		pub fn $operand(&self) -> $type {
			((self.serialized & ((1 << ($range.end - $range.start + 1)) - 1) << $range.start)
				>> $range.start) as $type
		}
		pub fn $write_operand(&mut self, bits: $type) {
			self.serialized = (self.serialized
				& !(((1 << ($range.end - $range.start + 1)) - 1) << $range.start))
				| (bits as usize) << $range.start
		}
	};
}

macro_rules! opmode {
	($opmode:ident, $($operand:ident: $type:ty),+ ) => {
		pub fn $opmode(&self) -> ($($type, )*) {
			($(
				self.$operand(),
			)*)
		}
	};
}

macro_rules! opcodes {
	($($idx:expr => $name:ident: $opmode:ident),+) => {

		#[repr(u8)]
		#[derive(Debug)]
		enum Opcode {
			$(
				$name(Inst) = $idx,
			)*
		}

		impl Opcode {
			pub fn deserialize(serialized: SerialziedInst) -> Self {
				let inst: Inst = serialized.into();
				let opcode = inst.opcode();

				match opcode {
					$($idx => Opcode::$name(inst),)*
					_ => unimplemented!()
				}
			}
		}
	};
}

pub(crate) use opcodes;
pub(crate) use operand;
pub(crate) use opmode;
