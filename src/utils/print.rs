

#[non_exhaustive]
pub enum UARTError {
    NonEmptyLSR,
}

#[macro_export]
macro_rules! unsafeprintln {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut uart = QemuUart {
            base: 0x10000000,
            thr: 0x10000000 as *mut u8,
            lsr: 0x10000005 as *mut u8,
            lsr_empty_mask: 0x20,
        };
        let _= uart.write_fmt(format_args!("{}\n", format_args!($($arg)*)));
    }};
}

pub struct QemuUart {
    pub base: usize,
    pub thr: *mut u8,
    pub lsr: *mut u8,
    pub lsr_empty_mask: u8,
}

impl QemuUart {
    fn try_write_byte(&self, byte: u8) -> Result<(), UARTError> {
        let is_lsr_empty =
            (unsafe { core::ptr::read_volatile(self.lsr) } & self.lsr_empty_mask) != 0;

        if is_lsr_empty {
            unsafe {
                core::ptr::write_volatile(self.thr, byte);
            }
            Ok(())
        } else {
            Err(UARTError::NonEmptyLSR)
        }
    }
}

impl core::fmt::Write for QemuUart {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for byte in s.bytes() {
            while let Err(_) = self.try_write_byte(byte) {}
        }
        while (unsafe { core::ptr::read_volatile(self.lsr) } & self.lsr_empty_mask) == 0 {}

        Ok(())
    }
}
