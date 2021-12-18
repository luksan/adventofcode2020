use num_integer::Roots;
use scan_fmt::scan_fmt;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Area {
    let s = line_source.into_iter().next().unwrap();
    let (x_min, x_max, y_min, y_max) = scan_fmt!(
        s.as_ref(),
        "target area: x={}..{}, y={}..{}",
        i32,
        i32,
        i32,
        i32
    )
    .unwrap();

    Area {
        x_max,
        x_min,
        y_max,
        y_min,
    }
}

struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn part1(target: &Area) -> usize {
    let v_x_min = ((target.x_min * 8 + 1).sqrt() - 1) / 2;
    let v_x_max = target.x_max;
    let mut best_y = 0;
    for vx0 in v_x_min..=v_x_max {
        for vy0 in 0..1000 {
            let mut vx = vx0;
            let mut vy = vy0;
            let mut x = 0;
            let mut y = 0;
            let mut y_max = 0;
            while x <= target.x_max && y >= target.y_min {
                x += vx;
                y += vy;
                y_max = y_max.max(y);
                vx -= vx.signum();
                vy -= 1;
                if (target.x_min..=target.x_max).contains(&x)
                    && (target.y_min..=target.y_max).contains(&y)
                {
                    best_y = best_y.max(y_max);
                    break;
                }
            }
        }
    }
    best_y as usize
}

fn part2(target: &Area) -> usize {
    let v_x_min = ((target.x_min * 8 + 1).sqrt() - 1) / 2;
    let v_x_max = target.x_max;
    let mut cnt = 0;
    for vx0 in v_x_min..=v_x_max {
        for vy0 in target.y_min..1000 {
            let mut vx = vx0;
            let mut vy = vy0;
            let mut x = 0;
            let mut y = 0;
            while x <= target.x_max && y >= target.y_min {
                x += vx;
                y += vy;
                vx -= vx.signum();
                vy -= 1;
                if (target.x_min..=target.x_max).contains(&x)
                    && (target.y_min..=target.y_max).contains(&y)
                {
                    cnt += 1;
                    break;
                }
            }
            if y > target.y_max {
                break;
            }
        }
    }
    cnt
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 7626);
    assert_eq!(part2(&d), 2032);
}

#[test]
fn test_data() {
    let data = // Example data
"target area: x=20..30, y=-10..-5";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 45);
    assert_eq!(part2(&d), 112);
}
