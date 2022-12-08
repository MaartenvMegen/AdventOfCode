pub struct Grid<T> {
    x_length: usize,
    contents: Vec<T>,
}

impl<T> Grid<T>
where T: Copy
{
    pub fn new(x_length: usize, contents: Vec<T>) -> Grid<T> {
        return Self { x_length, contents }
    }

    pub fn get_item_at_pos(&self, position: &Point) -> T {
        let (x, y) = *position;
        self.contents[x + self.x_length * y]
    }

    pub fn get_size_x(&self) -> usize {
        self.x_length
    }

    pub fn get_size_y(&self) -> usize {
        self.contents.len() / &self.x_length
    }

    pub fn index_to_point(&self, index: usize) -> Point {
        (index % self.x_length, index / self.x_length)
    }

    pub fn get_content(&self) -> &Vec<T> {
        &self.contents
    }
}

pub type Point = (usize, usize);
