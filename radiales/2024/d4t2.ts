import * as fs from 'node:fs';


let fileContent = fs.readFileSync('d4.txt', 'utf8');


let lines = fileContent.trim().split('\n');

let grid = lines.map(line => line.trim().split(''));

let numRows = grid.length;
let numCols = grid[0].length;

let count = 0;

// Possible strings
let diagonals = [
    { diag1: 'MAS', diag2: 'MAS' },
    { diag1: 'MAS', diag2: 'SAM' },
    { diag1: 'SAM', diag2: 'MAS' },
    { diag1: 'SAM', diag2: 'SAM' }
];


function checkXMAS(x, y) {
    let total = 0;
    for (let combo of diagonals) {
        let match = true;

        // cheacks diag from top lef to bottom right
        for (let i = -1; i <= 1; i++) {
            let nx = x + i;
            let ny = y + i;

            if (nx < 0 || nx >= numCols || ny < 0 || ny >= numRows) {
                match = false;
                break;
            }

            if (grid[ny][nx] !== combo.diag1[i + 1]) {
                match = false;
                break;
            }
        }

        if (!match) continue;

        // same but top right bottom left
        for (let i = -1; i <= 1; i++) {
            let nx = x + i;
            let ny = y - i;

            if (nx < 0 || nx >= numCols || ny < 0 || ny >= numRows) {
                match = false;
                break;
            }

            if (grid[ny][nx] !== combo.diag2[i + 1]) {
                match = false;
                break;
            }
        }

        if (match) {
            total++;
        }
    }

    return total;
}

// iterates through grid
for (let y = 0; y < numRows; y++) {
    for (let x = 0; x < numCols; x++) {
        // Only checks spots where an A marks the middle
        if (grid[y][x] === 'A') {
            count += checkXMAS(x, y);
        }
    }
}

console.log('#XMAS:', count);

// #XMAS: 1925