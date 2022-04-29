#! /usr/bin/env node
// eslint-disable-next-line @typescript-eslint/no-var-requires
const fs = require('fs');

let errlog = fs.createWriteStream('/tmp/kittymux.log');
process.stderr.write = errlog.write.bind(errlog);
require('../dist/index.js');
