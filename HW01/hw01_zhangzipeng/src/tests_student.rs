#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::{mat_mult};
use problem3::{sieve};
use problem4::{hanoi, Peg};

// Problem1

#[test]
fn test_sum_small() {
    let array = [1,2,3,4,5, 6];
    assert_eq!(sum(&array), 21);
}

#[test]
fn test_dedup_small() {
    let vs = vec![1,2,2,3,4,1,1,2,4];
    assert_eq!(dedup(&vs), vec![1,2,3,4]);
}

fn odd_predicate(x: i32) -> bool {
    (x % 2) == 1
}

#[test]
fn test_filter_small() {
    let vs = vec![1,2,3,4,5];
    assert_eq!(filter(&vs, &odd_predicate), vec![1,3,5]);
}

// Problem 2

#[test]
fn test_mat_mult_identity() {
    let mat1 = vec![vec![5.;3]; 3];

    let mut mat2 = vec![vec![0.;3]; 3];
    for i in 0..mat1.len() {
        mat2[i][i] = 1.;
    }

    let result = mat_mult(&mat1, &mat2);
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            assert_eq!(result[i][j], mat1[i][j]);
        }
    }
}

// Problem 3

#[test]
fn test_sieve_basic() {
    assert_eq!(vec![2,3,5,7,11,13], sieve(14));
}

// Problem 4

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(2, Peg::A, Peg::B, Peg::C);
    assert_eq!(vec![(Peg::A, Peg::B), (Peg::A, Peg::C), (Peg::B, Peg::C)], result);
    assert_eq!(3, result.len());
}
