if (typeof BigInt === 'undefined') global.BigInt = require('big-integer')

import init, {run} from './pkg/frontend.js';

init().then(() => {
    run()
})

