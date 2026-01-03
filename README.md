# Command Line Interface for `WLED`

A command-line interface for [WLED][WLED]! Control your WLED lighting directly from the terminal or via scripts.

> [!WARNING]
> Work-in-Progress. Under Construction.

## The Plan

CLI needs to know which IP address to talk to. Will need some sort of configuration file. Potentially look into mDNS?

### Commands

| Command | Subcommand         | Flags / Subcommands         | Description                         | Implemented |
| ------- | ------------------ | --------------------------- | ----------------------------------- | ----------- |
| `wled`  | `switch`/`power`   | `""`, `on`, `off`, `toggle` | Toggle the power on or off          | ✅           |
| `wled`  | `brightness <val>` |                             | Set the brightness (0-255)          | ✅           |
| `wled`  | `preset <id>`      |                             | Trigger a specific saved preset     |             |
| `wled`  | `effects`          | `list`, `set`, `get`        | Change the current animation effect | ✅           |
| `wled`  | `palettes`         |                             | Change the current color palette    | ✅           |
| `wled`  | `status`           |                             | Display current, power, info etc    |             |
| `wled`  | `info`             |                             | Display device info                 |             |
| `wled`  | `device`           | `--list` / `--new`          | Configure devices                   |             |

---

## 🌟 Features

- Toggle WLED device power
- Adjust the brightness levels
- Set effects and palettes
- See WLED device information

## 📘 Usage

```sh
wled <command> [subcommand] [options]
```

### Commands

- `power`: Controls the power state of the WLED device.
- `brightness`: Adjust the brightness level of the WLED device.
- `effects`: Manage effects
- `palettes`: Manage color palettes

---

## 📃 License

The project is licensed under the [MIT License](./LICENSE).

[WLED]: https://kno.wled.ge
