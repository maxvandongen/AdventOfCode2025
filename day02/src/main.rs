#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn all_invalid_ids_part_1(&self) -> Vec<usize> {
        (self.start..=self.end).filter(|i| i.is_invalid_part_1()).collect()
    }
    fn all_invalid_ids_part_2(&self) -> Vec<usize> {
        (self.start..=self.end).filter(|i| i.is_invalid_part_2()).collect()
    }
}

trait IsInvalid {
    fn is_invalid_part_1(&self) -> bool;
    fn is_invalid_part_2(&self) -> bool;
}

impl IsInvalid for usize {
    fn is_invalid_part_1(&self) -> bool {
        let as_string = self.to_string();
        if as_string.len() % 2 == 0 {
            let mid = as_string.len() / 2;
            let (first_half, second_half) = as_string.split_at(mid);
            first_half == second_half
        } else {
            false
        }
    }

    fn is_invalid_part_2(&self) -> bool {
        let as_string = self.to_string();
        let chars: Vec<char> = as_string.chars().collect();
        let initial_amount = chars.len() / 2;
        for segment_len in std::ops::RangeInclusive::new(1, initial_amount).rev() {
            if as_string.len() % segment_len != 0 {
                continue;
            }
            // println!("Checking segments of length {} for number {}", segment_len, as_string);
            let mut all_equal = true;
            for i in 0..(as_string.len() / segment_len) - 1 {
                let start_idx = i * segment_len;
                let end_idx = start_idx + segment_len;
                let next_start_idx = end_idx;
                let next_end_idx = next_start_idx + segment_len;
                let segment = &as_string[start_idx..end_idx];
                let next_segment = &as_string[next_start_idx..next_end_idx];
                if segment != next_segment {
                    all_equal = false;
                    break;  
                }
            }
            if all_equal {
                return true;
            }
        }
        false
    }
}

pub fn main() {
    let filename = "input.txt";
    let content = std::fs::read_to_string(filename).expect("Failed to read input file");

    let ranges: Vec<_> = content
        .split(",")
        .into_iter()
        .map(|l| {
            let parts: Vec<usize> = l.split("-").map(|n| n.parse::<usize>().unwrap()).collect();
            Range {
                start: parts[0],
                end: parts[1],
            }
        })
        .collect();
    let mut invalid_values_part_1: Vec<usize> = Vec::new();
    let mut invalid_values_part_2: Vec<usize> = Vec::new();
    for range in ranges.iter() {
        invalid_values_part_1.extend(range.all_invalid_ids_part_1());
        invalid_values_part_2.extend(range.all_invalid_ids_part_2());
    }
    

    println!("Part 1: {}", invalid_values_part_1.iter().sum::<usize>());
    println!("Part 2: {}", invalid_values_part_2.iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_part_1() {
        assert!(1111.is_invalid_part_1());
        assert!(123123.is_invalid_part_1());
        assert!(!123456.is_invalid_part_1());
        assert!(!12312.is_invalid_part_1());    
    }

    #[test]
    fn test_is_invalid_part_2() {
        assert!(1111.is_invalid_part_2());
        // assert!(123123.is_invalid_part_2());
        // assert!(121212.is_invalid_part_2());
        // assert!(!123456.is_invalid_part_2());
        // assert!(!1231234.is_invalid_part_2());  
    }
}
