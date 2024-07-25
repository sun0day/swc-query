import {describe, expect, it} from 'vitest'
import {audit, getSnapshotFile} from '../test-utils'

describe('test error report', () => {
  it('should report syntax error', () => {
    const file = "error/syntax.js"
    expect(audit(file)).toMatchFileSnapshot(getSnapshotFile(file))
  })
})