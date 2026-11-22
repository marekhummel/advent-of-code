---@enum AocStatus
local Status = {
    NoSample = "no_sample",
    NoPartTwo = "no_part_two",
    NotImplemented = "not_implemented",
    Unsolved = "unsolved",
}

---@class AocStatusResult
---@field status AocStatus

---@param status AocStatus
---@return AocStatusResult
local function info(status)
    return { status = status }
end

local solution = {
    NoSample = info(Status.NoSample),
    NoPartTwo = info(Status.NoPartTwo),
    NotImplemented = info(Status.NotImplemented),
    Unsolved = info(Status.Unsolved),
}

---@type table<AocStatus, string>
local labels = {
    [Status.NoSample] = "<No Sample Defined>",
    [Status.NoPartTwo] = "<No Part Two>",
    [Status.NotImplemented] = "<Not Implemented>",
    [Status.Unsolved] = "<Unsolved>",
}

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

---@param result AocResult
---@return string
function solution.format(result)
    if type(result) == "string" or (type(result) == "number" and math.type(result) == "integer") then
        return tostring(result)
    end
    if type(result) == "table" then
        local label = labels[result.status]
        if label then
            return label
        end
    end
    error("A solution must return an integer, text or a known status")
end

return solution
