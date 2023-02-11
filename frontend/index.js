if (typeof BigInt === 'undefined') global.BigInt = require('big-integer')

import init from './pkg/frontend.js';

init().then((instance) => {
    instance.run();
})