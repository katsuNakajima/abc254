#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let n = parse_line!(usize);
    let mut tpas: Vec<Vec<i64>> = Vec::new();
    for i in 1..=n {
        tpas.push(vec![0; i]);
    }

    for i in 0..n {
        for j in 0..=i {
            if j == 0 {
                tpas[i][j] = 1;
            } else if j == i {
                tpas[i][j] = 1;
                break;
            } else {
                tpas[i][j] = tpas[i - 1][j - 1] + tpas[i - 1][j];
            }
        }
    }
    for x in tpas {
        for y in x {
            print!("{} ", y);
        }
        println!();
    }
}
