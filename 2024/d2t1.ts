import * as fs from 'node:fs'


// Read Data
let fileContent = fs.readFileSync('d2.txt', 'utf8')
let lines = fileContent.split('\n')

let list: number[][] = []

// Normalize data
for (let i = 0; i < lines.length; i++) {
    let line = lines[i].trim()
    if (line.length > 0) {
        let parts = line.split(/\s+/).map(Number)
        list.push(parts)
    }
}

let ans = 0;

// For each line, check if the sequence is growing or falling
list.forEach((arrayLine) => {
    if (arrayLine[0] > arrayLine[1]) {
        // Falling sequence check
        ans += calculateRecursive(arrayLine.slice(), false)
    } else {
        // Growing sequence check
        ans += calculateRecursive(arrayLine.slice(), true)
    }
});

function calculateRecursive(array: number[], isGrowing: boolean): number {
    if (array.length <= 1) {
        // If the array has only one element left, the condition is satisfied
        return 1;
    }

    const current = array[0]
    const next = array[1]

    if (next === undefined) {
        // End of array reached
        return 1
    }

    if (isGrowing && current < next && safeDistance(current,next) ) {
        // Growing and condition satisfied
        return calculateRecursive(array.slice(1), isGrowing);
    } else if (!isGrowing && current > next && safeDistance(current,next)) {
        // Falling and condition satisfied
        return calculateRecursive(array.slice(1), isGrowing);
    } else {
        // Condition not satisfied
        return 0
    }
}

// Check if distance is between 1 and 3
function safeDistance(first: number, second: number){
   let distance = second - first

   if (distance < 0){
    distance *= -1
   }

   if (distance <= 3 && distance >= 1 ) {
    return true
   } else {
    return false
   }
}

console.log('Result:', ans);

// Result: 564
