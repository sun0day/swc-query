import process from 'node:process'
import {Scanner as InternalScanner} from './binding'

export class Scanner extends InternalScanner {
  constructor(root = process.cwd()) {
    super(Buffer.from(root));
  }

  scan(file: string | Buffer): Buffer {
    return super.scan(Buffer.isBuffer(file) ? file : Buffer.from(file))
  }
}

export * from './binding'