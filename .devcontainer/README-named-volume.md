# Using Named Volumes with Dev Containers

This dev container is configured to use a named volume (`mrf-workspace`) instead of bind mounting your local files. This provides better performance, especially on Windows, but requires a different workflow.

## Initial Setup

When you first open this project in a dev container with a named volume:

1. **The workspace will be empty** - The named volume starts with no files
2. **Clone your repository** inside the container:
   ```bash
   cd /workspace
   git clone https://github.com/yourusername/mrf-rs.git .
   ```
   Note the `.` at the end to clone into the current directory.

3. **Run the initialization script** (if needed):
   ```bash
   .devcontainer/init-workspace.sh
   ```

## Benefits

- **Better Performance**: Named volumes are significantly faster than bind mounts on Windows
- **Isolation**: Your code lives entirely within Docker
- **Consistency**: The same environment across all platforms

## Important Notes

- Your source code is stored in the Docker volume `mrf-workspace`, not on your local filesystem
- The volume persists between container rebuilds
- To access files from your host, use VS Code's file explorer or terminal
- To backup your work, make sure to push changes to git regularly

## Managing the Volume

- **View volume details**: `docker volume inspect mrf-rs_devcontainer_mrf-workspace`
- **Remove volume** (⚠️ deletes all data): `docker volume rm mrf-rs_devcontainer_mrf-workspace`

## Switching Back to Bind Mounts

If you want to switch back to using bind mounts, modify `.devcontainer/docker-compose.yml`:

```yaml
volumes:
  - ..:/workspace:cached  # Instead of mrf-workspace:/workspace
```

Then remove the `mrf-workspace:` entry from the volumes section at the bottom of the file. 