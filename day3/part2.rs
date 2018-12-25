use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};


#[derive(Debug)]
struct Claim {
    id: String,
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
    let (id, _, coordinates, dimensions) = (v[0], v[1], v[2], v[3]);
    
    let (left, top_edge) = parse_coordinates(coordinates);
    let (wide, tall) = parse_dimensions(dimensions);

    return Claim { id: String::from(id), left: left, top_edge: top_edge, wide: wide, tall: tall }
}


fn find_non_overlapping_claim(claims: Vec<Claim>) -> Option<String> {
    let mut usage_of_claims = HashMap::new();
    let mut claim_coordinates: HashMap<String, HashSet<(i32, i32)>> = HashMap::new();

    // Fill in two Hash Maps:
    // 1. Counter of usage of each coordinate (i32, i32) -> i32
    // 2, Claim's coordinates: String -> HashSet<(i32, i32)>
    for claim in claims {
        let wide = claim.wide;
        let tall = claim.tall;

        for w in claim.left..claim.left+wide {
            for t in claim.top_edge..claim.top_edge+tall {
                let usage = usage_of_claims.entry((w, t)).or_insert(0);
                *usage += 1;

                let coordinates = claim_coordinates.entry(claim.id.clone()).or_insert(HashSet::new());
                coordinates.insert((w, t));
            }
        }
    }


    // Find unique coordinates (i.e. such coordinates, that appeared only in one claim)
    let mut unique_coordinates = HashSet::new();
    for (coordinate, count) in usage_of_claims {
        if count == 1 {
            unique_coordinates.insert(coordinate);
        }
    }

    // Find Claim that has only unique coordinates.
    for (claim, coordinates) in claim_coordinates {
        if coordinates.difference(&unique_coordinates).collect::<HashSet<_>>().is_empty() {
            return Some(claim);
        }
    }

    return None;
}


fn main() {
    let claims_strings = read_claims_strings();
    let claims = claims_strings.iter().map(parse_claims_string).collect::<Vec<_>>();
    println!("{:?}", find_non_overlapping_claim(claims).unwrap());
}