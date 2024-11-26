const fs = require("fs");

var stackHolder = [[], [], [], [], [], [], [], [], []];

const readData = (data) => {
  let holder = data[8].split("");
  let indexer = 9;
  for (let i = 1; i <= indexer; i++) {
    let n = data[8].indexOf("" + i);
    for (let j = 0; j < indexer - 1; j++) {
      let element = data[7 - j].split("")[n];
      if (element && element !== " ") {
        stackHolder[i - 1].push(element);
      }
    }
  }
};

const readMove = (string) => {
  let commands = string.split(" ");

  let n = Number(commands[1]);
  let from = Number(commands[3]) - 1;
  let to = Number(commands[5]) - 1;
  let temp = [];
  for (let index = 0; index < n; index++) {
    temp.push(stackHolder[from].pop());
  }
  const x = temp.length;
  for (let index = 0; index < x; index++) {
    stackHolder[to].push(temp.pop());
  }
};

try {
  const data = fs.readFileSync("t.txt", "utf8").split("\n");
  readData(data);
  for (let i = 10; i < data.length; i++) {
    readMove(data[i]);
  }

  let print = "";
  stackHolder.forEach((element) => {
    print += element.pop();
  });
  console.log(print);
} catch (err) {
  console.error(err);
}
