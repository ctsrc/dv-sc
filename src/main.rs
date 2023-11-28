use phf::phf_map;

static DVORAK_SCORES_HOME_ROW_UPPER: phf::Map<char, i32> = phf_map! {
    // Top row scores: 0 for each character
    ';' => 0, '\'' => 0, ',' => 0, '.' => 0, 'P' => 0, 'Y' => 0, 'F' => 0, 'G' => 0, 'C' => 0, 'R' => 0, 'L' => 0, '/' => 0,

    // Home row scores: -2, -1, 3, 2, 2, 2, 2, 3, -1, -2 for each character
    'A' => -2, 'O' => -1, 'E' => 3, 'U' => 2, 'I' => 2, 'D' => 2, 'H' => 2, 'T' => 3, 'N' => -1, 'S' => -2,

    // Bottom row scores: 0 for each character
    'Q' => 0, 'J' => 0, 'K' => 0, 'X' => 0, 'B' => 0, 'M' => 0, 'W' => 0, 'V' => 0, 'Z' => 0, ':' => 0, '-' => 0,
};

static DVORAK_SCORES_HOME_ROW_LOWER: phf::Map<char, i32> = phf_map! {
    // Top row scores: 0 for each character
    ';' => 0, '\'' => 0, ',' => 0, '.' => 0, 'p' => 0, 'y' => 0, 'f' => 0, 'g' => 0, 'c' => 0, 'r' => 0, 'l' => 0, '/' => 0,

    // Home row scores: -2, -1, 3, 2, 2, 2, 2, 3, -1, -2 for each character
    'a' => -2, 'o' => -1, 'e' => 3, 'u' => 2, 'i' => 2, 'd' => 2, 'h' => 2, 't' => 3, 'n' => -1, 's' => -2,

    // Bottom row scores: 0 for each character
    'q' => 0, 'j' => 0, 'k' => 0, 'x' => 0, 'b' => 0, 'm' => 0, 'w' => 0, 'v' => 0, 'z' => 0, ':' => 0, '-' => 0,
};

fn main() {
    // Test the map
    println!("{:?}", DVORAK_SCORES_HOME_ROW_UPPER);
    // Test the map
    println!("{:?}", DVORAK_SCORES_HOME_ROW_LOWER);
}
