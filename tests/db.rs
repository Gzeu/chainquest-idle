use chainquest_idle::resources::DatabaseConnection;
use chainquest_idle::components::IdleProgress;

#[test]
fn db_save_and_load_roundtrip() {
    let db = DatabaseConnection::new();
    let p = IdleProgress { resources: 42.0, experience: 7.0, level: 3, last_update: 12345.0 };
    db.save_progress(&p).expect("save ok");
    let loaded = db.load_progress().expect("load ok");
    assert!((loaded.resources - 42.0).abs() < 1e-6);
    assert_eq!(loaded.level, 3);
}
