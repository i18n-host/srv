#!/usr/bin/env coffee

# dist cron to deno deploy for mod

> zx/globals:
  ./sh/MOD_LI.js:@ > DIR_MOD
  path > join basename
  fs > existsSync

$.verbose = true
await $"#{DIR_MOD}/sh/cronEnv.sh"

dist = (dir)=>
  cron = join dir, 'cron'
  if not existsSync cron
    return
  cd cron
  name = basename(dir).replace(/_$/,'')
  await $"#{DIR_MOD}/sh/cronDist.sh #{name}"
  return

for dir from MOD_LI
  await dist dir

process.exit()

