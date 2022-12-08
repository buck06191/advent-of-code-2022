type Items = Array<Array<string>>;

interface Instruction {
  move?: number;
  from?: number;
  to?: number;
}

interface ParsedOutput {
  items: Items;
  instructions: Array<Instruction>;
}

const getInputAsArray = async (
  filepath: string,
): Promise<ParsedOutput> => {
  const text = await Deno.readTextFile(filepath);
  const [layout, rawInstructions] = text.split("\n\n");
  const crates = layout.split("\n");

  const numCrates = (crates[0].length + 1) / 4;

  const items: Array<Array<string>> = [];

  for (let i = 0; i < numCrates; i++) {
    items.push([]);
  }

  crates.reverse().forEach((line) => {
    for (let i = 1; i <= line.length; i += 4) {
      const v = line[i];
      if (/[A-Z]/.test(v)) {
        items[Math.floor(i / 4)].push(v);
      }
    }
  });

  const instructionRegEx = /move\s([0-9]+)\sfrom\s([0-9]+)\sto\s([0-9]+)/;

  const instructions = rawInstructions.split("\n").map((instruction) => {
    const matches = instruction.match(instructionRegEx);

    const i = matches?.slice(1).map((n) => parseInt(n)) as number[];

    return {
      move: i[0],
      from: i[1],
      to: i[2],
    };
  });

  return { items, instructions };
};

const executeInstruction = (
  instructions: Array<Instruction>,
  items: Items,
  canDoMultiple = false,
): string => {
  console.log({ items });
  if (items) {
    for (const instruction of instructions) {
      const { move, from, to } = instruction;
      if (move && from && to) {
        if (!canDoMultiple) {
          for (let i = 0; i < move; i++) {
            const temp = items[from - 1].pop();
            temp && items[to - 1].push(temp);
          }
        } else {
          const split = items[from - 1].length - move;

          const shifted = items[from - 1].slice(split);
          const remainder = items[from - 1].slice(0, split);

          items[from - 1] = remainder;
          items[to - 1] = items[to - 1].concat(shifted);
        }
      }
    }
  }

  return items.map((stack) => stack.at(-1)).join("");
};

const run = async () => {
  const { items, instructions } = await getInputAsArray("./input.txt");

  const part1 = executeInstruction(instructions, items);

  const part2 = executeInstruction(instructions, items, true);

  console.log({ part1, part2 });
};

// Learn more at https://deno.land/manual/examples/module_metadata#concepts
if (import.meta.main) {
  await run();
}
