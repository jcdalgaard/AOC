import fs from "fs/promises";

let nodes = {};
let commands = "";
let lokation = "AAA";
let startinglokations = [];

async function readFile(file) {
  try {
    const data = await fs.readFile(file, "utf8");
    return data;
  } catch (err) {
    console.error(`${err}`);
  }
}
const handleData = (data) => {
  let splitDate = data.split("\n");
  commands = splitDate[0];
  for (let index = 2; index < splitDate.length; index++) {
    let element = splitDate[index];
    let id = element[0] + element[1] + element[2];
    if (element[2] === "A") {
      startinglokations.push(id);
    }

    let l = element[7] + element[8] + element[9];
    let r = element[12] + element[13] + element[14];

    nodes[id] = { r: r, l: l };
  }
};
function calculateroute() {
  let commandssplit = commands.split("");
  let steps = 0;
  while (true) {
    for (let index = 0; index < commandssplit.length; index++) {
      const element = commandssplit[index];
      if (lokation === "ZZZ") {
        return steps;
      }
      if (element == "R") {
        lokation = nodes[lokation].r;
        steps++;
      } else if (element == "L") {
        lokation = nodes[lokation].l;
        steps++;
      }
    }
  }
}
function gcd(a, b) {
  while (b !== 0) {
    let t = b;
    b = a % b;
    a = t;
  }
  return a;
}

function lcm(a, b) {
  return Math.abs(a * b) / gcd(a, b);
}
function lcmarr(arr) {
  let currentLcmm = arr[0];
  for (let i = 1; i < arr.length; i++) {
    currentLcmm = lcm(currentLcmm, arr[i]);
  }
  return currentLcmm;
}

function calculateroute2() {
  let commandssplit = commands.split("");
  let stoppingNumbers = [];
  for (let index = 0; index < startinglokations.length; index++) {
    let start = startinglokations[index];
    lokation = start;
    let steps = 0;
    let run = true;
    while (run) {
      for (let index = 0; index < commandssplit.length; index++) {
        let element = commandssplit[index];
        if (lokation[2] == "Z") {
          stoppingNumbers.push(steps);
          run = false;
        }
        if (element == "R") {
          lokation = nodes[lokation].r;
          steps++;
        } else if (element == "L") {
          lokation = nodes[lokation].l;
          steps++;
        }
      }
    }
  }
  return lcmarr(stoppingNumbers);
}

async function main() {
  let data = await readFile("example.txt");
  let handledData = handleData(data);
  console.log(calculateroute());
  console.log(calculateroute2());
}
main();
