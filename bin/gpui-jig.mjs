#!/usr/bin/env node
import { cp, mkdir, rm } from 'node:fs/promises';
import { resolve } from 'node:path';

const args = process.argv.slice(2);
if (args.length === 1 && ['--help', '-h'].includes(args[0])) {
  console.log('Usage: gpui-jig create <new-directory>');
} else if (args.length === 2 && args[0] === 'create' && args[1]) {
  const destination = resolve(args[1]);
  try {
    await mkdir(destination);
    try {
      await cp(new URL('../starter/', import.meta.url), destination, { recursive: true, force: false, errorOnExist: true });
    } catch (error) {
      await rm(destination, { recursive: true });
      throw error;
    }
    console.log('Created ' + destination + '. Run cargo run --release inside that directory.');
  } catch (error) {
    console.error(error.code === 'EEXIST' ? 'Choose a new directory. The destination already exists.' : error.message);
    process.exitCode = 1;
  }
} else {
  console.error('Usage: gpui-jig create <new-directory>');
  process.exitCode = 2;
}
