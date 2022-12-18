use aoc_2022::files::open_file;
extern crate nom;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, u8},
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    List(Vec<Value>),
    Int(u8),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Packet(Vec<Value>);

pub fn main() {
    let lines = open_file("examples/day13/input.txt").expect("Failed to open file");
    let parsed = parse(lines);

    let mut count = 0;
    for (i, (p1, p2)) in parsed.clone().into_iter().enumerate() {
        let v1 = Value::List(p1.0);
        let v2 = Value::List(p2.0);
        if *in_right_order(&v1, &v2).get_or_insert(false) {
            count += i + 1;
        }
    }
    println!("In right order {}", count);

    // PART TWO

    let mut flat_list: Vec<_> = parsed
        .iter()
        .flat_map(|(v1, v2)| vec![v1, v2])
        .map(|p| Value::List(p.0.clone()))
        .collect();
    let v1 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let v2 = Value::List(vec![Value::List(vec![Value::Int(6)])]);
    flat_list.append(&mut vec![v1.clone(), v2.clone()]);

    flat_list.sort_by(|a, b| match in_right_order(a, b) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Greater,
    });

    let mut indices = Vec::new();

    for (i, v) in flat_list.iter().enumerate() {
        if v == &v1 || v == &v2 {
            indices.push(i + 1);
        }
    }
    println!("Indices are {:?}", indices);
    println!("Product {}", indices.iter().product::<usize>());
}

fn in_right_order(v1: &Value, v2: &Value) -> Option<bool> {
    match (v1, v2) {
        // same number keep checking
        (Value::Int(l), Value::Int(r)) if l == r => None,
        // end of check
        (Value::Int(l), Value::Int(r)) => Some(l < r),
        (Value::Int(_), Value::List(_)) => in_right_order(&Value::List(vec![v1.clone()]), v2),
        (Value::List(_), Value::Int(_)) => in_right_order(v1, &Value::List(vec![v2.clone()])),
        (Value::List(l), Value::List(r)) => {
            let min_len = l.len().min(r.len());
            for i in 0..min_len {
                if let Some(res) = in_right_order(&l[i], &r[i]) {
                    return Some(res);
                }
            }
            if l.len() == r.len() {
                None
            } else {
                Some(l.len() < r.len())
            }
        }
    }
}

fn parse(lines: Vec<String>) -> Vec<(Packet, Packet)> {
    let non_empty: Vec<_> = lines.iter().filter(|l| !l.is_empty()).collect();
    let mut res = Vec::new();
    for i in 0..(non_empty.len() / 2) {
        let l1 = parse_packet(non_empty[2 * i]);
        let l2 = parse_packet(non_empty[2 * i + 1]);
        res.push((l1, l2));
    }
    res
}

pub fn parse_packet(s: &str) -> Packet {
    let (_, v) = parse_list(s).unwrap();
    match v {
        Value::List(vs) => Packet(vs),
        _ => panic!("Received integer!"),
    }
}

pub fn parse_list(s: &str) -> IResult<&str, Value> {
    let (s, elements) = delimited(
        char('['),
        separated_list0(char(','), u8.map(Value::Int).or(parse_list)),
        char(']'),
    )(s)?;
    Ok((s, Value::List(elements)))
}
