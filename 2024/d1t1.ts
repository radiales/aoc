import * as fs from 'node:fs';

// Read File
let fileContent = fs.readFileSync('d1.txt', 'utf8')
let lines = fileContent.split('\n')

let list1: number[] = []
let list2: number[] = []
let ans: number[] = []

// Splits the two numbers on every line
for (let i = 0; i < lines.length; i++) {
    let line = lines[i].trim(); 
    if (line.length > 0) {
        let parts = line.split(/\s+/)
        list1.push(parseFloat(parts[0]))
        list2.push(parseFloat(parts[1]))
    }
}

// Sorting the lists numerically
list1.sort((a, b) => a - b);
list2.sort((a, b) => a - b);

let distance = 0
// adds matching numbers into answer array
for (let i = 0; i < list1.length; i++) {
    distance = list1[i] - list2[i]
    // Distance cannot be negative, corrects if there is a negative number
    if (distance > 0) {
        ans.push(distance)
    } else {
        ans.push(-1 * distance)
    }
}

// Sums up answers
const sum = ans.reduce((acc, current) => acc + current, 0);
console.log('sum', sum);

// correct answer sum 2000468
