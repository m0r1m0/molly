use anyhow::Result;
use molly::{
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    table::SimpleTable,
};

fn main() -> Result<()> {
    // ヒープファイルを開く
    let disk = DiskManager::open("simple.mly")?;
    // バッファプール、バッファプールマネージャを初期化
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);
    
    // スキーマ定義
    let mut table = SimpleTable {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
    };
    // テーブル作成
    table.create(&mut bufmgr)?;
    // データ追加
    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;

    // ヒープファイルに書き出し
    bufmgr.flush()?;
    Ok(())
}
