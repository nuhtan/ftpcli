# File Transfer Protocol Command Line Interface

Much like the name would suggest this is a command line tool to facilitate transferring files remotely. This tool uses ssh and sftp for communication. This is essentially a wrapper over sftp.

# Goals for this project:
- UI for moving around the local and remote file systems.
    - Initial UI for input parameters.
    - Three boxes.
        - Left box is the current file system.
        - Middle box is the remote file system.
        - Right box is the current downloads.
- Thread for getting file system contents.
    - I think this can be a single thread for both local and remote.
- Mark multiple files for download.
    - Download using SFTP.
    - Single threaded by default.
    - Multithreaded as optional.
        - Using a threadpool.