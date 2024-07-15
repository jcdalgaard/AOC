const fs = require("fs");

const readData = (data) => {
  let total = 0;
  data.forEach((element) => {
    const partOne = element.slice(0, element.length / 2);
    const partTwo = element.slice(element.length / 2, element.length);
    total += getOverlaps(partOne, partTwo);
  });

  return total;
};
const getBadges = (data) => {
  let total = 0;
  for (let i = 0; i < data.length; i += 3) {
    let element = data[i].split("");
    let count = data[i + 1].split("");
    let badges = data[i + 2].split("");

    let holder = new Set();

    element.forEach((element) => {
      if (count.includes(element)) {
        holder.add(element);
      }
    });
    holder.forEach((element) => {
      if (!badges.includes(element)) {
        holder.delete(element);
      }
    });

    holder.delete("\r");

    total += getPoints(holder);
  }

  return total;
};
const getOverlaps = (string1, string2) => {
  let points = 0;
  const arr1 = string1.split("");
  const arr2 = string2.split("");
  let overlappingElements = new Set();
  arr1.forEach((element) => {
    if (arr2.includes(element)) {
      overlappingElements.add(element);
    }
  });

  return getPoints(overlappingElements);
};

const getPoints = (elements) => {
  let points = 0;
  elements.forEach((element) => {
    points +=
      element.toUpperCase() === element
        ? element.charCodeAt(0) - 38
        : element.charCodeAt(0) - 96;
  });
  return points;
};

try {
  const data = fs.readFileSync("t.txt", "utf8").split("\n");
  console.log(readData(data));

  console.log(getBadges(data));
} catch (err) {
  console.error(err);
}
