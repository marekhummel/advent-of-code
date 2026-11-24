---@enum AocStatusResult
local Status = {
    NoSample = "<No Sample Defined>",
    NoPartTwo = "<No Part Two>",
    NotImplemented = "<Not Implemented>",
    Unsolved = "<Unsolved>",
}

local solution = { Status = Status }

---@alias AocResult integer|string|AocStatusResult
---@alias AocPart fun(input: string[], sample: boolean): AocResult
---@class AocSolution
---@field part1 AocPart
---@field part2 AocPart

---@param day AocSolution
---@return AocSolution
function solution.from(day)
    assert(type(day.part1) == "function", "A solution must provide part1")
    assert(type(day.part2) == "function", "A solution must provide part2")
    return { part1 = day.part1, part2 = day.part2 }
end

return solution
