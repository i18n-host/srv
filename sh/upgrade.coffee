#!/usr/bin/env coffee

> zx/globals:
  ./MOD_LI.js
  path > join
  fs > existsSync

$.verbose = true

upgrade = (dir)=>
  cron = join dir, 'cron'
  if not existsSync cron
    return
  cd cron
  await $"deno outdated --update"
  return

for dir from MOD_LI
  await upgrade dir

process.exit()

