use std::{fmt, iter};

type FileID = u32;
type Blocks = Vec<Block>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Block {
    Empty,
    File(FileID),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SearchMode {
    SingleBlock,
    Exhaustive,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::Empty => write!(f, "."),
            Block::File(file_id) => write!(f, "{}", file_id),
        }
    }
}

pub fn run(input: String) {
    println!("Part 1: {}", solve(&input, SearchMode::SingleBlock));
    println!("Part 2: {}", solve(&input, SearchMode::Exhaustive));
}

fn parse_input(input: &str) -> Blocks {
    let mut empty_disk_entry = false;
    let mut disk_entries: Blocks = Vec::new();
    let mut file_id = 0;

    for character in input.chars() {
        let data_size = character.to_digit(10).unwrap();
        if empty_disk_entry {
            disk_entries.extend(iter::repeat(Block::Empty).take(data_size as usize));
        } else {
            for _ in 0..data_size {
                disk_entries.push(Block::File(file_id));
            }

            file_id += 1;
        }

        empty_disk_entry = !empty_disk_entry;
    }

    disk_entries
}

pub fn forwards_search_for_first_contiguous_empty(
    blocks: &Blocks,
    size: usize,
    until_index: usize,
) -> Option<(usize, usize)> {
    let mut current_start = None;
    let mut current_count = 0;

    for (i, block) in blocks[0..until_index].iter().enumerate() {
        match block {
            Block::Empty => {
                if current_start.is_none() {
                    current_start = Some(i);
                }

                current_count += 1;

                if current_count == size {
                    return Some((i + 1 - size, i + 1));
                }
            }
            _ => {
                current_start = None;
                current_count = 0;
            }
        }
    }

    None
}

pub fn backwards_search_for_first_contiguous_nonempty(
    blocks: &Blocks,
    before_index: usize,
    search_mode: SearchMode,
) -> Option<(usize, usize)> {
    let mut index = before_index;

    if index == 0 {
        return None;
    }

    let mut current_file_id: Option<FileID> = None;
    let mut start_index: Option<usize> = None;
    let mut end_index: Option<usize> = None;

    loop {
        if (index as i32) - 1 < 0 {
            if current_file_id == None {
                return None;
            }

            return Some((0, end_index.unwrap()));
        }

        index -= 1;

        if let Block::File(file_id) = blocks[index] {
            if current_file_id.is_none() {
                if search_mode == SearchMode::SingleBlock {
                    return Some((index, index + 1));
                } else {
                    current_file_id = Some(file_id);
                    end_index = Some(index + 1);
                    continue;
                }
            }

            if !current_file_id.is_none() && current_file_id == Some(file_id) {
                continue;
            }

            if !current_file_id.is_none() && current_file_id != Some(file_id) {
                start_index = Some(index + 1);
                break;
            }
        } else {
            if current_file_id.is_none() {
                continue;
            }

            start_index = Some(index + 1);
            break;
        }
    }

    if start_index.is_none() || end_index.is_none() {
        return None;
    }

    return Some((start_index.unwrap(), end_index.unwrap()));
}

pub fn checksum(blocks: &Blocks) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::Empty => 0,
            Block::File(file_id) => (index as u64) * (*file_id as u64),
        })
        .sum()
}

pub fn solve(input: &str, search_mode: SearchMode) -> u64 {
    let mut blocks = parse_input(input);

    let mut file_region: Option<(usize, usize)> = None;

    loop {
        let before_index = file_region.map(|region| region.0).unwrap_or(blocks.len());

        file_region =
            backwards_search_for_first_contiguous_nonempty(&blocks, before_index, search_mode);

        let Some((file_region_start, file_region_end)) = file_region else {
            break;
        };

        let Block::File(_) = blocks[file_region.unwrap().0] else {
            unreachable!("Expected Block::File")
        };

        let file_size = file_region_end - file_region_start;

        if let Some((empty_region_start, _)) =
            forwards_search_for_first_contiguous_empty(&blocks, file_size, file_region_start)
        {
            for i in 0..file_size {
                blocks.swap(empty_region_start + i, file_region_start + i);
            }
        }
    }

    checksum(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT, SearchMode::SingleBlock), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT, SearchMode::Exhaustive), 2858);
    }
}
