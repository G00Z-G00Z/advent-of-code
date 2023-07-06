use std::{cell::RefCell, rc::Rc};

pub type NodeHanlder<T> = Rc<RefCell<T>>;

/// File representation
#[derive(Debug)]
pub struct File {
    pub size: u64,
    pub name: String,
}

/// Folder representation
#[derive(Debug)]
pub struct Folder {
    name: String,
    children: Vec<NodeHanlder<Folder>>,
    files: Vec<File>,
    parent: Option<NodeHanlder<Folder>>,
}

impl Folder {
    pub fn new_rc(name: String, parent: Option<NodeHanlder<Folder>>) -> NodeHanlder<Folder> {
        Rc::new(RefCell::new(Folder {
            name,
            children: vec![],
            files: vec![],
            parent,
        }))
    }

    pub fn size(&self) -> u64 {
        let mut size = 0;
        for child in &self.children {
            let folder = child.borrow();
            size += folder.size();
        }
        for file in &self.files {
            size += file.size;
        }
        size
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn add_folder(&mut self, folder: NodeHanlder<Folder>) {
        self.children.push(Rc::clone(&folder));
    }
}

/// Representation of a command
#[derive(Debug)]
enum Command {
    Ls,
    CdBack,
    Cd(String),
}

/// Command representation
#[derive(Debug)]
enum ParseResult {
    Command(Command),
    File(File),
    Folder(String),
}

/// File system representation
#[derive(Debug)]
pub struct FileSystem {
    pub root: NodeHanlder<Folder>,
}

pub fn find_sum_at_most(size: u64, folder: &NodeHanlder<Folder>) -> u64 {
    let mut sum = 0;

    // Check size of current folder

    let f_size = folder.borrow().size();

    if f_size <= size {
        sum += f_size;
    }

    // Check for repeated folders
    for child in &folder.borrow().children {
        sum += find_sum_at_most(size, child);
    }
    sum
}

/// Finds the directories that can be deleted to free up the desired size
/// Returns the smallest size that can be freed (if any)
pub fn find_smallest_to_free(min_required_size: u64, folder: &NodeHanlder<Folder>) -> Option<u64> {
    let size = folder.borrow().size();

    // If folder size is less than the required size, return None
    if size < min_required_size {
        return None;
    }

    // Set default min_size
    let min_size = size;

    let min_child = folder
        .borrow()
        .children
        .iter()
        .filter_map(|c| find_smallest_to_free(min_required_size, c))
        .min();

    if min_child.is_some() {
        return min_child;
    }
    return Some(min_size);
}

pub fn create_file_system_from_cmd<'a>(input: &str) -> FileSystem {
    let root: NodeHanlder<Folder> = Folder::new_rc("/".into(), None);

    let mut current_folder = Rc::clone(&root);

    let input = parse_input(&input);

    for entry in input {
        match entry {
            ParseResult::Command(c) => match c {
                Command::Ls => {}
                Command::CdBack => {
                    let parent = Rc::clone(
                        current_folder
                            .borrow()
                            .parent
                            .as_ref()
                            .expect("Cannot go back"),
                    );

                    current_folder = parent;
                }
                Command::Cd(n) => {
                    let mut tmp = None;

                    for child in &current_folder.borrow().children {
                        if child.borrow().name == n {
                            tmp = Some(Rc::clone(child));
                            break;
                        }
                    }

                    if let Some(folder) = tmp {
                        current_folder = folder;
                    }
                }
            },
            ParseResult::File(f) => {
                let mut folder = current_folder.borrow_mut();
                folder.add_file(f);
            }
            ParseResult::Folder(f) => {
                let folder = Folder::new_rc(f, Some(Rc::clone(&current_folder)));
                // Aqui es donde muere
                current_folder.borrow_mut().add_folder(folder);
            }
        }
    }

    FileSystem { root }
}
fn parse_input(input: &str) -> Vec<ParseResult> {
    let mut results = vec![];

    for line in input.lines().skip(1) {
        if line.starts_with("$") {
            let mut splits = line.split(" ").skip(1);
            let command_str = splits.next().unwrap();

            match command_str {
                "ls" => {
                    results.push(ParseResult::Command(Command::Ls));
                }
                "cd" => {
                    let path = splits.next().unwrap();

                    if path == ".." {
                        results.push(ParseResult::Command(Command::CdBack));
                        continue;
                    }

                    results.push(ParseResult::Command(Command::Cd(String::from(path))));
                }
                _ => {
                    panic!("Unknown command");
                }
            }
            continue;
        }

        let mut splits = line.split(" ");

        match splits.next().unwrap() {
            "dir" => {
                let name = splits.next().unwrap().to_string();
                results.push(ParseResult::Folder(name));
            }
            number => {
                let name = splits.next().unwrap().to_string();
                let size = number
                    .parse::<u64>()
                    .expect(&format!("Cannot parse number {}", number));

                results.push(ParseResult::File(File { name, size }));
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use utility_2022::{get_input, is_demo_mode};

    use super::*;

    // Expected filesystem
    // - / (dir)
    //   - a (dir)
    //     - e (dir)
    //       - i (file, size=584)
    //     - f (file, size=29116)
    //     - g (file, size=2557)
    //     - h.lst (file, size=62596)
    //   - b.txt (file, size=14848514)
    //   - c.dat (file, size=8504156)
    //   - d (dir)
    //     - j (file, size=4060174)
    //     - d.log (file, size=8033020)
    //     - d.ext (file, size=5626152)
    //     - k (file, size=7214296)

    pub mod part1 {
        use super::*;

        #[test]
        fn test_create_demo_filesystem() {
            if !is_demo_mode() {
                return;
            }

            let expected_size = 48381165;

            let demo_input_file_system: String = get_input();

            let filesystem = create_file_system_from_cmd(&demo_input_file_system);

            assert_eq!(
                filesystem.root.borrow().size(),
                expected_size,
                "Size calculation do not match"
            );

            let sum = find_sum_at_most(100000, &filesystem.root);

            assert_eq!(sum, 95437, "Sum calculation do not match");
        }

        #[test]
        fn test_create_() {
            if is_demo_mode() {
                return;
            }

            let input_file_system: String = get_input();

            let filesystem = create_file_system_from_cmd(&input_file_system);

            let sum = find_sum_at_most(100000, &filesystem.root);

            println!("For part1: {}", sum)
        }
    }

    pub mod part2 {
        use super::*;

        const TOTAL_SPACE_DRIVE: u64 = 70_000_000;
        const MIN_FREE_SPACE: u64 = 30_000_000;

        #[test]
        fn test_create_sum_demo() {
            if !is_demo_mode() {
                return;
            }

            let input_file_system: String = get_input();

            let filesystem = create_file_system_from_cmd(&input_file_system);
            let total_mem = filesystem.root.borrow().size();
            let free_total = TOTAL_SPACE_DRIVE - total_mem;
            let need_to_delete = MIN_FREE_SPACE - free_total;

            let smallest_space = find_smallest_to_free(need_to_delete, &filesystem.root).unwrap();

            assert_eq!(
                smallest_space, 24933642,
                "Smallest space calculation do not match"
            );
        }

        #[test]
        fn test_part_2() {
            if is_demo_mode() {
                return;
            }

            let input_file_system: String = get_input();

            let filesystem = create_file_system_from_cmd(&input_file_system);
            let total_mem = filesystem.root.borrow().size();
            let free_total = TOTAL_SPACE_DRIVE - total_mem;
            let need_to_delete = MIN_FREE_SPACE - free_total;

            let smallest_space = find_smallest_to_free(need_to_delete, &filesystem.root).unwrap();

            println!("For part2: {}", smallest_space)
        }
    }
}
