import * as fs from 'node:fs'

let fileContent = fs.readFileSync('d3.txt', 'utf8')

const regex = /mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)/g

let match
let totalSum = 0

while ((match = regex.exec(fileContent)) !== null) {
    const x = parseInt(match[1], 10) 
    const y = parseInt(match[2], 10)
    totalSum += x * y
}

console.log('ans:', totalSum)

// ans: 182780583