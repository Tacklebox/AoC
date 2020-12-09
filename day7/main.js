const fs = require("fs");

const input = fs.readFileSync("./input.txt").toString().split('\n').filter(Boolean);
// const input = fs.readFileSync("./test_input.txt").toString().split('\n').filter(Boolean);

const memo = {};
function howManyContained(bagType, rules) {
  if (memo[bagType]) {
    return memo[bagType];
  }
  const [, contents] = rules.find(([bag]) => bag === bagType)
  if (contents[0][0] === "other") {
    return 0;
  }
  let numContents = 0;
  contents.forEach(([bag, number]) => {
    number = Number.parseInt(number);
    numContents += (number + (number * howManyContained(bag, rules)));
  })
  memo[bagType] = numContents;
  return numContents;
}

let rules = [];
for (line of input) {
  line = line.split(/ bags?/).join("");
  const [type, contents] = line.split(" contain ");
  filteredContents = contents
    .split(/\.|, /)
    .filter(Boolean)
    .map(el => [
      el.substr(el.indexOf(" ")).trim(),
      el.substr(0, el.indexOf(" ")).trim()
    ]);
  rules.push([type, filteredContents]);
}

console.log(`Found ${howManyContained("shiny gold", rules)} bag types to hold`);


// Part 1
//
// let rules = [];
// for (line of input) {
//   line = line.split(/ bags?/).join("");
//   const [type, contents] = line.split(" contain ");
//   filteredContents = contents
//     .split(/\.|, /)
//     .filter(Boolean)
//     .map(el => [
//       el.substr(el.indexOf(" ")).trim(),
//       el.substr(0, el.indexOf(" ")).trim()
//     ]);
//   rules.push([type, filteredContents]);
// }

// let toSearch = ["shiny gold"];
// let visited = {};
// while (toSearch.length) {
//   let currentSearch = toSearch.pop();
//   console.info(`searching for ${currentSearch}`);
//   visited[currentSearch] = true;
//   for ([bag, contents] of rules) {
//     if (contents.some(([type]) => type === currentSearch)) {
//       if (!visited[bag]) toSearch.push(bag);
//     }
//   }
// }
// Object.keys(visited).length
