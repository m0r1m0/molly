use crate::{btree::BTree, buffer::BufferPoolManager, disk::PageId, tuple};
use anyhow::Result;

pub struct SimpleTable {
    pub meta_page_id: PageId,
    pub num_key_elems: usize,
}

impl SimpleTable {
    pub fn create(&mut self, bufferPoolManager: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(bufferPoolManager)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }

    // PK, それ以外に分けてエンコードし、BTreeに入れる
    pub fn insert(
        &mut self,
        bufferPoolManager: &mut BufferPoolManager,
        record: &[&[u8]],
    ) -> Result<()> {
        let btree = BTree::new(self.meta_page_id);
        // PK
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);

        // PK 以外
        let mut value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);

        // BTree の key, value に入れる
        btree.insert(bufferPoolManager, &key, &value);
        Ok(())
    }
}
