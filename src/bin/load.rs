#![feature(map_first_last)]
use spoonerator::{GraphNode, Graph};
use std::collections::{HashMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::prelude::*;



fn prune_bad(g: Graph) -> Graph{
    let nodes = g.nodes.into_iter().filter(|spoon| {
        let GraphNode {from, to, cut1, cut2} = spoon;
        let word_from = &g.words[*from as usize];
        let word_to = &g.words[*to as usize];
        let prefix1 =  &word_from[..*cut1 as usize];
        let prefix2 =  &word_to[..*cut2 as usize];
        if word_from.len() < 3 || word_to.len() < 3 {
            return false;
        }

        let suffix1 =  &word_from[*cut1 as usize..];
        let suffix2 =  &word_to[*cut2 as usize..];
        if prefix1.len() > 4 || prefix2.len() > 4{
            return false
        }
        let diff_by_s = (suffix1 == suffix2.to_string() + "s")|| (suffix2 == suffix1.to_string() + "s");
        let diff_by_ed = (suffix1 == suffix2.to_string() + "ed")|| (suffix2 == suffix1.to_string() + "ed");
        let diff_by_ing = (suffix1 == suffix2.to_string() + "ing")|| (suffix2 == suffix1.to_string() + "ing");
        let diff_by_ment = (suffix1 == suffix2.to_string() + "ment")|| (suffix2 == suffix1.to_string() + "ment");
        if suffix1.ends_with("ed") || suffix2.ends_with("ed") {
            return false;
        }
        if suffix1.ends_with("s") || suffix2.ends_with("s") {
            return false;
        }
        if suffix1.ends_with("er") || suffix2.ends_with("er") {
            return false;
        }
        return !diff_by_s && !diff_by_ed && !diff_by_ing && !diff_by_ment

    }).collect();
    Graph { words: g.words, nodes: nodes }
}

fn main() {

    let mut f = File::open("spooner_graph").unwrap();
    let mut buffer: Vec<u8> = vec![]; 
    f.read_to_end(&mut buffer).unwrap();
    let mut decoded: Graph = bincode::deserialize(&buffer).unwrap();
    println!("Deserialized");
    let decoded = prune_bad(decoded);
    println!("Pruned");
    let word = "picker";
    let mut biggest = 0;
    let mut biggest_word = "".to_string();
    let mut gm = form_graph_map(&decoded.nodes);
    let sg = find_subgraphs(&gm);

    // for n in sg {
    let n = 25321; 
    // let word = &decoded.words[n as usize];

    let mut explored = HashSet::new();
    search_all(n, &gm, &mut explored);
    let keyz: Vec<u16> = gm.keys().map(|x| *x).collect();
    for k in keyz {
        if !explored.contains(&k) {
            gm.remove(&k);
        }
    }
    let keyz: HashSet<u16> = gm.keys().map(|x| *x).collect();
    bron_kerbosh(&decoded.words, &gm, HashSet::new(), keyz , HashSet::new());

    // }
    // let vs = [4154,
    // 36899,
    // 15196,
    // 33408,
    // 30839,
    // 21253,
    // 12939,
    // 25354];
    // for v in vs {
    //     println!("{}", decoded.words[v]);
    // }
    for spoon in decoded.nodes {
        let GraphNode {from, to, cut1, cut2} = spoon;
        let word_from = &decoded.words[from as usize];
        let word_to = &decoded.words[to as usize];
        let prefix1 =  &word_from[..cut1 as usize];
        let prefix2 =  &word_to[..cut2 as usize];

        let suffix1 =  &word_from[cut1 as usize..];
        let suffix2 =  &word_to[cut2 as usize..];
        if 
            suffix1 == "ing" && suffix2 == "ed" ||
            suffix2 == "ing" && suffix1 == "ed" {
                continue;
            }
        let total_len = prefix1.len() + prefix2.len() + suffix1.len() + suffix2.len();
        let diff_by_s = (suffix1 == suffix2.to_string() + "s")|| (suffix2 == suffix1.to_string() + "s");

        let diff_by_un = prefix1 == "un".to_owned() + prefix2|| (prefix2 == "un".to_owned() + prefix1);

        if word_from == word {
            biggest = total_len;
            println!("{}|{} {}|{} -> {}|{} {}|{} {}",
            prefix1, suffix1, prefix2, suffix2,
            prefix1, suffix2, prefix2, suffix1, total_len
        )
        
        }
    }
}

type GraphMap = HashMap<u16, Vec<u16>>;

fn bron_kerbosh(w: &Vec<String>, gm: &GraphMap, r: HashSet<u16>, mut p: HashSet<u16>, mut x: HashSet<u16>) {
    if p.len() == 0 && x.len() == 0 {
        // dbg!(&r);
        let mut first_lets = HashSet::new();
        let mut good = true;
        for v in r.iter() {
            let wo = w[*v as usize].clone();
            let fc = wo[..1].to_string();
            if first_lets.contains(&fc) {
                good = false;
                break
            }
            first_lets.insert(fc);
        }
        if good {
            // if r.len() >= 5 {
            println!("------{}-----", r.len());
            for v in r {
                let wo = w[v as usize].clone();
                println!("{}", wo);
            }
        // }
        }
        return;
    }
    // if P and X are both empty then
    //     report R as a maximal clique

    let mut pivot = 0;
    for p in p.iter().chain(x.iter()) {
        pivot = *p;
        break
    }
    // dbg!(&pivot);
    let pivot_nodes = &gm[&pivot];
    let mut keys = p.clone();
    for pn in pivot_nodes {
        keys.remove(&pn);
    }
    for v in keys.into_iter() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let mut new_p = HashSet::new();
        let mut new_x = HashSet::new();
        for n in &gm[&v] {
            if p.contains(&n) {
                new_p.insert(*n);
            }
            if x.contains(&n) {
                new_x.insert(*n);
            }
        }
        bron_kerbosh(&w, gm, new_r, new_p, new_x);
        x.insert(v);
        p.remove(&v);
    }
}
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}
// }

fn search_all_dbg(start: u16, gm: &GraphMap, explored: &mut HashSet<u16>, words: &Vec<String>, indent: usize) {
    if explored.contains(&start){
        return
    }

    println!("{} {}",  indent, words[start as usize]);

    explored.insert(start);
    let nodes = gm.get(&start).unwrap();
    for node in nodes {
        search_all_dbg(*node, gm, explored, &words, indent + 1);
    }
}

fn search_all(start: u16, gm: &GraphMap, explored: &mut HashSet<u16>) {
    if explored.contains(&start){
        return
    }
    explored.insert(start);
    let nodes = &gm[&start];
    for node in nodes {
        search_all(*node, gm, explored);
    }
}

fn find_subgraphs(gm: &GraphMap) -> Vec<u16> {
   let mut unexplored = gm.keys().collect::<BTreeSet<_>>();
   let mut v = vec![];
   while unexplored.len() != 0 {
        let unex = unexplored.pop_last().unwrap();
        v.push(*unex);
        let mut explored = HashSet::new();
        search_all(*unex, gm, &mut explored);
        println!("start {} n_nodes {}", unex, explored.len());
        for node in explored {
            unexplored.remove(&node);
        }
   }
   v
}

fn form_graph_map(raw_nodes: &Vec<GraphNode>) -> GraphMap{
    let mut nodes = HashMap::new();
    for g in raw_nodes {
        (*nodes.entry(g.from).or_insert_with(Vec::new)).push(
            g.to
        ); 
        (*nodes.entry(g.to).or_insert_with(Vec::new)).push(
            g.from
        ); 
    }
    nodes
}