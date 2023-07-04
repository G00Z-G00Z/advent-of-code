/// File representation
#[derive(Debug)]
struct File {
    pub size: u64,
}

/// Folder representation
#[derive(Debug)]
struct Folder {
    name: String,
    children: Vec<Folder>,
    files: Vec<File>,
}

impl Folder {
    fn size(&self) -> u64 {
        let mut size = 0;
        for child in &self.children {
            size += child.size();
        }
        for file in &self.files {
            size += file.size;
        }
        size
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_folder(&mut self, folder: Folder) {
        self.children.push(folder);
    }
}

/// File system representation
#[derive(Debug)]
struct FileSystem {
    root: Folder,
}

fn create_file_system_from_cmd(input: &str) -> FileSystem {
    let mut root = Folder {
        name: String::from("/"),
        children: vec![],
        files: vec![],
    };

    let mut current_folder: &mut Folder;

    current_folder = &mut root;

    for line in input.lines() {
        println!("line: {}", line);
    }

    FileSystem { root }
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

    #[test]
    fn test_create_demo_filesystem() {
        if !is_demo_mode() {
            return;
        }

        let expected_filesystem = FileSystem {
            root: Folder {
                name: String::from("/"),
                children: vec![
                    Folder {
                        name: String::from("a"),
                        children: vec![Folder {
                            name: String::from("e"),
                            children: vec![],
                            files: vec![File { size: 584 }],
                        }],
                        files: vec![
                            File { size: 29116 },
                            File { size: 2557 },
                            File { size: 62596 },
                        ],
                    },
                    Folder {
                        name: String::from("d"),
                        children: vec![],
                        files: vec![
                            File { size: 4060174 },
                            File { size: 8033020 },
                            File { size: 5626152 },
                            File { size: 7214296 },
                        ],
                    },
                ],
                files: vec![File { size: 14848514 }, File { size: 8504156 }],
            },
        };

        let demo_input_file_system: String = get_input();

        println!("{}", demo_input_file_system)
    }
}
