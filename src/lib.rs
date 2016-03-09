#![crate_type = "lib"]

pub fn index (depth: u64, offset: u64) -> u64 {
  return (offset << depth + 1) | ((1 << depth) - 1);
}

pub fn depth (i: u64) -> u64 {
  let mut depth = 0;
  let mut i = i;
  while i & 1 != 0 {
    i >>= 1;
    depth += 1;
  }
  return depth;
}

pub fn offset_with_depth (i: u64, depth: u64) -> u64 {
  return i >> (depth + 1);
}

pub fn offset (i: u64) -> u64 {
  return offset_with_depth(i, depth(i));
}

pub fn parent_with_depth (i: u64, depth: u64) -> u64 {
  return index(depth + 1, offset_with_depth(i, depth) >> 1);
}

pub fn parent (i: u64) -> u64 {
  return parent_with_depth(i, depth(i));
}

pub fn sibling_with_depth (i: u64, depth: u64) -> u64 {
  return index(depth, offset(i) ^ 1);
}

pub fn sibling (i: u64) -> u64 {
  return sibling_with_depth(i, depth(i));
}

pub fn uncle_with_depth (i: u64, depth: u64) -> u64 {
  return sibling_with_depth(parent_with_depth(i, depth), depth + 1);
}

pub fn uncle (i: u64) -> u64 {
  return uncle_with_depth(i, depth(i));
}

pub fn children_with_depth(i: u64, depth: u64) -> (u64, u64) {
  if depth == 0 {
    return (i, i);
  }

  let off = offset_with_depth(i, depth) >> 1;
  return (index(depth - 1, off), index(depth - 1, off + 1));
}

pub fn children (i: u64) -> (u64, u64) {
  return children_with_depth(i, depth(i));
}

pub fn left_child_with_depth (i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return index(depth - 1, offset_with_depth(i, depth) << 1);
}

pub fn left_child (i: u64) -> u64 {
  return left_child_with_depth(i, depth(i))
}

pub fn right_child_with_depth (i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return index(depth - 1, (offset_with_depth(i, depth) << 1) + 1);
}

pub fn right_child (i: u64) -> u64 {
  return right_child_with_depth(i, depth(i))
}

pub fn right_span_with_depth (i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return (offset_with_depth(i, depth) + 1) * (2 << depth) - 2;
}

pub fn right_span (i: u64) -> u64 {
  return right_span_with_depth(i, depth(i));
}

pub fn left_span_with_depth (i: u64, depth: u64) -> u64 {
  if depth == 0 {
    return i;
  }
  return offset_with_depth(i, depth) * (2 << depth);
}

pub fn left_span (i: u64) -> u64 {
  return left_span_with_depth(i, depth(i));
}

pub fn spans_with_depth (i: u64, depth: u64) -> (u64, u64) {
  return (
    left_span_with_depth(i, depth),
    right_span_with_depth(i, depth)
  );
}

pub fn spans (i: u64) -> (u64, u64) {
  return spans_with_depth(i, depth(i));
}

pub fn count_with_depth (_: u64, depth: u64) -> u64 {
  return (2 << depth) - 1;
}

pub fn count (i: u64) -> u64 {
  return count_with_depth(i, depth(i));
}

pub fn full_roots (i: u64) -> Vec<u64> {
  let mut result = Vec::with_capacity(64);

  if i & 1 != 0 {
    return result;
  }

  let mut tmp = i >> 1;
  let mut offset = 0;
  let mut factor = 1;

  loop {
    if tmp == 0 {
      return result;
    }
    while factor * 2 <= tmp {
      factor *= 2;
    }
    result.push(offset + factor - 1);
    offset += 2 * factor;
    tmp -= factor;
    factor = 1;
  }
}

#[test]
fn test_base_blocks () {
  assert_eq!(index(0, 0), 0);
  assert_eq!(index(0, 1), 2);
  assert_eq!(index(0, 2), 4);
}

#[test]
fn test_parents () {
  assert_eq!(index(1, 0), 1);
  assert_eq!(index(1, 1), 5);
  assert_eq!(index(2, 0), 3);

  assert_eq!(parent(0), 1);
  assert_eq!(parent(2), 1);
  assert_eq!(parent(1), 3);
}

#[test]
fn test_children () {
  assert_eq!(children(0), (0, 0));
  assert_eq!(children(1), (0, 2));
  assert_eq!(children(3), (1, 5));
}

#[test]
fn test_left_child () {
  assert_eq!(left_child(0), 0);
  assert_eq!(left_child(1), 0);
  assert_eq!(left_child(3), 1);
}

#[test]
fn test_right_child () {
  assert_eq!(right_child(0), 0);
  assert_eq!(right_child(1), 2);
  assert_eq!(right_child(3), 5);
}

#[test]
fn test_siblings () {
  assert_eq!(sibling(0), 2);
  assert_eq!(sibling(2), 0);
  assert_eq!(sibling(1), 5);
  assert_eq!(sibling(5), 1);
}

#[test]
fn test_full_roots () {
  assert_eq!(full_roots(0), []);
  assert_eq!(full_roots(2), [0]);
  assert_eq!(full_roots(8), [3]);
  assert_eq!(full_roots(20), [7, 17]);
  assert_eq!(full_roots(18), [7, 16]);
  assert_eq!(full_roots(16), [7]);
}

#[test]
fn test_depths () {
  assert_eq!(depth(0), 0);
  assert_eq!(depth(1), 1);
  assert_eq!(depth(2), 0);
  assert_eq!(depth(3), 2);
  assert_eq!(depth(4), 0);
}

#[test]
fn test_offsets () {
  assert_eq!(offset(0), 0);
  assert_eq!(offset(1), 0);
  assert_eq!(offset(2), 1);
  assert_eq!(offset(3), 0);
  assert_eq!(offset(4), 2);
}

#[test]
fn test_spans () {
  assert_eq!(spans(0), (0, 0));
  assert_eq!(spans(1), (0, 2));
  assert_eq!(spans(3), (0, 6));
  assert_eq!(spans(23), (16, 30));
  assert_eq!(spans(27), (24, 30));
}

#[test]
fn test_left_span () {
  assert_eq!(left_span(0), 0);
  assert_eq!(left_span(1), 0);
  assert_eq!(left_span(3), 0);
  assert_eq!(left_span(23), 16);
  assert_eq!(left_span(27), 24);
}

#[test]
fn test_right_span () {
  assert_eq!(right_span(0), 0);
  assert_eq!(right_span(1), 2);
  assert_eq!(right_span(3), 6);
  assert_eq!(right_span(23), 30);
  assert_eq!(right_span(27), 30);
}

#[test]
fn test_count () {
  assert_eq!(count(0), 1);
  assert_eq!(count(1), 3);
  assert_eq!(count(3), 7);
  assert_eq!(count(5), 3);
  assert_eq!(count(23), 15);
  assert_eq!(count(27), 7);
}

#[test]
fn test_parent_gt_int32 () {
  assert_eq!(parent(10000000000), 10000000001);
}

#[test]
fn test_child_to_parent_to_child () {
  let mut child = 0;
  for _ in 0..50 {
    child = parent(child)
  }
  assert_eq!(child, 1125899906842623);
  for _ in 0..50 {
    child = left_child(child)
  }
  assert_eq!(child, 0);
}
