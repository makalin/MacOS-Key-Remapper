# MacOS Key Remapper in Rust

A customizable key remapping utility for macOS that allows you to transform your Caps Lock key into a browser launcher and create custom text shortcuts using the Fn key.

## Features

- **Caps Lock Browser Launch**: Opens a configurable URL when the Caps Lock key is pressed
- **Custom Text Shortcuts**: Define text expansions triggered by Fn key combinations
- **JSON Configuration**: Easy-to-modify configuration file for personalizing shortcuts
- **Non-intrusive**: Runs in the background with minimal system impact

## Installation

1. Ensure you have Rust installed on your system. If not, install it using:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/makalin/MacOS-Key-Remapper.git
   cd MacOS-Key-Remapper
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. The executable will be available at `target/release/MacOS-Key-Remapper`

## Configuration

The application uses a configuration file located at `~/.mac-key-remapper.json`. If the file doesn't exist, it will be created automatically with default settings.

Example configuration:
```json
{
  "browser_url": "https://www.google.com",
  "text_shortcuts": [
    {
      "trigger": "h",
      "text": "Hello, World!"
    },
    {
      "trigger": "t",
      "text": "Thank you very much"
    }
  ]
}
```

### Configuration Options

- `browser_url`: The URL to open when Caps Lock is pressed
- `text_shortcuts`: An array of shortcuts with:
  - `trigger`: The character to type after pressing Fn
  - `text`: The text to insert when the shortcut is triggered

## Usage

1. Run the application:
   ```bash
   ./mac-key-remapper
   ```

2. Use the shortcuts:
   - Press Caps Lock to open your configured URL
   - Press Fn and enter a trigger character to expand configured text shortcuts

3. To stop the application, press Ctrl+C

## Dependencies

The application uses the following main dependencies:
- `enigo`: For keyboard control
- `serde`: For JSON serialization/deserialization
- `dirs`: For handling home directory paths
- `ctrlc`: For handling Ctrl+C signal
- `open`: For opening URLs in the default browser

## System Requirements

- macOS (tested on macOS 10.15 and later)
- Rust toolchain
- System accessibility permissions (required for keyboard monitoring)

## Security Note

This application requires accessibility permissions to monitor keyboard events. You'll need to grant these permissions in System Preferences > Security & Privacy > Privacy > Accessibility.

## How It Works

The application works by:
1. Creating an AppleScript to monitor key events
2. Writing key events to a temporary file
3. Processing these events to trigger appropriate actions
4. Using the Enigo library to simulate keyboard input for text expansion

## Troubleshooting

If the application isn't working:
1. Ensure accessibility permissions are granted
2. Check the configuration file format
3. Verify the temporary files are writable (`/tmp/key_events`)
4. Make sure no other applications are intercepting the same keys

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This script is licensed under the MIT License.

## Acknowledgments

- Thanks to the Rust community for the excellent crates used in this project
- Inspired by various key remapping utilities in the macOS ecosystem
