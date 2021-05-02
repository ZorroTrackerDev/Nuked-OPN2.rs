use nuked_opn2_sys::*;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ChipType {
    YM2612 = ym3438_mode_ym2612,
    YM3438 = ym3438_mode_readmode
}

#[derive(Debug)]
pub struct Chip {
    ptr: Box<ym3438_t>
}

impl Chip {
    pub fn new() -> Self {
        let mut chip = unsafe {
            Self {
                ptr: Box::new(std::mem::zeroed::<ym3438_t>())
            }
        };
        chip.reset();
        chip
    }

    pub fn with_type(chip_type: ChipType) -> Self {
        let mut chip = Self::new();
        chip.set_type(chip_type);
        chip
    }

    pub fn reset(&mut self) {
        unsafe {
            OPN2_Reset(&mut *self.ptr);
        }
    }

    pub fn set_type(&mut self, chip_type: ChipType) {
        unsafe {
            OPN2_SetChipType(&mut *self.ptr, chip_type as u32);
        }
    }

    pub fn clock(&mut self) -> [i16; 2] {
        let mut buffer: [i16; 2] = [0; 2];

        unsafe {
            OPN2_Clock(&mut *self.ptr, buffer.as_mut_ptr());
        }

        buffer
    }

    pub fn write(&mut self, port: u8, data: u8) {
        assert!(port <= 3);

        unsafe {
            OPN2_Write(&mut *self.ptr, port as u32, data);
        }
    }
}



