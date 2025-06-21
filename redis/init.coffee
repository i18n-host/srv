#!/usr/bin/env coffee

> @3-/read
  @3-/camel
  @3-/redis/R.js
  @3-/snake > SNAKE
  @3-/walk
  @3-/write
  fs > existsSync
  path > join dirname basename

PWD = import.meta.dirname
ROOT = dirname PWD

flagsDef = (name, flags)=>
  if flags.length && false # kvrocks hack https://github.com/apache/kvrocks/issues/2133
    def = \
    """
{function_name='#{name}',callback=#{name},flags={'#{flags.join('\',\'')}'}}
    """
  else
    def = "('#{name}',#{name})"
  return def

load = (mod, fp)=>
  li = []
  name_li = []

  for i from read(fp).split('\n')
    i = i.trimEnd()

    trimStart = i.trimStart()
    if trimStart.startsWith('--')
      i = trimStart.slice(3)
      if i.startsWith('flags ')
        flags = flags.concat i.slice(6).trim().split(' ')
      continue

    if i.startsWith('function ')
      flags = []
      name = i.slice(9,i.indexOf('(',10)).trim()
      i = 'local '+i+'\nredis.setresp(3)'
    else if ~i.indexOf('function(')
      name = undefined

    li.push i

    if i == 'end' and name
      def = flagsDef(name,flags)
      name_li.push name
      li.push "redis.register_function#{def}"


  if name_li.length
    lua_rs = name_li.map(
      (i)=>
        "pub const #{SNAKE i}: &str = \"#{i}\";\n"
    ).join('')

  return [
    li.join('\n').trimEnd()
    lua_rs
  ]

export default main = =>
  name = 'redis.lua'
  [code, rs] = load('',join PWD, name)
  write(
    join PWD, 'r_/src/lib.rs'
    rs
  )
  console.log "-- #{name}\n\n"+code

  code = '#!lua name=hpc\n'+code

  # base = dirname ROOT
  # for mod from MOD
  #   rustdir = join base,'mod',mod
  #   lua = join rustdir, 'redis.lua'
  #   if existsSync lua
  #     [c,rs] = load(mod, lua)
  #     code += c
  #     console.log "\n-- #{mod}\n"+c
  #     if rs
  #       lua_rs = join rustdir,'src/LUA.rs'
  #       write(
  #         lua_rs
  #         rs
  #       )

  await R.function('LOAD','REPLACE', code)
  return

if process.argv[1] == decodeURI (new URL(import.meta.url)).pathname
  await main()
  process.exit()
