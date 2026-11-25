package = "aoc"
version = "dev-1"
source = { url = "git+https://github.com/marekhummel/advent-of-code.git" }
description = { summary = "Advent of Code solution dependencies", license = "UNLICENSED" }
dependencies = {
    "lua >= 5.4, < 5.5",
    "penlight == 1.15.0-1",
}
build = { type = "builtin", modules = {} }
