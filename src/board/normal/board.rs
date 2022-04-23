///! Board manipulation utils

/// Board 64 position

type Board64Position = u8;

/// Board 120 position
type Board120Position = u8;

/// Board 120 to 64 mapping
const BOARD_120_TO_64: [Board64Position; 120] = [
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  8,  9, 10, 11, 12, 13, 14, 15, 64,
    64, 16, 17, 18, 19, 20, 21, 22, 23, 64,
    64, 24, 25, 26, 27, 28, 29, 30, 31, 64,
    64, 32, 33, 34, 35, 36, 37, 38, 39, 64,
    64, 40, 41, 42, 43, 44, 45, 46, 47, 64,
    64, 48, 49, 50, 51, 52, 53, 54, 55, 64,
    64, 56, 57, 58, 59, 60, 61, 62, 63, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64
];

/// Board 120 to 64 mapping
const BOARD_120_TO_RANK: [u8; 120] = [
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64,  0,  0,  0,  0,  0,  0,  0,  0, 64,
    64,  1,  1,  1,  1,  1,  1,  1,  1, 64,
    64,  2,  2,  2,  2,  2,  2,  2,  2, 64,
    64,  3,  3,  3,  3,  3,  3,  3,  3, 64,
    64,  4,  4,  4,  4,  4,  4,  4,  4, 64,
    64,  5,  5,  5,  5,  5,  5,  5,  5, 64,
    64,  6,  6,  6,  6,  6,  6,  6,  6, 64,
    64,  7,  7,  7,  7,  7,  7,  7,  7, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64
];

/// Board 120 to 64 mapping
const BOARD_120_TO_FILE: [u8; 120] = [
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64,  0,  1,  2,  3,  4,  5,  6,  7, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64
];

/// Board 64 to 120 mapping
const BOARD_64_TO_120: [Board120Position; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

/// Board 64 to 120 mapping
const BOARD_64_TO_RANK: [u8; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

/// Board 64 to 120 mapping
const BOARD_64_TO_FILE: [u8; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];


/// Board 64 flip mapping
const BOARD_64_FLIP: [u8; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
    8 , 9 , 10 ,11, 12, 13, 14, 15,
    0 , 1 , 2 , 3 , 4 , 5 , 6 , 7
];

/// Board (rank,file) to 120 mapping
const BOARD_RANK_FILE_TO_120: [[u8; 8]; 8] = [
    [ 21, 22, 23, 24, 25, 26, 27, 28 ],
    [ 31, 32, 33, 34, 35, 36, 37, 38 ],
    [ 41, 42, 43, 44, 45, 46, 47, 48 ],
    [ 51, 52, 53, 54, 55, 56, 57, 58 ],
    [ 61, 62, 63, 64, 65, 66, 67, 68 ],
    [ 71, 72, 73, 74, 75, 76, 77, 78 ],
    [ 81, 82, 83, 84, 85, 86, 87, 88 ],
    [ 91, 92, 93, 94, 95, 96, 97, 98 ]
];

/// Convert a board 120 position to a board 120 position
///
/// # Arguments
///
/// * `position` - The board 64 position
#[inline]
pub const fn board_120_to_64(position: Board120Position) -> Board64Position {
    BOARD_120_TO_64[position as usize]
}

/// Get a rank from a board 120 position
///
/// # Arguments
///
/// * `position` - The board 120 position
#[inline]
pub const fn board_120_to_rank(position: Board120Position) -> u8 {
    BOARD_120_TO_RANK[position as usize]
}

/// Get a file from a board 120 position
///
/// # Arguments
///
/// * `position` - The board 120 position
#[inline]
pub const fn board_120_to_file(position: Board120Position) -> u8 {
    BOARD_120_TO_FILE[position as usize]
}

/// Convert a board 64 position to a board 120 position
///
/// # Arguments
///
/// * `position` - The board 64 position
#[inline]
pub const fn board_64_to_120(position: Board64Position) -> Board120Position {
    BOARD_64_TO_120[position as usize]
}

/// Get a rank from a board 64 position
///
/// # Arguments
///
/// * `position` - The board 64 position
#[inline]
pub const fn board_64_to_rank(position: Board64Position) -> u8 {
    BOARD_64_TO_RANK[position as usize]
}

/// Get a file from a board 64 position
///
/// # Arguments
///
/// * `position` - The board 64 position
#[inline]
pub const fn board_64_to_file(position: Board64Position) -> u8 {
    BOARD_64_TO_FILE[position as usize]
}

/// Flip a board 64 position
///
/// # Arguments
///
/// * `position` - The board 64 position
#[inline]
pub const fn board_64_flip(position: Board64Position) -> Board64Position {
    BOARD_64_TO_FILE[position as usize]
}

/// Convert a [rank, file] position to a board 120 position
///
/// # Arguments
///
/// * `rank` - The rank
/// * `file` - The file
#[inline]
pub const fn board_rank_file_to_120(rank: u8, file: u8) -> Board120Position {
    // 21 + file + rank * 10
    BOARD_RANK_FILE_TO_120[file as usize][rank as usize]
}

/// Convert a [rank, file] position to a board 64 position
///
/// # Arguments
///
/// * `rank` - The rank
/// * `file` - The file
#[inline]
pub const fn board_rank_file_to_64(rank: u8, file: u8) -> Board64Position {
    file + rank * 8
}
