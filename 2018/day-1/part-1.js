var fs = require('fs');

var input = fs.readFileSync('input.txt', 'utf8');

console.log(
  eval(
    input
      .replace(/ /g, '')
      .replace(/\n/g, '')
  )
)
