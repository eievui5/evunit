use std::io::{Error, Write};

use gb_cpu_sim::memory;

#[derive(Clone)]
pub struct AddressSpace<'a> {
	pub rom: &'a Vec<u8>,
	pub vram: [u8; 0x2000], // VRAM locking is not emulated as there is not PPU present.
	pub sram: Vec<[u8; 0x2000]>,
	pub wram: [u8; 0x1000 * 8],
	// Accessing echo ram will throw a warning.
	pub oam: [u8; 0x100], // This includes the 105 unused bytes of OAM; they will throw a warning.
	                      // All MMIO registers are special-cased; many serve no function.
}

impl memory::AddressSpace for AddressSpace<'_> {
	fn read(&self, address: u16) -> u8 {
		let address = address as usize;
		match address {
			0x0000..=0x3FFF => self.rom[address],
			0xC000..=0xDFFF => self.wram[address - 0xC000],
			_ => panic!("Unimplemented address range for {address}"),
		}
	}

	fn write(&mut self, address: u16, value: u8) {
		let address = address as usize;
		match address {
			0x0000..=0x3FFF => eprintln!("Wrote to ROM (MBC registers are not yet emulated)"),
			0xC000..=0xDFFF => self.wram[address - 0xC000] = value,
			_ => panic!("Unimplemented address range for {address}"),
		};
	}
}

impl AddressSpace<'_> {
	pub fn with(rom: &Vec<u8>) -> AddressSpace {
		AddressSpace {
			rom,
			vram: [0; 0x2000],
			sram: vec![],
			wram: [0; 0x1000 * 8],
			oam: [0; 0x100],
		}
	}

	pub fn dump<W: Write>(&self, mut file: W) -> Result<(), Error> {
		let mut output = String::from("");

		let mut address = 0x8000;
		output += "[VRAM]";
		for byte in self.vram {
			if address % 16 == 0 {
				output += format!("\n0x{address:x}:").as_str();
			}
			output += format!(" 0x{byte:x}").as_str();
			address += 1;
		}
		output += "\n";

		let mut address = 0xC000;
		output += "[WRAM 0]";
		for i in 0..0x2000 {
			if address % 16 == 0 {
				output += format!("\n0x{address:x}:").as_str();
			}
			output += format!(" 0x{:x}", self.vram[i]).as_str();
			address += 1;
		}
		output += "\n";

		file.write_all(output.as_bytes())
	}
}
