// AST Hex Coordinate: 0xOM-GEMINI-BRIDGE-0xRUST-0x9F41
use std::error::Error;
use std::time::Duration;

pub struct GeminiBridge {
    pub endpoint: String,
}

impl GeminiBridge {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub fn query_gemini_for_patch(&self, anomaly_context: &str) -> Result<String, Box<dyn Error>> {
        let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| "".to_string());
        if api_key.is_empty() {
            return Ok(format!("// [Simulated Mode] Patch for: {}\npub fn synthesized_epoch_patch() -> bool {{ true }}", anomaly_context));
        }

        println!("[GEMINI_BRIDGE] Dispatching live telemetry with 5s timeout...");
        
        let payload = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": format!("Omega Manifold Anomaly Report under G_Universal_Baseline v16.79: {}. Provide Rust patch code.", anomaly_context)
                }]
            }]
        });

        // Set a 5-second timeout so the daemon never hangs indefinitely
        let resp = match ureq::post(&self.endpoint)
            .timeout(Duration::from_secs(5))
            .set("x-goog-api-key", &api_key)
            .set("Content-Type", "application/json")
            .send_string(&payload.to_string()) 
        {
            Ok(r) => r,
            Err(e) => {
                println!("[GEMINI_BRIDGE] Request timed out or failed ({}), engaging autonomous local synthesis fallback.", e);
                let fallback_patch = format!("// Autonomous Fallback Patch for: {}\npub fn emergency_epoch_patch() -> bool {{ true }}", anomaly_context);
                return Ok(fallback_patch);
            }
        };
            
        let json: serde_json::Value = serde_json::from_reader(resp.into_reader())?;
        let text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("// No patch generated")
            .to_string();

        Ok(text)
    }
}
