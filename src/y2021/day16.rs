use itertools::Itertools;
use nom::bits::complete::take;
use nom::error::Error;
use nom::IResult;

type Input = Vec<u8>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Input {
    let line = line_source.into_iter().next().unwrap();
    line.as_ref()
        .as_bytes()
        .chunks(2)
        .map(|h| u8::from_str_radix(std::str::from_utf8(h).unwrap(), 16).unwrap())
        .collect()
}

struct Packet {
    version: u32, // 3 bits
    type_id: u32, // 3 bits
    payload: Payload,
}

enum Payload {
    Literal(u64),
    Operator(Vec<Packet>),
}

type Bits<'a> = (&'a [u8], usize);
type NomRes<'a, O> = IResult<Bits<'a>, O, Error<Bits<'a>>>;

macro_rules! take {
    ($bits:expr, $n:expr) => {
        take::<_, _, _, Error<(&[u8], usize)>>($n as usize)($bits).unwrap()
    };
}

fn decode_packet(input: Bits) -> NomRes<Packet> {
    let (bits, version) = take!(input, 3);
    let (bits, type_id) = take!(bits, 3);

    let (bits, payload) = match type_id {
        4 => decode_literal(bits)?,
        _ => decode_operator(bits)?,
    };
    Ok((
        bits,
        Packet {
            version,
            type_id,
            payload,
        },
    ))
}

fn decode_operator(bits: Bits) -> NomRes<Payload> {
    let (bits, length_type): (_, u8) = take!(bits, 1);
    if length_type == 0 {
        let (mut bits, tot_len): (_, usize) = take!(bits, 15);
        let end_len = (bits.0.len() * 8 - bits.1) - tot_len;
        let mut packets = vec![];
        while bits.0.len() * 8 - bits.1 > end_len {
            let (b, pkt) = decode_packet(bits)?;
            bits = b;
            packets.push(pkt);
        }
        Ok((bits, Payload::Operator(packets)))
    } else {
        let (mut bits, tot_cnt): (_, u32) = take!(bits, 11);
        let mut packets = vec![];
        for _p in 0..tot_cnt {
            let (b, pkt) = decode_packet(bits)?;
            bits = b;
            packets.push(pkt)
        }
        Ok((bits, Payload::Operator(packets)))
    }
}

fn decode_literal(mut bits: Bits) -> NomRes<Payload> {
    let mut literal = 0;
    loop {
        let (b, grp): (_, u64) = take(5usize)(bits)?;
        bits = b;
        literal = literal << 4 | grp & 0x0F;
        if grp & 0x10 == 0 {
            break;
        }
    }
    Ok((bits, Payload::Literal(literal)))
}

fn sum_version(pkt: &Packet) -> usize {
    if let Payload::Operator(ref pkts) = &pkt.payload {
        pkts.iter().map(sum_version).sum::<usize>() + pkt.version as usize
    } else {
        pkt.version as usize
    }
}

fn part1(input: &Input) -> usize {
    let (_, pkt) = decode_packet((input.as_slice(), 0)).unwrap();
    sum_version(&pkt)
}

fn calc_packet(pkt: &Packet) -> usize {
    let sub_pkts = match &pkt.payload {
        Payload::Literal(int) => return *int as usize,
        Payload::Operator(lst) => lst.as_slice(),
    };
    let iter = sub_pkts.iter().map(calc_packet);
    match pkt.type_id {
        0 => iter.sum(),
        1 => iter.product(),
        2 => iter.min().unwrap(),
        3 => iter.max().unwrap(),
        5 | 6 | 7 => {
            let (a, b) = iter.collect_tuple().unwrap();
            match pkt.type_id {
                5 => a > b,
                6 => a < b,
                7 => a == b,
                _ => unreachable!(),
            }
            .into()
        }
        _ => unreachable!(),
    }
}
fn part2(input: &Input) -> usize {
    let (_, pkt) = decode_packet((input.as_slice(), 0)).unwrap();
    calc_packet(&pkt)
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 906);
    assert_eq!(part2(&d), 819324480368);
}

#[test]
fn test_data() {
    let data = // Example data
"C0015000016115A2E0802F182340";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 23);
    let data = // Example data
"9C0141080250320F1802104A08";
    let d = load_input(data.lines());
    assert_eq!(part2(&d), 1);
}
