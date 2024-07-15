const fs = require("fs");

const readData = (data, goal) => {
  let arr = [];
  for (let i = 0; i < goal; i++) {
    arr.push(data[i]);
  }

  for (let index = goal; index < data.length; index++) {
    if (setComparer(arr)) {
      console.log(arr);
      return index;
    }
    arr.shift();
    arr.push(data[index]);
  }
};

const setComparer = (arr) => {
  let set = new Set(arr);

  return set.size === arr.length;
};

try {
  const data = fs.readFileSync("t.txt", "utf8").split("\n");
  let temp = data[0].split("");
  console.log(readData(temp, 14));
} catch (err) {
  console.log(err);
}
