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

local runner = {
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
---@field part1? AocPart
---@field part2? AocPart

---@param result AocResult
---@return string
function runner.format(result)
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

---Run one DAY PART MODE case; batches and verification belong to the scripts.
---@param year integer
---@param solutions table<integer, AocSolution>
---@param args string[]
function runner.run(year, solutions, args)
    local day = tonumber(args[1])
    local part = tonumber(args[2])
    local mode = args[3]
    local last_day = year < 2025 and 25 or 12
    if #args ~= 3
        or not day
        or day % 1 ~= 0
        or day < 1
        or day > last_day
        or (part ~= 1 and part ~= 2)
        or (mode ~= "real" and mode ~= "sample")
    then
        io.stderr:write("Usage: main.lua DAY PART MODE (real|sample)\n")
        os.exit(2)
    end

    local solution = solutions[day]
    if solution == nil then
        print(runner.format(runner.Unsolved))
        return
    end
    local solve = solution[part == 1 and "part1" or "part2"]
    if solve == nil then
        print(runner.format(runner.NotImplemented))
        return
    end

    local sample = mode == "sample"
    local prefix = sample and "sample" or "input"
    local stem = string.format("inputs/%d/%s%02d", year, prefix, day)
    local filename = stem .. ".txt"
    local file, message, code = io.open(filename, "rb")
    -- Only a missing file should select the part-specific fallback.
    if not file and code == 2 then
        filename = string.format("%s_%d.txt", stem, part)
        file, message = io.open(filename, "rb")
    end
    if not file then
        io.stderr:write("Error: Cannot read input: " .. tostring(message) .. "\n")
        os.exit(1)
    end

    local input = {}
    for line in file:lines() do
        input[#input + 1] = line:gsub("\r$", "")
    end
    file:close()

    print(runner.format(solve(input, sample)))
end

return runner
