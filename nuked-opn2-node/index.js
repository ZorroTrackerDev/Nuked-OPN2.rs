const addon = require("./build/nuked-opn2-node.node")

class YMChip {
    constructor(type) {
        this.inner = addon.withType(type)
    }

    reset() {
        addon.reset(this.inner)
    }

    setType(type) {
        addon.setType(this.inner, type)
    }

    clock() {
        return addon.clock(this.inner)
    }

    write(port, data) {
        addon.write(this.inner, port, data)
    }

    setTestPin(value) {
        addon.setTestPin(this.inner, value)
    }

    readTestPin() {
        return addon.readTestPin(this.inner)
    }
    
    readIrqPin() {
        return addon.readIrqPin(this.inner)
    }

    read(port) {
        return addon.read(this.inner, port)
    }

    setClockRate(clock, rate) {
        addon.setClockRate(this.inner, clock, rate)
    }

    resetWithClockRate(clock, rate) {
        addon.resetWithClockRate(this.inner, clock, rate)
    }

    writeBuffered(port, data) {
        addon.writeBuffered(this.inner, port, data)
    }

    generateResampled() {
        return addon.generateResampled(this.inner)
    }

    update(samplesSize) {
        return addon.update(this.inner, samplesSize)
    }

    setMutemask(mutemask) {
        return addon.setMutemask(this.inner, mutemask)
    }
}

const YM2612 = "YM2612"
const ASICYM3438 = "ASICYM3438"
const DiscreteYM3438 = "DiscreteYM3438"
const YM2612WithMD1 = "YM2612WithMD1"

const newYM2612Chip = () => new YMChip(YM2612)
const newASICYM3438Chip = () => new YMChip(ASICYM3438)
const newDiscreteYM3438Chip = () => new YMChip(DiscreteYM3438)
const newYM2612WithMD1Chip = () => new YMChip(YM2612WithMD1)

exports = module.exports = {
    YM2612,
    ASICYM3438,
    DiscreteYM3438,
    YM2612WithMD1,

    newYM2612Chip,
    newASICYM3438Chip,
    newDiscreteYM3438Chip,
    newYM2612WithMD1Chip,
    
    YMChip
}