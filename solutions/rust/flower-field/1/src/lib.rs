pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    
    for (i, row) in minefield.iter().enumerate() {
        let mut annotated_row = String::new();
        for (j, &cell) in row.as_bytes().iter().enumerate() {
            if cell == b'*' {
                annotated_row.push('*');
            } else {
                let count = count_adjacent_mines(minefield, i, j);
                if count == 0 {
                    annotated_row.push(' ');
                } else {
                    annotated_row.push_str(&count.to_string());
                }
            }
        }
        result.push(annotated_row);
    }
    result
}

fn count_adjacent_mines(minefield: &[&str], row: usize, col: usize) -> u32 {
    let mut count = 0;
    
    // Check all 8 adjacent positions
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue; // Skip the current position
            }
            
            let new_row = row as i32 + di;
            let new_col = col as i32 + dj;
            
            if let Some(row_bytes) = minefield.get(new_row as usize) {
                if let Some(&cell) = row_bytes.as_bytes().get(new_col as usize) {
                    if cell == b'*' {
                        count += 1;
                    }
                }
            }
        }
    }
    
    count
}