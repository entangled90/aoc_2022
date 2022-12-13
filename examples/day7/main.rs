use std::{cmp::min, collections::VecDeque, fs::File, io::Write, path::Path};

use aoc_2022::files::open_file;
use regex::Regex;

#[derive(Debug)]
pub enum FsNodeOutput {
    Dir(String),
    File(usize, String),
}
#[derive(Debug)]
pub enum Session {
    Cd(String),
    LsResult(Vec<FsNodeOutput>),
}

#[derive(Debug, Clone)]
pub enum FsNode {
    Folder { name: String, children: Vec<FsNode> },
    File { size: usize, name: String },
}

impl FsNode {
    pub fn name(&self) -> &str {
        match self {
            FsNode::File { size: _, name } => name,
            FsNode::Folder { name, children: _ } => name,
        }
    }

    pub fn pretty_print(&self, indentation: usize) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        let mut s = FsNode::empty_indentend_string(indentation);
        match self {
            FsNode::Folder { name, children } => {
                s.push_str(name);
                vec.push(s);
                for c in children {
                    let mut printed = c.pretty_print(indentation + 2);
                    vec.append(&mut printed);
                }
            }
            FsNode::File { size, name } => {
                s.push_str(&format!("{} {}", size, name));
                vec.push(s)
            }
        }
        vec
    }

    fn empty_indentend_string(indentation: usize) -> String {
        let mut s = String::new();
        for _ in 0..indentation {
            s.push(' ');
        }
        s
    }
}

#[derive(Debug, Default)]
pub struct SmallFolders {
    small_folders: Vec<(String, usize)>,
}

impl SmallFolders {
    fn get_folder_size(&mut self, root: &FsNode, max_size: usize) -> usize {
        match root {
            FsNode::Folder { name, children } => {
                let children_size: Vec<_> = children
                    .iter()
                    .map(|c| (c.name(), self.get_folder_size(c, max_size)))
                    .collect();

                let size: usize = children_size.iter().map(|t| t.1).sum();
                if size < max_size {
                    self.small_folders.push((name.to_string(), size));
                }
                size
            }
            FsNode::File { size, name: _ } => *size,
        }
    }
}

pub fn main() {
    let lines = open_file("examples/day7/input.txt").expect("Failed to open file");
    let sessions = parse(&lines);
    let mut interpreter = Interpreter {
        root: FsNode::Folder {
            name: "/".to_string(),
            children: Vec::new(),
        },
        cwd: Vec::default(),
    };
    println!("Sessions are {:?}", sessions);

    for s in sessions {
        interpreter.interpret(&s);
    }

    let mut out_file = File::create("examples/day7/fs.txt").expect("Failed to create output");
    for l in interpreter.root.pretty_print(0) {
        write!(out_file, "{}\n", l).expect("Failed to write to file");
    }

    let mut folders = SmallFolders::default();

    folders.get_folder_size(&interpreter.root, 100000);
    println!("Folders {:?}", folders);

    println!(
        "Result {:?}",
        folders.small_folders.iter().map(|t| t.1).sum::<usize>()
    );

    solution_two(&interpreter.root);
}

fn solution_two(root: &FsNode) -> usize {
    let mut folders = SmallFolders::default();
    let file_system_size = 70000000;
    let used_size = folders.get_folder_size(root, file_system_size);
    let available = file_system_size - used_size;
    let required_space = 30000000;
    let space_to_free = required_space - available;

    let smallest = folders
        .small_folders
        .iter()
        .filter(|(_, size)| size > &space_to_free)
        .min_by(|a, b| a.1.cmp(&b.1));
    println!("Smallest folder is {:?}", smallest);

    smallest.unwrap().1
}

#[derive(Debug)]
pub struct Interpreter {
    root: FsNode,
    cwd: Vec<String>,
}

impl Interpreter {
    pub fn interpret(&mut self, session: &Session) {
        match session {
            Session::Cd(path) if path == "/" => {
                self.cwd = vec!["/".to_string()];
            }
            Session::Cd(path) if path == ".." => {
                self.cwd.pop();
            }
            Session::Cd(path) => self.cwd.push(path.into()),
            Session::LsResult(new_children) => {
                let folder = navigate(&mut self.root, &self.cwd, &[]);
                println!(
                    "Inserting ls result ({:?})(in cwd {:?}) in folder {:?}",
                    new_children,
                    self.cwd,
                    folder.pretty_print(0)
                );
                for c in new_children {
                    let node = match c {
                        FsNodeOutput::Dir(name) => FsNode::Folder {
                            name: name.to_string(),
                            children: Vec::new(),
                        },
                        FsNodeOutput::File(size, name) => FsNode::File {
                            size: *size,
                            name: name.to_string(),
                        },
                    };
                    match folder {
                        FsNode::Folder { name: _, children } => children.push(node),
                        _ => panic!("Invalid!"),
                    }
                }
            }
        }
    }
}

fn navigate<'a>(root: &'a mut FsNode, cwd: &[String], parent_folder: &[String]) -> &'a mut FsNode {
    let cloned = root.clone();
    match cwd.split_first() {
        Some((dir, tail)) => {
            if dir == "/" {
                navigate(root, tail, &[])
            } else if dir == ".." {
                navigate(
                    root,
                    cwd,
                    parent_folder
                        .split_last()
                        .map(|(_, last)| last)
                        .get_or_insert(&[]),
                )
            } else {
                if let FsNode::Folder { name: _, children } = root {
                    let child = children.iter_mut().find(|c| {
                        if let FsNode::Folder { name, children: _ } = c {
                            name == dir
                        } else {
                            false
                        }
                    });

                    match child {
                        Some(child) => navigate(child, tail, cwd),
                        None => panic!(
                            "Cannot find child with dir name {:?} in node {:?} ",
                            dir, cloned
                        ),
                    }
                } else {
                    panic!("Cannot cd into a file!");
                }
            }
        }
        None => root,
    }
}

fn parse(lines: &[String]) -> Vec<Session> {
    let cd = Regex::new(r"^\$\scd\s([\w/\.]+)$").unwrap();
    let ls = Regex::new(r"^\$\sls").unwrap();
    let dir = Regex::new(r"^dir\s([\w]+)").unwrap();
    let file = Regex::new(r"^(\d+)\s([\w\.]*)").unwrap();
    let mut res = Vec::new();
    let mut parsing_output = false;
    let mut output = Vec::new();
    for line in lines {
        if cd.is_match(&line) {
            if parsing_output {
                res.push(Session::LsResult(output));
                output = Vec::new();
            }
            parsing_output = false;
            let m = cd.captures(&line).unwrap().get(1).unwrap();
            res.push(Session::Cd(m.as_str().to_string()));
        } else if ls.is_match(&line) {
            if parsing_output {
                res.push(Session::LsResult(output));
                output = Vec::new();
            }
            parsing_output = true;
        } else if parsing_output {
            if let Some(capt) = dir.captures(line) {
                output.push(FsNodeOutput::Dir(capt.get(1).unwrap().as_str().to_string()));
            } else if let Some(capt) = file.captures(line) {
                let size = capt.get(1).unwrap().as_str().parse().unwrap();
                let name = capt.get(2).unwrap().as_str().to_string();
                output.push(FsNodeOutput::File(size, name));
            }
        }
    }
    if parsing_output {
        res.push(Session::LsResult(output));
    }
    res
}
