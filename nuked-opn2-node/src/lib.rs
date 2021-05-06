use std::cell::RefCell;

use nuked_opn2_rs::*;
use neon::{context::Context, prelude::*};

fn f64_to_u8(number: f64) -> u8 {
    (number % f64::from(u8::MAX)).floor() as u8
}

fn f64_to_u32(number: f64) -> u32 {
    (number % f64::from(u32::MAX)).floor() as u32
}

pub struct ChipWrapper {
    pub chip: Chip
}

impl Finalize for ChipWrapper {}

type BoxedChip = JsBox<RefCell<ChipWrapper>>;

fn new(mut cx: FunctionContext) -> JsResult<BoxedChip> {
    Ok(cx.boxed(
        RefCell::new(
            ChipWrapper { chip: Chip::new()})
        )
    )
}

fn with_type(mut cx: FunctionContext) -> JsResult<BoxedChip> {
    let maybe_chip_type = cx.argument::<JsString>(0)?.value(&mut cx);
    let chip_type: ChipType = maybe_chip_type.parse().unwrap();
    Ok(cx.boxed(
        RefCell::new( 
            ChipWrapper {chip: Chip::with_type(chip_type)})
        )
    )
}

fn reset(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);
    chip.chip.reset();

    Ok(cx.undefined())
}

fn set_type(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let maybe_chip_type = cx.argument::<JsString>(1)?.value(&mut cx);
    let chip_type: ChipType = maybe_chip_type.parse().unwrap();

    chip.chip.set_type(chip_type);

    Ok(cx.undefined())
}

fn clock(mut cx: FunctionContext) -> JsResult<JsArray> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let buf = chip.chip.clock();
    let array = JsArray::new(&mut cx, 2);

    for (index, data) in buf.iter().enumerate() {
        let number = JsNumber::new(&mut cx, *data);
        array.set(&mut cx, index as u32, number)?;
    }

    Ok(array)
}

fn write(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let port = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let port = f64_to_u32(port);

    let data = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let data = f64_to_u8(data);

    chip.chip.write(port, data);

    Ok(cx.undefined())
}

fn set_test_pin(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let value = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let value = f64_to_u32(value);

    chip.chip.set_test_pin(value);

    Ok(cx.undefined())
}

fn read_test_pin(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    Ok(JsNumber::new(&mut cx, chip.chip.read_test_pin()))
}

fn read_irq_pin(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    Ok(JsNumber::new(&mut cx, chip.chip.read_irq_pin()))
}

fn read(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let port = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let port = f64_to_u32(port);

    Ok(JsNumber::new(&mut cx, chip.chip.read(port)))
}

fn set_clock_rate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let clock = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let clock = f64_to_u32(clock);

    let rate = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let rate = f64_to_u32(rate);

    chip.chip.set_clock_rate(clock, rate);


    Ok(cx.undefined())
}

fn reset_with_clock_rate(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let clock = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let clock = f64_to_u32(clock);

    let rate = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let rate = f64_to_u32(rate);

    chip.chip.reset_with_clock_rate(clock, rate);

    Ok(cx.undefined())
}


fn write_buffered(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let port = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let port = f64_to_u8(port);

    let data = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let data = f64_to_u8(data);

    chip.chip.write_buffered(port, data);

    Ok(cx.undefined())
}

fn generate_resampled(mut cx: FunctionContext) -> JsResult<JsArray> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let buf = chip.chip.generate_resampled();
    let array = JsArray::new(&mut cx, 2);

    for (index, data) in buf.iter().enumerate() {
        let number = JsNumber::new(&mut cx, *data);
        array.set(&mut cx, index as u32, number)?;
    }

    Ok(array)
}

fn update(mut cx: FunctionContext) -> JsResult<JsArray> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let samples_size = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let samples_size = f64_to_u32(samples_size);

    let buf = chip.chip.update(samples_size as usize);
    let array = JsArray::new(&mut cx, buf.len() as u32);

    for (index, data) in buf.iter().enumerate() {
        let number = JsNumber::new(&mut cx, *data);
        array.set(&mut cx, index as u32, number)?;
    }

    Ok(array)
}

fn set_mutemask(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let chip = *cx.argument::<BoxedChip>(0)?;
    let mut chip = RefCell::borrow_mut(&chip);

    let mutemask = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let mutemask = f64_to_u8(mutemask);

    chip.chip.set_mutemask(mutemask);

    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("new", new)?;
    cx.export_function("withType", with_type)?;

    cx.export_function("reset", reset)?;
    cx.export_function("setType", set_type)?;
    cx.export_function("clock", clock)?;
    cx.export_function("write", write)?;
    cx.export_function("setTestPin", set_test_pin)?;

    cx.export_function("readTestPin", read_test_pin)?;
    cx.export_function("readIrqPin", read_irq_pin)?;
    cx.export_function("read", read)?;

    cx.export_function("setClockRate", set_clock_rate)?;
    cx.export_function("resetWithClockRate", reset_with_clock_rate)?;
    cx.export_function("writeBuffered", write_buffered)?;
    cx.export_function("generateResampled", generate_resampled)?;
    cx.export_function("update", update)?;
    cx.export_function("setMutemask", set_mutemask)?;

    Ok(())
});
