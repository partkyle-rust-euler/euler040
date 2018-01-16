

struct Champernowne {
    current: usize,
    index: Option<usize>,
}

impl Iterator for Champernowne {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.index {
            None => {
                self.index = Some(0)
            }

            _ => {}
        }

        let index = self.index.unwrap();
        let string = self.current.to_string();
        let mut chars = string.chars();

        let c = chars.nth(index);
        
        let new_index = index + 1;

        if new_index > chars.count() {
            self.index = None;
            self.current += 1;
        } else {
            self.index = Some(new_index);
        }

        c
    }
}

fn main() {
    let c = Champernowne{ current: 0, index: None };

    let d: Vec<_> = c.take(1000001).map( |c| c.to_string().parse::<i64>().unwrap() ).collect();

    let result = d[1] * d[10]* d[100] * d[1000] * d[10000] * d[100000] * d[1000000];

    println!("{:?}", d[1] );
    println!("{:?}", d[10] );
    println!("{:?}", d[100] );
    println!("{:?}", d[1000] );
    println!("{:?}", d[10000] );
    println!("{:?}", d[100000] );
    println!("{:?}", d[1000000]);

    println!("{:?}", result);
}
