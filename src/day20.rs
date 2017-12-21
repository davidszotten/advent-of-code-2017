use nom::IResult;
use parsers::integer;
use shared::AppResult;

type Triple = (i32, i32, i32);

named!(parse_triple <Triple>,
    do_parse!(
        tag!("<") >>
        x: integer >>
        tag!(",") >>
        y: integer >>
        tag!(",") >>
        z: integer >>
        tag!(">") >>
        ((x, y, z))
    )
);

named!(parse <(Triple, Triple, Triple)>,
    do_parse!(
        tag!("p=") >>
        p: parse_triple >>
        tag!(", v=") >>
        v: parse_triple >>
        tag!(", a=") >>
        a: parse_triple >>
        ((p, v, a))
    )
);


pub fn part1(input: &str) -> AppResult<u32> {
    let rows = input.split('\n')
        .filter_map(|row| match parse(row.as_bytes()) {
            IResult::Done(_, entry) => Some(entry),
            o => panic!("parsing failed {:?}", o),
        })
        .collect::<Vec<_>>();
    let mut largest_val = 0;
    let mut largest_pos = 0;

    for (index, ((mut px, mut py, mut pz), (mut vx, mut vy, mut vz), (ax, ay, az))) in rows.into_iter().enumerate() {
        // pretty hacky, rely on overflow
        let mut val = 0;
        loop {
            vx = if let Some(res) = vx.checked_add(ax) {res}
            else {break};
            px = if let Some(res) = px.checked_add(vx) {res}
            else {break};

            vy = if let Some(res) = vy.checked_add(ay) {res}
            else {break};
            py = if let Some(res) = py.checked_add(vy) {res}
            else {break};

            vz = if let Some(res) = vz.checked_add(az) {res}
            else {break};
            pz = if let Some(res) = pz.checked_add(vz) {res}
            else {break};

            let partial = if let Some(res) = px.abs().checked_add(py) {res}
            else {break};
            let _pos = if let Some(res) = partial.checked_add(pz.abs()) {res}
            else {break};
            val += 1;
        }

        // println!("{}", val);

        if val > largest_val {
            largest_val = val;
            largest_pos = index;
        }

    }
    Ok(largest_pos as u32)
}


pub fn part2(_input: &str) -> AppResult<u32> {
    Ok(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&b"p=<-14,1556,-678>, v=<-17,-131,-2>, a=<2,2,8>"[..]),
            IResult::Done(&b""[..], ((-14,1556,-678), (-17, -131, -2), (2, 2, 8)))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>").unwrap(), 0);
    }
}
