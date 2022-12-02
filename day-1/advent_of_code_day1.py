from collections import defaultdict

elf = 0
d = defaultdict(int)
with open("./day-1/input.txt", "r") as f:
    for line in f:
        if line == "\n":
            elf += 1
        else:
            d["elf-{}".format(elf)] += int(line)

part_1 = max(d.values())

part_2 = sum(sorted(d.values())[-3:])

print("{}\n{}".format(part_1, part_2))
