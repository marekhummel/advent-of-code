#include "runner.hpp"

#include <charconv>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string_view>

namespace aoc {

int run(int argc, char* argv[], int year, const std::unordered_map<int, Solution>& solutions) {
    int day = 0;
    int part = 0;
    const auto parse = [](std::string_view text, int& value) {
        const auto [end, error] = std::from_chars(text.data(), text.data() + text.size(), value);
        return error == std::errc{} && end == text.data() + text.size();
    };
    if (argc != 4 || !parse(argv[1], day) || !parse(argv[2], part) || day < 1 ||
        day > (year < 2025 ? 25 : 12) || part < 1 || part > 2 ||
        (std::string_view(argv[3]) != "real" && std::string_view(argv[3]) != "sample")) {
        std::cerr << "Usage: mainYEAR DAY PART MODE (real|sample)\n";
        return 2;
    }

    const auto entry = solutions.find(day);
    if (entry == solutions.end()) {
        std::cout << Unsolved << '\n';
        return 0;
    }
    const auto solve = part == 1 ? entry->second.part1 : entry->second.part2;
    if (solve == nullptr) {
        std::cout << NotImplemented << '\n';
        return 0;
    }

    try {
        const bool sample = std::string_view(argv[3]) == "sample";
        const auto stem = std::string(sample ? "sample" : "input") + (day < 10 ? "0" : "") +
                          std::to_string(day);
        const auto directory = std::filesystem::path("inputs") / std::to_string(year);
        auto filename = directory / (stem + ".txt");
        if (!std::filesystem::exists(filename)) {
            filename = directory / (stem + "_" + std::to_string(part) + ".txt");
        }
        std::ifstream file(filename);
        if (!file) {
            throw std::runtime_error("Cannot read input: " + filename.string());
        }
        Input input;
        for (std::string line; std::getline(file, line);) {
            if (!line.empty() && line.back() == '\r') {
                line.pop_back();
            }
            input.push_back(line);
        }
        if (file.bad()) {
            throw std::runtime_error("Error reading input: " + filename.string());
        }
        std::visit([](const auto& answer) { std::cout << answer; }, solve(input, sample));
        std::cout << '\n';
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "Error: " << error.what() << '\n';
        return 1;
    }
}

} // namespace aoc
