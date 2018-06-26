const testFolder = 'target/deploy/';
const fs = require('fs');
const brotli = require('brotli');
fs.readdirSync(testFolder).forEach(file => {
    let fileName = testFolder + file;
    let data = brotli.compress(fs.readFileSync(fileName)));
    fs.writeFileSync(fileName, data);}
);