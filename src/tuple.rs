use crate::memcomparable;

// memcomparable formatでエンコード
pub fn encode(elems: impl Iterator<Item = impl AsRef<[u8]>>, bytes: &mut Vec<u8>) {
  elems.for_each(|elem| {
    let elem_bytes = elem.as_ref();
    let len = memcomparable::encoded_size(elem_bytes.len());
    bytes.reserve(len);
    memcomparable::encode(elem_bytes, bytes);
  });
}

// memcomparable formatでデコード
pub fn decode(bytes: &[u8], elems: &mut Vec<Vec<u8>>) {
  let mut rest = bytes;
  while !rest.is_empty() {
    let mut elem = vec![];
    memcomparable::decode(&mut rest, &mut elem);
    elems.push(elem);
  }
}