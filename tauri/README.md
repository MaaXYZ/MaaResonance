# MaaZ Backend

This is the Rust backend for MaaZ.

For reference, the module structure is listed here.

- `commands` - This module contains the tauri commands definitions.
- `config` - This module contains the configuration for different tasks.
- `callback` - This is where callback from MAA Framework is handled.
- `maa` - This module contains the MAA Framework related FFI bindings.
- `queue` - This module contains the task queue implementation.
- `task` - This module contains the task definitions.
