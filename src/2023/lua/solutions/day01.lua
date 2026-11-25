local stringx = require("pl.stringx")

local words = stringx.split("one two three four five six seven eight nine")

---@param line string
---@param spelled boolean
---@return integer
local function calibration(line, spelled)
    local first, last

    for i = 1, #line do
        -- Check digit
        local value = tonumber(line:sub(i, i))

        -- Check spelled number
        if value == nil and spelled then
            for number, word in ipairs(words) do
                if stringx.startswith(line:sub(i), word) then
                    value = number
                    break
                end
            end
        end

        -- Update first and last found
        if value then
            first = first or value
            last = value
        end
    end

    assert(first and last, "No digit found")
    return first * 10 + last
end

---@param input string[]
---@param _sample boolean
---@return AocResult
local function part1(input, _sample)
    local sum = 0
    for _, line in ipairs(input) do
        sum = sum + calibration(line, false)
    end

    return sum
end

---@param input string[]
---@param _sample boolean
---@return AocResult
local function part2(input, _sample)
    local sum = 0
    for _, line in ipairs(input) do
        sum = sum + calibration(line, true)
    end

    return sum
end

return { part1 = part1, part2 = part2 }
