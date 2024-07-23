import { Scanner } from '../index.js'

import test from 'ava'


test('Scanner binding', (t) => {
  const scanner = new Scanner()
  t.deepEqual(JSON.parse(scanner.scan(Buffer.from('./__test__/fixtures/a.js'))), 'xxxx')
  // t.assert(true)
})