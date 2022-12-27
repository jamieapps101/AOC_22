// ignore warnings here as code was abandoned
#![allow(clippy::all)]

use std::cell::{Ref, RefCell};
use std::io;
use std::ops::Deref;
use std::rc::Rc;

enum FSObject {
    File(FileData),
    Directory(DirectoryData),
}

impl FSObject {
    fn name(&self) -> &str {
        match self {
            Self::Directory(data) => data.name.as_str(),
            Self::File(data) => data.name.as_str(),
        }
    }

    fn get_content(&self) -> Option<&[Rc<RefCell<FSObject>>]> {
        if let FSObject::Directory(dir) = self {
            return Some(&dir.content);
        }
        None
    }

    fn create_directory<S: ToString>(name: S) -> Self {
        Self::Directory(DirectoryData {
            name: name.to_string(),
            content: vec![],
        })
    }

    fn create_file<S: ToString>(name: S, size: usize) -> Self {
        Self::File(FileData {
            name: name.to_string(),
            size,
        })
    }

    fn add_obj(&mut self, d: FSObject) -> Result<(), ()> {
        if let Self::Directory(data) = self {
            data.content.push(Rc::new(RefCell::new(d)));
        } else {
            return Err(());
        }
        return Ok(());
    }
}

struct FileData {
    name: String,
    size: usize,
}

struct DirectoryData {
    name: String,
    content: Vec<Rc<RefCell<FSObject>>>,
}

struct FileSystem {
    root: Rc<RefCell<FSObject>>,
    head: Vec<String>,
}

impl FileSystem {
    fn cd<S: AsRef<str>>(&mut self, dir: S) -> Result<(), String> {
        match dir.as_ref() {
            ".." => {
                // cd back
                let _ = self.head.pop();
                return Ok(());
            }
            "/" => {
                // top level dir
                self.head.clear();
                return Ok(());
            }
            _ => {
                // anything else
                let head = self.get_head();
                let head_ref = head.borrow();
                if let Some(content) = head_ref.get_content() {
                    for item in content {
                        if item.borrow().name() == dir.as_ref() {
                            self.head.push(dir.as_ref().to_owned());
                            return Ok(());
                        }
                    }
                    return Err(format!(
                        "No dir {} in dir {}. (Head: {:?})",
                        dir.as_ref(),
                        self.head.last().unwrap(),
                        self.head
                    ));
                } else {
                    return Err(format!(
                        "Head currently not on a file (Head: {:?})",
                        self.head
                    ));
                }
            }
        }
    }

    fn get_head(&mut self) -> Rc<RefCell<FSObject>> {
        let mut pointer: Rc<RefCell<FSObject>> = self.root.clone();
        for item_name in &self.head {
            let t = pointer.clone();
            let u: Ref<FSObject> = t.borrow();
            match u.deref() {
                FSObject::File(data) => {
                    if item_name == &data.name {
                        break;
                    } else {
                        unreachable!()
                    }
                }
                FSObject::Directory(data) => {
                    if item_name == &data.name {
                        break;
                    } else {
                        for pointer_sub_obj in &data.content {
                            if pointer_sub_obj.borrow().name() == item_name {
                                pointer = pointer_sub_obj.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
        pointer
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            root: Rc::new(RefCell::new(FSObject::create_directory("root"))),
            head: Vec::new(),
        }
    }
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let root_ref = self.root.borrow();
        let root_data_dir = if let FSObject::Directory(data) = root_ref.deref() {
            data
        } else {
            unreachable!()
        };
        root_data_dir.fmt(f, 0)
    }
}

impl DirectoryData {
    fn fmt(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        let indent = "  ".repeat(depth);
        write!(f, "{}- {}: # (dir)\n", indent, self.name)?;
        // let file_indent = "  ".repeat(depth+1);
        for item in &self.content {
            let item_ref = item.borrow();
            match item_ref.deref() {
                FSObject::Directory(data) => data.fmt(f, depth + 1),
                FSObject::File(data) => data.fmt(f, depth + 1),
            }?;
        }
        Ok(())
    }
}

impl FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        let indent = "  ".repeat(depth);
        write!(
            f,
            "{}- {}: # (file, size={})\n",
            indent, self.name, self.size
        )
    }
}

fn create_filesystem<S: Iterator<Item = String>>(source: S) -> FileSystem {
    let mut fs = FileSystem::default();
    for line in source {
        if line.starts_with("$") {
            // a command
            if line.starts_with("$ c") {
                // cd
                // change head to a specified directory
                let dir_name = line.strip_prefix("$ cd ").unwrap();
                fs.cd(dir_name).unwrap();
            }
        } else {
            let head = fs.get_head();
            // a listing result
            if let Some(dir_name) = line.strip_prefix("dir ") {
                // a dir
                let dir = FSObject::create_directory(dir_name);
                head.borrow_mut().add_obj(dir).unwrap();
            } else {
                // a file
                let mut spliterator = line.split(' ');
                let size = spliterator.next().unwrap().parse::<usize>().unwrap();
                let file_name = spliterator.next().unwrap();
                let file_obj = FSObject::create_file(file_name, size);
                head.borrow_mut().add_obj(file_obj).unwrap();
            }
        }
    }
    fs
}

fn explore_dir(data: &DirectoryData) -> (Vec<(String, usize)>, usize) {
    let mut large_dirs = Vec::new();
    let mut this_dir_size = 0;
    for item in &data.content {
        let item_ref: Ref<FSObject> = item.borrow();
        match item_ref.deref() {
            FSObject::Directory(data) => {
                let (mut results, dir_size) = explore_dir(data);
                this_dir_size += dir_size;
                large_dirs.append(&mut results);
            }
            FSObject::File(data) => {
                // println!("accounting for {}", data.name);
                this_dir_size += data.size;
            }
        }
    }
    large_dirs.push((data.name.clone(), this_dir_size));
    (large_dirs, this_dir_size)
}

fn main() {
    let lines = io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0);
    let fs = create_filesystem(lines);
    println!("fs:\n{fs}");
    let root_ref = fs.root.borrow();
    let root_data_dir = if let FSObject::Directory(data) = root_ref.deref() {
        data
    } else {
        unreachable!()
    };
    let (mut sized_dirs, total_size) = explore_dir(root_data_dir);
    println!("total_size = {}", total_size);
    sized_dirs.sort_by_key(|k| k.1);
    // println!("{:?}", sized_dirs);
    // println!("biggest is: {:?}", small_dirs.last().unwrap());

    let sum: usize = sized_dirs
        .iter()
        .filter_map(|d| if d.1 <= 100_000 { Some(d.1) } else { None })
        .sum();
    println!("sum: {:?}", sum);
}
