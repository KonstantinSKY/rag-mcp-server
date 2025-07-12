# RAG MCP Server

A minimal Model Context Protocol (MCP) server built in Rust using the Rocket framework. This server provides a simple HTTP API for search functionality and is designed to be extended with RAG (Retrieval-Augmented Generation) capabilities.

## Features

- **HTTP API**: Simple REST endpoint for search queries
- **Rocket Framework**: Built using Rust's Rocket web framework
- **JSON Support**: Accepts and returns JSON payloads
- **Console Logging**: Prints received queries to console for debugging
- **Minimal Design**: Clean, focused implementation ready for extension

## API Endpoints

### POST /search

Accepts a JSON payload with a search query and returns a simple response.

**Request:**
```json
{
  "query": "your search query here"
}
```

**Response:**
```json
{
  "result": "ok"
}
```

## Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- No additional dependencies required

### Installation & Running

1. **Clone the repository:**
   ```bash
   git clone https://github.com/KonstantinSKY/rag-mcp-server.git
   cd rag-mcp-server
   ```

2. **Build and run the server:**
   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:8000`

3. **Test the server:**
   ```bash
   curl -X POST http://localhost:8000/search \
     -H "Content-Type: application/json" \
     -d '{"query": "test search query"}'
   ```

   **Expected response:**
   ```json
   {"result":"ok"}
   ```

   **Console output:**
   ```
   Received query: test search query
   ```

## Project Structure

```
rag_mcp_server/
├── src/
│   └── main.rs          # Main server implementation
├── promts/
│   └── p1.md           # Project specification
├── Cargo.toml          # Dependencies and project config
└── README.md           # This file
```

## Dependencies

- **rocket**: Web framework with JSON support
- **serde**: Serialization/deserialization
- **serde_json**: JSON handling
- **tokio**: Async runtime

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Start the server
cargo run

# In another terminal, test the endpoint
curl -X POST http://localhost:8000/search \
  -H "Content-Type: application/json" \
  -d '{"query": "hello world"}'
```

## Future Enhancements

This minimal server is designed to be extended with:

- **RAG Functionality**: Vector embeddings and similarity search
- **Document Storage**: Integration with vector databases
- **AI Model Integration**: Connection to language models
- **Authentication**: API key or token-based auth
- **Rate Limiting**: Request throttling and abuse protection
- **Model Context Protocol**: Full MCP specification compliance

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Created by [KonstantinSKY](https://github.com/KonstantinSKY)

## Links

- **Repository**: https://github.com/KonstantinSKY/rag-mcp-server
- **Issues**: https://github.com/KonstantinSKY/rag-mcp-server/issues
- **Rocket Framework**: https://rocket.rs/
- **Model Context Protocol**: https://modelcontextprotocol.io/