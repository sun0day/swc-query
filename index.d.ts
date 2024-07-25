import { Scanner as InternalScanner } from './binding';
export declare class Scanner extends InternalScanner {
    constructor(root?: string);
    scan(file: string | Buffer): Buffer;
}
export * from './binding';
