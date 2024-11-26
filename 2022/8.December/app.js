const fs = require("fs");

const data = fs.readFileSync("t.txt", "utf8").split("\n");
console.log(data);

let leftSeen = [];
let rightSeen = [];
let topSeen = [];
let bottomSeen = [];

const cleanData = (data) => {
  let matrice = [];
  for (let i = 0; i < data.length; i++) {
    let temp = data[i].replace("\r", "");
    matrice.push(temp.split(""));
  }
  return matrice;
};

const checkLeft = (matrix) => {
  for (let i = 0; i < matrix.length; i++) {
    let currentTop = -1;
    for (let j = 0; j < matrix[i].length; j++) {
      if (matrix[i][j] > currentTop) {
        currentTop = matrix[i][j];

        leftSeen.push({ x: j, y: i, value: matrix[i][j] });
      }
    }
  }
};
const checkRight = (matrix) => {
  for (let i = 0; i < matrix.length; i++) {
    let currentTop = -1;
    for (let j = matrix[i].length - 1; j >= 0; j--) {
      if (matrix[i][j] > currentTop) {
        currentTop = matrix[i][j];

        rightSeen.push({ x: j, y: i, value: matrix[i][j] });
      }
    }
  }
};
const checkTop = (matrix) => {
  for (let i = 0; i < matrix[0].length; i++) {
    let currentTop = -1;
    for (let j = 0; j < matrix.length; j++) {
      let node = matrix[j][i];
      if (node > currentTop) {
        currentTop = node;

        topSeen.push({ x: i, y: j, value: node });
      }
    }
  }
};
const checkBottom = (matrix) => {
  for (let i = 0; i < matrix[0].length; i++) {
    let currentTop = -1;
    for (let j = matrix.length - 1; j >= 0; j--) {
      let node = matrix[j][i];
      if (node > currentTop) {
        currentTop = node;

        bottomSeen.push({ x: i, y: j, value: node });
      }
    }
  }
};

let matrix = cleanData(data);
checkLeft(matrix);
checkRight(matrix);
checkTop(matrix);
checkBottom(matrix);

const addTrees = (arr) => {
  for (let index = 0; index < arr.length; index++) {
    if (!trees.has("" + arr[index].x + arr[index].y)) {
      trees.add(arr[index].x + "" + arr[index].y);
    }
  }
};

var trees = new Set();
addTrees(leftSeen);
addTrees(rightSeen);
addTrees(topSeen);
addTrees(bottomSeen);
console.log(trees);
console.log(trees.size);
