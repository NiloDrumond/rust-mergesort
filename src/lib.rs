use std::cmp;

use rand::Rng;

pub fn get_rand_vec(capacity: usize) -> Vec<i32> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

fn merge<T: Copy + PartialOrd>(arr1: &[T], arr2: &[T], aux: &mut [T]) {
    assert_eq!(arr1.len() + arr2.len(), aux.len());

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            aux[k] = arr1[i];
            i += 1;
            k += 1;
        } else {
            aux[k] = arr2[j];
            j += 1;
            k += 1;
        }
    }

    if i < arr1.len() {
        aux[k..].copy_from_slice(&arr1[i..]);
    }
    if j < arr2.len() {
        aux[k..].copy_from_slice(&arr2[j..]);
    }
}

pub fn serial_sort<T: Copy + Ord>(arr: &mut [T]) {
    let n = arr.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    serial_sort(&mut arr[0..m]);
    serial_sort(&mut arr[m..n]);

    let mut aux: Vec<T> = arr.to_vec();

    merge(&arr[0..m], &arr[m..n], &mut aux[..]);

    arr.copy_from_slice(&aux);
}

fn merge_chunks<T: Copy + Ord>(arr: &mut [T], chunk_count: usize) {
    if chunk_count < 2 {
        return;
    }
    let width = arr.len() / chunk_count;
    let mut left_end: usize = width;
    loop {
        if left_end == arr.len() {
            break;
        }
        let right_end = cmp::min(arr.len(), left_end + width);
        let mut aux: Vec<T> = arr[0..right_end].to_vec();
        merge(&arr[0..left_end], &arr[left_end..right_end], &mut aux);
        arr[0..right_end].copy_from_slice(&aux);
        left_end = right_end;
    }
}

pub fn parallel_sort<T: Copy + Ord + std::marker::Send>(arr: &mut [T], threads: usize) {
    let chunk_count = cmp::min(arr.len(), threads);
    let _ = crossbeam::scope(|scope| {
        for chunk in arr.chunks_mut(arr.len() / chunk_count) {
            scope.spawn(move |_| serial_sort(chunk));
        }
    });
    merge_chunks(arr, chunk_count);
}
