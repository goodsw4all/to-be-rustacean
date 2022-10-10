pub fn get_adjacent_list(point: (usize, usize), max_coord: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let (max_x, max_y) = max_coord;
    let mut coords: Vec<(usize, usize)> = Vec::new();

    // upper row
    if y > 0 {
        let upper_y = y - 1;
        // upper left
        if x > 0 {
            coords.push((x - 1, upper_y));
        }
        // upper center
        coords.push((x, upper_y));

        // upper right
        if x < max_x {
            coords.push((x + 1, upper_y));
        }
    }

    // same row
    // left
    if x > 0 {
        coords.push((x - 1, y));
    }
    // right
    if x < max_x {
        coords.push((x + 1, y));
    }

    // lower row
    if y < max_y {
        let lower_y = y + 1;
        // lower left
        if x > 0 {
            coords.push((x - 1, lower_y));
        }
        // lower center
        coords.push((x, lower_y));

        // lower right
        if x < max_x {
            coords.push((x + 1, lower_y));
        }
    }
    coords
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let y_len = minefield.len();
    if y_len == 0 {
        return minefield.iter().map(|x| x.to_string()).collect();
    }

    let x_len = minefield[0].len();
    if x_len == 0 {
        return minefield.iter().map(|x| x.to_string()).collect();
    }

    let mut rows: Vec<Vec<char>> = minefield.iter().map(|row| row.chars().collect()).collect();

    for y in 0..y_len {
        for x in 0..x_len {
            if rows[y][x] == '*' {
                continue;
            }

            let mut total = 0;
            let adj_list = get_adjacent_list((x, y), (x_len - 1, y_len - 1));

            for coord in adj_list {
                if rows[coord.1][coord.0] == '*' {
                    total += 1
                }
            }
            rows[y][x] = if total == 0 {
                ' '
            } else {
                char::from_digit(total, 10).unwrap()
            }
        }
    }

    rows.iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}
