use std::io;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let mut forest = Forest::from(lines);
    forest.assess_visibility();
    println!("Visibile from outside: {}", forest.count_visible());
    println!("Best scenic score: {}", forest.assess_scenic_scores());

    // println!("Hello, world!");
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct Tree {
    height: u8,
    visibility: u8,
}

#[derive(Copy, Clone)]
enum Visibility {
    Left = 1,
    Right = 1 << 1,
    Top = 1 << 2,
    Bottom = 1 << 3,
}

impl Tree {
    fn set_visibility(&mut self, v: Visibility) {
        self.visibility |= v as u8
    }
    // fn unset_visibility(&mut self, v: Visibility) {
    //     self.visibility ^= v as u8
    // }
    fn get_visibility(&self, v: Visibility) -> bool {
        (self.visibility & v as u8) > 0
    }
    fn is_visible(&self) -> bool {
        self.visibility > 0
    }
}

impl From<u8> for Tree {
    fn from(height: u8) -> Self {
        Self {
            height,
            visibility: 0,
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let shape = self.shape();
        write!(f, "   ")?;
        for x in 0..shape.x {
            write!(f, " {:02}", x)?;
        }
        writeln!(f)?;
        for y in 0..shape.y {
            // top
            write!(f, "   ")?;
            for x in 0..shape.x {
                if self[(x, y)].get_visibility(Visibility::Top) {
                    write!(f, " \u{25B2} ")?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f)?;
            // middle
            write!(f, "{:02} ", y)?;
            for x in 0..shape.x {
                if self[(x, y)].get_visibility(Visibility::Left) {
                    write!(f, "\u{25C0}")?;
                } else {
                    write!(f, " ")?;
                }
                write!(f, "{}", self[(x, y)].height)?;
                if self[(x, y)].get_visibility(Visibility::Right) {
                    write!(f, "\u{25B6}")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
            // bottom
            write!(f, "   ")?;
            for x in 0..shape.x {
                if self[(x, y)].get_visibility(Visibility::Bottom) {
                    write!(f, " \u{25BC} ")?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: AsRef<str>, S: Iterator<Item = T>> From<S> for Forest {
    fn from(source: S) -> Self {
        let trees = source
            .map(|l| {
                l.as_ref()
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|x| x as u8))
                    .map(|n| n.into())
                    .collect::<Vec<Tree>>()
            })
            .collect::<Vec<Vec<Tree>>>();
        Forest { trees }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(c: (usize, usize)) -> Self {
        Self { x: c.0, y: c.1 }
    }
}

struct Coord {
    x: usize,
    y: usize,
}

use std::ops::{Index, IndexMut};
impl<C: Into<Coord>> Index<C> for Forest {
    type Output = Tree;

    fn index(&self, index: C) -> &Self::Output {
        let index = index.into();
        &self.trees[index.y][index.x]
    }
}

impl<C: Into<Coord>> IndexMut<C> for Forest {
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        let index = index.into();
        &mut self.trees[index.y][index.x]
    }
}

/// Returns the indexes of the trees that can be seen with a given view
fn get_view(view: &[&Tree]) -> Vec<usize> {
    let mut heights: Vec<u8> = view.iter().map(|t| t.height).collect();
    heights.sort();
    heights.dedup();
    let mut viewed_trees = vec![];
    for h in heights {
        for (index, tree) in view.iter().enumerate() {
            if tree.height >= h {
                // tree.set_visibility(direction);
                viewed_trees.push(index);
                break;
            }
        }
    }
    viewed_trees
}

impl Forest {
    fn count_visible(&self) -> usize {
        (0..self.shape().x)
            .map(|x| {
                (0..self.shape().y)
                    .filter(|&y| self[(x, y)].is_visible())
                    .count()
            })
            .sum()
    }

    fn shape(&self) -> Coord {
        (self.trees[0].len(), self.trees.len()).into()
    }
    /// Given a forest, assess based on tree heights which trees are visible from
    /// the outside.
    fn assess_visibility(&mut self) {
        // edges
        let shape = self.shape();
        (0..shape.x).for_each(|x| self[(x, 0)].set_visibility(Visibility::Top));
        (0..shape.x).for_each(|x| self[(x, shape.y - 1)].set_visibility(Visibility::Bottom));
        (0..shape.y).for_each(|y| self[(0, y)].set_visibility(Visibility::Left));
        (0..shape.y).for_each(|y| self[(shape.x - 1, y)].set_visibility(Visibility::Right));

        // for each direction
        // horizontal
        for x in 0..shape.x {
            // bottom up
            let b_u_slice: Vec<&Tree> = (0..shape.y).map(|y| &self[(x, y)]).collect();
            let trees_viewed = get_view(&b_u_slice);
            trees_viewed
                .into_iter()
                .for_each(|y| self[(x, y)].set_visibility(Visibility::Top));

            // top down
            let t_d_slice: Vec<&Tree> = (0..shape.y).rev().map(|y| &self[(x, y)]).collect();
            let trees_viewed = get_view(&t_d_slice);
            trees_viewed
                .into_iter()
                .for_each(|y| self[(x, shape.y - y - 1)].set_visibility(Visibility::Bottom));
        }

        // horizontal
        for y in 0..shape.y {
            // right going left
            let r_l_slice: Vec<&Tree> = (0..shape.x).map(|x| &self[(x, y)]).collect();
            let trees_viewed = get_view(&r_l_slice);
            trees_viewed
                .into_iter()
                .for_each(|x| self[(x, y)].set_visibility(Visibility::Left));

            // // left going right
            let l_r_slice: Vec<&Tree> = (0..shape.x).rev().map(|x| &self[(x, y)]).collect();
            let trees_viewed = get_view(&l_r_slice);
            trees_viewed
                .into_iter()
                .for_each(|x| self[(shape.x - x - 1, y)].set_visibility(Visibility::Right));
        }

        // dbg!(self.count_visible());
    }

    fn assess_scenic_scores(&self) -> usize {
        let shape = self.shape();
        let mut max_score = 0;
        for y in 0..shape.y {
            for x in 0..shape.x {
                #[cfg(test)]
                dbg!(&x);
                #[cfg(test)]
                dbg!(&y);

                let left_vec = if x != 0 {
                    (0..x).map(|x_l| &self[(x_l, y)]).rev().collect()
                } else {
                    vec![]
                };
                #[cfg(test)]
                dbg!(&left_vec);

                let right_vec = if x != shape.x - 1 {
                    (x + 1..shape.x).map(|x_l| &self[(x_l, y)]).collect()
                } else {
                    vec![]
                };
                #[cfg(test)]
                dbg!(&right_vec);

                let top_vec = if y != 0 {
                    (0..y).map(|y_l| &self[(x, y_l)]).rev().collect()
                } else {
                    vec![]
                };
                #[cfg(test)]
                dbg!(&top_vec);

                let bottom_vec = if y != shape.y - 1 {
                    (y + 1..shape.y).map(|y_l| &self[(x, y_l)]).collect()
                } else {
                    vec![]
                };
                #[cfg(test)]
                dbg!(&bottom_vec);

                #[cfg(test)]
                print!("--(");
                let score = [top_vec, bottom_vec, left_vec, right_vec]
                    .into_iter()
                    .map(|v| {
                        let score = v
                            .iter()
                            .enumerate()
                            .find_map(|(index, &t)| {
                                if t.height >= self[(x, y)].height {
                                    Some(index + 1)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(v.len());
                        #[cfg(test)]
                        print!("{score},");
                        score
                    })
                    .product::<usize>();
                #[cfg(test)]
                print!(")({score})");
                max_score = max_score.max(score);
            }
            #[cfg(test)]
            println!("");
        }
        max_score
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_forrest_from_str_iter() {
        let input = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];

        let ref_output = Forest {
            trees: vec![
                vec![
                    Tree::from(3),
                    Tree::from(0),
                    Tree::from(3),
                    Tree::from(7),
                    Tree::from(3),
                ],
                vec![
                    Tree::from(2),
                    Tree::from(5),
                    Tree::from(5),
                    Tree::from(1),
                    Tree::from(2),
                ],
                vec![
                    Tree::from(6),
                    Tree::from(5),
                    Tree::from(3),
                    Tree::from(3),
                    Tree::from(2),
                ],
                vec![
                    Tree::from(3),
                    Tree::from(3),
                    Tree::from(5),
                    Tree::from(4),
                    Tree::from(9),
                ],
                vec![
                    Tree::from(3),
                    Tree::from(5),
                    Tree::from(3),
                    Tree::from(9),
                    Tree::from(0),
                ],
            ],
        };

        let dut_output = Forest::from(input.iter());

        assert_eq!(ref_output, dut_output);
    }

    #[test]
    #[ignore]
    fn test_display_forest() {
        let input = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];

        let mut f = Forest::from(input.iter());
        f.assess_visibility();
        println!("forest:\n{f}")
    }

    #[test]
    fn visibility() {
        let mut t = Tree::from(5);
        t.set_visibility(Visibility::Top);
        assert!(t.get_visibility(Visibility::Top));
        assert!(!t.get_visibility(Visibility::Bottom));
        assert!(!t.get_visibility(Visibility::Left));
        assert!(!t.get_visibility(Visibility::Right));
        assert!(t.is_visible());
        // t.unset_visibility(Visibility::Top);
        // assert!(!t.is_visible());
    }

    #[test]
    fn count_visible() {
        let input = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];
        let mut f = Forest::from(input.iter());
        f.assess_visibility();
        assert_eq!(f.count_visible(), 21);
    }

    #[test]
    fn test_view() {
        //
        //   N
        //   N
        //   N   N   N
        // N N N N N N N
        // N N N N N N N N
        // N N M N M N N N << Seen from this direction
        let trees = vec![3, 6, 3, 4, 3, 4, 3, 2]
            .iter()
            .rev()
            .map(|h: &u8| Tree::from(*h))
            .collect::<Vec<Tree>>();
        let mut tree_refs: Vec<&Tree> = trees.iter().map(|t_r| t_r).collect();

        let trees_seen = get_view(&mut tree_refs);

        assert_eq!(trees_seen.len(), 4);

        let ref_trees_seen: Vec<usize> = vec![0, 1, 2, 6];
        //
        assert_eq!(trees_seen, ref_trees_seen);
    }

    #[test]
    fn test_assess_scenic_score() {
        let input = vec![
            "30373".to_owned(),
            "25512".to_owned(),
            "65332".to_owned(),
            "33549".to_owned(),
            "35390".to_owned(),
        ];
        let f = Forest::from(input.iter());
        let max_score = f.assess_scenic_scores();
        assert_eq!(max_score, 8);
    }
}
