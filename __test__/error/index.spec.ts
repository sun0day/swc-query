import {describe, expect, it} from 'vitest'
import {audit, getSnapshotFile, isWin} from '../test-utils'

describe('test error report', () => {
  it('should report syntax error', () => {
    const file = "error/syntax.js"
    expect(audit(file)).toMatchFileSnapshot(getSnapshotFile(file))
  })

  it('should report file read error', () => {
    if(isWin) {
      const file = "error/win32-file-read.js"
      expect(audit(file)).toMatchFileSnapshot(getSnapshotFile(file))
    } else {
      const file = "error/file-read.js"
      expect(audit(file)).toMatchFileSnapshot(getSnapshotFile(file))
    }
  })
})