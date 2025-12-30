# Command Line Interface for `WLED`

A command-line interface for [WLED][WLED]! Control your WLED lighting directly from the terminal or via scripts.

## The Plan

CLI needs to know which IP address to talk to. Will need some sort of configuration file. Potentially look into mDNS?

### Commands

| Command | Subcommand         | Flags / Subcommands        | Description                         | Implemented |
| ------- | ------------------ | -------------------------- | ----------------------------------- | ----------- |
| `wled`  | `switch`/`power`   | `--on` (default) / `--off` | Toggle the power on or off          |             |
| `wled`  | `brightness <val>` |                            | Set the brightness (0-255)          |             |
| `wled`  | `preset <id>`      |                            | Trigger a specific saved preset     |             |
| `wled`  | `effect`           | `--list` / `--set`         | Change the current animation effect |             |
| `wled`  | `status`           |                            | Display current, power, info etc    |             |
| `wled`  | `info`             |                            | Display device info                 |             |
| `wled`  | `device`           | `--list` / `--new`         | Configure devices                   |             |

---

## 📃 License

The project is licensed under the [MIT License](./LICENSE).

[WLED]: https://kno.wled.ge
