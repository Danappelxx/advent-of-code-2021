let testInput = """
199
200
208
210
200
207
240
269
260
263
"""

func numIncreasing(input: [Int]) -> Int {
    var increasing = 0

    _ = input.reduce(nil) { (prev: Int?, curr: Int) -> Int? in
        guard let prev = prev else {
            return curr
        }
        if curr > prev {
            increasing += 1
        }
        return curr
    }

    return increasing
}

func prepare(input: String) -> [Int] {
    return input
        .split(separator: "\n")
        .map { Int($0)! }
}

print(numIncreasing(input: prepare(input: testInput)))
