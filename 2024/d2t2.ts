import * as fs from 'fs';

// Read Data
let fileContent = fs.readFileSync('d2.txt', 'utf8');
let lines = fileContent.split('\n');

let list: number[][] = [];

// Normalize data
for (let i = 0; i < lines.length; i++) {
    let line = lines[i].trim();
    if (line.length > 0) {
        let parts = line.split(/\s+/).map(Number); 
        list.push(parts);
    }
}

let safeReports = 0;

// For each report, check if it's safe or can be made safe
list.forEach((arrayLine) => {
    if (isSafe(arrayLine)) {
        // Safe report
        safeReports++;
    } else if (canBeMadeSafe(arrayLine)) {
        // Report can be made safe by removing one level
        safeReports++;
    }
});

// Function to check if a report is safe
function isSafe(array: number[]): boolean {
    let isGrowing = array[0] < array[1];
    for (let i = 0; i < array.length - 1; i++) {
        if (!safeDistance(array[i], array[i + 1])) {
            // Distance rule violated
            return false;
        }
        if (isGrowing && array[i] > array[i + 1]) {
            // Growing sequence broken
            return false;
        }
        if (!isGrowing && array[i] < array[i + 1]) {
            // Falling sequence broken
            return false;
        }
    }
    return true;
}

// Function to check if removing one level makes the report safe
function canBeMadeSafe(array: number[]): boolean {
    for (let i = 0; i < array.length; i++) {
        // Create a new array without the current level
        const modifiedArray = array.slice(0, i).concat(array.slice(i + 1));
        if (isSafe(modifiedArray)) {
            // If the modified array is safe, the report can be made safe
            return true;
        }
    }
    return false;
}

// Check if distance between two levels is safe
function safeDistance(first: number, second: number): boolean {
    let distance = Math.abs(second - first);
    return distance >= 1 && distance <= 3;
}

console.log('Number of safe reports:', safeReports);
