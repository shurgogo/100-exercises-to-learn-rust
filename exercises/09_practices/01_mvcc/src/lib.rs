pub mod engine;
pub mod mvcc;
pub mod transaction;

#[cfg(test)]
mod tests {
    use crate::{engine::Engine, mvcc::MVCC};

    #[test]
    fn test_transaction() {
        let engine = Engine::new();
        let mvcc = MVCC::new(engine);

        let tx0 = mvcc.begin_transaction();
        tx0.set(b"a", b"a1".to_vec());
        tx0.set(b"b", b"b1".to_vec());
        tx0.set(b"c", b"c1".to_vec());
        tx0.set(b"d", b"d1".to_vec());
        tx0.set(b"e", b"e1".to_vec());
        tx0.commit();
    }

    #[test]
    #[should_panic]
    fn test_version_controll() {
        let engine = Engine::new();
        let mvcc = MVCC::new(engine);
        let tx0 = mvcc.begin_transaction();
        tx0.set(b"a", b"a1".to_vec());
        tx0.set(b"b", b"b1".to_vec());
        tx0.set(b"c", b"c1".to_vec());
        tx0.set(b"d", b"d1".to_vec());
        tx0.set(b"e", b"e1".to_vec());
        tx0.commit();

        let tx1 = mvcc.begin_transaction();
        tx1.set(b"a", b"a2".to_vec());
        tx1.set(b"e", b"e2".to_vec());
        tx1.print_all();

        let tx2 = mvcc.begin_transaction();
        tx2.delete(b"b");
        tx2.print_all();

        tx1.commit();
        tx2.print_all();

        let tx3 = mvcc.begin_transaction();
        tx3.print_all();

        tx3.set(b"f", b"f1".to_vec());
        tx2.set(b"f", b"f1".to_vec());
    }
}
