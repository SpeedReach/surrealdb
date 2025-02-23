#[tokio::test]
#[serial]
async fn multireader() {
	// Create a new datastore
	let node_id = Uuid::parse_str("b7afc077-2123-476f-bee0-43d7504f1e0a").unwrap();
	let ds = new_ds(node_id).await;
	// Insert an initial key
	let mut tx = ds.transaction(true, false).await.unwrap();
	tx.set("test", "some text").await.unwrap();
	tx.commit().await.unwrap();
	// Create a readonly transaction
	let mut tx1 = ds.transaction(false, false).await.unwrap();
	let val = tx1.get("test").await.unwrap().unwrap();
	assert_eq!(val, b"some text");
	// Create a readonly transaction
	let mut tx2 = ds.transaction(false, false).await.unwrap();
	let val = tx2.get("test").await.unwrap().unwrap();
	assert_eq!(val, b"some text");
	// Create a readonly transaction
	let mut tx3 = ds.transaction(false, false).await.unwrap();
	let val = tx3.get("test").await.unwrap().unwrap();
	assert_eq!(val, b"some text");
	// Cancel both readonly transactions
	tx1.cancel().await.unwrap();
	tx2.cancel().await.unwrap();
	tx3.cancel().await.unwrap();
}
