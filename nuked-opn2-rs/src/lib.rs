use std::str::FromStr;

use nuked_opn2_sys::*;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ChipType {
    YM2612 = ym3438_mode_ym2612,
    ASICYM3438 = ym3438_mode_readmode,
    DiscreteYM3438 = ym3438_mode_ym2612 | ym3438_mode_readmode
}

impl FromStr for ChipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "YM2612" => Ok(Self::YM2612),
            "ASICYM3438" => Ok(Self::ASICYM3438),
            "DiscreteYM3438" => Ok(Self::DiscreteYM3438),
            _ => Err(())
        }
    }
}

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

    pub fn clock(&mut self) -> [i32; 2] {
        let mut buffer: [i32; 2] = [0; 2];

        unsafe {
            OPN2_Clock(&mut *self.ptr, buffer.as_mut_ptr());
        }

        buffer
    }

    pub fn write(&mut self, port: u32, data: u8) {
        unsafe {
            OPN2_Write(&mut *self.ptr, port, data);
        }
    }

    pub fn set_test_pin(&mut self, value: u32) {
        unsafe {
            OPN2_SetTestPin(&mut *self.ptr, value);
        }
    }

    pub fn read_test_pin(&mut self) -> u32 {
        unsafe {
            OPN2_ReadTestPin(&mut *self.ptr)
        }
    }

    pub fn read_irq_pin(&mut self) -> u32 {
        unsafe {
            OPN2_ReadIRQPin(&mut *self.ptr)
        }
    }

    pub fn read(&mut self, port: u32) -> u8 {
        unsafe {
            OPN2_Read(&mut *self.ptr, port)
        }
    }

    pub fn set_clock_rate(&mut self, clock: u32, rate: u32) {
        unsafe {
            OPN2_SetClockRate(&mut *self.ptr, clock, rate);
        }
    }

    pub fn reset_with_clock_rate(&mut self, clock: u32, rate: u32) {
        self.reset();
        self.set_clock_rate(clock, rate);
    }

    pub fn write_buffered(&mut self, port: u8, data: u8) {
        unsafe {
            OPN2_WriteBuffered(&mut *self.ptr, port, data);
        }
    }

    pub fn generate_resampled(&mut self) -> [i32; 2]  {
        let mut buffer: [i32; 2] = [0; 2];

        unsafe {
            OPN2_GenerateResampled(&mut *self.ptr, buffer.as_mut_ptr());
        }

        buffer
    }

    pub fn update(&mut self, samples_size: usize) -> Vec<i32> {
        let mut buffer = Vec::<i32>::with_capacity(samples_size * 2);
        
        for _ in 0..samples_size {
            let sample = self.generate_resampled();
            buffer.extend_from_slice(&sample)
        }

        buffer
    }
}
