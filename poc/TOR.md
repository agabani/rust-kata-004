# Documentation
https://2019.www.torproject.org/docs/tor-manual.html.en

# Configure Tor client

1. Create configuration file `~/torrc`

# Launch Tor client

1. Verify configuration and run
   ```shell
   tor --verify-config -f torrc && tor -f torrc
   ```

# Add new hidden service

1. Create hidden service directory with chmod `700`
1. Append `torrc` file with new configuration
    ```torrc
    HiddenServiceDir ~/tor-http-bin
    HiddenServicePort 80 127.0.0.1:8080
    ```
1. Verify configuration and reload
   ```shell
   tor --verify-config -f torrc && kill -1 $(ps -e -o pid,comm | grep tor | awk {'print $1'})
   ```
