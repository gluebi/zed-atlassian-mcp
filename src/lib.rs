use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

struct AtlassianMcpServer;

const ATLASSIAN_MCP_URL: &str = "https://mcp.atlassian.com/v1/mcp";

impl zed::Extension for AtlassianMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Ok(Command {
            command: "npx".to_string(),
            args: vec![
                "-y".to_string(),
                "mcp-remote".to_string(),
                ATLASSIAN_MCP_URL.to_string(),
            ],
            env: vec![],
        })
    }
}

zed::register_extension!(AtlassianMcpServer);
