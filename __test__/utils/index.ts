import {execSync} from 'node:child_process'
import process from 'node:process'
import {join, isAbsolute} from 'node:path'
import {platform} from 'node:os'

const root = process.cwd()
const cmdPath = join(root, "bin/cmd.js")
const getPath = (file: string) => isAbsolute(file) ? file : join(root, '__test__/', file)

export const isWin = platform().includes("win32");

export const audit = (file: string) => {
  return execSync(`node ${cmdPath} ${getPath(file)}`).toString('utf8')
}

export const getSnapshotFile = (file: string) => {
  return getPath(file.replace(/\.(js|ts)$/, '.snapshot').replace(/\/code\//, "/expected/"))
}