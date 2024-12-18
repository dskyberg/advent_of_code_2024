use anyhow::Result;
use std::time::Instant;

use day_9::*;

/// Swap file blocks in place
fn sort_blocks(blocks: &mut [Option<usize>]) {
    // The last block should have the largest file id
    let Some(mut file_id) = blocks.last().unwrap() else {
        panic!("Last block was not a file id")
    };

    let mut file_len: usize;
    let mut file_start: usize;
    let mut empties_start: usize;
    let mut empties_len: usize;

    while file_id > 1 {
        // Skip empties until we come to the next file id
        file_start = 0;
        while file_start < blocks.len() {
            if blocks[file_start].is_some() && blocks[file_start].unwrap() == file_id {
                break;
            }
            file_start += 1;
        }

        // blocks[file_start] is now the first block of file_id.  Find the first block
        file_len = 1;

        while file_start + file_len < blocks.len()
            && blocks[file_start + file_len].is_some()
            && blocks[file_start + file_len].unwrap() == file_id
        {
            file_len += 1;
        }

        // The file runs from blocks[file_start..file_start + file_len]

        // We now have the start and length of the next file id.  Now we need to
        // find a spot big enough to move it to.

        // Starting at the front, look for a gap
        empties_start = 0;
        empties_len = 0;

        loop {
            if empties_start >= file_start {
                break;
            }
            while blocks[empties_start].is_some() {
                empties_start += 1;
            }
            // We are at the start of a gap.  Measure the size.
            empties_len = 1;
            while blocks[empties_start + empties_len].is_none() {
                empties_len += 1;
            }
            if empties_len >= file_len {
                break;
            } else {
                if file_id == 2 {
                    println!("FILE 2: {}, {}", empties_len, file_len);
                }
                empties_start += empties_len;
                empties_len = 0;
            }
        }

        // Did we find a gap?
        if empties_len < file_len {
            // We did not find a big enough gap for this file id
            file_id -= 1;
            continue;
        }

        // We found a suitable block!  Swap the file and the empty blocks
        for loc in file_start..file_start + file_len {
            blocks[empties_start] = blocks[loc];
            blocks[loc] = None;
            empties_start += 1;
        }
        file_id -= 1;
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let disk_map = read_data(DATA);
    let mut blocks = decode(&disk_map);

    sort_blocks(&mut blocks);
    let result = calc_check_sum(&blocks);
    println!("Part2: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
