use std::{io::{self, BufRead}, fs, rc::{Weak, Rc}, borrow::BorrowMut};

type Size = u32;

#[derive(Debug)]
enum Inode<'a> {
    File(File),
    Directory(Directory<'a>)
}

#[derive(Debug)]
struct File(Size);
#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    parent: &'a Option<Weak<Directory<'a>>>,
    children: Vec<Rc<Inode<'a>>>,
}

fn parse_line(line: &String) {
    let mut tokens = line.split(" ");
    match tokens.next() {
        Some("$") => {}
        Some("dir") => {}
        Some(size) => {}
        None => panic!("TODO")
    }
}

fn part_one() {
    let file_path = "data.txt";
    let file = fs::File::open(file_path).expect("File not found");
    let _lines = io::BufReader::new(file).lines();
}

fn main() {
    part_one();
    // part_two();

    let _d1 = Directory {
        name: "root",
        parent: &None,
        children: vec![
            Rc::new(Inode::File(File(3)))
        ]
    };

    let mut _d2 = Directory {
        name: "subdir",
        parent: &Some(Weak::new()),
        children: vec![
            Rc::new(Inode::File(File(3)))
        ]
    };

    *_d2.parent.borrow_mut() = &Some(Rc::downgrade(&Rc::new(_d1)));

    println!("_d1: {:?}", &_d1);
}
