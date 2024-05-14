# JSInjector

JSInjector is a powerful tool written in Rust that allows you to inject JavaScript into web pages. It uses the Chrome DevTools Protocol to connect to a running instance of Google Chrome and execute scripts on selected pages.

## Features

- **Script Execution**: Execute custom JavaScript on any open page in the browser.
- **Hot Reloading**: Automatically reload the script when it changes on disk.
- **Page Selection**: Select the target page from a list of all open pages.
- **Script Selection**: Choose the script to execute from a list of available scripts.
- **Browser Setup**: Automatically starts a new instance of Google Chrome with remote debugging enabled.
- **Server Scripting**: Serve scripts from a local server to have ability to use deno or node modules.

## Building

To build the project, you need to have Rust installed. You can then use Cargo, Rust's package manager, to build the project:

```sh
cargo build --release
```

This will create a new executable in the `target/release` directory.

## Usage

To use JSInjector, simply run the built executable. It will start a new instance of Google Chrome and present you with a list of open pages to choose from. After selecting a page, you can choose a script to execute on that page.

## Contributing

Contributions to JSInjector are welcome! Please submit a pull request or create an issue to discuss any changes you wish to make.
