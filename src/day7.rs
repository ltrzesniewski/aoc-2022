use crate::common::get_input_lines;

struct FileSystem {
    entries: Vec<Entry>,
}

enum Entry {
    File(File),
    Dir(Dir),
}

#[derive(Copy, Clone)]
struct FileHandle(usize);

#[derive(Copy, Clone)]
struct DirHandle(usize);

struct EntryId(usize, String);

struct File {
    _id: EntryId,
    size: usize,
}

struct Dir {
    id: EntryId,
    contents: Vec<usize>,
}

#[allow(dead_code)]
pub fn run() {
    let input = parse(get_input_lines().iter());

    let result = part1(&input);
    println!("Result (part 1): {result}");

    let result = part2(&input);
    println!("Result (part 2): {result}");
}

fn part1(fs: &FileSystem) -> usize {
    let mut result = 0;

    fs.traverse_get_size(|size| {
        if size <= 100000 {
            result += size;
        }
    });

    result
}

fn part2(fs: &FileSystem) -> usize {
    let used_size = fs.traverse_get_size(|_| {});
    let unused_size = 70000000 - used_size;
    let required_size = 30000000 - unused_size;

    let mut result = usize::MAX;

    fs.traverse_get_size(|size| {
        if size >= required_size && size < result {
            result = size;
        }
    });

    result
}

fn parse<T: AsRef<str>>(script: impl Iterator<Item = T>) -> FileSystem {
    let mut fs = FileSystem::new();
    let mut dirs = vec![fs.get_root()];

    macro_rules! cd {
        () => {
            dirs.last().copied().unwrap()
        };
    }

    for line in script {
        let line = line.as_ref().split_whitespace().collect::<Vec<_>>();

        match line[..] {
            ["$", "cd", "/"] => {
                dirs.clear();
                dirs.push(fs.get_root());
            }
            ["$", "cd", ".."] => {
                dirs.pop();
            }
            ["$", "cd", name] => {
                dirs.push(fs.get_subdir(cd!(), name));
            }
            ["$", "ls"] => {}
            ["dir", name] => {
                fs.add_dir(cd!(), name);
            }
            [size, name] => {
                fs.add_file(cd!(), name, size.parse().unwrap());
            }
            _ => panic!("Invalid input"),
        }
    }

    fs
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Dir::new(EntryId(0, "/".to_string()));
        FileSystem {
            entries: vec![Entry::Dir(root)],
        }
    }

    fn get_root(&self) -> DirHandle {
        DirHandle(0)
    }

    fn add_file(&mut self, dir: DirHandle, name: &str, size: usize) -> FileHandle {
        let id = self.entry_id(name);
        let handle = FileHandle(id.0);
        self.add_to_dir(dir, &id);
        let file = File::new(id, size);
        self.entries.push(Entry::File(file));
        handle
    }

    fn add_dir(&mut self, dir: DirHandle, name: &str) -> DirHandle {
        let id = self.entry_id(name);
        let handle = DirHandle(id.0);
        self.add_to_dir(dir, &id);
        let dir = Dir::new(id);
        self.entries.push(Entry::Dir(dir));
        handle
    }

    fn entry_id(&self, name: &str) -> EntryId {
        EntryId(self.entries.len(), name.to_string())
    }

    fn add_to_dir(&mut self, dir: DirHandle, id: &EntryId) {
        self.get_dir_mut(dir).contents.push(id.0);
    }

    fn get_subdir(&self, dir: DirHandle, name: &str) -> DirHandle {
        for entry in self.iter_dir(dir) {
            if let Entry::Dir(dir) = entry {
                if dir.id.1 == name {
                    return DirHandle(dir.id.0);
                }
            }
        }

        panic!("Dir not found")
    }

    fn get_dir(&self, dir: DirHandle) -> &Dir {
        if let Some(Entry::Dir(dir)) = self.entries.get(dir.0) {
            dir
        } else {
            panic!("Not a dir")
        }
    }

    fn get_dir_mut(&mut self, dir: DirHandle) -> &mut Dir {
        if let Some(Entry::Dir(dir)) = self.entries.get_mut(dir.0) {
            dir
        } else {
            panic!("Not a dir")
        }
    }

    fn iter_dir(&self, dir: DirHandle) -> impl Iterator<Item = &Entry> {
        self.get_dir(dir)
            .contents
            .iter()
            .copied()
            .map(|i| self.entries.get(i).unwrap())
    }

    fn traverse_get_size<DirCallback: FnMut(usize)>(&self, mut callback: DirCallback) -> usize {
        fn exec<DirCallback: FnMut(usize)>(
            fs: &FileSystem,
            dir: DirHandle,
            callback: &mut DirCallback,
        ) -> usize {
            let mut size = 0;

            for entry in fs.iter_dir(dir) {
                match entry {
                    Entry::File(file) => {
                        size += file.size;
                    }
                    Entry::Dir(dir) => {
                        size += exec(fs, dir.handle(), callback);
                    }
                }
            }

            callback(size);
            size
        }

        exec(self, self.get_root(), &mut callback)
    }
}

impl File {
    fn new(id: EntryId, size: usize) -> File {
        File { _id: id, size }
    }
}

impl Dir {
    fn new(id: EntryId) -> Dir {
        Dir {
            id,
            contents: vec![],
        }
    }

    fn handle(&self) -> DirHandle {
        DirHandle(self.id.0)
    }
}
