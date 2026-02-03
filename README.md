# Atlassian MCP Server for Zed

A Zed extension that connects to [Atlassian's Remote MCP Server](https://support.atlassian.com/atlassian-rovo-mcp-server/docs/getting-started-with-the-atlassian-remote-mcp-server/), enabling AI-assisted interactions with Jira, Confluence, and Compass.

## Features

- **Jira:** Search issues, create/modify tickets, bulk operations
- **Confluence:** Summarize pages, create documentation, navigate spaces
- **Compass:** Query service dependencies, create components

## Prerequisites

- [Node.js](https://nodejs.org/) v18 or later (for npx)
- An Atlassian Cloud site with Jira, Confluence, or Compass
- A modern browser for OAuth authentication

## Installation

Install from Zed Extensions: `zed://extension/atlassian-mcp-server`

Or search for "Atlassian MCP Server" in Zed's extension panel.

## Usage

1. Enable the context server in Zed's Agent Panel settings
2. When first connecting, complete the OAuth flow in your browser
3. Use natural language to interact with your Atlassian products

**Example queries:**
- "Search Jira for open bugs assigned to me"
- "Create a new ticket for the login issue"
- "Summarize the architecture documentation in Confluence"

## How It Works

This extension uses [mcp-remote](https://www.npmjs.com/package/mcp-remote) to bridge Zed's local MCP protocol with Atlassian's remote MCP endpoint at `https://mcp.atlassian.com/v1/mcp`.

Authentication is handled via OAuth 2.1 - all actions respect your existing Atlassian permissions.

## License

MIT
