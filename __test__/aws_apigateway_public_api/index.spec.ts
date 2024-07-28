import {describe, expect, it} from 'vitest'
import {audit, getSnapshotFile} from '../utils'

const cwd = 'aws_apigateway_public_api'

describe('test error report', () => {
  it('should report syntax error', () => {
    const file = `${cwd}/code/index.ts`
    expect(audit(file)).toMatchFileSnapshot(getSnapshotFile(file))
  })
})