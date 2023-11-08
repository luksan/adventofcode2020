pub fn alu(mut input: impl Iterator<Item = i32>) -> i64 {
    let mut w: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;

    w = input.next().unwrap() as i64;

    y = 0;
    y += w;
    y += 6;
    z += y;

    w = input.next().unwrap() as i64;

    y = 0;
    y += 25;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 14;
    z += y;

    w = input.next().unwrap() as i64;

    y = 0;
    y += 25;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 13;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -14;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 1;
    y *= x;
    z += y;

    w = input.next().unwrap() as i64;

    y = 0;
    y += 25;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 6;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 13;
    y *= x;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -6;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 6;
    y *= x;
    z += y;

    w = input.next().unwrap() as i64;

    y = 0;
    y += 25;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 3;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -3;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 8;
    y *= x;
    z += y;

    w = input.next().unwrap() as i64;

    y = 0;
    y += 25;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 14;
    z += y;

    w = input.next().unwrap() as i64;

    y = 0;
    y += 25;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 4;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -2;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 7;
    y *= x;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -9;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 15;
    y *= x;
    z += y;

    w = input.next().unwrap() as i64;

    x = 0;
    x += z;
    x %= 26;
    z /= 26;
    x += -2;
    x = (x == w) as i64;
    x = (x == 0) as i64;

    y = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;

    y = 0;
    y += w;
    y += 1;
    y *= x;
    z += y;
    z
}
