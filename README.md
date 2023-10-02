# static-serve

`static-serve` is a simple command-line tool for serving static files over HTTP. It allows you to quickly set up a local web server for serving files from a specified directory.

## Installation

Installation of `static-serve` is manual, and you can download one of the releases from the [Releases](https://github.com/ThibSrb/static-serve/releases) section of this repository.

Once you've downloaded the appropriate release for your system, you can follow these steps to install the tool:

1. Download the desired release binary for your platform.

2. Make the binary executable, if necessary, using the following command:
   
   ```bash
   chmod +x static-serve
   ```

3. Place the binary in a directory included in your system's PATH to make it accessible from anywhere in the terminal.

## Usage

You can use **`static-serve`** with the following command-line options:

```shell
static-serve [OPTIONS]
```

### Options

- `-p, --port <PORT>`: Specifies the port on which the server should listen (default: 3000).

- `-d, --directory <DIRECTORY>`: Sets the directory from which the server will serve files (default: ./).

- `-s, --suffix <SUFFIX>`: Specifies one or more suffixes to append to requested files when necessary.

- `-a, --allow-origin <ALLOW_ORIGIN>`: Specifies the allowed origin for Cross-Origin Resource Sharing (CORS) requests.

- `-c, --disable-compression`: Disables compression for served files, which can reduce CPU usage but may result in larger file transfers.

- `-f, --fallback-file <FALLBACK_FILE>`: Sets a fallback file to serve when a requested file is not found.

- `-n, --no-auto-index`: Prevents automatic appending of "index.html" to directory paths.

- `-h, --help`: Print help.



### Examples

1. Serve files from the current directory on port 8080:
   
   ```shell
   static-serve -p 8080
   ```
2. Serve files from a specific directory and allow CORS requests from a specific origin:
   
   ```shell
   static-serve -d /path/to/my/files -a https://example.com
   ```
3. Disable compression for served files:
   
   ```shell
   static-serve -c
   ```
4. Set a custom fallback file to serve when a requested file is not found:
   
   ```shell
   static-serve -f not_found.html
   ```

For more information, run `static-serve -h` to display the help message.


