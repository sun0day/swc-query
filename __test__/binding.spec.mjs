import { Scanner } from '../index.js'
import {join} from 'node:path'
import process from 'node:process'

import test from 'ava'
console.log(process.cwd())
console.log(join(process.cwd(),'./__test__/fixtures/a.js'))

test('Scanner binding', (t) => {
  const scanner = new Scanner()
  scanner.scan(Buffer.from('./__test__/fixtures/a.js'))
  scanner.report();
  t.deepEqual(true, 'xxxx')
  // t.assert(true)
})