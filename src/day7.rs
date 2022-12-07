use std::{collections::HashMap, path::PathBuf};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{map, rest},
    Finish, IResult,
};

enum Token {
    ChangeDirectory { target: String },
    List,
    OutputFile { name: String, size: u32 },
    OutputDirectory { name: String },
}

enum Entry {
    Directory { children: Vec<String> },
    File { size: u32 },
}

fn parse_cd_command(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("cd")(input)?;
    let (input, _) = space1(input)?;
    map(rest, |target: &str| Token::ChangeDirectory {
        target: target.to_owned(),
    })(input)
}

fn parse_ls_command(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("ls")(input)?;
    Ok((input, Token::List))
}

fn parse_command(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("$")(input)?;
    let (input, _) = space1(input)?;
    alt((parse_cd_command, parse_ls_command))(input)
}

fn parse_output_directory(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("dir")(input)?;
    let (input, _) = space1(input)?;
    map(rest, |name: &str| Token::OutputDirectory {
        name: name.to_owned(),
    })(input)
}

fn parse_output_file(input: &str) -> IResult<&str, Token> {
    let (input, size) = nom::character::complete::u32(input)?;
    let (input, _) = space1(input)?;
    let (input, token) = map(rest, |name: &str| Token::OutputFile {
        name: name.to_owned(),
        size,
    })(input)?;

    Ok((input, token))
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((parse_command, parse_output_directory, parse_output_file))(input)
}

type FileSystem = HashMap<PathBuf, Entry>;

struct WalkState {
    current_directory: PathBuf,
    filesystem: FileSystem,
}

impl Default for WalkState {
    fn default() -> Self {
        let current_directory = PathBuf::from("/");
        let mut filesystem = FileSystem::new();
        filesystem.insert(
            current_directory.clone(),
            Entry::Directory { children: vec![] },
        );

        WalkState {
            current_directory,
            filesystem,
        }
    }
}

fn tokenize(input: &str) -> eyre::Result<Vec<Token>> {
    input
        .lines()
        .map(|line| match parse_token(line).finish() {
            Ok((_, token)) => Ok(token),
            Err(e) => bail!("error tokenizing {:?}: {}", line, e),
        })
        .collect::<Result<Vec<Token>, _>>()
}

fn build_filesystem(tokens: &[Token]) -> eyre::Result<FileSystem> {
    let state = tokens
        .iter()
        .try_fold(WalkState::default(), |mut walk, token| {
            match token {
                Token::ChangeDirectory { target } => {
                    if target == ".." {
                        walk.current_directory.pop();
                    } else {
                        walk.current_directory.push(target);
                    }
                }
                Token::List => {}
                Token::OutputFile { name, size } => {
                    let path = walk.current_directory.join(name);
                    walk.filesystem.insert(path, Entry::File { size: *size });
                    walk.filesystem
                        .entry(walk.current_directory.clone())
                        .and_modify(|dir| {
                            let Entry::Directory { children, .. } = dir else {
                                panic!("trying to add a file to a non-directory")
                            };

                            children.push(name.clone());
                        });
                }
                Token::OutputDirectory { name } => {
                    let path = walk.current_directory.join(name);
                    walk.filesystem
                        .entry(path)
                        .or_insert_with(|| Entry::Directory { children: vec![] });
                    walk.filesystem
                        .entry(walk.current_directory.clone())
                        .and_modify(|dir| {
                            let Entry::Directory { children, .. } = dir else {
                                panic!("trying to add a directory to a non-directory")
                            };

                            children.push(name.clone());
                        });
                }
            }

            Ok::<_, eyre::Report>(walk)
        })?;

    Ok(state.filesystem)
}

struct Search {
    parent: PathBuf,
    current: PathBuf,
}

fn find_sizes(filesystem: &FileSystem) -> eyre::Result<HashMap<PathBuf, usize>> {
    let mut sizes = HashMap::<PathBuf, usize>::new();
    let mut walk = filesystem
        .iter()
        .filter_map(|(path, entry)| match entry {
            Entry::Directory { .. } => Some(Search {
                parent: path.clone(),
                current: path.clone(),
            }),
            Entry::File { .. } => None,
        })
        .collect::<Vec<Search>>();

    while let Some(search) = walk.pop() {
        match &filesystem[&search.current] {
            Entry::Directory { children, .. } => {
                for child in children {
                    let path = search.current.join(child);
                    walk.push(Search {
                        parent: search.parent.clone(),
                        current: path,
                    });
                }
            }
            Entry::File { size, .. } => {
                *sizes.entry(search.parent).or_default() += *size as usize;
            }
        }
    }

    Ok(sizes)
}

#[aoc_generator(day7)]
fn generator(input: &str) -> eyre::Result<HashMap<PathBuf, usize>> {
    let tokens = tokenize(input)?;
    let filesystem = build_filesystem(&tokens)?;
    find_sizes(&filesystem)
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<PathBuf, usize>) -> usize {
    input.values().copied().filter(|&size| size <= 100000).sum()
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<PathBuf, usize>) -> Option<usize> {
    let root = PathBuf::from("/");
    let used = input[&root];
    let remaining = 70000000 - used;
    let needed = 30000000 - remaining;

    input.values().copied().filter(|&size| size >= needed).min()
}
