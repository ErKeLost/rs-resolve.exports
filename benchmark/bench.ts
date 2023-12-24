import b from 'benny'
import { loop } from './bench-js'
import { plus100 } from '../index'

import * as resolve from 'resolve.exports'

// // package.json contents
// const pkg = {
//   name: 'foobar',
//   module: 'dist/module.mjs',
//   main: 'dist/require.js',
//   imports: {
//     '#hash': {
//       import: {
//         browser: './hash/web.mjs',
//         node: './hash/node.mjs',
//       },
//       default: './hash/detect.js',
//     },
//   },
//   exports: {
//     '.': {
//       import: './dist/module.mjs',
//       require: './dist/require.js',
//     },
//     './lite': {
//       worker: {
//         browser: './lite/worker.browser.js',
//         node: './lite/worker.node.js',
//       },
//       import: './lite/module.mjs',
//       require: './lite/require.js',
//     },
//   },
// }
function factorial(n) {
  if (n === 0 || n === 1) {
      return 1;
  } else {
      return n * factorial(n - 1);
  }
}

async function run() {
  await b.suite(
    'resolve.exports',

    b.add('Native', () => {
      plus100(600)
    }),

    b.add('JavaScript', () => {
      // resolve.exports(pkg, 'foobar')
      factorial(600)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
