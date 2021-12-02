let testInput = """
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""

func lines(input: String) -> [String.SubSequence] {
    input
        .split(separator: "\n")
}

enum Command {
    case forward(Int)
    case down(Int)
    case up(Int)

    init?(string: String.SubSequence) {
        let split = string.split(separator: " ")
        guard split.count == 2 else {
            return nil
        }
        let command = split[0]
        guard let x = Int(split[1]) else {
            return nil
        }
        switch command {
        case "forward":
            self = .forward(x)
        case "down":
            self = .down(x)
        case "up":
            self = .up(x)
        default:
            return nil
        }
    }
}

struct Submarine {
    var position: (horizontal: Int, depth: Int, aim: Int) = (0, 0, 0)

    var answer: Int {
        return position.horizontal * position.depth
    }

    mutating func apply(command: Command) {
        switch command {
        case .up(let x):
            self.position.aim -= x
        case .down(let x):
            self.position.aim += x
        case .forward(let x):
            self.position.horizontal += x
            self.position.depth += self.position.aim * x
        }
    }

    mutating func apply(commands: [Command]) {
        for command in commands {
            self.apply(command: command)
        }
    }
}

let commands = lines(input: testInput)
    .map(Command.init(string:))
    .map { $0! }

var submarine = Submarine()
submarine.apply(commands: commands)

print(submarine.position)
print(submarine.answer)
