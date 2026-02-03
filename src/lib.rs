use std::env;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
    settings::ContextServerSettings,
};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "0.1.30"; // Pinned version - 0.1.31 has known issues
const PACKAGE_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
const ATLASSIAN_MCP_URL: &str = "https://mcp.atlassian.com/v1/sse";

struct AtlassianMcpServer;

impl zed::Extension for AtlassianMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        // Install mcp-remote package if not already installed or wrong version
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        // Get the path to the proxy.js file
        let proxy_path = env::current_dir()
            .unwrap()
            .join(PACKAGE_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![proxy_path, ATLASSIAN_MCP_URL.to_string()],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: r#"# Atlassian MCP Server

This extension connects Zed to Atlassian's Remote MCP Server for Jira, Confluence, and Compass integration.

## Requirements

- Node.js v18 or later
- Active Atlassian Cloud account

## Setup

1. Enable the context server in Zed's Agent Panel settings
2. On first connection, a browser window will open for OAuth authentication
3. Authorize the connection with your Atlassian account
4. Return to Zed and start using Atlassian tools

## Available Tools

- **Jira**: Search issues, create/modify tickets, manage workflows
- **Confluence**: Search pages, create/edit documentation
- **Compass**: Query service dependencies

## Troubleshooting

If you encounter authentication issues:
1. Clear the auth cache: `rm -rf ~/.mcp-auth`
2. Restart Zed
3. Re-authenticate when prompted
"#.to_string(),
            default_settings: "{}".to_string(),
            settings_schema: "{}".to_string(),
        }))
    }
}

zed::register_extension!(AtlassianMcpServer);
