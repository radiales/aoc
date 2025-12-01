import * as fs from 'node:fs'

let fileContent = fs.readFileSync('d3.txt', 'utf8')

const mulRegex = /mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)/g
const doRegex = /do\(\)/g
const dontRegex = /don't\(\)/g

let isMulEnabled = true 
let totalSum = 0


// Start processing
let position = 0
while (position < fileContent.length) {
    console.log(`Debug: Current position: ${position}, isMulEnabled: ${isMulEnabled}`)

    // Reset lastIndex of regexes to the current position
    dontRegex.lastIndex = position
    doRegex.lastIndex = position
    mulRegex.lastIndex = position

    // Find the next match for each regex
    const dontMatch = dontRegex.exec(fileContent)
    const doMatch = doRegex.exec(fileContent)
    const mulMatch = mulRegex.exec(fileContent)

    // Collect all matches with their type and index
    let matches = []

    if (dontMatch) {
        matches.push({ type: "dont", match: dontMatch, index: dontMatch.index })
    }
    if (doMatch) {
        matches.push({ type: "do", match: doMatch, index: doMatch.index })
    }
    if (mulMatch) {
        matches.push({ type: "mul", match: mulMatch, index: mulMatch.index })
    }

    if (matches.length === 0) {
        // No more matches
        console.log('Debug: No more instructions found')
        break
    }

    // Find the match with the smallest index (earliest in the string)
    matches.sort((a, b) => a.index - b.index)
    const nextMatch = matches[0]

    // Update the current position to the end of the matched instruction
    position = nextMatch.index + nextMatch.match[0].length

    // Process the instruction based on its type
    if (nextMatch.type === "dont") {
        isMulEnabled = false
        console.log(`Debug: Found don't() at position ${nextMatch.index}, disabling mul instructions`)
    } else if (nextMatch.type === "do") {
        isMulEnabled = true
        console.log(`Debug: Found do() at position ${nextMatch.index}, enabling mul instructions`)
    } else if (nextMatch.type === "mul") {
        if (isMulEnabled) {
            const x = parseInt(nextMatch.match[1], 10)
            const y = parseInt(nextMatch.match[2], 10)
            console.log(`Debug: Found mul(${x},${y}) at position ${nextMatch.index}, adding ${x * y} to total`)
            totalSum += x * y
        } else {
            console.log(`Debug: Found mul(${nextMatch.match[1]},${nextMatch.match[2]}) at position ${nextMatch.index}, but ignored (disabled)`)
        }
    }
}

console.log('Sum of enabled multiplications:', totalSum)

// Sum of enabled multiplications: 90772405