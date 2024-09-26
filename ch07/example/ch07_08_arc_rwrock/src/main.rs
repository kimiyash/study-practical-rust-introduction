use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();
    let dogs = Arc::new(RwLock::new(dogs));

    {
        let ds = dogs.read().map_err(stringfy)?;
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"));
    } // ds がスコープを外れロックが解除される

    dogs.write().map_err(stringfy)?.insert("ブル・テリア");

    let dogs1 = Arc::clone(&dogs);
    let _ = std::thread::spawn(move || {
        // 別スレッドで write ロックを取得し HashSet に要素を追加する
        dogs1
            .write()
            .map(|mut ds| ds.insert("コーギー"))
            .map_err(stringfy)
    })
    .join()
    .expect("Thead error");

    assert!(dogs.read().map_err(stringfy)?.contains("ブル・テリア"));
    assert!(dogs.read().map_err(stringfy)?.contains("コーギー"));

    Ok(())
}

fn stringfy(x: impl ToString) -> String {
    x.to_string()
}
