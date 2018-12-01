var fs = require('fs');

var input = fs.readFileSync('input.txt', 'utf8');

const seen = new Set();
let currFreq = 0;
const splitInput = input.trim().split('\n').map(n => +n);

while (1) {
    for (operation of splitInput) {
        if (seen.has(currFreq)) {
            console.log(currFreq);
            return;
        }
        seen.add(currFreq);
        currFreq += operation;
    }
}
