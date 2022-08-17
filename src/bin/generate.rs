use std::fs;
use std::io::prelude::*;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use spoonerator::{GraphNode, Graph};




fn allowed(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

fn rev_str(s: &String) -> String {
    s.chars().rev().collect::<String>()
}


fn main() {
    let raw_words = fs::read_to_string("popular.txt").unwrap();
    let mut words: Vec<String> = raw_words
        .lines()
        .filter(|x| allowed(*x))
        .map(|x| x.to_string())
        .collect();
    words.sort_unstable();
    let mut reversed_words = words.clone().iter().map(rev_str).collect::<Vec<String>>();
    reversed_words.sort_unstable();

    let words_map = words.iter().enumerate().map(|(x, y)| (y.clone(), x)).collect::<HashMap<_, _>>();


    let counter = Arc::new(Mutex::new(0));
    let big_vs = (0..words.len()).into_par_iter().map(|idx| {
        let word = &words[idx];
        let mut big_v = vec![];
        for cut in 1..(word.len() - 1)
        {
            let mut v = get_spooners(&words, &reversed_words, &words_map, word, cut);
            big_v.append(&mut v);
        }
        let c = counter.clone();
        let mut c = c.lock().unwrap();
        if *c % 100 == 0 {
            println!("{:.2}%", 100.0*(*c as f32)/(words.len() as f32));
        }

        *c += 1;
        
        big_v
    }).collect::<Vec<Vec<_>>>();
    let mut map = HashMap::<(u16, u16), (u8, u8)>::new();

    for v in big_vs.iter().flatten() {
        let GraphNode {from, to, cut1, cut2} = v;
        let tup = (*cut1, *cut2);
        let key = (*from, *to);
        match map.get(&key) {
            None => {map.insert(key, tup);},
            Some(val) => {
                let v = *val;
                if v.0 + v.1 > tup.0 + tup.1 {
                    map.insert(key, tup);
                }
            }
        };
    }
    let set_vec = map.into_iter().map(|(k, v)| GraphNode{from: k.0, to: k.1, cut1: v.0, cut2: v.1}).collect::<Vec<_>>();
    let graph = Graph {
        words,
        nodes: set_vec
    };
    let bytes = bincode::serialize(&graph).unwrap();
    let mut f = fs::File::create("spooner_graph").unwrap();
    f.write_all(&bytes).unwrap();
}


fn get_spooners(words: &Vec<String>, reversed_words: &Vec<String>, words_map: &HashMap<String, usize>, word: &str, cut: usize) -> Vec<GraphNode> {
    let prefix = &word[..cut];
    let suffix = &word[cut..];
    let mut v = vec![];
    let rev_suffix = rev_str(&suffix.to_string());

    let start_idx = match reversed_words.binary_search(&rev_suffix) {
        Ok(i) => i,
        Err(i) => i,
    };
    let mut idx = start_idx;
    let mut current_word = &reversed_words[idx];

    while rev_suffix.len() <= current_word.len() && rev_suffix == current_word[..rev_suffix.len()] {
        idx += 1;
        let pw = rev_str(&current_word);
        let ci = pw.len() - suffix.len();
        let potential_prefix = pw[..ci].to_string();
        if idx >= words.len()  {
            break
        }

        current_word = &reversed_words[idx];

        // dbg!(&pw);
        if potential_prefix == prefix || potential_prefix.len() == 0 || word == pw {
            continue
        }

        let start_idx2 = match words.binary_search(&potential_prefix) {
            Ok(i) => i,
            Err(i) => i,
        };
        let mut idx2 = start_idx2;
        let mut current_word2 = &words[idx2];
        while potential_prefix.len() <= current_word2.len() && potential_prefix == pw[..potential_prefix.len()] {
            idx2 += 1;
            let potential_suffix = &current_word2[potential_prefix.len()..];
            // dbg!(&potential_prefix, &potential_suffix);
            let s1 = prefix.clone().to_owned() + potential_suffix.clone(); 
            let s2 = potential_prefix.clone() + suffix.clone(); 
            let s3 = potential_prefix.clone() + potential_suffix.clone(); 
            if idx2 >= words.len() {
                break
            }
            current_word2 = &words[idx2];
            if potential_suffix.len() == 0 || s1 == word || s2 == word{
                continue;
            }
            if words_map.contains_key(&s1) && words_map.contains_key(&s2) && words_map.contains_key(&s3) {
                let cut2 =  potential_prefix.len();
                // println!(
                //     "{}|{} {}|{} -> {}|{} {}|{}, c1 {}, c2 {}"  
                //     ,prefix, suffix, 
                //     potential_prefix, potential_suffix,
                //     prefix, potential_suffix,
                //     potential_prefix, suffix,
                //     cut, cut2
                // );
                let w1_idx = words_map.get(word).unwrap();
                let w2_idx = words_map.get(&s3).unwrap();
                let arr = GraphNode {from: *w1_idx as u16, to:*w2_idx as u16, cut1: cut as u8, cut2: cut2 as u8};
                v.push(arr);
            }
        }
    }
    v

    // println!("AFTER {}", rev_str(&reversed_words[idx + 1]));

    // idx = start_idx;
    // current_word = &reversed_words[idx];
    // while rev_suffix.len() <= current_word.len() && rev_suffix == current_word[..rev_suffix.len()] {
    //     let pw = rev_str(&current_word);
    //     let ci = pw.len() - suffix.len();
    //     // println!("{}|{}", &pw[..ci], &pw[ci..]);
    //     if idx == 0 {
    //         break
    //     }
    //     idx -= 1;
    //     current_word = &reversed_words[idx];
    // }
    // println!("BEFORE {}", rev_str(&reversed_words[idx - 1]));
    // println!("--------------");

}
