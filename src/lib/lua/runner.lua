local solution = require("lib.lua.solution")

local runner = {}

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

    local implementation = solutions[day]
    if implementation == nil then
        print(solution.format(solution.Unsolved))
        return
    end
    local solve = implementation[part == 1 and "part1" or "part2"]

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

    print(solution.format(solve(input, sample)))
end

return runner
