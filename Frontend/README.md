# 🧠 PersonaVault

## 🔒 Overview

**PersonaVault** is an innovative **privacy-first AI assistant** designed to operate entirely on your local machine. Built as a Final Year Computer Science project, it demonstrates how powerful AI capabilities can exist without sacrificing user privacy.

Unlike cloud-based assistants that send your data to remote servers, PersonaVault processes everything locally using:

- 🤖 **Local Large Language Models** for intelligent conversations
- 🔐 **Military-grade encryption** for data storage
- 🧠 **Vector embeddings** for context-aware document understanding
- 🛡️ **Zero-Knowledge Proofs** for privacy verification
- ⚡ **Rust-powered security layer** for maximum performance

> **Note:** This repository is in active development as part of an academic project. The architecture and implementation are subject to change as research progresses.

---

## ✨ Key Features

### Core Capabilities

| Feature | Description | Status |
|---------|-------------|--------|
| 🤖 **Offline AI Processing** | Run powerful LLMs locally without internet connectivity | ✅ In Progress |
| 🔐 **Encrypted Data Vault** | AES-256 encrypted storage for all personal data | ✅ In Progress |
| 📚 **RAG System** | Retrieval-Augmented Generation using local documents | ✅ In Progress |
| 🔍 **Semantic Search** | Vector-based similarity search across personal knowledge base | ⏳ Planned |
| 🗣️ **Voice Interaction** | Natural voice commands and responses | ⏳ Planned |
| 👤 **Biometric Authentication** | Face recognition for secure vault access | ⏳ Planned |
| 🖥️ **System Integration** | Execute OS-level commands securely | ⏳ Planned |
| 🌐 **Hybrid Mode** | Optional encrypted API bridge to cloud LLMs | ⏳ Planned |

### Privacy & Security

- **Zero data transmission** to external servers in offline mode
- **End-to-end encryption** for all stored conversations and documents
- **Zero-Knowledge Proof verification** for authentication without exposing credentials
- **Local vector embeddings** preventing metadata leakage
- **Secure memory handling** using Rust's ownership system

---

## 🏗️ System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                        │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              React + Vite Frontend                    │  │
│  │  • Modern UI with Tailwind CSS                        │  │
│  │  • Real-time chat interface                           │  │
│  │  • Document management                                │  │
│  └───────────────┬───────────────────────────────────────┘  │
│                  │ IPC Bridge                               │
│  ┌───────────────▼───────────────────────────────────────┐  │
│  │           Rust Core Layer                             │  │
│  │  • Security & Encryption                              │  │
│  │  • API Proxy & Request Validation                     │  │
│  │  • System Commands Execution                          │  │
│  │  • File System Management                             │  │
│  └───────────────┬───────────────────────────────────────┘  │
└──────────────────┼───────────────────────────────────────────┘
                   │ HTTP/WebSocket
   ┌───────────────▼───────────────────────────────────────┐
   │            FastAPI Backend (Python)                   │
   │  ┌─────────────────────────────────────────────────┐  │
   │  │  AI Processing Engine                           │  │
   │  │  • Local LLM Inference                          │  │
   │  │  • Embedding Generation                         │  │
   │  │  • RAG Pipeline                                 │  │
   │  └─────────────────────────────────────────────────┘  │
   │  ┌─────────────────────────────────────────────────┐  │
   │  │  Vector Database (ChromaDB)                     │  │
   │  │  • Semantic search                              │  │
   │  │  • Document indexing                            │  │
   │  └─────────────────────────────────────────────────┘  │
   │  ┌─────────────────────────────────────────────────┐  │
   │  │  Encrypted Storage (SQLite)                     │  │
   │  │  • Conversation history                         │  │
   │  │  • User preferences                             │  │
   │  └─────────────────────────────────────────────────┘  │
   └───────────────────────────────────────────────────────┘
```

---

## 🛠️ Tech Stack

### Frontend Layer
- **React 18+** with Hooks and Context API
- **Vite** for lightning-fast development
- **Tailwind CSS** for responsive, modern styling
- **Lucide Icons** for consistent iconography
- **Framer Motion** for smooth animations (optional)

### Application Shell
- **Tauri 2.0** for secure, lightweight desktop wrapper
- **Rust** for native performance and memory safety
- **IPC Communication** for frontend-backend bridge

### Backend Services
- **FastAPI** for high-performance async API
- **Python 3.10+** with type hints
- **Pydantic** for data validation
- **LangChain** for LLM orchestration
- **ChromaDB** for vector storage
- **SQLite** with SQLCipher for encrypted persistence

### AI & ML
- **LLaMA 2/3** or **Phi-3** for language understanding
- **Sentence Transformers** for embeddings
- **Ollama** for local model management
- **ONNX Runtime** for optimized inference (planned)

### Security
- **AES-256-GCM** encryption for data at rest
- **Argon2** for password hashing
- **JWT** for session management
- **Zero-Knowledge Proof libraries** (zk-SNARKs)

---

## 🚀 Installation & Setup

### Prerequisites

Ensure you have the following installed:

- **Node.js** 18+ and npm
- **Rust** 1.70+ (install via [rustup](https://rustup.rs/))
- **Python** 3.10+ with pip
- **Git** for version control

### Step 1: Clone the Repository

```bash
git clone https://github.com/AdeelAliYousaf/PersonaVault.git
cd PersonaVault
```

### Step 2: Frontend Setup

```bash
# Navigate to frontend directory
cd Frontend

# Install dependencies
npm install

# Return to root
cd ..
```

### Step 3: Tauri Configuration

```bash
# Install Tauri CLI
npm install @tauri-apps/cli --save-dev

# Initialize Tauri (if not already configured)
npx tauri init

# Follow prompts to configure Tauri settings
```

### Step 4: Backend Setup

```bash
# Navigate to backend directory
cd Backend

# Create virtual environment (recommended)
python -m venv venv

# Activate virtual environment
# On Windows:
venv\Scripts\activate
# On macOS/Linux:
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Install additional AI libraries
pip install langchain chromadb sentence-transformers ollama
```

### Step 5: Download Local AI Model

```bash
# Install Ollama (https://ollama.ai)
# Then pull a model (e.g., Phi-3 or LLaMA)
ollama pull phi3
```

### Step 6: Environment Configuration

Create a `.env` file in the Backend directory:

```env
# Server Configuration
HOST=127.0.0.1
PORT=8000

# Security
SECRET_KEY=your-super-secret-key-change-this
ENCRYPTION_KEY=your-aes-encryption-key-32-bytes

# AI Model
MODEL_NAME=phi3
EMBEDDING_MODEL=all-MiniLM-L6-v2

# Database
DB_PATH=./data/personavault.db
VECTOR_DB_PATH=./data/chroma_db
```

### Step 7: Run the Application

**Terminal 1 - Backend:**
```bash
cd Backend
uvicorn main:app --reload --host 127.0.0.1 --port 8000
```

**Terminal 2 - Frontend (Development):**
```bash
npm run tauri dev
```

**Build for Production:**
```bash
npm run tauri build
```

---

## 📁 Project Structure

```
PersonaVault/
├── Frontend/                    # React frontend application
│   ├── src/
│   │   ├── components/         # Reusable UI components
│   │   ├── pages/              # Application pages
│   │   ├── hooks/              # Custom React hooks
│   │   ├── utils/              # Utility functions
│   │   └── App.tsx             # Main application component
│   ├── public/                 # Static assets
│   └── package.json
│
├── src-tauri/                  # Rust backend for Tauri
│   ├── src/
│   │   ├── main.rs            # Entry point
│   │   ├── encryption.rs      # Encryption utilities
│   │   ├── commands.rs        # Tauri commands
│   │   └── security.rs        # Security layer
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
│
├── Backend/                    # Python AI backend
│   ├── app/
│   │   ├── main.py            # FastAPI application
│   │   ├── models/            # Data models
│   │   ├── services/          # Business logic
│   │   │   ├── llm_service.py
│   │   │   ├── rag_service.py
│   │   │   └── embedding_service.py
│   │   ├── routes/            # API endpoints
│   │   └── utils/             # Helper functions
│   ├── requirements.txt
│   └── .env
│
├── docs/                       # Documentation
├── tests/                      # Test suites
└── README.md
```

---

## 🗺️ Roadmap

### Phase 1: Foundation (Current)
- [x] Project structure setup
- [x] Basic Tauri + React integration
- [x] FastAPI backend initialization
- [ ] Local LLM integration
- [ ] Basic chat interface
- [ ] Encrypted storage implementation

### Phase 2: Core Features
- [ ] RAG pipeline with ChromaDB
- [ ] Document upload and indexing
- [ ] Semantic search functionality
- [ ] Context-aware responses
- [ ] Conversation history
- [ ] Export/import functionality

### Phase 3: Advanced Features
- [ ] Voice command support
- [ ] Biometric authentication
- [ ] Zero-Knowledge Proof integration
- [ ] System command execution
- [ ] Plugin architecture
- [ ] Multi-language support

### Phase 4: Optimization
- [ ] Model quantization for better performance
- [ ] Caching layer for frequent queries
- [ ] Batch processing for embeddings
- [ ] GPU acceleration support
- [ ] Memory optimization

### Phase 5: Polish & Release
- [ ] Comprehensive documentation
- [ ] User onboarding flow
- [ ] Performance benchmarks
- [ ] Security audit
- [ ] Academic paper publication

---

## 🧪 Testing

```bash
# Backend tests
cd Backend
pytest tests/ -v

# Frontend tests
cd Frontend
npm test

# Integration tests
npm run test:integration
```

---

## 📊 Performance Benchmarks

*Benchmarks will be added as the project progresses*

| Metric | Target | Current |
|--------|--------|---------|
| Cold start time | < 3s | TBD |
| Query response | < 2s | TBD |
| Memory usage | < 2GB | TBD |
| CPU usage (idle) | < 5% | TBD |

---

## 🤝 Contributing

This project is currently under academic development and is not open for external contributions. However, feedback and suggestions are welcome through GitHub issues.

---

## 📜 License

This project is licensed for **academic and demonstration purposes only**.

**Restrictions:**
- ❌ No commercial use
- ❌ No redistribution
- ❌ No modification without permission
- ✅ Academic reference with proper citation

For licensing inquiries, contact the author.

---

## 🎓 Academic Context

This project serves as a Final Year Project (FYP) for Bachelor of Science in Computer Science, demonstrating:

- **Research Topic:** Privacy-preserving AI systems
- **Key Contributions:** Local LLM deployment with encrypted RAG
- **Technologies Explored:** Rust security, ZKP, vector databases
- **Expected Outcomes:** Functional prototype and research paper

---

## 👨‍💻 Author

**Adeel Ali Yousaf**  
Final Year BSCS Student | Full-Stack Developer | AI Enthusiast

- 🌐 Portfolio: [adeelaliyousaf.vercel.app](https://adeelaliyousaf.vercel.app)
- 💼 LinkedIn: [linkedin.com/in/adeelaliyousaf](https://linkedin.com/in/adeelaliyousaf)
- 📧 Email: contact@adeelaliyousaf.dev
- 🐙 GitHub: [@AdeelAliYousaf](https://github.com/AdeelAliYousaf)

---

## 🙏 Acknowledgments

- **Tauri Team** for the excellent desktop framework
- **Ollama / llama.cpp** for simplified local LLM deployment
- **ChromaDB / SQLite** for efficient vector storage
- **FastAPI** for the powerful async framework

---

## 📝 Citation

If you reference this project in academic work, please cite:

```bibtex
@misc{yousaf2025personavault,
  author = {Yousaf, Adeel Ali},
  title = {PersonaVault: Privacy-Preserving Personalized Offline AI Assistant},
  year = {2025},
  publisher = {GitHub},
  url = {https://github.com/AdeelAliYousaf/PersonaVault}
}
```

---

## 📌 Status Updates

- **October 2025:** Initial architecture design completed
- **October 2025:** Frontend-Backend integration in progress
- **November 2025  - Feburary 2026:** Local LLM integration (planned)
- **March - May 2026 :** RAG pipeline implementation (planned)

---

<div align="center">

**Built with ❤️ and a commitment to privacy**

⭐ Star this repository if you believe in privacy-first AI!

</div>
