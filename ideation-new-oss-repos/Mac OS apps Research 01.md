# The macOS app ecosystem mapped by where the compute actually happens

**The vast majority of professional macOS applications perform their heavy computation locally on your Mac**, with Apple Silicon's unified memory architecture dramatically expanding what's possible on-device. Of the roughly 100+ major apps surveyed across 11 categories, approximately 70% are primarily frontend-heavy (local processing), 15% are cloud-dependent, and 15% are true hybrids. The most significant recent shift is in AI/ML: workloads that required cloud GPU clusters just two years ago — running 70B-parameter language models, generating images with Stable Diffusion, real-time AI video masking — now run entirely on a Mac Mini. Apple Silicon's unified memory gives Macs a unique structural advantage: a $2,000 Mac Mini with 48GB unified memory can run AI models that physically cannot fit on a $1,600 NVIDIA RTX 4090 with only 24GB of VRAM.

---

## Creative and professional tools are overwhelmingly local

Professional creative software represents the most compute-intensive category on macOS, and nearly every app in this space does its heavy lifting on-device. These apps tax CPU, GPU, RAM, and storage simultaneously — and Apple Silicon's architecture was designed specifically to excel here.

**Video editing** is entirely local. Final Cut Pro uses a **Metal-based rendering engine** for GPU-accelerated compositing, real-time effects, and export, while also leveraging the **Neural Engine for face and object tracking**. Its tight integration with Apple's hardware Media Engine enables dedicated ProRes encode/decode in silicon — a capability that previously required a $6,000 Afterburner card. DaVinci Resolve (Studio) runs its "DaVinci Neural Engine" entirely on-device via Metal GPU acceleration, powering AI features like Magic Mask person isolation, face recognition, Super Scale upscaling, AI noise reduction, and Voice Isolation. A typical 4K editing timeline demands **32GB+ RAM, 6–16 CPU cores, sustained SSD reads of 150+ MB/s** for ProRes, and continuous GPU utilization for real-time color grading and effects. Adobe Premiere Pro's Mercury Playback Engine uses Metal on Mac and runs natively on Apple Silicon with roughly **80% faster performance** than Intel Macs.

**Audio production (DAWs)** is 100% local and uniquely CPU-bound. Logic Pro was the first major DAW with native Apple Silicon support and now uses the Neural Engine for AI features including Session Players and Stem Splitter. Ableton Live, Pro Tools, FL Studio, and GarageBand all process audio entirely on-device. DAWs care about **single-thread CPU performance and audio buffer latency** more than any other resource — GPU is irrelevant for audio DSP. The critical metric is how many real-time plugin instances the CPU can sustain before audio glitches occur.

**3D and CAD** apps are local with one notable exception. Blender has used a Metal GPU backend for Cycles rendering since version 3.1, contributed by Apple engineers, delivering roughly 2x faster renders than CPU-only (though still **2.5–3x slower than NVIDIA RTX GPUs** for ray tracing due to Apple Silicon lacking dedicated BVH traversal hardware). Cinema 4D with Redshift leverages **M3/M4 hardware ray tracing**. Maya has native Apple Silicon support since 2024 but lacks GPU rendering via Arnold on Mac. ZBrush is CPU/RAM-centric for sculpting. The outlier is **Fusion 360**, which moved all simulation to cloud-only in 2022 and uses cloud rendering as its primary path — local work is limited to CAD modeling.

**Photo editing** is fully local. Pixelmator Pro is the showcase for Apple's ML stack, using **Core ML and the Neural Engine** for ML Super Resolution (15x faster on Neural Engine vs. Rosetta), ML Denoise, ML Crop, and background removal — all on-device with models shipped inside the app bundle. Lightroom Classic processes RAW files locally with Metal GPU acceleration. Affinity Photo and Capture One are native Mac apps with full Metal pipelines.

**Design tools split clearly**: Sketch is a native macOS app rendering vectors through **Metal API** and storing files locally, while Figma is an **Electron wrapper** around a WebGL browser engine with files stored on Figma's servers. Canva is similarly cloud/web-based. This is the starkest local-vs-cloud divide in the creative category.

| App | Compute location | Key local resources | Metal / Neural Engine |
|-----|-----------------|--------------------|-----------------------|
| Final Cut Pro | Local | CPU, GPU, RAM, Media Engine, SSD | Metal ✅ Neural Engine ✅ |
| DaVinci Resolve Studio | Local | GPU-heavy, CPU, 32GB+ RAM, SSD | Metal ✅ Neural Engine ✅ |
| Logic Pro | Local | CPU (real-time DSP), RAM, audio I/O | Neural Engine ✅ (Session Players, Stem Splitter) |
| Blender | Local | GPU (Cycles Metal), CPU, RAM | Metal ✅ |
| Photoshop | Local (Firefly AI uses cloud) | CPU, GPU (Metal canvas), 16–32GB RAM | Metal ✅ |
| Pixelmator Pro | Local | GPU (Metal), Neural Engine | Metal ✅ Core ML ✅ Neural Engine ✅ |
| Sketch | Local | CPU, GPU (Metal vectors) | Metal ✅ |
| Figma | Cloud | WebGL in Electron, 400–700MB RAM | ❌ |
| Fusion 360 | Hybrid (cloud-heavy) | Local CPU for modeling only | ❌ |

---

## Developer tools run locally, but AI coding assistants live in the cloud

The developer tool ecosystem splits neatly: **traditional development tools are 100% local**, while AI-powered coding assistants are almost entirely cloud-dependent.

**IDEs and compilers** perform all their work on-device. Xcode is extraordinarily CPU-intensive during compilation, parallelizing across all available cores, with indexing capable of consuming 100% CPU for hours on large projects. SwiftUI previews use both CPU and GPU. Notably, Xcode 16+ includes an **on-device ML model for predictive code completion** trained specifically for Swift, running locally on Apple Silicon. JetBrains IDEs (IntelliJ, PyCharm) are JVM-based and consume **2–8GB RAM** for code analysis and indexing. Zed, written in Rust, stands out with its custom **GPUI framework that renders the entire UI through Metal**, achieving 120fps rendering on Apple Silicon with just 200–400MB RAM. LLVM/Clang, Swift compiler, Rust compiler, and all build tools (Gradle, CMake, Bazel, Webpack, Vite) are 100% local and CPU-bound.

**AI coding assistants are cloud-dependent by design.** GitHub Copilot gathers code context locally, encrypts it, and sends it to GitHub's cloud servers where OpenAI/Anthropic LLMs generate suggestions — all AI inference happens remotely. Cursor follows the same pattern, routing prompts through its cloud infrastructure to GPT-4o or Claude. The exceptions are meaningful: **Tabnine** runs a small local model (~2GB, providing 2–3 word completions on CPU) alongside cloud models for longer suggestions, and **Continue.dev** (open-source) can operate entirely offline when paired with Ollama, making it the only fully local AI coding assistant option.

**Containers and virtualization** are local but architecturally interesting on Apple Silicon. Docker Desktop runs a lightweight Linux VM via Apple's Virtualization.framework, with ARM64 containers at near-native speed and x86 containers emulated through **Rosetta 2 (2–5x faster than QEMU)**. However, Docker Desktop is notorious for resource consumption — often **2–4GB+ RAM idle**. OrbStack, purpose-built for Apple Silicon, achieves the same functionality with **<0.1% background CPU and <10MB disk** through dynamic RAM allocation and native Swift/Rust components.

---

## Local-first AI inference has become Apple Silicon's killer feature

The most dramatic shift in the macOS ecosystem is the emergence of **local AI inference as a first-class capability**. Apple Silicon's unified memory architecture — where CPU, GPU, and Neural Engine share one high-bandwidth memory pool with zero-copy access — gives Macs a structural advantage no other consumer platform can match.

**Ollama** (52M+ monthly downloads in Q1 2026) switched to Apple's **MLX framework** as its primary Apple Silicon backend in March 2026, delivering **1.6–2x faster prompt processing and token generation** versus its previous llama.cpp/Metal backend. On M5 chips, it leverages GPU Neural Accelerators for up to 4x speedup in time-to-first-token. Benchmarks show **1,810 tok/s prefill and 134 tok/s decode** on M5 MacBook Pro with Qwen3.5-35B. LM Studio provides a GUI for the same local inference with integrated MLX support.

**MLX**, Apple's open-source ML framework released in late 2023, has become the standard for local AI on Mac. Its zero-copy unified memory access means arrays live in shared CPU/GPU memory without data transfers. On M4 Pro (64GB), MLX achieves roughly **130 tok/s for Qwen3-Coder-30B** versus Ollama/llama.cpp's 43 tok/s — a 3x advantage. Apple positioned MLX as the preferred inference framework at WWDC 2025 with three dedicated sessions, and the mlx-community on HuggingFace hosts 4,316+ pre-converted models.

The practical model-size guidelines reveal the unified memory advantage:

- **8GB unified memory** → 7B parameter models (Q4 quantization)
- **32GB** → up to 30B models
- **48GB** → 70B models comfortably
- **64GB+** → 70B at higher precision with headroom
- **128–192GB (Max/Ultra)** → 175B+ quantized models

A Mac Mini M4 Pro with 48GB ($2,000) running Llama 3.1 70B at Q4 quantization handles a model that an RTX 4090 (24GB VRAM, $1,600) physically cannot load. NVIDIA delivers 2–4x higher raw throughput for models that fit in its VRAM, but Apple Silicon wins on **capacity** — the ability to run larger models at all.

**Cloud AI apps** remain thin clients: ChatGPT, Claude, and Gemini apps for Mac send all prompts to remote servers with minimal local resource usage. Midjourney performs zero local processing. The divide is binary — either the model runs entirely on your Mac or entirely in the cloud. **Local image generation** via DiffusionBee, Draw Things, or MLX Stable Diffusion produces 1024×1024 images locally, with FLUX-dev-4bit (12B parameters) running **3.8x faster on M5 than M4**.

---

## Productivity apps reveal the cloud-sync versus cloud-compute distinction

A critical taxonomy emerges in productivity software: many apps **sync data through the cloud but compute locally**, which is fundamentally different from apps that **depend on cloud servers for computation**.

**Cloud sync, local compute** — Apple Notes, Obsidian, Bear, Craft, Ulysses, iA Writer, Microsoft Office, Apple iWork, Fantastical, BusyCal, and Apple Mail all store and edit data on-device, using iCloud or other services purely for synchronization. Obsidian stores notes as plain-text Markdown files, achieving **sub-16ms input latency** and handling 10,000+ notes with instant local search. Bear (native Swift) opens a 94,000-word document in 55ms. Microsoft Office runs as a native Universal Binary with all document editing, rendering, and calculations happening locally — OneDrive co-authoring is optional.

**Cloud compute** — Notion, Google Docs/Sheets/Slides, Roam Research, and all project management tools (Asana, Jira, Linear, Monday.com, ClickUp, Trello) require server round-trips for core operations. Notion's every keystroke involves a network call, resulting in **50–150ms typing latency** versus Obsidian's sub-16ms. Google Docs has no native Mac app and runs entirely in the browser.

**Grammarly** is a revealing hybrid: basic spell-check runs locally, but all sophisticated grammar analysis, stylistic suggestions, and AI features send your text to AWS cloud servers. Security researchers have compared it to a functional keylogger given its Accessibility permissions to read text from any foreground app.

The **native versus Electron** divide remains significant for resource usage. Native menu bar apps use ~15MB RAM idle; Electron apps start at 80–150MB for runtime overhead alone. Slack consumes **130–960MB RAM per workspace** as an Electron app. Microsoft Teams' migration from Electron to WebView2 cut memory usage by 50%. Discord, WhatsApp, and Signal on Mac are all Electron-based.

---

## Video conferencing does surprisingly heavy local work

**Zoom** is often perceived as cloud-based but performs **heavy local computation**: continuous H.264 SVC encoding of your camera feed, decoding all received video streams, ML-based virtual background segmentation, noise suppression, and echo cancellation. Zoom's key architectural innovation is its "multimedia router" — the cloud routes SVC-encoded streams to participants without server-side transcoding. This explains why Zoom consumes **20–80%+ CPU** during calls. For 2-person calls, Zoom uses peer-to-peer protocol with minimal cloud involvement.

**FaceTime** is even more local — video encoding/decoding on-device with Apple's hardware Media Engine, with Apple servers handling only signaling while media flows peer-to-peer. Google Meet runs in the browser via WebRTC with local video encoding.

---

## Gaming, media, browsers, and security complete the map

**Native Mac games** are 100% local GPU workloads via Metal API. Over **340 games have native M1 support** and 1,700+ run on Apple Silicon. Cyberpunk 2077, Resident Evil Village, Baldur's Gate 3, and Death Stranding all run natively with Metal, including hardware ray tracing on M3+. MetalFX upscaling provides 20–30% performance gains. Cloud gaming (GeForce Now, Xbox Cloud Gaming) is the opposite extreme — zero local GPU for game rendering; the Mac only decodes a compressed video stream using its hardware video decoder at under 1W of power. CrossOver and Whisky perform 100% local computation with significant CPU overhead from x86→ARM instruction translation via Rosetta 2.

**Media streaming** apps involve cloud delivery with local hardware decoding. On Apple Silicon, DRM-protected video (Apple TV+, Netflix in Safari) uses FairPlay DRM with frames flowing directly from the **hardware video decoder to the display controller, bypassing the GPU entirely** — explaining why DRM content screenshots show black. **VLC and IINA** are fully local players using VideoToolbox hardware decode. Plex Media Server supports VideoToolbox hardware transcoding on Apple Silicon — an M1 Mac Mini handles 3x simultaneous 4K Dolby Vision HEVC transcodes with HDR-to-SDR tone mapping.

**Safari versus Chrome** shows meaningful differences: Safari uses **12–73MB per tab** versus Chrome's **290–730MB**, scores ~19% higher in Speedometer benchmarks, and routes DRM video through an efficient hardware path. The battery gap has narrowed to near-parity on Apple Silicon, but Safari's memory advantage remains substantial with many tabs open.

**Science and engineering tools** are overwhelmingly local. MATLAB R2025a now defaults to Apple Accelerate as its BLAS library, delivering **up to 3.7x faster matrix multiplication and 2x faster LU factorization** on Apple Silicon. Python's NumPy uses Accelerate for linear algebra; PyTorch's MPS backend provides GPU acceleration via Metal Performance Shaders. The limitation: MATLAB has no Mac GPU compute support, and PyTorch MPS **cannot access the Neural Engine** — only the GPU portion of Apple Silicon.

**Security tools** are local by nature. FileVault leverages a **dedicated hardware AES engine** in Apple Silicon's DMA Storage Controller with all key management in the Secure Enclave — encryption is virtually transparent with negligible performance overhead. Little Snitch monitors all outbound connections at the kernel level with low-moderate resource usage. VPN encryption is CPU-bound but manageable on Apple Silicon; WireGuard-based protocols (NordLynx, Tailscale) show only **5–10% speed reduction** with minimal CPU impact.

---

## Apple Silicon redefined the boundary between local and cloud

The M-series progression has systematically expanded on-device capability. The M1 (2020) delivered **11 TOPS** from its Neural Engine; the M4 (2024) reaches **38 TOPS** — a 3.45x improvement. The M5 (late 2025) introduced **Neural Accelerators embedded in each GPU core**, enabling dedicated matrix-multiplication hardware that accelerates LLM inference by up to 4x versus M4. Memory bandwidth — the primary bottleneck for LLM token generation — has risen from 68 GB/s (M1 base) to 546 GB/s (M4 Max), rivaling high-end enterprise GPUs.

Apple's framework stack is vertically integrated from silicon to software: **Metal** provides GPU compute, **Metal Performance Shaders** offers pre-optimized ML kernels, **Core ML** targets the Neural Engine for deployed models, **MLX** provides a NumPy-like research framework optimized for unified memory, and the **Foundation Models framework** (WWDC 2025) gives all developers free access to Apple's on-device ~3B parameter model with zero API costs. This on-device model uses 2-bit quantization-aware training and outperforms Phi-3-mini, Mistral-7B, Gemma-7B, and Llama-3-8B in human preference evaluations despite being only ~3B parameters.

The industry trend is clear: AI compute is shifting from training (still cloud-bound) to inference, which will account for **roughly two-thirds of all AI compute by 2026**, up from one-third in 2023. Gartner predicts organizations will use task-specific small language models **3x more than general-purpose LLMs by 2027**. The same inference that costs $0.50 in the cloud costs approximately $0.05 on-device — a 90% reduction. Apple Silicon Macs are positioned as the premier edge AI platform, with unified memory providing a structural advantage that discrete GPU architectures cannot replicate.

## Conclusion

The macOS app ecosystem clusters into three clear tiers. **The compute-intensive creative and developer tier** — video editors, DAWs, 3D tools, compilers, local AI inference — is overwhelmingly frontend-heavy, taxing every component of Apple Silicon from CPU cores to GPU Metal pipelines to Neural Engine to dedicated media engines. **The productivity and communication tier** splits between local-first apps (native note-taking, office suites, automation tools) and cloud-dependent platforms (project management, collaborative editors, AI chatbots), with the native-versus-Electron distinction driving a 5–10x difference in resource consumption for similar functionality. **The emerging local AI tier** — Ollama, LM Studio, MLX, DiffusionBee — represents the fastest-growing category, enabled entirely by Apple Silicon's unified memory architecture letting a laptop run models that previously required cloud GPU clusters. The boundary between local and cloud capability is not static; it shifts with each M-series generation, and the direction is consistently toward more on-device processing.