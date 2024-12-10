pub const CARAT: u8 = b'^';
pub const OPEN: u8 = b'.';
pub const BLOCKED: u8 = b'#';

pub fn read_data(data: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut matrix: Vec<Vec<u8>> = data
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let mut x = 0;
    let mut y = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == CARAT {
                x = col;
                y = row;
                matrix[row][col] = OPEN;
                break;
            }
        }
    }
    (matrix, (y, x))
}

pub const TEST_DATA: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub const DATA: &str = r".......#.........#..............#........................#......#....#............................#..#...............#............
...................#.............................................#...................................#............................
...............#.................................##.........#..........#.....##...#...#.....#....#..................#.............
#.............................................#..#.............#..........#.##.............................................#......
#.........................................................#.....#........................................##.......................
...........#.......#.#...........#....#.#...#......................................#...........................#...##............#
........................................................#..#.....................................#..#................#....#...#...
#.#.....#.........................#...........#.......................................#.......#.....................##............
.......#..........##.....#........#..................................................#.........................................#.#
#.#..............#.#.............................#............#....#...............#.........................#............#..#.#.#
..........................................................#....................................................#..................
.......................#....#...........#.....................#....#...#.......#..............#.......................#...........
..............................#..............#.......#....................................#.....................#...#...#.........
........#.................#...#....##..............#...........................#............................#.................#...
..................#........................................#......#......................................................#........
.............#.................#...#..............................................................................#...#...........
.....................................#..........................................................#.#.#......#.........#............
..........#...................................................................#.........##.......#..#.............................
...#.#....................................#..#...........#.......................................................##...............
........................................................................................................#.........................
.........#...........#......#..............#........#....................#...#......#.............................................
........#....#...............#....................#..........#.....................................#......#......#........#.#.....
........................#..........................#.......#........................#................#......#.....................
......................................#.........................................#............................#....................
...................................#.#...#...........#.........................................#.....................#............
.................#....#.....................................................................#..................#.........#........
...........#...........................................................#.............................#...............#..#.........
..#.....................#.........................................................................................................
....................#....#.#...........................#...........................................#..............#...............
...#...................................................#.............................................#.........#..........#.......
.............................###.....#..#..#......................................................................................
##................#.#.......#..............#.....#...............#..#............#................................................
......#...............................................#.#..#............................#............#.................#..........
........................................#.#...................................................#.....#.............................
#.......#.#.......................#.....#....................................#...................#................................
...##................................................................................................#.....................#......
............#.............#..................#.................#..................................................................
...#..............#..............................................................................#..........#......#..............
...#..#...#................................................#.....................#......#......................#.......#..........
.................#..#........#...............#.#................#......##.................#.................................#.....
#.....#.......#...........................................#..........#..............................#...#.#..#......#.....#.......
....................................#.........#........#..................#.............................................#.........
#...............................#........................#.................................................#.....#................
..#..................................#.......#.....................#..............................................................
........#...#.....................................#......#..............#..#..#..........................#.#....#.................
.............#..........#....#................#.......#..........................................#.....#..........................
.............................................#................................#.........................#...#.....................
.........#........#...................................................................#.......#....#..............................
....................................#.......................#..............#...............................#......................
....#....................#................................#....#...........#.............#........................................
....#...........#...................#.#..................................................#.....................#..................
..........#....................#............#.##...#.......#....#...............#.....#...#.........................#.............
.......#....................................................................................................#.................#...
...............................................................................#......#.................#...#.....................
..................#........#..........#..........................................................................#................
..........................................................................................#.......................................
.#..#...........#.................................##...........................#.....#...................#..#.....................
........................#..#..........................................................#............#..................#..........#
...........#...................................................##...........................#...............................#.....
........#......................................................................................................#..................
........#..#............................#..#...........#..............................#......................#.##...........#....#
........#...#.#..............#...#.....................#....................#.#..............................#....................
.......................................................#....................................................#..#..................
...........................................#......................#................#........................#....................#
.............#...............#................................................................................#...................
..#...#............#.............#.....#........................#............#....................#...............................
.....#...................................................................................................................#........
....................#..#...........#....#.............................................................................#.......#...
..............#......#.........................#...........#...#...........................................#...#..................
..............................#.....#.........................................................#......#.#......#...................
............................................................#...................#.#............................#..................
.............###.#........................................#........................................................#..............
......#................................#...............##.........................................................................
.........#.....#.#.....................#.......................................#...............#..........#....#....#.............
..............................................#....................................#...........................................#..
............#.#..........#........#..#.....#.............#..........#............#......#...................##..........#.........
............#....#...................................#.............................#.........................#....................
.#..................................#....#.........................................................................#..............
.........................#...#...#.................#..................#.............#.#...#...............#.......................
.........#...............................#...........#.............................#...........................#..........#.......
......#..........................#.............#..........#.......................................................................
.......#...............#.........#.............#...............................#..................................................
..#......#....................##...........................................#........................................#.............
..........................#.....#.......................#...........................................................#.#...........
..#....................#.............#................................................................#.#.........................
.........#....................#.....#....#...............#.......................#........#.......................................
..............#...................................................................#................#...#..........................
....................#.....#.......#..#......................................#...........#..................................##....#
.......#.......................................................................#..................................................
......#.............##..........................#..#......................^.......................................................
..............................#..........#...................#.................................#.....................#............
.......................##..........................#.......#..............................#...................................#.#.
................#....................................................................#...............................#.........#..
..........................................#......#.....................................#................#........#................
#......................................#.......................................................#..#...........#...................
.......#.............#..................#..#...#............#.......#.....#.....#.#...............................................
.............................#........#.............#.....#.......#...................................#.#..................#......
.................#......................................................................................#........#..#.............
........................................................................#..............#.....#..............................#.....
....#....#..........................................#......##...#....................#...................#..........#.............
.................##.......................##.#.........#................................................#.........................
.#..............................................#.........................#.#.#..........#.....................................##.
............................................................#..........#....#...................................................#.
....#...........#.............................#.....................#.........#..................#...........................#....
......#.#.......#..#......#........#..#...................#....#..................................................................
............#........................................................................................................#............
.....................................................#..........................................#................#................
...#.......#..#..............#.............................#....................................#.................................
.................##.........#........................#....................................#...............#.#.#...................
................#......#......................#..#..........................................#.....................................
......#..................................................##.#.#........#...........................#.......................#......
........................#............#......#.........#.............#.............................................#.....#.##......
......................#........................................#.......#.........#.................#..#...........#...............
..#.......#.........##..#.#................................................................................#.........#............
........................................................................................#............#...........#................
................#...#....................#..##..#...............#.............................................................#...
...#........#..........................#.....#..........................#....................................#.........#.......##.
.................#...........................#.........#......#............................#.....................#................
...................#..........#....#.......#...................#.............................#..........#.........................
.......#..............#.................................#.........#....................................#.................#....##.#
..................................#.......................#....#..#.........................#....................#................
.....#.........#............#...#.........#........#...................#...#......................#.....#............#............
#.#....#...#........#................................##..........................#.............................##...#....#........
.............##..........##...#......#.......#.#................#.#.......................#.............#.........................
.............#....................................#....................................#..........................................
..................................#..............................#.......................................#................#.......
...................#...............#..............................................................................................
....#........#.................................#........#.....................................#.......#................#........#.
.................#......#.......................................................##.....#..................#.......................
...............#.......................#.#........#...........................................#........#..........................";