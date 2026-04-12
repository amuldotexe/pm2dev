This is a list of new Tauri apps that I want to build

- Maybe we can create a Tori app over this using this. https://github.com/dr-Akari/agentic-search-context-1

- Debate-brainstorming-workflow Where we use Apache AGI as a common place to constantly read input from multiple LMS agents. We talk to the agents in the tori app using the normal CLI versions, and they brainstorm and can read each other's stuff using Apache Iggy

- Tauri App for https://github.com/that-in-rust/tweet-scrolls-convert-twitter-archive-to-txt

- Tauri App for https://github.com/that-in-rust/campfire-on-rust

- Tauri App for https://github.com/that-in-rust/hackerrank-exploration-202604

- Tauri App for https://github.com/that-in-rust/floo-network-message-queue-visual-lab

- Tauri App for https://github.com/that-in-rust/superset-on-rust

- Tauri App for https://github.com/that-in-rust/pensieve-local-llm-server
    - a multi-agent debate server that can be used to build a multi-agent debate system
    - a local LLM server that can be used to debate an idea without needing to connect to the internet

## PMF-Ranked Tauri Ideas (GitHub Repo Virality Lens)

- PDF Surgeon (BUILD THIS FIRST)
    - Drag-and-drop PDF page management. Merge, split, rearrange. 4 weeks to MVP.
    - Why it wins: Everyone merges PDFs. Monthly need. Adobe charges $240/yr for this.
    - Tauri stack: lopdf + pdfium-render (Rust) + draggable grid (web frontend)
    - Viral sentence: "I just cancelled Acrobat — this free app does it in 15 seconds"

- Upscaler (SHIP SECOND)
    - AI image upscaling with before/after slider. 5 weeks to MVP.
    - Why it's #2: The before/after slider IS the marketing — people screenshot it for you
    - Tauri stack: ort (ONNX Runtime) + Real-ESRGAN model + CoreML on Apple Silicon
    - Viral sentence: "This free app makes blurry photos sharp using AI, no upload needed"

- ScanSnap (COMPLETE THE TRILOGY)
    - Photo-to-searchable-PDF via on-device OCR. 6 weeks to MVP.
    - Why it's #3: Zero macOS FOSS competition. VueScan charges $100.
    - Tauri stack: Apple Vision framework via objc2 + lopdf for PDF generation
    - Viral sentence: "I can search my handwritten notes now — this free app does OCR locally"

- The Meta-Insight
    - You're not inventing technology. PDF manipulation, image upscaling, and OCR are solved problems with BSD-licensed implementations. The gap is purely packaging. Nobody wrapped them in a beautiful, free, native Mac app. Ship PDF Surgeon in 4 weeks. Let the product be the marketing.
