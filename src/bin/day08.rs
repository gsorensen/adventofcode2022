use take_until::TakeUntilExt;

#[derive(Debug, Copy, Clone)]
struct Tree {
    height: usize,
    row: usize,
    col: usize,
}

impl Tree {
    fn from(height: usize, row: usize, col: usize) -> Tree {
        Tree { height, row, col }
    }
}

#[derive(Debug)]
struct Grove {
    trees: Vec<Tree>,
    rows: usize,
    cols: usize,
}

impl Grove {
    fn from(trees: Vec<Tree>) -> Grove {
        let rows = trees.iter().map(|t| t.row + 1).max().unwrap();
        let cols = trees.iter().map(|t| t.col + 1).max().unwrap();

        Grove { trees, rows, cols }
    }

    fn is_tree_visible_left(&self, tree: &Tree) -> bool {
        self.trees
            .iter()
            .filter(|t| t.row == tree.row && t.col < tree.col)
            .rev()
            .skip_while(|t| t.height < tree.height)
            .next()
            .is_none()
    }

    fn is_tree_visible_right(&self, tree: &Tree) -> bool {
        self.trees
            .iter()
            .filter(|t| t.row == tree.row && t.col > tree.col)
            .skip_while(|t| t.height < tree.height)
            .next()
            .is_none()
    }

    fn is_tree_visible_up(&self, tree: &Tree) -> bool {
        self.trees
            .iter()
            .filter(|t| t.row < tree.row && t.col == tree.col)
            .rev()
            .skip_while(|t| t.height < tree.height)
            .next()
            .is_none()
    }

    fn is_tree_visible_down(&self, tree: &Tree) -> bool {
        self.trees
            .iter()
            .filter(|t| t.row > tree.row && t.col == tree.col)
            .skip_while(|t| t.height < tree.height)
            .next()
            .is_none()
    }

    fn is_tree_visible(&self, tree: &Tree) -> bool {
        if self.is_tree_visible_up(tree) {
            return true;
        }
        
        if self.is_tree_visible_left(tree) {
            return true;
        }

        if self.is_tree_visible_right(tree) {
            return true;
        }

        if self.is_tree_visible_down(tree) {
            return true;
        }

        false
    }

    fn visible_trees(&self) -> Vec<&Tree> {
        self.trees
            .iter()
            .filter(|t| self.is_tree_visible(t))
            .collect::<Vec<&Tree>>()
    }

    fn scenic_score_left(&self, tree: &Tree) -> usize {
        if tree.col == 0 {
            return 0
        }

        self.trees
            .iter()
            .filter(|t| t.row == tree.row && t.col < tree.col)
            .rev()
            .take_until(|t| t.height >= tree.height)
            .count()
    }

    fn scenic_score_right(&self, tree: &Tree) -> usize {
        if tree.col == self.cols - 1 {
            return 0
        }
        
        self.trees
            .iter()
            .filter(|t| t.row == tree.row && t.col > tree.col)
            .take_until(|t| t.height >= tree.height)
            .count()
    }

    fn scenic_score_up(&self, tree: &Tree) -> usize {
        if tree.row == 0 {
            return 0
        }

        self.trees
            .iter()
            .filter(|t| t.row < tree.row && t.col == tree.col)
            .rev()
            .take_until(|t| t.height >= tree.height)
            .count()
    }

    fn scenic_score_down(&self, tree: &Tree) -> usize {
        if tree.row == self.rows - 1 {
            return 0
        }

        self.trees
            .iter()
            .filter(|t| t.row > tree.row && t.col == tree.col)
            .take_until(|t| t.height >= tree.height)
            .count()
    }

    fn scenic_score(&self, tree: &Tree) -> usize {
        let left = self.scenic_score_left(tree);
        let right = self.scenic_score_right(tree);
        let up = self.scenic_score_up(tree);
        let down = self.scenic_score_down(tree);

        left * right * down * up
    }
}

fn main() -> Result<(), anyhow::Error> {
    let height_map_str = include_str!("../inputs/day08.txt");

    let trees = height_map_str
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.split("")
                .enumerate()
                .filter_map(move |(col, height_str)| {
                    let height = height_str.parse::<usize>();

                    match height {
                        Ok(value) => Some(Tree::from(value, row, col - 1)),
                        Err(_) => None,
                    }
                })
        })
        .flatten()
        .collect::<Vec<Tree>>();

    let grove = Grove::from(trees);

    let num_visible_trees = grove.visible_trees().len();
    let highest_scenic_score = grove.trees.iter().map(|tree| grove.scenic_score(tree)).max().unwrap();

    println!("Number of visible trees: {}", num_visible_trees);
    println!("Highest scenic score: {}", highest_scenic_score);
    Ok(())
}
