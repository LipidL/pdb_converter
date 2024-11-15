// necessary imports
use std::io::Write;
use regex::Regex;

enum AtomType{
    Atom,
    Other,
}
struct Coordinate{
    x: f64,
    y: f64,
    z: f64,
}
pub struct Atom{
    atom_type: AtomType,
    number: usize,
    name: String,
    kind: String,
    aa_name: String,
    aa_number: usize,
    coordinate: Coordinate,
    second_to_last: usize,
    last: usize,
}

pub struct AtomParser{
    re: Regex,
}
impl AtomParser{
    pub fn new() -> Self{
        let regex = Regex::new(r"^(?P<type>\w+)\s+(?P<num>\d+)\s+(?P<name>\w+)\s+(?P<aa_name>\w+)\s+(?P<aa_num>\d+)\s+(?P<x>-?\d+\.\d+)\s+(?P<y>-?\d+\.\d+)\s+(?P<z>-?\d+\.\d+)\s+(?P<second_to_last>\d+)\s+(?P<last>\d+)").unwrap();
        Self { re: regex }        
    }
    pub fn parse(&self, input:&str) -> Option<Atom>{
        if let Some(caps) = self.re.captures(input){
            let atom_type = if caps.name("type").unwrap().as_str() == "ATOM" {
                AtomType::Atom
            } else {
                AtomType::Other
            };
            let number = caps.name("num").unwrap().as_str().parse().unwrap();
            let name = caps.name("name").unwrap().as_str().to_string();
            let kind = name[0..1].to_string();
            let aa_name = caps.name("aa_name").unwrap().as_str().to_string();
            let aa_number = caps.name("aa_num").unwrap().as_str().parse().unwrap();
            let x = caps.name("x").unwrap().as_str().parse().unwrap();
            let y = caps.name("y").unwrap().as_str().parse().unwrap();
            let z = caps.name("z").unwrap().as_str().parse().unwrap();
            let second_to_last = caps.name("second_to_last").unwrap().as_str().parse().unwrap();
            let last = caps.name("last").unwrap().as_str().parse().unwrap();
            return Some(Atom{
                atom_type,
                number,
                name,
                kind,
                aa_name,
                aa_number,
                coordinate: Coordinate{x, y, z},
                second_to_last,
                last,
            });
        }
        None
    }
}

pub fn print_to_file(atoms: Vec<Atom>, filename: &str){
    let mut file = std::fs::File::create(filename).unwrap();
    for atom in atoms{
        // generate the format
        // if atom.name.len() < 4, format is {: <5} {:>6} (2 spaces) {:<3} {:>4} (1 space)A {:>4} {:>12.3} {:>8.3} {:>8.3} {:>6.2} {:>6.2} {:>12}
        // if atom.name.len() == 4, format is {: <5} {:>6} (1 space) {:<5} {:>3} (1 space)A {:>4} {:>12.3} {:>8.3} {:>8.3} {:>6.2} {:>6.2} {:>12}
        // atom.name.len() > 4 will never happen
        assert!(atom.name.len() < 5);
        let line = if atom.name.len() < 4 {
            format!("{:<5}{:>6}  {:<3}{:>4} A{:>4}{:>12.3}{:>8.3}{:>8.3}{:>6.2}{:>6.2}{:>12}\n", 
                match atom.atom_type{
                    AtomType::Atom => "ATOM",
                    AtomType::Other => "OTHR",
                },
                atom.number,
                atom.name,
                atom.aa_name,
                atom.aa_number,
                atom.coordinate.x,
                atom.coordinate.y,
                atom.coordinate.z,
                atom.second_to_last as f64,
                atom.last as f64,
                atom.kind
        )} else {
            format!("{:<5}{:>6}  {:<5}{:>3} A{:>4}{:>12.3}{:>8.3}{:>8.3}{:>6.2}{:>6.2}{:>12}\n", 
                match atom.atom_type{
                    AtomType::Atom => "ATOM",
                    AtomType::Other => "OTHR",
                },
                atom.number,
                atom.name,
                atom.aa_name,
                atom.aa_number,
                atom.coordinate.x,
                atom.coordinate.y,
                atom.coordinate.z,
                atom.second_to_last,
                atom.last,
                atom.kind
        )};

        file.write_all(line.as_bytes()).unwrap();
    }
}

fn main() {
    // command line argument: -f <filename> -o <output>
    let args: Vec<String> = std::env::args().collect();
    let mut filename = "input.pdb";
    let mut output = "output.pdb";
    for i in 0..args.len(){
        if args[i] == "-f"{
            filename = &args[i+1];
        }
        if args[i] == "-o"{
            output = &args[i+1];
        }
    }
    let mut atoms: Vec<Atom> = Vec::new();
    let parser = AtomParser::new();
    let file = std::fs::read_to_string(filename).unwrap();
    for line in file.lines(){
        if let Some(atom) = parser.parse(line){
            atoms.push(atom);
        }
    }
    print_to_file(atoms, output);
}
