# load-striker

Performance testing tool that generates concurrent HTTP requests

## Docker

The tool can be run in Docker. Example:

```
docker run -e CONCURRENT_USERS=10 -e TARGETS_FILE=/data/targets.txt -v <local-targets-txt-dir>:/data thereisnospoon/load-striker:latest
```