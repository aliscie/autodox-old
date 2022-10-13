if (typeof BigInt === 'undefined') global.BigInt = require('big-integer')
// the above line to solve the error
//     [Error] Unhandled Promise Rejection: ReferenceError: Can't find variable: BigInt
//         (anonymous function)
//         promiseReactionJob

import('./pkg').then((module) => {
    module.run()
})
