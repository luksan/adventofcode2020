use itertools::Itertools;
use std::collections::HashSet;

type Connections = Vec<Connection>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Connections {
    line_source.into_iter().map(parse).collect()
}

fn parse<S: AsRef<str>>(s: S) -> Connection {
    let (a, b) = s.as_ref().splitn(2, '-').collect_tuple().unwrap();
    Connection([a.into(), b.into()])
}

struct Connection([Box<str>; 2]);

impl Connection {
    fn contains(&self, other: impl AsRef<str>) -> Option<&str> {
        let o = other.as_ref();
        let a = &*self.0[0];
        let b = &*self.0[1];
        if a == o {
            Some(b)
        } else if b == o {
            Some(a)
        } else {
            None
        }
    }
}

fn find_exit<'a, 'b>(
    current: &'a str,
    visited: &mut Vec<&'b str>,
    connections: &'b Vec<Connection>,
) -> usize
where
    'a: 'b,
{
    if current == "end" {
        return 1;
    }
    let pop = if current.as_bytes()[0].is_ascii_lowercase() {
        visited.push(current);
        true
    } else {
        false
    };
    let mut paths = 0;
    for c in connections {
        if let Some(next) = c.contains(current) {
            if visited.contains(&next) {
                continue;
            }
            paths += find_exit(next, visited, connections)
        }
    }
    if pop {
        visited.pop();
    }
    paths
}

fn part1(connections: &Connections) -> usize {
    find_exit("start", &mut vec![], connections)
}

fn find_exit_detour<'a, 'b>(
    current: &'a str,
    visited: &mut Vec<&'b str>,
    curr_path: &mut Vec<&'b str>,
    paths: &mut HashSet<String>,
    time_for_detour: bool,
    connections: &'b Vec<Connection>,
) -> usize
where
    'a: 'b,
{
    if current == "end" {
        paths.insert(curr_path.join(","));
        return 1;
    }
    curr_path.push(current);

    let is_small = current.as_bytes()[0].is_ascii_lowercase();
    for c in connections {
        if let Some(next) = c.contains(current) {
            if visited.contains(&next) {
                continue;
            }
            if is_small {
                visited.push(current);
            }
            // save detour for later
            find_exit_detour(
                next,
                visited,
                curr_path,
                paths,
                time_for_detour,
                connections,
            );
            if is_small {
                visited.pop();

                // use detour for this node
                if time_for_detour {
                    find_exit_detour(next, visited, curr_path, paths, false, connections);
                }
            }
        }
    }
    curr_path.pop();
    paths.len()
}

fn part2(connections: &Connections) -> usize {
    find_exit_detour(
        "start",
        &mut vec!["start"],
        &mut vec![],
        &mut HashSet::new(),
        true,
        connections,
    )
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 3000);
    assert_eq!(part2(&d), 74222);
}

#[test]
fn test_data() {
    let data = // Example data
"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 226);
    assert_eq!(part2(&d), 3509);
}
