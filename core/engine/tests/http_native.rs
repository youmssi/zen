use serde_json::json;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use zen_engine::{Decision, ZEN_CONFIG};

/// A local server that counts the connections it accepts.
fn server() -> (String, Arc<AtomicUsize>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/secret", listener.local_addr().unwrap());
    let hits = Arc::new(AtomicUsize::new(0));
    let counted = hits.clone();
    std::thread::spawn(move || {
        for mut stream in listener.incoming().flatten() {
            counted.fetch_add(1, Ordering::SeqCst);
            let _ = stream.read(&mut [0; 1024]);
            let body = r#"{"secret":"internal"}"#;
            let _ = write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            );
        }
    });
    (url, hits)
}

fn decision(url: &str) -> Decision {
    let source = format!(
        "import http from 'http';\nexport const handler = async () => {{\n  const response = await http.get('{url}');\n  return {{ data: response.data }};\n}};"
    );
    let content = json!({
        "nodes": [
            { "id": "i", "type": "inputNode", "name": "Request", "position": { "x": 0, "y": 0 } },
            { "id": "f", "type": "functionNode", "name": "Call", "position": { "x": 1, "y": 0 }, "content": { "source": source } },
            { "id": "o", "type": "outputNode", "name": "Response", "position": { "x": 2, "y": 0 } }
        ],
        "edges": [
            { "id": "a", "sourceId": "i", "targetId": "f", "type": "edge" },
            { "id": "b", "sourceId": "f", "targetId": "o", "type": "edge" }
        ]
    });
    Decision::from(serde_json::from_value::<zen_engine::model::GraphContent>(content).unwrap())
}

// One test: the switch is process-wide, so the two cases must not run concurrently.
#[tokio::test]
#[cfg_attr(miri, ignore)]
async fn function_nodes_reach_the_network_only_when_allowed() {
    let (url, hits) = server();

    let allowed = decision(&url).evaluate(json!({}).into()).await.unwrap();
    assert_eq!(
        allowed.result,
        json!({ "data": { "secret": "internal" } }).into()
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    ZEN_CONFIG.http_native.store(false, Ordering::Relaxed);
    let refused = decision(&url).evaluate(json!({}).into()).await.unwrap_err();
    assert!(
        refused
            .to_string()
            .contains("HTTP requests from function nodes are disabled"),
        "{refused}"
    );
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}
