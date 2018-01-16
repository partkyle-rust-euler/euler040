struct Champernowne {
    current: usize,
    index: usize,
}

impl Champernowne {
    fn next_char(&mut self) -> Option<char> {
        let string = self.current.to_string();
        let mut chars = string.chars();

        let c = chars.nth(self.index);
        match c {
            Some(ch) => {
                self.index += 1;
                Some(ch)
            }

            None => {
                self.index = 0;
                self.current += 1;

                self.next_char()
            }
        }
    }
}

impl Iterator for Champernowne {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.next_char()
    }
}

fn main() {
    // starting with 0 to make the subscripts easier (+1 below)
    let c = Champernowne{ current: 0, index: 0 };

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
