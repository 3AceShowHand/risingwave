
#[tokio::test]
async fn test_predicate_pushdown() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("predicate_pushdown.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("predicate_pushdown", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_column_pruning() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("column_pruning.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("column_pruning", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_basic_query_1() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("basic_query_1.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("basic_query_1", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_subquery() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("subquery.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("subquery", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_order_by() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("order_by.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("order_by", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_basic_query_2() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("basic_query_2.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("basic_query_2", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_tpch() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("tpch.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("tpch", &file_content)
        .await;
}
                        

#[tokio::test]
async fn test_stream_proto() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("testdata")
        .join("stream_proto.yaml");
    let file_content = tokio::fs::read_to_string(path).await.unwrap();
    risingwave_frontend_test_runner::run_test_file("stream_proto", &file_content)
        .await;
}
                        