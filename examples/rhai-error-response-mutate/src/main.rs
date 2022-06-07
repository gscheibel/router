//! % curl -v \
//!    --header 'content-type: application/json' \
//!    --url 'http://127.0.0.1:4000' \
//!    --data '{"operationName": "me", "query":"query Query {\n  me {\n    name\n  }\n}"}'

use anyhow::Result;

// `cargo run -- -s ../graphql/supergraph.graphql -c ./router.yaml`
fn main() -> Result<()> {
    apollo_router::main()
}

#[cfg(test)]
mod tests {
    use apollo_router::plugins::rhai::{Conf, Rhai};
    use apollo_router::utils::test::IntoSchema::Canned;
    use apollo_router::utils::test::PluginTestHarness;
    use apollo_router::{Context, Plugin, RouterRequest};
    use futures::StreamExt;
    use http::StatusCode;

    #[tokio::test]
    async fn test_subgraph_mutates_error() {
        // Define a configuration to use with our plugin
        let conf: Conf = serde_json::from_value(serde_json::json!({
            "filename": "src/rhai_error_response_mutate.rhai",
        }))
        .expect("valid conf supplied");

        // Build an instance of our plugin to use in the test harness
        let plugin = Rhai::new(conf).await.expect("created plugin");

        // Build a test harness.
        let mut test_harness = PluginTestHarness::builder()
            .plugin(plugin)
            .schema(Canned)
            .build()
            .await
            .expect("building harness");

        // The expected reply is going to be JSON returned in the RouterResponse { error } section.
        let _expected_mock_response_error = "error created within the mock";

        // ... Call our test harness
        let query = "query {topProducts{name}}";
        let operation_name: Option<&str> = None;
        let context: Option<Context> = None;
        let service_response = test_harness
            .call(
                RouterRequest::fake_builder()
                    .header("name_header", "test_client")
                    .header("version_header", "1.0-test")
                    .query(query)
                    .and_operation_name(operation_name)
                    .and_context(context)
                    .build()
                    .expect("a valid RouterRequest"),
            )
            .await
            .expect("a router response")
            .next()
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, service_response.response.status());
        /* TBD: Figure out how to run this as a test
        // Rhai should return a 200...
        println!("RESPONSE: {:?}", service_response);
        assert_eq!(StatusCode::OK, service_response.response.status());

        // with the expected message
        if let apollo_router::ResponseBody::GraphQL(response) =
            service_response.response.body()
        {
            assert!(response.errors.is_empty());
            assert_eq!(expected_mock_response_data, response.data.as_ref().unwrap());
        } else {
            panic!("unexpected response");
        }
        */
    }
}
