#!/bin/bash

# Define the base directory for the project
PROJECT_DIR="."

# Check if the project directory exists
if [ ! -d "$PROJECT_DIR" ]; then
    mkdir -p "$PROJECT_DIR"
fi

# Get the current day or accept it as an argument
DAY=${1:-$(date +%d)}

DAY=$((10#$DAY))

# Pad the day with a leading zero if necessary
DAY=$(printf "%02d" "$DAY")

# Define the Rust file name
FILE_NAME="day_${DAY}.rs"
FILE_PATH="$PROJECT_DIR/src/$FILE_NAME"

# Create input file directory and placeholder
INPUT_DIR="$PROJECT_DIR/input"
INPUT_TEST_DIR="$PROJECT_DIR/input_test"
INPUT_FILE="$INPUT_DIR/day_${DAY}.txt"
INPUT_TEST_FILE="$INPUT_TEST_DIR/day_${DAY}.txt"

mkdir -p "$INPUT_DIR"
mkdir -p "$INPUT_TEST_DIR"
touch "$INPUT_FILE"
touch "$INPUT_TEST_FILE"

# Generate boilerplate Rust code
if [ ! -f "$FILE_PATH" ]; then
    cat > "$FILE_PATH" <<EOL
use std::fs;

// Main function
pub fn day_${DAY}() {
    let input_file = "$INPUT_FILE";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    0
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("$INPUT_TEST_FILE"), 0);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("$INPUT_TEST_FILE"), 0);
    }
}
EOL
    echo "Created boilerplate for day $DAY at $FILE_PATH"
    code "$FILE_PATH"
else
    echo "Boilerplate for day $DAY already exists at $FILE_PATH"
fi
