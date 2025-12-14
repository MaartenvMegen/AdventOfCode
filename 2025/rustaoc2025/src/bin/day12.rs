use rustaoc2025::get_input;

#[derive(Debug)]
struct Shape {
    index: usize,
    /// grid[row][col] == true if '#', false if '.'
    grid: Vec<Vec<bool>>,
}

#[derive(Clone)]
struct ShapeRotations {
    rotations: Vec<Vec<(i32, i32)>>, // different orientations
    area: usize,                     // number of '#' cells
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    /// counts[i] = required number of presents of shape i
    counts: Vec<usize>,
}

#[derive(Debug)]
struct PuzzleInput {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_puzzle_input(input: &str) -> Result<PuzzleInput, String> {
    let mut lines = input.lines().peekable();
    let mut shapes: Vec<Shape> = Vec::new();

    // --- parse shapes section ---
    loop {
        // Skip empty lines
        while let Some(line) = lines.peek() {
            if line.trim().is_empty() {
                lines.next();
            } else {
                break;
            }
        }

        let Some(&line) = lines.peek() else {
            return Err("No regions section found".to_string());
        };

        // If line looks like "12x5: ..." we reached the regions section
        if is_region_line(line) {
            break;
        }

        // Expect something like "0:" or "1:"
        let index_line = lines.next().unwrap(); // safe, we peeked above
        let (idx_str, after_colon) = index_line
            .split_once(':')
            .ok_or_else(|| format!("Invalid shape header line: {index_line}"))?;

        let index: usize = idx_str
            .trim()
            .parse()
            .map_err(|e| format!("Invalid shape index `{idx_str}`: {e}"))?;

        // After "0:" there's usually nothing else on that line, but ignore trailing stuff if any
        if !after_colon.trim().is_empty() {
            // allow but ignore trailing stuff
        }

        // Read grid lines until a blank line or region section or EOF
        let mut grid_rows: Vec<Vec<bool>> = Vec::new();

        while let Some(next_line) = lines.peek() {
            let trimmed = next_line.trim_end();
            if trimmed.is_empty() {
                break;
            }
            if is_region_line(trimmed) {
                break;
            }

            // Consume the line
            let line_str = lines.next().unwrap().trim_end().to_string();

            // convert line to Vec<bool>
            let row: Vec<bool> = line_str
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    other => {
                        // you can relax this if needed
                        panic!("Unexpected character in shape grid: {other}");
                    }
                })
                .collect();

            grid_rows.push(row);
        }

        if grid_rows.is_empty() {
            return Err(format!("Shape {index} has no grid rows"));
        }

        shapes.push(Shape {
            index,
            grid: grid_rows,
        });

        // Next loop iteration: will skip blank line if present
    }

    // Optional: ensure shapes are sorted by index
    shapes.sort_by_key(|s| s.index);

    // --- parse regions section ---
    let mut regions: Vec<Region> = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // e.g. "12x5: 1 0 1 0 2 2"
        let (size_part, counts_part) = line
            .split_once(':')
            .ok_or_else(|| format!("Invalid region line (no colon): {line}"))?;

        let size_part = size_part.trim();
        let counts_part = counts_part.trim();

        // size_part like "12x5"
        let (w_str, h_str) = size_part
            .split_once('x')
            .ok_or_else(|| format!("Invalid region size `{size_part}`"))?;

        let width: usize = w_str
            .trim()
            .parse()
            .map_err(|e| format!("Invalid width `{w_str}`: {e}"))?;
        let height: usize = h_str
            .trim()
            .parse()
            .map_err(|e| format!("Invalid height `{h_str}`: {e}"))?;

        // counts: "1 0 1 0 2 2"
        let counts: Vec<usize> = counts_part
            .split_whitespace()
            .map(|tok| {
                tok.parse::<usize>()
                    .map_err(|e| format!("Invalid shape count `{tok}`: {e}"))
            })
            .collect::<Result<_, _>>()?;

        regions.push(Region {
            width,
            height,
            counts,
        });
    }

    Ok(PuzzleInput { shapes, regions })
}

fn is_region_line(line: &str) -> bool {
    // Rough heuristic: something like "12x5: ..." (starts with digits, then 'x', then digits, then ':')
    let line = line.trim();
    if let Some((left, _)) = line.split_once(':') {
        if let Some((w, h)) = left.split_once('x') {
            return w.trim().chars().all(|c| c.is_ascii_digit())
                && h.trim().chars().all(|c| c.is_ascii_digit());
        }
    }
    false
}

// Convert grid of '#' / '.' into a list of cell coordinates for the base orientation
fn shape_cells(shape: &Shape) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    for (y, row) in shape.grid.iter().enumerate() {
        for (x, &filled) in row.iter().enumerate() {
            if filled {
                cells.push((x as i32, y as i32));
            }
        }
    }
    normalize_cells(&cells)
}

// Translate so that min x,y becomes (0,0)
fn normalize_cells(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
    cells.iter().map(|(x, y)| (x - min_x, y - min_y)).collect()
}

// Rotate 90° clockwise around origin, then normalize
fn rotate_90(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let rotated: Vec<(i32, i32)> = cells.iter().map(|(x, y)| (*y, -*x)).collect();
    normalize_cells(&rotated)
}

// Get up to 4 unique rotations for a shape
fn all_rotations(shape: &Shape) -> Vec<Vec<(i32, i32)>> {
    let mut rots = Vec::new();
    let mut current = shape_cells(shape);
    for _ in 0..4 {
        if !rots.contains(&current) {
            rots.push(current.clone());
        }
        current = rotate_90(&current);
    }
    rots
}

fn precompute_shape_rotations(shapes: &[Shape]) -> Vec<ShapeRotations> {
    shapes
        .iter()
        .map(|s| {
            let rotations = all_rotations(s);
            let area = rotations[0].len(); // all rotations have same area
            ShapeRotations { rotations, area }
        })
        .collect()
}

fn can_fit_region(
    shape_rots: &[ShapeRotations], // precomputed cache
    region: &Region,
) -> bool {
    let w = region.width as i32;
    let h = region.height as i32;

    // 0) Area check: if total shape area > region area, impossible
    let total_shape_area: usize = region
        .counts
        .iter()
        .enumerate()
        .map(|(i, &count)| count * shape_rots[i].area)
        .sum();

    let region_area = region.width * region.height;

    if total_shape_area > region_area {
        return false;
    }

    // (optional) if total_shape_area < region_area, still might be OK (unused space allowed)

    // 1) Expand counts into a multiset of shape indices
    let mut to_place: Vec<usize> = Vec::new();
    for (shape_idx, &count) in region.counts.iter().enumerate() {
        for _ in 0..count {
            to_place.push(shape_idx);
        }
    }

    if to_place.is_empty() {
        return true; // nothing to place
    }

    // 2) Sort shapes by area (descending) – larger ones first
    to_place.sort_by_key(|&idx| {
        let area = shape_rots[idx].area as i32;
        -area
    });

    // 3) region grid: false = empty, true = occupied
    let mut grid = vec![vec![false; region.width]; region.height];

    // 4) Backtracking
    fn backtrack(
        pos: usize,
        to_place: &[usize],
        shape_rots: &[ShapeRotations],
        grid: &mut [Vec<bool>],
        w: i32,
        h: i32,
    ) -> bool {
        if pos == to_place.len() {
            return true; // all shapes placed
        }

        let shape_idx = to_place[pos];
        let rotations = &shape_rots[shape_idx].rotations;

        for cells in rotations {
            let max_x = cells.iter().map(|(x, _)| *x).max().unwrap();
            let max_y = cells.iter().map(|(_, y)| *y).max().unwrap();

            for oy in 0..=(h - max_y - 1) {
                for ox in 0..=(w - max_x - 1) {
                    if can_place(cells, ox, oy, grid, w, h) {
                        place(cells, ox, oy, grid, true);
                        if backtrack(pos + 1, to_place, shape_rots, grid, w, h) {
                            return true;
                        }
                        place(cells, ox, oy, grid, false);
                    }
                }
            }
        }

        false
    }

    backtrack(0, &to_place, shape_rots, &mut grid, w, h)
}
fn can_place(cells: &[(i32, i32)], ox: i32, oy: i32, grid: &[Vec<bool>], w: i32, h: i32) -> bool {
    for (dx, dy) in cells {
        let x = ox + dx;
        let y = oy + dy;
        if x < 0 || y < 0 || x >= w || y >= h {
            return false;
        }
        if grid[y as usize][x as usize] {
            return false; // collision
        }
    }
    true
}

fn place(cells: &[(i32, i32)], ox: i32, oy: i32, grid: &mut [Vec<bool>], value: bool) {
    for (dx, dy) in cells {
        let x = (ox + dx) as usize;
        let y = (oy + dy) as usize;
        grid[y][x] = value;
    }
}

fn main() {
    let input = get_input("day12-input.txt");

    println!("{}", solve(&input));
}

fn solve(input: &str) -> u64 {
    let parser = parse_puzzle_input(input).unwrap();
    let mut count = 0;
    let shape_rots = precompute_shape_rotations(&parser.shapes);
    for region in parser.regions {
        let fits = can_fit_region(&shape_rots, &region);
        if fits {
            count += 1;
        }
        println!("Region ({},{}) fits: {}", region.width, region.width, fits);
    }
    count
}
