const { Command } = require('commander');
const {Scanner} = require('../index.js')
const pkgJson = require('../package.json')

const program = new Command();

program
  .name('js-audit')
  .description('CLI to audit Javascript/Typescript code')
  .version(pkgJson.version)
  .argument('<files>', 'files to audit')
  .option('--first', 'display just the first substring')
  .option('-s, --separator <char>', 'separator character', ',')
  .action((files, options) => {
    const scanner = new Scanner()
  scanner.scan(Buffer.from(files))
  scanner.report();
  });

program.parse();