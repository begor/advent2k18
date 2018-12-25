use std::io::{self, BufRead};
use std::collections::HashMap;


#[derive(Debug)]
struct Claim {
    left: i32,
    top_edge: i32,
    wide: i32,
    tall: i32
}


fn read_claims_strings() -> Vec<String> {
    let stdin = io::stdin();


    return stdin.lock().lines().map(|s| s.unwrap()).collect();
}


fn parse_coordinates(coordinates: &str) -> (i32, i32) {
    let v: Vec<&str> = coordinates.split(|c: char| c == ':' || c == ',').collect();

    return (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap());
}


fn parse_dimensions(dimensions: &str) -> (i32, i32) {
    let v: Vec<&str> = dimensions.split("x").collect();

    return (v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap());
}


fn parse_claims_string(claims_string: &String) -> Claim {
    let v: Vec<&str> = claims_string.split(' ').collect();
    let (_, _, coordinates, dimensions) = (v[0], v[1], v[2], v[3]);
    
    let (left, top_edge) = parse_coordinates(coordinates);
    let (wide, tall) = parse_dimensions(dimensions);

    return Claim { left: left, top_edge: top_edge, wide: wide, tall: tall }
}


fn find_overlaps_over_claims(claims: Vec<Claim>) -> i32 {
    let mut usage_of_claims = HashMap::new();

    for claim in claims {
        let wide = claim.wide;
        let tall = claim.tall;

        for w in claim.left.. claim.left+wide {
            for t in claim.top_edge..claim.top_edge+tall {
                let usage = usage_of_claims.entry((w, t)).or_insert(0);
                *usage += 1;
            }
        }
    }

    return usage_of_claims.iter().filter(|&(_, count)| *count > 1).count() as i32;

}


fn main() {
    let claims_strings = read_claims_strings();
    let claims = claims_strings.iter().map(parse_claims_string).collect::<Vec<_>>();
    println!("{:?}", find_overlaps_over_claims(claims));
}