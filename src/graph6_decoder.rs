//! Graph6 file format input and output.

use crate::graph::{self, IndexType};

const N: usize = 63;

pub trait FromGraph6 {
    fn from_graph6_string(graph6_string: String) -> Self;
}

pub fn from_graph6_representation<Ix>(graph6_representation: String) -> (usize, Vec<(Ix, Ix)>)
where
    Ix: IndexType,
{
    let (graph_order_bytes, matrix_bytes) = get_graph_bytes(graph6_representation);

    let graph_order = get_graph_order(graph_order_bytes);

    let matrix_bits = matrix_bytes
        .iter()
        .flat_map(|&byte| get_number_as_bits(byte, 6))
        .collect();

    let matrix = get_edges(graph_order, matrix_bits);

    (graph_order, matrix)
}

fn get_edges<Ix>(order: usize, bits: Vec<u8>) -> Vec<(Ix, Ix)>
where
    Ix: IndexType,
{
    let mut edges = vec![];

    let mut i = 0;
    for col in 1..=order {
        for lin in 0..col {
            let is_adjacent = bits[i] == 1;

            if is_adjacent {
                edges.push((Ix::new(col), Ix::new(lin)));
            };

            i += 1;
        }
    }

    edges
}

fn get_graph_bytes(graph6_representation: String) -> (Vec<usize>, Vec<usize>) {
    let bytes: Vec<usize> = graph6_representation
        .chars()
        .map(|c| (c as usize) - N)
        .collect();

    let mut order_bytes: Vec<usize> = vec![];
    let mut matrix_bytes: Vec<usize> = vec![];

    let first_byte = *bytes.first().unwrap();
    if first_byte == N {
        order_bytes.extend_from_slice(&bytes[1..=3]);
        matrix_bytes.extend_from_slice(&bytes[4..]);
    } else {
        order_bytes.push(first_byte);
        matrix_bytes.extend_from_slice(&bytes[1..]);
    };

    (order_bytes, matrix_bytes)
}

fn get_graph_order(bytes: Vec<usize>) -> usize {
    let bits_str = bytes
        .iter()
        .flat_map(|&byte| get_number_as_bits(byte, 6))
        .map(|bit| bit.to_string())
        .collect::<Vec<String>>()
        .join("");

    usize::from_str_radix(&bits_str, 2).unwrap()
}

// Get binary representation of `n` as a vector of bits with `bits_length` length.
fn get_number_as_bits(n: usize, bits_length: usize) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in (0..bits_length).rev() {
        bits.push(((n >> i) & 1) as u8);
    }
    bits
}
