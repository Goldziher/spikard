```python
try:
    # Handler logic
    pass
except ValueError as e:
    # Maps to INVALID_ARGUMENT
    raise
except PermissionError as e:
    # Maps to PERMISSION_DENIED
    raise
except NotImplementedError as e:
    # Maps to UNIMPLEMENTED
    raise
except Exception as e:
    # Maps to INTERNAL
    raise
```

Mapping is automatic via FFI layer (`pyerr_to_grpc_status`).
