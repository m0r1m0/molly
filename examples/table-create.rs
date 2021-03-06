use anyhow::Result;

use molly::buffer::{BufferPool, BufferPoolManager};
use molly::disk::{DiskManager, PageId};
use molly::table::{Table, UniqueIndex};

fn main() -> Result<()> {
    let disk = DiskManager::open("table.mly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let mut table = Table {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
        unique_indices: vec![
            UniqueIndex {
                meta_page_id: PageId::INVALID_PAGE_ID,
                skey: vec![2],
            },
        ]
    };
    table.create(&mut bufmgr)?;
    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;

    bufmgr.flush()?;
    Ok(())
}