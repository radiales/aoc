import * as fs from 'fs'

let fileContent = fs.readFileSync('d1.txt', 'utf8')
let lines = fileContent.split('\n')

let list1: number[] = []
let list2: number[] = []
let similarityScore = 0

for (let i = 0; i < lines.length; i++) {
    let line = lines[i].trim(); 
    if (line.length > 0) {
        let parts = line.split(/\s+/)
        list1.push(parseFloat(parts[0]))
        list2.push(parseFloat(parts[1]))
    }
}


// Counts up one time each time the first number is present in the List
for (let i = 0; i < list1.length; i++) {
    let num = list1[i];
    let count = 0;

    for (let j = 0; j < list2.length; j++) {
        if (list2[j] === num) {
            count++;
        }
    }

    similarityScore += num * count;
}

console.log('Sim:', similarityScore);

// Solution 18567089