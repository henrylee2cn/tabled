pub struct Grid {
    size: (usize, usize),
    cells: Vec<Cell>,
}

#[derive(Clone)]
pub struct Cell {
    content: String,
    alignment: Alignment,
    border: Border,
    ident: Ident,
}

#[derive(Clone)]
struct Border {
    top: String,
    bottom: String,
    left: String,
    right: String,
    corner: String,
}

#[derive(Clone)]
struct Ident {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Center,
    Left,
    Right,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        Grid {
            size: (rows, columns),
            cells: vec![Cell::new(); rows * columns],
        }
    }

    pub fn cell(&mut self, i: usize, j: usize) -> &mut Cell {
        let index = self.count_columns() * i + j;
        self.cells.get_mut(index).unwrap()
    }

    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    pub fn count_columns(&self) -> usize {
        self.size.1
    }

    fn rows(&self) -> Vec<&[Cell]> {
        (0..self.size.0).map(|i| self.row(i)).collect()
    }

    fn row(&self, i: usize) -> &[Cell] {
        let start_index = self.count_columns() * i;
        &self.cells[start_index..start_index + self.count_columns()]
    }

    fn columns(&self) -> Vec<Vec<&Cell>> {
        (0..self.count_columns()).map(|i| self.column(i)).collect()
    }

    fn column(&self, j: usize) -> Vec<&Cell> {
        (0..self.count_rows())
            .map(|i| self.count_columns() * i + j)
            .map(|i| &self.cells[i])
            .collect()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows_height = self
            .rows()
            .iter()
            .map(|r| r.iter().map(|c| c.height()).max().map_or(0, |m| m))
            .collect::<Vec<usize>>();

        let columns_weight = self
            .columns()
            .iter()
            .map(|r| r.iter().map(|c| c.weight()).max().map_or(0, |m| m))
            .collect::<Vec<usize>>();

        let cells = self
            .rows()
            .iter()
            .enumerate()
            .map(|(row_index, r)| {
                r.iter()
                    .enumerate()
                    .fold(Vec::new(), |mut rows, (column_index, cell)| {
                        let mut formatter = CellFormatter::new()
                            .weight(columns_weight[column_index])
                            .height(rows_height[row_index])
                            .boxed();

                        if column_index != 0 {
                            formatter = formatter.un_left().un_left_connection();
                        }

                        if row_index != 0 {
                            formatter = formatter.un_top();
                        }

                        rows.push(formatter.format(&cell));

                        rows
                    })
            })
            .collect::<Vec<Vec<String>>>();

        cells.iter().for_each(|row| {
            writeln!(f, "{}", concat_row(row));
        });

        Ok(())
    }
}

impl Cell {
    fn new() -> Self {
        Cell {
            alignment: Alignment::Center,
            content: String::new(),
            border: Border {
                top: "-".to_owned(),
                bottom: "-".to_owned(),
                left: "|".to_owned(),
                right: "|".to_owned(),
                corner: "+".to_owned(),
            },
            ident: Ident {
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
            },
        }
    }

    pub fn set_content(&mut self, s: &str) -> &mut Self {
        self.content = s.to_owned();
        self
    }

    pub fn set_corner(&mut self, s: &str) -> &mut Self {
        self.border.corner = s.to_owned();
        self
    }

    pub fn set_alignment(&mut self, a: Alignment) -> &mut Self {
        self.alignment = a;
        self
    }

    pub fn set_vertical_ident(&mut self, size: usize) -> &mut Self {
        self.ident.top = size;
        self.ident.bottom = size;
        self
    }

    pub fn set_horizontal_ident(&mut self, size: usize) -> &mut Self {
        self.ident.left = size;
        self.ident.right = size;
        self
    }

    fn height(&self) -> usize {
        self.content.lines().count()
    }

    fn weight(&self) -> usize {
        self.content
            .lines()
            .map(|l| l.len())
            .max()
            .map_or(0, |max| max)
    }
}

struct CellFormatter {
    left: Option<()>,
    right: Option<()>,
    top: Option<()>,
    bottom: Option<()>,
    left_connection: Option<()>,
    right_connection: Option<()>,
    weight: usize,
    height: usize,
}

impl CellFormatter {
    fn new() -> Self {
        CellFormatter {
            left: None,
            right: None,
            top: None,
            bottom: None,
            left_connection: None,
            right_connection: None,
            weight: 0,
            height: 0,
        }
    }

    fn un_left(mut self) -> Self {
        self.left = None;
        self
    }

    fn un_left_connection(mut self) -> Self {
        self.left_connection = None;
        self
    }

    fn un_top(mut self) -> Self {
        self.top = None;
        self
    }

    fn boxed(mut self) -> Self {
        self.left = Some(());
        self.right = Some(());
        self.top = Some(());
        self.bottom = Some(());
        self.right_connection = Some(());
        self.left_connection = Some(());
        self
    }

    fn weight(mut self, w: usize) -> Self {
        self.weight = w;
        self
    }

    fn height(mut self, h: usize) -> Self {
        self.height = h;
        self
    }

    fn format(&self, c: &Cell) -> String {
        let weight = if self.weight == 0 {
            c.content
                .lines()
                .map(|l| l.chars().count())
                .max()
                .map_or(0, |max| max)
        } else {
            self.weight
        };

        let mut content = c.content.clone();
        let count_lines = c.content.lines().count();

        if count_lines < self.height {
            content.push_str(&"\n".repeat(self.height - count_lines + 1))
        }

        content.push_str(&"\n".repeat(c.ident.bottom));
        content.insert_str(0, &"\n".repeat(c.ident.top));

        let left_ident = " ".repeat(c.ident.left);
        let right_ident = " ".repeat(c.ident.right);

        let left_border = self.left.map_or("", |_| &c.border.left);
        let right_border = self.right.map_or("", |_| &c.border.right);

        let mut lines = content
            .lines()
            .map(|l| align(l, c.alignment, weight))
            .map(|l| format!("{}{}{}", left_ident, l, right_ident))
            .map(|l| {
                format!(
                    "{left:}{}{right:}",
                    l,
                    left = left_border,
                    right = right_border,
                )
            })
            .collect::<Vec<String>>();

        let lhs = self.left_connection.map_or("", |_| &c.border.corner);
        let rhs = self.right_connection.map_or("", |_| &c.border.corner);

        let weight = weight + c.ident.left + c.ident.right;

        if self.top.is_some() {
            let line = lhs.to_owned() + &c.border.top.repeat(weight) + rhs;
            lines.insert(0, line);
        }
        if self.bottom.is_some() {
            let line = lhs.to_owned() + &c.border.bottom.repeat(weight) + rhs;
            lines.push(line);
        }

        lines.join("\n")
    }
}

fn align(text: &str, a: Alignment, length: usize) -> String {
    match a {
        Alignment::Center => format!("{: ^1$}", text, length),
        Alignment::Left => format!("{: <1$}", text, length),
        Alignment::Right => format!("{: >1$}", text, length),
    }
}

fn concat_row(row: &[String]) -> String {
    let mut iter = row.iter();
    if let Some(row) = iter.next() {
        let mut row = row.to_owned();
        for c in iter {
            row = concat_lines(&row, c);
        }

        row
    } else {
        "".to_owned()
    }
}

fn concat_lines(a: &str, b: &str) -> String {
    assert_eq!(a.lines().count(), b.lines().count());
    a.lines()
        .zip(b.lines())
        .map(|(a, b)| a.to_owned() + b)
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod grid {
        use super::super::*;

        #[test]
        fn render() {
            let mut grid = Grid::new(2, 2);
            grid.cell(0, 0).set_content("0-0");
            grid.cell(0, 1).set_content("0-1");
            grid.cell(1, 0).set_content("1-0");
            grid.cell(1, 1).set_content("1-1");

            let expected = concat!(
                "+---+---+\n",
                "|0-0|0-1|\n",
                "+---+---+\n",
                "|1-0|1-1|\n",
                "+---+---+\n",
            );

            assert_eq!(expected, grid.to_string());
        }

        #[test]
        fn render_multilane() {
            let mut grid = Grid::new(2, 2);
            grid.cell(0, 0).set_content("left\ncell");
            grid.cell(0, 1).set_content("right one");
            grid.cell(1, 0)
                .set_content("the second column got the beginning here");
            grid.cell(1, 1)
                .set_content("and here\nwe\nsee\na\nlong\nstring");

            let expected = concat!(
                "+----------------------------------------+---------+\n",
                "|                  left                  |right one|\n",
                "|                  cell                  |         |\n",
                "+----------------------------------------+---------+\n",
                "|the second column got the beginning here|and here |\n",
                "|                                        |   we    |\n",
                "|                                        |   see   |\n",
                "|                                        |    a    |\n",
                "|                                        |  long   |\n",
                "|                                        | string  |\n",
                "+----------------------------------------+---------+\n",
            );

            assert_eq!(expected, grid.to_string());
        }

        #[test]
        fn render_one_line() {
            let mut grid = Grid::new(1, 1);
            grid.cell(0, 0).set_content("one line");

            let expected = concat!("+--------+\n", "|one line|\n", "+--------+\n",);

            assert_eq!(expected, grid.to_string());
        }

        #[test]
        fn render_not_quadratic() {
            let mut grid = Grid::new(1, 2);
            grid.cell(0, 0).set_content("hello");
            grid.cell(0, 1).set_content("world");

            let expected = concat!("+-----+-----+\n", "|hello|world|\n", "+-----+-----+\n",);

            assert_eq!(expected, grid.to_string());
        }

        #[test]
        fn render_empty() {
            let grid = Grid::new(0, 0);

            let expected = "";

            assert_eq!(expected, grid.to_string());
        }
    }

    #[test]
    // Might this behavior should be changed
    fn cell_formating_empty() {
        let mut cell = Cell::new();
        cell.set_content("").set_corner("-");

        let expected = concat!("--\n", "--");

        assert_eq!(expected, CellFormatter::new().boxed().format(&cell));
    }

    #[test]
    fn cell_formating_single() {
        let mut cell = Cell::new();
        cell.set_content("hello world").set_corner("-");

        let expected = concat!("-------------\n", "|hello world|\n", "-------------");

        assert_eq!(expected, CellFormatter::new().boxed().format(&cell));
    }

    #[test]
    fn cell_formating_multiline() {
        let mut cell = Cell::new();
        cell.set_content("hello\nworld").set_corner("-");

        let expected = concat!("-------\n", "|hello|\n", "|world|\n", "-------");

        assert_eq!(expected, CellFormatter::new().boxed().format(&cell));
    }

    #[test]
    fn cell_formating_multilane_forced() {
        let mut cell = Cell::new();
        cell.set_content("hello").set_corner("-");

        let expected = concat!("-------\n", "|hello|\n", "|     |\n", "-------");

        assert_eq!(
            expected,
            CellFormatter::new().boxed().height(2).format(&cell)
        );
    }
}