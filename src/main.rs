use std::env;

#[derive(Default)]
struct Registers {
  // 32, 8-bit registers
  // 3 special 16-bit registers for addressing
  _x: u16,
  _y: u16,
  _z: u16,
  // status register
  _status: u8, // Todo: make this a separate struct for access to individual bits
  _stack: u16,  // Todo: make this a separate struct for access to SPH and SPL

  _reg_osccal: u8, // Oscillator Calibration Register
  _reg_clkpr: u8,  // Clock prescaler Register
  _reg_smcr: u8,   // Sleep Mode Control Register
  _reg_prr: u8,    // Power Reduction Register
  _reg_mcusr: u8,  // MCU Status Register
  _reg_wdtcsr: u8, // Watchdog Timer Register
  _reg_eicra: u8,  // External Interrupt Control Register A
  _reg_eimsk: u8,  // External Interrupt Mask Register
  _reg_eifr: u8,   // External Interrupt Flag Register
  _reg_pcicr: u8,  // Pin Change Interrupt Control Register
  _reg_pcifr: u8,  // Pin Change Interrupt Flag Register
  _reg_pcmsk0: u8, // Pin Change Mask Register 0
  _reg_pcmsk1: u8, // Pin Change Mask Register 1
  _reg_pcmsk2: u8  // Pin Change Mask Register 2
}

#[derive(Default)]
struct Eeprom {
  _reg_eearh: u8,  // EEPROM Address Register high
  _reg_eearl: u8,  // EEPROM Address Register low
  _reg_eerd: u8,   // EEPROM Data Register
  _reg_eecr: u8    // EEPROM Control Register
}

#[derive(Default)]
struct Memory {
  // 32 Registers:         0x0000 - 0x001F
  // 64 I/O Registers:     0x0020 - 0x005F
  // 160 Ext IO Registers: 0x0060 - 0x00FF
  // Internal SRAM
}

#[derive(Default)]
struct Fuses {
  // Storage for the system fuses.  This isn't generally included in the .bin
  // dumps.
}

#[derive(Default)]
struct Atmega328p {
  // 32K Flash Memory
  // 1K EEProm
  // 2K RAM

  // Stack Pointer points to the top of the stack.  Starts at MemHigh location
}

impl Atmega328p {
  fn new() -> Atmega328p {
    Atmega328p {
    }
  }
}

fn main() {
  let bin_file = env::args().nth(1).unwrap();
  let _ = Atmega328p::new();
  println!("{:#?}", bin_file);
}
