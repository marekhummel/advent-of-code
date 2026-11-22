package.path = "src/?.lua;" .. package.path

local runner = require("lib.lua.runner")
local Solution = require("lib.lua.solution")
local day01 = require("2023.lua.solutions.day01")
-- aoc:imports

runner.run(2023, {
    [1] = Solution.from(day01),
    -- aoc:solutions
}, arg)
