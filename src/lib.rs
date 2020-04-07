#[derive(Debug, PartialEq, Clone)]
pub enum MapElement {
    Mine { open: bool },
    Empty { open: bool },
    Number { open: bool, count: i32 },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        let x = x as i32;
        let y = y as i32;
        Point { x, y }
    }
}

pub struct Board {
    map: Vec<Vec<MapElement>>,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Board {
    pub fn at(self: &Self, p: &Point) -> Option<&MapElement> {
        let width = self.width as i32;
        let height = self.height as i32;
        if p.x < 0 || p.x >= width || p.y < 0 || p.y >= height {
            return None;
        } else {
            let x = p.x as usize;
            let y = p.y as usize;
            return Some(&self.map[y][x]);
        }
    }

    pub fn replace(self: &Self, p: &Point, el: MapElement) -> Board {
        let map = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        if Point::new(x, y) == *p {
                            el.clone()
                        } else {
                            self.at(&Point::new(x, y)).unwrap().clone()
                        }
                    })
                    .collect()
            })
            .collect();
        Board {
            width: self.width,
            height: self.height,
            mines: self.mines,
            map: map,
        }
    }
}

pub fn create_board(
    width: usize,
    height: usize,
    mines: usize,
    mut rand: impl FnMut(usize, usize) -> usize,
) -> Board {
    let mut points: Vec<Point> = Vec::with_capacity(mines);
    for _ in 0..mines {
        loop {
            let x = rand(0, width);
            let y = rand(0, height);
            let p = Point::new(x, y);
            if points.contains(&p) {
                continue;
            }
            points.push(p);
            break;
        }
    }

    let map = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| match points.contains(&Point::new(x, y)) {
                    true => MapElement::Mine { open: false },
                    false => MapElement::Empty { open: false },
                })
                .collect()
        })
        .collect();
    Board {
        map,
        width,
        height,
        mines,
    }
}

pub fn surrounding_points(p: &Point) -> Vec<Point> {
    [p.x - 1, p.x, p.x + 1]
        .iter()
        .flat_map(|&x| {
            [p.y - 1, p.y, p.y + 1]
                .iter()
                .map(|&y| Point { x, y })
                .filter(|&Point { x, y }| p.x != x || p.y != y)
                .collect::<Vec<Point>>()
        })
        .collect()
}

pub fn numbers_on_board(board: Board) -> Board {
    let map = (0..board.height)
        .map(|y| {
            (0..board.width)
                .map(|x| {
                    let point = Point::new(x, y);
                    match board.at(&point) {
                        Some(MapElement::Mine { open: _ }) => MapElement::Mine { open: false },
                        Some(MapElement::Empty { open: _ }) => {
                            let count = surrounding_points(&point)
                                .iter()
                                .map(|p| match board.at(p) {
                                    None => 0,
                                    Some(MapElement::Mine { open: _ }) => 1,
                                    Some(MapElement::Empty { open: _ }) => 0,
                                    _ => 0,
                                })
                                .sum();
                            match count {
                                0 => MapElement::Empty { open: false },
                                _ => MapElement::Number { open: false, count },
                            }
                        }
                        _ => unreachable!(),
                    }
                })
                .collect()
        })
        .collect();
    Board {
        height: board.height,
        width: board.width,
        mines: board.mines,
        map: map,
    }
}

pub fn open_item(board: Board, point: Point) -> Board {
    let board_point = board.at(&point);

    let newpoint = match board_point {
        Some(MapElement::Empty { open: false }) => MapElement::Empty { open: true },
        Some(MapElement::Number { open: false, count }) => MapElement::Number {
            open: true,
            count: *count,
        },
        _ => unreachable!(),
    };

    board.replace(&point, newpoint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let width = 5;
        let height = 4;
        let mines = 4;
        let mut v = vec![3, 3, 2, 2, 1, 1, 0, 0];
        let rand = move |_start: usize, _end: usize| -> usize {
            return v.pop().unwrap();
        };
        let board = create_board(width, height, mines, rand);
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
            ],
        ];
        assert_eq!(board.map, expected_map);
    }

    #[test]
    fn test_create_board_without_repeated_mines() {
        let width = 5;
        let height = 4;
        let mines = 4;
        let mut v = vec![3, 3, 2, 2, 0, 0, 1, 1, 0, 0];
        let rand = move |_start: usize, _end: usize| -> usize {
            return v.pop().unwrap();
        };
        let board = create_board(width, height, mines, rand);
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
                MapElement::Mine { open: false },
                MapElement::Empty { open: false },
            ],
        ];
        assert_eq!(board.map, expected_map);
    }

    #[test]
    fn test_numbers_on_board() {
        let board = Board {
            height: 4,
            width: 5,
            mines: 4,
            map: vec![
                vec![
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                ],
            ],
        };
        let board_with_numbers = numbers_on_board(board);
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
            ],
            vec![
                MapElement::Empty { open: false },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
                MapElement::Number {
                    open: false,
                    count: 2,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    open: false,
                    count: 1,
                },
            ],
        ];
        assert_eq!(board_with_numbers.map, expected_map);
    }

    #[test]
    fn test_surrounding_points() {
        assert_eq!(
            surrounding_points(&Point { x: 1, y: 10 }),
            vec![
                Point { x: 0, y: 9 },
                Point { x: 0, y: 10 },
                Point { x: 0, y: 11 },
                Point { x: 1, y: 9 },
                Point { x: 1, y: 11 },
                Point { x: 2, y: 9 },
                Point { x: 2, y: 10 },
                Point { x: 2, y: 11 }
            ]
        );
    }

    #[test]
    fn test_open_item() {
        let board = Board {
            height: 2,
            width: 5,
            mines: 4,
            map: vec![
                vec![
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
                vec![
                    MapElement::Empty { open: false },
                    MapElement::Mine { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                    MapElement::Empty { open: false },
                ],
            ],
        };
        let board = numbers_on_board(board);
        let board = open_item(board, Point::new(1, 0));
        let expected_map = vec![
            vec![
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 2,
                    open: true,
                },
                MapElement::Number {
                    count: 1,
                    open: false,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
            vec![
                MapElement::Number {
                    count: 2,
                    open: false,
                },
                MapElement::Mine { open: false },
                MapElement::Number {
                    count: 1,
                    open: false,
                },
                MapElement::Empty { open: false },
                MapElement::Empty { open: false },
            ],
        ];
        assert_eq!(board.map, expected_map);
    }
}
