use std::io;
use std::io::BufRead;

use std::collections::VecDeque;

fn in_range(v: i64, ran: &(i64, i64, i64, i64, String)) -> bool {
    ran.0 <= v && v <= ran.1 || ran.2 <= v && v <= ran.3
}

fn main() {
    let mut lns = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap());

    let props = lns.by_ref().take_while(|l| !l.is_empty()).map(|l| {
        let nums = l.split(|c| c == ' ' || c == '-')
            .filter_map(|w| w.parse::<i64>().ok())
            .collect::<Vec<_>>();
        (nums[0], nums[1], nums[2], nums[3], l.split(':').next().unwrap().to_string())
    }).collect::<Vec<_>>();

    let my_ticket = lns.by_ref().nth(1).unwrap();
    let tickets = std::iter::once(my_ticket).chain(lns.skip(2))
        .filter(|l| !l.is_empty())
        .map(|l|
            l.split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
        )
        .filter(|vv| vv.iter().all(|v| props.iter().any(|ran| in_range(*v, ran))))
        .collect::<Vec<_>>();

    let n = props.len();

    let prop_pos = (0..n).map(|propi| 
        (0..n).map(|posi| tickets.iter().all(|ti| in_range(ti[posi], &props[propi]))).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let (from_prop_edges, _) = (0..n).fold((vec![None; n], vec![None; n]), |(mut from_prop_edges, mut from_pos_edges): (Vec<Option<usize>>, Vec<Option<usize>>), _| {
        let mut qq = (0..n).filter(|i| from_prop_edges[*i].is_none()).collect::<VecDeque<_>>();
        let mut vis_from = (0..n).map(|i| if from_prop_edges[i].is_none() { Some(n) } else { None }).collect::<Vec<Option<usize>>>();
        loop {
            let cur = qq.pop_front().unwrap();
            let tpos = (0..n).find(|i| prop_pos[cur][*i] && from_pos_edges[*i].is_none());
            match tpos {
                Some(tposv) => {
                    let mut nex = cur;
                    let mut tar = Some(tposv);
                    while let Some(tart) = tar {
                        let ntar = from_prop_edges[nex];
                        from_prop_edges[nex] = Some(tart);
                        from_pos_edges[tart] = Some(nex);
                        tar = ntar;
                        nex = vis_from[nex].unwrap();
                    }
                    break;
                },
                None => {
                    (0..n).filter(|i| prop_pos[cur][*i]).map(|i| from_pos_edges[i].unwrap()).for_each(|i| {
                        if vis_from[i].is_none() {
                            vis_from[i] = Some(cur);
                            qq.push_back(i);
                        }
                    });
                }
            }
        }
        (from_prop_edges, from_pos_edges)
    });

    let res = (0..n).filter(|i| props[*i].4.starts_with("departure")).map(|i| tickets[0][from_prop_edges[i].unwrap()]).product::<i64>();
    println!("{res}");
}
