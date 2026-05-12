# 🩺 Health Check

Monitoring powered by **Rust**, **WebAssembly**, and **Next.js**.

## ⚙️ Configuration

Add your target URLs to `config.json` in the root directory:

```json
{
	"urls": [
		"[https://www.google.com](https://www.google.com)",
		"[https://github.com](https://github.com)"
	]
}
```

## 🚀 Quick start

### CLI

&ensp;Run the health checker immediately:

```bash
cargo make scan
```

### WASM build

&ensp;Build for Next.js integration:

```bash
cargo make build-wasm
```
