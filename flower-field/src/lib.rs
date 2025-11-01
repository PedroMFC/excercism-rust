fn valid_index(i: &isize, j: &isize, maxi: &isize, maxj: &isize) -> bool {
    i >= &0 && i < maxi && j >= &0 && j < maxj
}

fn convolution(row: &isize, col: &isize, garden: &[&str]) -> u8 {
    let mut count = 0;
    let maxi = garden.len() as isize;
    let maxj = garden[*row as usize].len() as isize;

    for i in -1..=1 {
        for j in -1..=1 {
            let iy = row + i;
            let ix = col + j;

            if valid_index(&iy, &ix, &maxi, &maxj) {
                if garden[iy as usize].as_bytes()[ix as usize] == b'*' {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..garden.len() {
        let mut new_row = String::new();
        for j in 0..garden[i].len() {
            if garden[i].as_bytes()[j] == b'*' {
                new_row.push('*'); 
            } else {
                let count = convolution(&(i as isize), &(j as isize), garden);
                if count == 0 {
                    new_row.push(' ');
                } else {
                    new_row.push_str(&count.to_string());
                }
            }
        }
        result.push(new_row);
    }
    result
}

// USING map
// pub fn annotate(garden: &[&str]) -> Vec<String> {
//     (0..garden.len())
//         .map(|r| {
//             (0..garden[r].len())
//                 .map(|c| get_garden_char(garden, r as i32, c as i32))
//                 .collect::<String>()
//         })
//         .collect::<Vec<String>>()
// }
// fn get_garden_char(garden: &[&str], r: i32, c: i32) -> char {
//     match garden[r as usize].as_bytes()[c as usize] {
//         b'*' => '*',
//         _ => {
//             let maxr = garden.len() as i32 - 1;
//             let maxc = garden[r as usize].len() as i32 - 1;
//             match ((r - 1).max(0)..=(r + 1).min(maxr))
//                 .flat_map(|newr| ((c - 1).max(0)..=(c + 1).min(maxc)).map(move |newc| (newr, newc)))
//                 .filter(|&(r, c)| garden[r as usize].as_bytes()[c as usize] == b'*')
//                 .count()
//             {
//                 0 => ' ',
//                 count => (count as u8 + b'0') as char,
//             }
//         }
//     }
// }