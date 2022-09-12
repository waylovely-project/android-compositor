// SPDX-FileCopyrightText: 2022 Nefo Fortressia <nefothingy@hotmail.com>
//
// SPDX-License-Identifier: LGPL-2.1-or-later

use itertools::{iproduct, Itertools};
use regex::Regex;

lazy_static::lazy_static! {
    /// See this regex in the playground: regexr.com/6ohnu
    static ref PARSE_GLOB_PATTERN: Regex = Regex::new(r"\[(.*?)\]").unwrap();
}

pub fn glob_to_mime(glob: &str) -> Vec<String> {
    let extension = glob.split("*.").last().unwrap();

    //  let mut current_possibilities: Option<Vec<(&char, &char)>> = None;
    let mut allthings = Vec::new();

    let patterns: Vec<&str> = PARSE_GLOB_PATTERN
        .find_iter(extension)
        .map(|raw_pattern| {
            let mut pattern = raw_pattern.as_str().to_string();
            pattern.pop();
            pattern.remove(0);

            let allchars = pattern.chars().collect::<Vec<char>>();
            let mut chars = Vec::new();
            for char in allchars {
                let chara = char.to_lowercase().collect::<Vec<char>>()[0];
                if !chars.contains(&chara) {
                    chars.push(chara)
                }
            }

            allthings.push(chars);

            raw_pattern.as_str()
        })
        .collect();
    iproduct!(&*&mut allthings[..])
        .map(|combination| {
            let mut extension = String::from(extension);
            for (i, combination_part) in combination.iter().enumerate() {
                extension = extension.split(patterns[i]).join(&combination_part.to_string());
            }

            extension
        })
        .collect()
}
