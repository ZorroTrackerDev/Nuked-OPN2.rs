const fs = require('fs');

const pkg = fs.readFileSync("./package.json")
const json = JSON.parse(pkg)

console.log(json.name)
