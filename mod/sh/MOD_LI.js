import {
  join,
  dirname
} from 'path';

import read from '@3-/read';

import {
  parse
} from '@iarna/toml';


export const DIR_MOD = dirname(import.meta.dirname);
export const ROOT = dirname(DIR_MOD);

const MOD_LI = [];

Object.values(parse(read(join(DIR_MOD, 'Cargo.toml'))).dependencies).forEach(({path}) => {
  if (path) {
    MOD_LI.push(join(DIR_MOD, path));
  }
})

export default MOD_LI
