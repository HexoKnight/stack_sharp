use std::collections::HashMap;

use super::interpret::{HEAP_START, MEMORY_SIZE};

pub fn free_heap(heap: &[i64], heap_free_pointer: &usize) -> HashMap<usize, u8> {
    let mut free_heap: HashMap<usize, u8> = HashMap::new();
    let mut pointer: usize = *heap_free_pointer;
    let mut num: u8 = 0;
    while pointer != 0 {
        free_heap.extend((pointer .. pointer + heap[pointer + 1] as usize).map(|x| (x, num)));
        pointer = heap[pointer] as usize;
        num += 1;
    }
    return free_heap;
}

pub fn print_heap(memory: &[i64; MEMORY_SIZE], heap_pointer: &usize, heap_free_pointer: &usize) {
    let freeheap = free_heap(&memory[..], heap_free_pointer);
    let mut heap: String = String::new();
    for (i, int) in memory[HEAP_START..std::cmp::min(*heap_pointer, HEAP_START + 20)].iter().enumerate() {
        if let Some(num) = freeheap.get(&(i + HEAP_START)) {
            heap.push_str(&format!("{}({}), ", int, num));
        } else {
            heap.push_str(&format!("{}, ", int));
        }
    }
    heap.truncate(heap.len().saturating_sub(2));
    println!("heap: [{}] | end: {}, free: {}", heap, heap_pointer, heap_free_pointer);
}