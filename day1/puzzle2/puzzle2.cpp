#include <algorithm>
#include <charconv>
#include <cstdint>
#include <iostream>
#include <ranges>
#include <vector>

#include "input.hpp"

int main() {
  std::cout << "Solving Advent of Code 2024 Day 1 Puzzle 2." << std::endl;

  std::vector<uint64_t> a;
  std::vector<uint64_t> b;

  for (auto line_ : kPuzzleInput | std::views::split('\n')) {
    for (auto num_ : std::string_view(line_.begin(), line_.end()) |
                         std::views::split(' ')) {
      auto num = std::string_view(num_.begin(), num_.end());
      // C++ views library doesn't provide a way to skip multiple whitespaces,
      // yet.
      if (num.empty()) {
        continue;
      }

      unsigned long long result;
      auto [ptr, err] =
          std::from_chars(num.data(), num.data() + num.size(), result);

      if (err != std::errc() || ptr != num.data() + num.size()) {
        continue;
      }

      if (b.size() < a.size()) {
        b.push_back(result);
      } else {
        a.push_back(result);
      }
    }
  }

  size_t similarity = 0;
  for (auto num : a) {
    size_t count = std::count(b.begin(), b.end(), num);
    similarity += num * count;
  }

  std::cout << "The similarity score is " << similarity << std::endl;
}
