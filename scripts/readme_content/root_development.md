Use the task runner to keep the workspace aligned with CI:

```bash
task setup
task build
task test
task lint
task readme:validate
```

The authoritative README set is template-driven from `scripts/readme_config.yaml`, `scripts/readme_templates/`, and `scripts/readme_content/`.
