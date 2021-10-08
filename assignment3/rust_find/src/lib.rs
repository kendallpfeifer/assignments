use clap::ArgMatches;
use regex::Regex;
use std::fs::File;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

pub struct MyFile {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

impl MyFile {
    /// Instantiate a MyFile struct from the path of a file.
    pub fn from_path(path: &Path) -> Result<Self, &'static str> {
        let mut pb = false;
        let mut size = 0;
        match path.metadata() {
            Ok(_x) => pb = true,
            Err(_x) => print!("Invalid file"),
        };
        if pb {
            match path.metadata() {
                Ok(x) => size = x.len(),
                Err(_x) => print!("Invalid path"),
            };
        } else {
            return Err("Invalid path");
        }

        let file = MyFile {
            name: path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
            path: pb.to_string(),
            size_bytes: size,
        };

        Ok(file)
    }
}

pub fn get_matched_files(files: &mut Vec<MyFile>, dir: &Path, pats: &[Regex], size: Option<u64>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        // To get path from read_dir() result
        let path = entry.unwrap().path();

        if path.is_dir() {
            get_matched_files(files, &path, pats, size)
        } else if path.is_file() {
            let file = MyFile::from_path(&path);
            let filename = &file.as_ref().unwrap().name;
            let mut ismatch = true;
            for reg in pats {
                if !reg.is_match(filename) {
                    ismatch = false;
                    break;
                }
            }
            if ismatch {
                files.push(file.unwrap());
            }
        }
    }
}

pub fn display(files: &[MyFile], output: &mut Option<File>) -> Option<Vec<String>> {
    if output.as_ref().is_none() {
        let mut vec = Vec::<String>::new();
        for file in files.iter() {
            vec.push(file.name.clone());
            //writeln!(vec,  "{}", file.name);
        }
        Some(vec)
    } else {
        let mut filetowrite = output.as_ref().unwrap();
        for file in files {
            writeln!(filetowrite, "{}", file.name).ok()?;
        }
        None
    }
}

pub fn run(config: &Config) -> Result<(), &'static str> {
    // 1. parse patterns
    let v_pats: Vec<Regex> = config.parse_patterns()?;

    // 2. get directories
    let v_dirs: Vec<PathBuf> = config.parse_dirs()?;

    // 3. parse optional arguments
    let mut output: Option<File> = config.parse_output();

    let size: Option<u64> = config.parse_size();

    // 4. get files and output
    let mut matched_files = Vec::with_capacity(v_dirs.len());
    for dir in v_dirs.iter() {
        get_matched_files(&mut matched_files, dir, &v_pats[..], size);

        // print or write
        if let Some(sv) = display(&matched_files, &mut output) {
            for s in sv {
                println!("{}", s);
            }
        };
        matched_files.clear();
    }

    Ok(())
}

pub struct Config<'a> {
    pub dirs: Vec<&'a str>,
    pub patterns: Vec<&'a str>,
    pub output: Option<&'a str>,
    pub size: Option<&'a str>,
}
impl<'a> Config<'a> {
    // you need to use explit lifetime here as well
    pub fn from_args(args: &'a ArgMatches) -> Self {
        let mut newdirs = Vec::new();
        match args.value_of("dirs") {
            None => newdirs = Vec::new(),
            Some(x) => newdirs.push(x),
        };
        let mut pat = Vec::new();
        match args.value_of("patterns") {
            None => pat = Vec::new(),
            Some(x) => pat.push(x),
        };

        let config = Config {
            dirs: newdirs,
            patterns: pat,
            output: args.value_of("output"),
            size: args.value_of("size"),
        };

        config
    }
}

impl<'a> Config<'a> {
    pub fn parse_patterns(&self) -> Result<Vec<Regex>, &'static str> {
        let mut v = Vec::new();
        if self.patterns.is_empty() {
            return Err("No patterns provided");
        }

        for p in self.patterns.iter() {
            let reg = Regex::new(p).unwrap();
            //match reg to check for validity
            //if patterns return error, return erro
            v.push(reg);
        }

        Ok(v)
    }

    pub fn parse_dirs(&self) -> Result<Vec<PathBuf>, &'static str> {
        let mut v = Vec::new();
        if self.dirs.is_empty() {
            return Err("No directories provided");
        }

        for d in self.dirs.iter() {
            let pb = PathBuf::from(d);
            //can you access the path? with metadata function
            match pb.metadata() {
                Ok(_x) => v.push(pb),
                Err(_x) => print!("Invalid directory"),
            };
        }

        Ok(v)
    }

    pub fn parse_output(&self) -> Option<File> {
        self.output.map(|x| File::create(x).unwrap())
    }

    pub fn parse_size(&self) -> Option<u64> {
        self.output.map(|x| x.parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
