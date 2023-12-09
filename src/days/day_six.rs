// Time:        46     68     98     66
// Distance:   358   1054   1807   1080

// Time:      7  15   30
// Distance:  9  40  200

pub fn day_six_part_one() -> u128 {
    let times = vec![46, 68, 98, 66];
    let distances = vec![358, 1054, 1807, 1080];
    // let times = vec![7, 15, 30];
    // let distances = vec![9, 40, 200];
    let mut index = 0;
    times
        .into_iter()
        .map(|time| {
            let mut count = 0;
            for i in 0..=time {
                let can_travel = (time - i) * i;
                if can_travel > distances[index] {
                    count += 1;
                }
            }
            index += 1;
            count
        })
        .product()
}

pub fn day_six_part_two() -> u128 {
    let time: u128 = 46689866;
    let distance: u128 = 358105418071080;
    let mut start = 0;
    let mut end = 46689866;
    while start < time {
        let can_travel = (time - start) * start;
        if can_travel > distance {
            break;
        }
        start += 1;
    }
    while end > 0 {
        let can_travel = (time - end) * end;
        if can_travel > distance {
            break;
        }
        end -= 1;
    }
    end - start + 1
}

