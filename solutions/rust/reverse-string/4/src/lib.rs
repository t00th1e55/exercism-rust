/*
For bonus track, grapheme
e.g ประเทศไทย -> ยทไศทเะรป
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_grapheme(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
*/

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
