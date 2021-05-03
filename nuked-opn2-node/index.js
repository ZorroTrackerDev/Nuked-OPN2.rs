const addon = require("./addon.node")

class Chip {
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
}

const YM2612 = "YM2612"
const YM3438 = "YM3438"

const newYM2612Chip = () => new Chip(YM2612)
const newYM3438Chip = () => new Chip(YM3438)

exports = module.exports = {
    YM2612,
    YM3438,
    newYM2612Chip,
    newYM3438Chip,
    Chip
}