import * as fs from 'node:fs';

let fileContent = fs.readFileSync('d4.txt', 'utf8');

let lines = fileContent.trim().split('\n');

let grid = lines.map(line => line.trim().split(''));

let numRows = grid.length;
let numCols = grid[0].length;

// Right possible directions
let directions = [
    { dx: 0, dy: 1 },   // down
    { dx: 1, dy: 0 },   //  right
    { dx: 0, dy: -1 },  // up
    { dx: -1, dy: 0 },  // left
    { dx: 1, dy: 1 },   // diag bottom right
    { dx: -1, dy: -1 }, // diag top left
    { dx: 1, dy: -1 },  // diag top right
    { dx: -1, dy: 1 }   // diag bottom left
];

let word = 'XMAS';

let count = 0;

function checkWord(x, y, direction) {
    let dx = direction.dx;
    let dy = direction.dy;

    for (let i = 0; i < word.length; i++) {
        let nx = x + dx * i;
        let ny = y + dy * i;

        // is word in grid
        if (nx < 0 || nx >= numCols || ny < 0 || ny >= numRows) {
            return false;
        }

        if (grid[ny][nx] !== word[i]) {
            return false;
        }
    }

    return true;
}

// Iterates through grid
for (let y = 0; y < numRows; y++) {
    for (let x = 0; x < numCols; x++) {
        // checks in every direction possible
        for (let dir of directions) {
            if (checkWord(x, y, dir)) {
                count++;
            }
        }
    }
}

console.log('#XMAS:', count);

//  #XMAS: 2483