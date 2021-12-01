struct PubKeys {
    door: usize,
    key: usize,
}

impl PubKeys {
    fn new(door: usize, key: usize) -> PubKeys {
        PubKeys { door, key }
    }
}

fn part1(pks: &PubKeys) -> usize {
    let mut transform_card = 1;
    let mut transform_door = 1;
    while transform_card != pks.key {
        transform_card *= 7;
        transform_card %= 20201227;

        transform_door *= pks.door;
        transform_door %= 20201227;
    }
    transform_door
}

#[test]
fn real_data() {
    let d = PubKeys::new(14205034, 18047856);
    assert_eq!(part1(&d), 297257);
}

#[test]
fn test_data() {
    let d = PubKeys::new(17807724, 5764801);
    assert_eq!(part1(&d), 14897079);
}
