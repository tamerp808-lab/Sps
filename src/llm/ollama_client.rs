use reqwest::blocking::Client;
use serde_json::json;
use std::cell::RefCell;
use std::process::Command;

pub struct OllamaClient {
    base_url: String,
    client: Client,
    model: RefCell<String>,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        // محاولة تشغيل Ollama تلقائياً
        let _ = Command::new("ollama")
            .arg("serve")
            .spawn(); // يعمل في الخلفية

        // انتظر قليلاً حتى يبدأ الخادم
        std::thread::sleep(std::time::Duration::from_secs(1));

        OllamaClient {
            base_url: base_url.to_string(),
            client: Client::new(),
            model: RefCell::new("qwen3:4b".into()), // النموذج الأكبر كافتراضي
        }
    }

    pub fn set_model(&self, model_name: &str) {
        *self.model.borrow_mut() = model_name.to_string();
    }

    pub fn current_model(&self) -> String {
        self.model.borrow().clone()
    }

    pub fn chat(&self, prompt: &str) -> Result<String, String> {
        let model = self.model.borrow();
        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&json!({
                "model": model.clone(),
                "prompt": prompt,
                "stream": false
            }))
            .send()
            .map_err(|e| format!("فشل الاتصال بـ Ollama. تأكد من تثبيته: curl -fsSL https://ollama.com/install.sh | sh\nالخطأ: {}", e))?;

        let body: serde_json::Value = response.json().map_err(|e| e.to_string())?;
        body["response"].as_str().map(String::from).ok_or("لم يرد النموذج".into())
    }
}
